use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::models::{
    AttackEdge, AttackNode, Evidence, Finding, FindingKind, JournalEntry, LogEntry, Priority, Recommendation,
    RecommendationStatus, Snapshot, TargetInfo, Task, TaskState,
};

#[derive(Clone)]
pub struct DatabaseManager {
    conn: Arc<Mutex<Connection>>,
}

impl DatabaseManager {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let path = db_path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create database directory structure")?;
        }
        let conn = Connection::open(path).context("Failed to open SQLite database")?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_tables()?;
        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Failed to open in-memory SQLite database")?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS targets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                ip TEXT NOT NULL UNIQUE,
                hostname TEXT,
                os TEXT,
                phase TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS findings (
                id TEXT PRIMARY KEY,
                target_ip TEXT NOT NULL,
                source_tool TEXT NOT NULL,
                kind_json TEXT NOT NULL,
                confidence REAL NOT NULL DEFAULT 0.9,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS recommendations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                recommended_tool TEXT NOT NULL,
                suggested_command TEXT NOT NULL,
                reasoning_json TEXT NOT NULL,
                confidence REAL NOT NULL,
                priority TEXT NOT NULL DEFAULT 'MEDIUM',
                status TEXT NOT NULL DEFAULT 'PENDING',
                target_phase TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS attack_nodes (
                id TEXT PRIMARY KEY,
                node_type TEXT NOT NULL,
                label TEXT NOT NULL,
                metadata_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS attack_edges (
                id TEXT PRIMARY KEY,
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relationship TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS evidence (
                id TEXT PRIMARY KEY,
                finding_id TEXT NOT NULL,
                tool_name TEXT NOT NULL,
                raw_output_path TEXT NOT NULL,
                checksum_sha256 TEXT NOT NULL,
                mime_type TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                target_name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                file_path TEXT NOT NULL,
                checksum TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                plugin_name TEXT NOT NULL,
                target_ip TEXT NOT NULL,
                command TEXT NOT NULL,
                state TEXT NOT NULL,
                progress_percentage INTEGER NOT NULL,
                current_operation TEXT NOT NULL,
                elapsed_seconds INTEGER NOT NULL,
                estimated_seconds INTEGER NOT NULL,
                cpu_usage REAL NOT NULL,
                memory_mb INTEGER NOT NULL,
                started_at TEXT,
                finished_at TEXT
            );

            CREATE TABLE IF NOT EXISTS decision_journal (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                decision TEXT NOT NULL,
                reason TEXT NOT NULL,
                confidence REAL NOT NULL,
                triggered_finding_ids_json TEXT NOT NULL,
                generated_command TEXT NOT NULL,
                user_action TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS logs (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                level TEXT NOT NULL,
                source TEXT NOT NULL,
                message TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS timeline_events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                phase TEXT NOT NULL,
                event_type TEXT NOT NULL,
                summary TEXT NOT NULL,
                details_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS rule_packs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                description TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                rule_count INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS workflow_state (
                target_ip TEXT PRIMARY KEY,
                active_template TEXT NOT NULL,
                current_phase TEXT NOT NULL,
                completion_percentage REAL NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS installed_tools (
                tool_name TEXT PRIMARY KEY,
                version TEXT NOT NULL,
                install_source TEXT NOT NULL,
                checksum TEXT NOT NULL,
                install_date TEXT NOT NULL,
                last_updated TEXT NOT NULL,
                status TEXT NOT NULL
            );
            ",
        )?;

        tracing::debug!("Database schema initialized with attack graph, evidence, snapshots, timeline, rule_packs, and installed_tools tables");
        Ok(())
    }

    pub fn save_target(&self, target: &TargetInfo) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO targets (name, ip, hostname, os, phase, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(ip) DO UPDATE SET phase=excluded.phase",
            params![
                target.name,
                target.ip,
                target.hostname,
                target.os,
                target.phase.to_string(),
                target.created_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_target(&self, ip: &str) -> Result<Option<TargetInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT name, ip, hostname, os, phase, created_at FROM targets WHERE ip = ?1")?;
        let mut rows = stmt.query_map(params![ip], |row| {
            let name: String = row.get(0)?;
            let ip: String = row.get(1)?;
            let hostname: Option<String> = row.get(2)?;
            let os: Option<String> = row.get(3)?;
            let phase_str: String = row.get(4)?;
            let created_str: String = row.get(5)?;

            let phase = match phase_str.as_str() {
                "Reconnaissance" => crate::models::Phase::Recon,
                "Enumeration" => crate::models::Phase::Enumeration,
                "Technology Detection" => crate::models::Phase::TechnologyDetection,
                "Vulnerability Discovery" => crate::models::Phase::VulnerabilityDiscovery,
                "Exploitation" => crate::models::Phase::Exploitation,
                "Privilege Escalation" => crate::models::Phase::PrivilegeEscalation,
                "Post Exploitation" => crate::models::Phase::PostExploitation,
                "Flag Collection" => crate::models::Phase::FlagCollection,
                _ => crate::models::Phase::Reporting,
            };

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            Ok(TargetInfo {
                name,
                ip,
                hostname,
                os,
                phase,
                created_at,
            })
        })?;

        if let Some(target) = rows.next() {
            Ok(Some(target?))
        } else {
            Ok(None)
        }
    }

    pub fn insert_finding(&self, finding: &Finding) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let kind_json = serde_json::to_string(&finding.kind)?;
        conn.execute(
            "INSERT INTO findings (id, target_ip, source_tool, kind_json, confidence, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                finding.id,
                finding.target_ip,
                finding.source_tool,
                kind_json,
                finding.confidence,
                finding.timestamp.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_findings(&self) -> Result<Vec<Finding>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, target_ip, source_tool, kind_json, confidence, timestamp FROM findings")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let target_ip: String = row.get(1)?;
            let source_tool: String = row.get(2)?;
            let kind_json: String = row.get(3)?;
            let confidence: f32 = row.get(4)?;
            let timestamp_str: String = row.get(5)?;

            let kind: FindingKind = serde_json::from_str(&kind_json).unwrap_or(FindingKind::Vulnerability {
                cve: None,
                name: "Unknown".into(),
                severity: "Low".into(),
                details: kind_json,
            });

            let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            Ok(Finding {
                id,
                target_ip,
                source_tool,
                kind,
                confidence,
                timestamp,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn insert_attack_node(&self, node: &AttackNode) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO attack_nodes (id, node_type, label, metadata_json) VALUES (?1, ?2, ?3, ?4)",
            params![node.id, node.node_type, node.label, node.metadata_json],
        )?;
        Ok(())
    }

    pub fn insert_attack_edge(&self, edge: &AttackEdge) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO attack_edges (id, source_id, target_id, relationship) VALUES (?1, ?2, ?3, ?4)",
            params![edge.id, edge.source_id, edge.target_id, edge.relationship],
        )?;
        Ok(())
    }

    pub fn save_evidence(&self, evidence: &Evidence) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO evidence (id, finding_id, tool_name, raw_output_path, checksum_sha256, mime_type, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                evidence.id,
                evidence.finding_id,
                evidence.tool_name,
                evidence.raw_output_path,
                evidence.checksum_sha256,
                evidence.mime_type,
                evidence.timestamp.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_evidence(&self) -> Result<Vec<Evidence>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, finding_id, tool_name, raw_output_path, checksum_sha256, mime_type, timestamp FROM evidence")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let finding_id: String = row.get(1)?;
            let tool_name: String = row.get(2)?;
            let raw_output_path: String = row.get(3)?;
            let checksum_sha256: String = row.get(4)?;
            let mime_type: String = row.get(5)?;
            let timestamp_str: String = row.get(6)?;

            let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            Ok(Evidence {
                id,
                finding_id,
                tool_name,
                raw_output_path,
                checksum_sha256,
                mime_type,
                timestamp,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn save_task(&self, task: &Task) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let started_at_str = task.started_at.map(|dt| dt.to_rfc3339());
        let finished_at_str = task.finished_at.map(|dt| dt.to_rfc3339());

        conn.execute(
            "INSERT OR REPLACE INTO tasks (id, plugin_name, target_ip, command, state, progress_percentage, current_operation, elapsed_seconds, estimated_seconds, cpu_usage, memory_mb, started_at, finished_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                task.id,
                task.plugin_name,
                task.target_ip,
                task.command,
                task.state.to_string(),
                task.progress_percentage,
                task.current_operation,
                task.elapsed_seconds,
                task.estimated_seconds,
                task.cpu_usage,
                task.memory_mb,
                started_at_str,
                finished_at_str
            ],
        )?;
        Ok(())
    }

    pub fn save_journal_entry(&self, entry: &JournalEntry) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let triggered_json = serde_json::to_string(&entry.triggered_finding_ids)?;

        conn.execute(
            "INSERT INTO decision_journal (id, timestamp, decision, reason, confidence, triggered_finding_ids_json, generated_command, user_action)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                entry.id,
                entry.timestamp.to_rfc3339(),
                entry.decision,
                entry.reason,
                entry.confidence,
                triggered_json,
                entry.generated_command,
                entry.user_action
            ],
        )?;
        Ok(())
    }

    pub fn get_journal_entries(&self) -> Result<Vec<JournalEntry>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, timestamp, decision, reason, confidence, triggered_finding_ids_json, generated_command, user_action FROM decision_journal")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let timestamp_str: String = row.get(1)?;
            let decision: String = row.get(2)?;
            let reason: String = row.get(3)?;
            let confidence: f32 = row.get(4)?;
            let triggered_json: String = row.get(5)?;
            let generated_command: String = row.get(6)?;
            let user_action: String = row.get(7)?;

            let triggered_finding_ids: Vec<String> = serde_json::from_str(&triggered_json).unwrap_or_default();
            let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            Ok(JournalEntry {
                id,
                timestamp,
                decision,
                reason,
                confidence,
                triggered_finding_ids,
                generated_command,
                user_action,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn save_recommendations(&self, recs: &[Recommendation]) -> Result<()> {
        for r in recs {
            self.insert_recommendation(r)?;
        }
        Ok(())
    }

    pub fn save_log(&self, log: &LogEntry) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO logs (id, timestamp, level, source, message) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                log.id,
                log.timestamp.to_rfc3339(),
                log.level,
                log.source,
                log.message
            ],
        )?;
        Ok(())
    }

    pub fn insert_recommendation(&self, rec: &Recommendation) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let reasoning_json = serde_json::to_string(&rec.reasoning)?;
        conn.execute(
            "INSERT OR REPLACE INTO recommendations (id, title, description, recommended_tool, suggested_command, reasoning_json, confidence, priority, status, target_phase)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                rec.id,
                rec.title,
                rec.description,
                rec.recommended_tool,
                rec.suggested_command,
                reasoning_json,
                rec.confidence,
                rec.priority.to_string(),
                rec.status.to_string(),
                rec.target_phase.to_string()
            ],
        )?;
        Ok(())
    }

    pub fn update_recommendation_status(&self, id: &str, status: RecommendationStatus) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE recommendations SET status = ?1 WHERE id = ?2",
            params![status.to_string(), id],
        )?;
        Ok(())
    }

    pub fn get_recommendations(&self) -> Result<Vec<Recommendation>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, description, recommended_tool, suggested_command, reasoning_json, confidence, priority, status, target_phase FROM recommendations")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let description: String = row.get(2)?;
            let recommended_tool: String = row.get(3)?;
            let suggested_command: String = row.get(4)?;
            let reasoning_json: String = row.get(5)?;
            let confidence: f32 = row.get(6)?;
            let priority_str: String = row.get(7)?;
            let status_str: String = row.get(8)?;
            let target_phase_str: String = row.get(9)?;

            let reasoning: Vec<String> = serde_json::from_str(&reasoning_json).unwrap_or_default();
            let priority = match priority_str.as_str() {
                "CRITICAL" => Priority::Critical,
                "HIGH" => Priority::High,
                "LOW" => Priority::Low,
                _ => Priority::Medium,
            };
            let status = match status_str.as_str() {
                "ACCEPTED" => RecommendationStatus::Accepted,
                "RUNNING" => RecommendationStatus::Running,
                "COMPLETED" => RecommendationStatus::Completed,
                "IGNORED" => RecommendationStatus::Ignored,
                "REJECTED" => RecommendationStatus::Rejected,
                "EXPIRED" => RecommendationStatus::Expired,
                _ => RecommendationStatus::Pending,
            };
            let target_phase = match target_phase_str.as_str() {
                "Reconnaissance" | "Recon" => crate::models::Phase::Recon,
                "Enumeration" => crate::models::Phase::Enumeration,
                "Technology Detection" => crate::models::Phase::TechnologyDetection,
                "Vulnerability Discovery" => crate::models::Phase::VulnerabilityDiscovery,
                "Exploitation" => crate::models::Phase::Exploitation,
                "Privilege Escalation" => crate::models::Phase::PrivilegeEscalation,
                "Post Exploitation" => crate::models::Phase::PostExploitation,
                "Flag Collection" => crate::models::Phase::FlagCollection,
                _ => crate::models::Phase::Reporting,
            };

            Ok(Recommendation {
                id,
                title,
                description,
                recommended_tool,
                suggested_command,
                reasoning,
                confidence,
                priority,
                status,
                target_phase,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn insert_timeline_event(&self, id: &str, phase: crate::models::Phase, event_type: &str, summary: &str, details_json: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO timeline_events (id, timestamp, phase, event_type, summary, details_json) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id,
                chrono::Utc::now().to_rfc3339(),
                phase.to_string(),
                event_type,
                summary,
                details_json
            ],
        )?;
        Ok(())
    }

    pub fn get_timeline_events(&self) -> Result<Vec<crate::models::TimelineEvent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, timestamp, phase, event_type, summary, details_json FROM timeline_events ORDER BY timestamp ASC")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let timestamp_str: String = row.get(1)?;
            let phase_str: String = row.get(2)?;
            let event_type: String = row.get(3)?;
            let summary: String = row.get(4)?;
            let details_json: String = row.get(5)?;

            let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            let phase = match phase_str.as_str() {
                "Reconnaissance" | "Recon" => crate::models::Phase::Recon,
                "Enumeration" => crate::models::Phase::Enumeration,
                "Technology Detection" => crate::models::Phase::TechnologyDetection,
                "Vulnerability Discovery" => crate::models::Phase::VulnerabilityDiscovery,
                "Exploitation" => crate::models::Phase::Exploitation,
                "Privilege Escalation" => crate::models::Phase::PrivilegeEscalation,
                "Post Exploitation" => crate::models::Phase::PostExploitation,
                "Flag Collection" => crate::models::Phase::FlagCollection,
                _ => crate::models::Phase::Reporting,
            };

            Ok(crate::models::TimelineEvent {
                id,
                timestamp,
                phase,
                event_type,
                summary,
                details_json,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn save_snapshot(&self, snap: &Snapshot) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO snapshots (id, target_name, created_at, file_path, checksum) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                snap.id,
                snap.target_name,
                snap.created_at.to_rfc3339(),
                snap.file_path,
                snap.checksum
            ],
        )?;
        Ok(())
    }

    pub fn get_snapshots(&self) -> Result<Vec<Snapshot>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, target_name, created_at, file_path, checksum FROM snapshots")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let target_name: String = row.get(1)?;
            let created_at_str: String = row.get(2)?;
            let file_path: String = row.get(3)?;
            let checksum: String = row.get(4)?;

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            Ok(Snapshot {
                id,
                target_name,
                created_at,
                file_path,
                checksum,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn delete_snapshot(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM snapshots WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_attack_nodes(&self) -> Result<Vec<AttackNode>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, node_type, label, metadata_json FROM attack_nodes")?;
        let rows = stmt.query_map([], |row| {
            Ok(AttackNode {
                id: row.get(0)?,
                node_type: row.get(1)?,
                label: row.get(2)?,
                metadata_json: row.get(3)?,
            })
        })?;
        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn get_attack_edges(&self) -> Result<Vec<AttackEdge>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, source_id, target_id, relationship FROM attack_edges")?;
        let rows = stmt.query_map([], |row| {
            Ok(AttackEdge {
                id: row.get(0)?,
                source_id: row.get(1)?,
                target_id: row.get(2)?,
                relationship: row.get(3)?,
            })
        })?;
        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, plugin_name, target_ip, command, state, progress_percentage, current_operation, elapsed_seconds, estimated_seconds, cpu_usage, memory_mb, started_at, finished_at FROM tasks")?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let plugin_name: String = row.get(1)?;
            let target_ip: String = row.get(2)?;
            let command: String = row.get(3)?;
            let state_str: String = row.get(4)?;
            let progress_percentage: u8 = row.get(5)?;
            let current_operation: String = row.get(6)?;
            let elapsed_seconds: u64 = row.get(7)?;
            let estimated_seconds: u64 = row.get(8)?;
            let cpu_usage: f32 = row.get(9)?;
            let memory_mb: u64 = row.get(10)?;

            let state = match state_str.as_str() {
                "RUNNING" => TaskState::Running,
                "PAUSED" => TaskState::Paused,
                "CANCELLED" => TaskState::Cancelled,
                "FAILED" => TaskState::Failed,
                "COMPLETED" => TaskState::Completed,
                "TIMEOUT" => TaskState::Timeout,
                "SKIPPED" => TaskState::Skipped,
                _ => TaskState::Queued,
            };

            Ok(Task {
                id,
                plugin_name,
                target_ip,
                command,
                state,
                progress_percentage,
                current_operation,
                elapsed_seconds,
                estimated_seconds,
                cpu_usage,
                memory_mb,
                started_at: None,
                finished_at: None,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }
        Ok(list)
    }
}

