use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tracing::info;

use crate::models::{Task, TaskState};

#[derive(Debug, Clone)]
pub struct SchedulerStatus {
    pub queued_count: usize,
    pub running_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub max_concurrency: usize,
}

pub struct Scheduler {
    queue: Arc<Mutex<VecDeque<Task>>>,
    active_tasks: Arc<Mutex<Vec<Task>>>,
    completed_tasks: Arc<Mutex<Vec<Task>>>,
    max_concurrency: usize,
}

impl Scheduler {
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            active_tasks: Arc::new(Mutex::new(Vec::new())),
            completed_tasks: Arc::new(Mutex::new(Vec::new())),
            max_concurrency,
        }
    }

    pub fn schedule_task(&self, mut task: Task) {
        task.state = TaskState::Queued;
        info!(task_id = %task.id, command = %task.command, "Task enqueued into Scheduler");
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(task);
    }

    pub fn poll_next_task(&self) -> Option<Task> {
        let mut active = self.active_tasks.lock().unwrap();
        if active.len() >= self.max_concurrency {
            return None;
        }

        let mut queue = self.queue.lock().unwrap();
        if let Some(mut task) = queue.pop_front() {
            task.state = TaskState::Running;
            task.started_at = Some(chrono::Utc::now());
            active.push(task.clone());
            Some(task)
        } else {
            None
        }
    }

    pub fn complete_task(&self, task_id: &str, success: bool) {
        let mut active = self.active_tasks.lock().unwrap();
        if let Some(pos) = active.iter().position(|t| t.id == task_id) {
            let mut task = active.remove(pos);
            task.state = if success { TaskState::Completed } else { TaskState::Failed };
            task.finished_at = Some(chrono::Utc::now());
            let mut completed = self.completed_tasks.lock().unwrap();
            completed.push(task);
        }
    }

    pub fn status(&self) -> SchedulerStatus {
        let queued = self.queue.lock().unwrap().len();
        let running = self.active_tasks.lock().unwrap().len();
        let completed_list = self.completed_tasks.lock().unwrap();
        let completed = completed_list.iter().filter(|t| t.state == TaskState::Completed).count();
        let failed = completed_list.iter().filter(|t| t.state == TaskState::Failed).count();

        SchedulerStatus {
            queued_count: queued,
            running_count: running,
            completed_count: completed,
            failed_count: failed,
            max_concurrency: self.max_concurrency,
        }
    }
}
