use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::models::Evidence;

pub struct EvidenceManager;

impl EvidenceManager {
    pub fn record_evidence(
        finding_id: &str,
        tool_name: &str,
        raw_output_content: &str,
        dest_dir: impl AsRef<Path>,
    ) -> Result<Evidence> {
        let id = Uuid::new_v4().to_string();
        let evidence_dir = dest_dir.as_ref().join("evidence");
        fs::create_dir_all(&evidence_dir)?;

        let file_path = evidence_dir.join(format!("evidence_{}.log", &id[..8]));
        fs::write(&file_path, raw_output_content)?;

        // SHA256 checksum simulation / length hashing
        let checksum = format!("{:x}", raw_output_content.as_bytes().iter().fold(0u64, |acc, &x| acc.wrapping_add(x as u64)));

        let mime = if raw_output_content.trim_start().starts_with("<?xml") || raw_output_content.trim_start().starts_with("<nmaprun") {
            "application/xml"
        } else if raw_output_content.trim_start().starts_with('{') || raw_output_content.trim_start().starts_with('[') {
            "application/json"
        } else {
            "text/plain"
        };

        Ok(Evidence {
            id,
            finding_id: finding_id.to_string(),
            tool_name: tool_name.to_string(),
            raw_output_path: file_path.to_string_lossy().to_string(),
            checksum_sha256: checksum,
            mime_type: mime.into(),
            timestamp: Utc::now(),
        })
    }
}
