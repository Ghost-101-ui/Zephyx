use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::workspace::CentralWorkspaceManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub id: String,
    pub name: String,
    pub target_ip: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: String, // Active, Completed, Paused
    pub active_profile: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub metadata: SessionMetadata,
    pub base_dir: PathBuf,
    pub artifacts_dir: PathBuf,
    pub evidence_dir: PathBuf,
    pub reports_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub exports_dir: PathBuf,
}

pub struct SessionManager {
    central_ws: CentralWorkspaceManager,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        let central_ws = CentralWorkspaceManager::init()?;
        Ok(Self { central_ws })
    }

    pub fn create_session(&self, name: &str, target_ip: &str) -> Result<Session> {
        let id = format!("session-{}", &Uuid::new_v4().to_string()[..8]);
        let base_dir = self.central_ws.sessions_dir.join(&id);

        let artifacts_dir = base_dir.join("artifacts");
        let evidence_dir = base_dir.join("evidence");
        let reports_dir = base_dir.join("reports");
        let logs_dir = base_dir.join("logs");
        let exports_dir = base_dir.join("exports");

        let dirs = [&base_dir, &artifacts_dir, &evidence_dir, &reports_dir, &logs_dir, &exports_dir];
        for d in &dirs {
            fs::create_dir_all(d)?;
        }

        let metadata = SessionMetadata {
            id: id.clone(),
            name: name.to_string(),
            target_ip: target_ip.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: "Active".to_string(),
            active_profile: "default".to_string(),
        };

        let meta_file = base_dir.join("metadata.json");
        let json = serde_json::to_string_pretty(&metadata)?;
        fs::write(meta_file, json)?;

        Ok(Session {
            metadata,
            base_dir,
            artifacts_dir,
            evidence_dir,
            reports_dir,
            logs_dir,
            exports_dir,
        })
    }

    pub fn list_sessions(&self) -> Result<Vec<SessionMetadata>> {
        let mut list = Vec::new();
        let sessions_root = &self.central_ws.sessions_dir;

        if sessions_root.exists() {
            for entry in fs::read_dir(sessions_root)? {
                let entry = entry?;
                let path = entry.path();
                let meta_path = path.join("metadata.json");
                if meta_path.exists() {
                    if let Ok(content) = fs::read_to_string(meta_path) {
                        if let Ok(meta) = serde_json::from_str::<SessionMetadata>(&content) {
                            list.push(meta);
                        }
                    }
                }
            }
        }

        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(list)
    }

    pub fn resume_session(&self, session_id: &str) -> Result<Session> {
        let base_dir = self.central_ws.sessions_dir.join(session_id);
        let meta_file = base_dir.join("metadata.json");

        if !meta_file.exists() {
            return Err(anyhow!("Session '{}' not found at {:?}", session_id, meta_file));
        }

        let content = fs::read_to_string(&meta_file)?;
        let mut metadata: SessionMetadata = serde_json::from_str(&content)?;

        metadata.updated_at = Utc::now();
        metadata.status = "Active".to_string();
        fs::write(&meta_file, serde_json::to_string_pretty(&metadata)?)?;

        Ok(Session {
            metadata,
            base_dir: base_dir.clone(),
            artifacts_dir: base_dir.join("artifacts"),
            evidence_dir: base_dir.join("evidence"),
            reports_dir: base_dir.join("reports"),
            logs_dir: base_dir.join("logs"),
            exports_dir: base_dir.join("exports"),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub session_name: String,
    pub target_ip: String,
    pub active_workflow: Option<String>,
    pub current_phase: crate::models::Phase,
    pub active_profile: String,
}

impl SessionContext {
    pub fn new(session: &Session, target_ip: &str) -> Self {
        Self {
            session_id: session.metadata.id.clone(),
            session_name: session.metadata.name.clone(),
            target_ip: target_ip.to_string(),
            active_workflow: None,
            current_phase: crate::models::Phase::Recon,
            active_profile: session.metadata.active_profile.clone(),
        }
    }
}
