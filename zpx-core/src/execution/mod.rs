use anyhow::{Context, Result};
use std::process::Stdio;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tracing::{error, info};
use uuid::Uuid;

use crate::db::DatabaseManager;
use crate::events::{EventBus, SystemEvent};
use crate::models::{LogEntry, Task, TaskState};

pub struct ExecutionEngine {
    db: DatabaseManager,
    event_bus: EventBus,
}

impl ExecutionEngine {
    pub fn new(db: DatabaseManager, event_bus: EventBus) -> Self {
        Self { db, event_bus }
    }

    pub async fn run_task(
        &self,
        plugin_name: String,
        target_ip: String,
        raw_cmd: String,
        args: Vec<String>,
    ) -> Result<Task> {
        let task_id = Uuid::new_v4().to_string();
        let full_command = format!("{} {}", raw_cmd, args.join(" "));

        let mut task = Task {
            id: task_id.clone(),
            plugin_name: plugin_name.clone(),
            target_ip: target_ip.clone(),
            command: full_command.clone(),
            state: TaskState::Starting,
            progress_percentage: 0,
            current_operation: "Initializing subprocess".into(),
            elapsed_seconds: 0,
            estimated_seconds: 60,
            cpu_usage: 0.0,
            memory_mb: 0,
            started_at: Some(chrono::Utc::now()),
            finished_at: None,
        };

        self.db.save_task(&task)?;
        self.event_bus.publish(SystemEvent::ProcessStarted {
            tool_name: plugin_name.clone(),
            command: full_command.clone(),
        });

        info!(task_id = %task.id, command = %full_command, "Launching process in execution engine");

        let resolved_binary = match crate::tool_manager::ToolManager::new() {
            Ok(tm) => tm.resolve(&raw_cmd).unwrap_or(raw_cmd.clone()),
            Err(_) => raw_cmd.clone(),
        };

        let mut child = match Command::new(&resolved_binary)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                task.state = TaskState::Failed;
                task.current_operation = format!("Failed to spawn executable: {}", e);
                task.finished_at = Some(chrono::Utc::now());
                self.db.save_task(&task)?;

                let err_log = LogEntry {
                    id: Uuid::new_v4().to_string(),
                    timestamp: chrono::Utc::now(),
                    level: "ERROR".into(),
                    source: plugin_name.clone(),
                    message: format!("Binary execution error: {}", e),
                };
                self.db.save_log(&err_log)?;
                self.event_bus.publish(SystemEvent::LogEmitted(err_log));
                return Err(e.into());
            }
        };

        task.state = TaskState::Running;
        task.current_operation = "Capturing stdout/stderr output stream".into();
        task.progress_percentage = 25;
        self.db.save_task(&task)?;

        let stdout = child.stdout.take().context("Failed to capture stdout pipe")?;
        let stderr = child.stderr.take().context("Failed to capture stderr pipe")?;

        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();

        let plugin_name_clone = plugin_name.clone();
        let db_clone = self.db.clone();
        let event_bus_clone = self.event_bus.clone();

        tokio::spawn(async move {
            while let Ok(Some(line)) = stdout_reader.next_line().await {
                let log = LogEntry {
                    id: Uuid::new_v4().to_string(),
                    timestamp: chrono::Utc::now(),
                    level: "INFO".into(),
                    source: plugin_name_clone.clone(),
                    message: line,
                };
                let _ = db_clone.save_log(&log);
                event_bus_clone.publish(SystemEvent::LogEmitted(log));
            }
        });

        let plugin_name_clone2 = plugin_name.clone();
        let db_clone2 = self.db.clone();
        let event_bus_clone2 = self.event_bus.clone();

        tokio::spawn(async move {
            while let Ok(Some(line)) = stderr_reader.next_line().await {
                let log = LogEntry {
                    id: Uuid::new_v4().to_string(),
                    timestamp: chrono::Utc::now(),
                    level: "WARN".into(),
                    source: plugin_name_clone2.clone(),
                    message: line,
                };
                let _ = db_clone2.save_log(&log);
                event_bus_clone2.publish(SystemEvent::LogEmitted(log));
            }
        });

        let start_time = Instant::now();
        let status = child.wait().await?;
        let elapsed = start_time.elapsed().as_secs();

        task.elapsed_seconds = elapsed;
        task.progress_percentage = 100;
        task.finished_at = Some(chrono::Utc::now());

        if status.success() {
            task.state = TaskState::Completed;
            task.current_operation = "Execution finished successfully".into();
            info!(task_id = %task.id, elapsed, "Subprocess completed cleanly");
        } else {
            task.state = TaskState::Failed;
            task.current_operation = format!("Exit code: {}", status.code().unwrap_or(-1));
            error!(task_id = %task.id, code = ?status.code(), "Subprocess exited with non-zero code");
        }

        self.db.save_task(&task)?;
        self.event_bus.publish(SystemEvent::ProcessFinished {
            tool_name: plugin_name,
            exit_code: status.code().unwrap_or(-1),
        });

        Ok(task)
    }
}
