pub mod central;
pub use central::CentralWorkspaceManager;

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;
use tracing::info;

use crate::db::DatabaseManager;
use crate::models::{Phase, TargetInfo};

pub struct WorkspaceManager {
    pub base_dir: PathBuf,
    pub target: TargetInfo,
    pub db: DatabaseManager,
}

impl WorkspaceManager {
    pub fn init(target_name: &str, target_ip: &str, base_path: impl AsRef<Path>) -> Result<Self> {
        let root = base_path.as_ref().join(".zpx").join(target_name);
        fs::create_dir_all(&root).context("Failed to create workspace root directory")?;

        let dirs = ["reports", "notes", "loot", "downloads", "commands"];
        for d in dirs {
            fs::create_dir_all(root.join(d))?;
        }

        let db_path = root.join("timeline.db");
        let db = DatabaseManager::new(db_path)?;

        let target = TargetInfo {
            name: target_name.to_string(),
            ip: target_ip.to_string(),
            hostname: None,
            os: None,
            phase: Phase::Recon,
            created_at: Utc::now(),
        };

        db.save_target(&target)?;

        info!(target_name, target_ip, "Workspace initialized successfully");

        Ok(Self {
            base_dir: root,
            target,
            db,
        })
    }

    pub fn get_notes_path(&self) -> PathBuf {
        self.base_dir.join("notes").join("notes.md")
    }

    pub fn get_loot_dir(&self) -> PathBuf {
        self.base_dir.join("loot")
    }
}
