use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CollaborationMode {
    Automatic,
    Interactive,
    Assisted,
    Explain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub decision_id: String,
    pub finding_id: Option<String>,
    pub objective_id: String,
    pub hypothesis_id: Option<String>,
    pub capability_name: String,
    pub tool_name: String,
    pub evidence_id: Option<String>,
    pub user_interaction: Option<String>,
    pub duration_ms: u64,
}

impl TimelineRecord {
    pub fn new(decision_id: &str, objective_id: &str, capability: &str, tool: &str) -> Self {
        Self {
            id: format!("tl-{}", &uuid::Uuid::new_v4().to_string()[..8]),
            timestamp: Utc::now(),
            decision_id: decision_id.to_string(),
            finding_id: None,
            objective_id: objective_id.to_string(),
            hypothesis_id: None,
            capability_name: capability.to_string(),
            tool_name: tool.to_string(),
            evidence_id: None,
            user_interaction: None,
            duration_ms: 0,
        }
    }
}
