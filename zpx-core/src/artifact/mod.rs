use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use sha2::{Digest, Sha256};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedArtifact {
    pub id: String,
    pub session_id: String,
    pub name: String,
    pub artifact_type: String, // XML, JSON, HTML, PCAP, Loot, Hash
    pub file_path: String,
    pub checksum_sha256: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

pub struct ArtifactStore;

impl ArtifactStore {
    pub fn calculate_checksum(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    pub fn create_artifact(
        session_id: &str,
        name: &str,
        artifact_type: &str,
        content: &[u8],
        dest_dir: impl AsRef<Path>,
        tags: Vec<String>,
    ) -> Result<ManagedArtifact> {
        let id = format!("art-{}", &Uuid::new_v4().to_string()[..8]);
        let file_name = format!("{}_{}", id, name);
        let target_path = dest_dir.as_ref().join(&file_name);

        fs::create_dir_all(dest_dir.as_ref())?;
        fs::write(&target_path, content)?;

        let checksum_sha256 = Self::calculate_checksum(content);
        let mime_type = match artifact_type.to_lowercase().as_str() {
            "xml" => "application/xml",
            "json" => "application/json",
            "html" => "text/html",
            "pcap" => "application/vnd.tcpdump.pcap",
            _ => "text/plain",
        };

        let artifact = ManagedArtifact {
            id,
            session_id: session_id.to_string(),
            name: name.to_string(),
            artifact_type: artifact_type.to_string(),
            file_path: target_path.to_string_lossy().to_string(),
            checksum_sha256,
            mime_type: mime_type.into(),
            size_bytes: content.len() as u64,
            created_at: Utc::now(),
            tags,
        };

        Ok(artifact)
    }

    pub fn export_artifact(artifact: &ManagedArtifact, output_dir: impl AsRef<Path>) -> Result<PathBuf> {
        let source_path = Path::new(&artifact.file_path);
        if !source_path.exists() {
            return Err(anyhow!("Artifact file does not exist at {:?}", source_path));
        }

        let dest_path = output_dir.as_ref().join(&artifact.name);
        fs::copy(source_path, &dest_path)?;
        Ok(dest_path)
    }
}
