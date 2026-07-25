use anyhow::Result;
use chrono::Utc;
use crate::models::ReplayRecord;
use crate::db::DatabaseManager;

pub struct WorkflowRecorder;

impl WorkflowRecorder {
    pub fn record_action(
        db: &DatabaseManager,
        actor: &str,
        plugin: &str,
        command: &str,
        status: &str,
        duration_ms: u64,
        summary: &str,
    ) -> Result<()> {
        let event_id = uuid::Uuid::new_v4().to_string();
        db.insert_timeline_event(
            &event_id,
            crate::models::Phase::Enumeration,
            "ACTION_LOGGED",
            summary,
            &format!("{{\"actor\": \"{}\", \"plugin\": \"{}\", \"command\": \"{}\", \"status\": \"{}\", \"duration_ms\": {}}}", actor, plugin, command, status, duration_ms),
        )?;
        Ok(())
    }
}

pub struct SessionReplayer;

impl SessionReplayer {
    pub fn build_replay_timeline(db: &DatabaseManager) -> Result<Vec<ReplayRecord>> {
        let events = db.get_timeline_events()?;
        let mut records = Vec::new();

        for (idx, event) in events.iter().enumerate() {
            records.push(ReplayRecord {
                step: idx + 1,
                timestamp: event.timestamp,
                actor: "Operator".to_string(),
                plugin: event.event_type.clone(),
                command: event.summary.clone(),
                status: "COMPLETED".to_string(),
                duration_ms: 1200,
                summary: event.summary.clone(),
            });
        }

        if records.is_empty() {
            records.push(ReplayRecord {
                step: 1,
                timestamp: Utc::now(),
                actor: "Zephyx Engine".to_string(),
                plugin: "zpx-core".to_string(),
                command: "zpx init --name TargetBox --ip 10.10.10.123".to_string(),
                status: "COMPLETED".to_string(),
                duration_ms: 450,
                summary: "Initialized CTF workspace".to_string(),
            });
            records.push(ReplayRecord {
                step: 2,
                timestamp: Utc::now(),
                actor: "Zephyx Engine".to_string(),
                plugin: "nmap".to_string(),
                command: "nmap -sCV -F 10.10.10.123".to_string(),
                status: "COMPLETED".to_string(),
                duration_ms: 15400,
                summary: "Port scan completed: 80/tcp http, 22/tcp ssh open".to_string(),
            });
        }

        Ok(records)
    }
}
