use anyhow::{anyhow, Result};
use chrono::Utc;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::models::Snapshot;

pub struct SnapshotManager;

impl SnapshotManager {
    pub fn create_snapshot(target_name: &str, workspace_path: impl AsRef<Path>) -> Result<Snapshot> {
        let id = Uuid::new_v4().to_string();
        let snapshots_dir = workspace_path.as_ref().join("snapshots");
        fs::create_dir_all(&snapshots_dir)?;

        let backup_path = snapshots_dir.join(format!("snapshot_{}.bak", &id[..8]));
        
        let content = format!(
            "Zephyx Workspace Snapshot\nID: {}\nTarget: {}\nTimestamp: {}\nSchema Version: 0.3.0\n",
            id, target_name, Utc::now().to_rfc3339()
        );
        fs::write(&backup_path, &content)?;

        let checksum = format!("{:x}", content.as_bytes().iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64 * 31)));

        Ok(Snapshot {
            id,
            target_name: target_name.to_string(),
            created_at: Utc::now(),
            file_path: backup_path.to_string_lossy().to_string(),
            checksum,
        })
    }

    pub fn restore_snapshot(snapshot_path: impl AsRef<Path>) -> Result<()> {
        if !snapshot_path.as_ref().exists() {
            return Err(anyhow!("Snapshot file does not exist at {:?}", snapshot_path.as_ref()));
        }
        let content = fs::read_to_string(snapshot_path)?;
        if !content.contains("Zephyx Workspace Snapshot") {
            return Err(anyhow!("Invalid snapshot file header or corrupted backup"));
        }
        Ok(())
    }
}
