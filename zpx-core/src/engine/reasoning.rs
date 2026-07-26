use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningTrace {
    pub trace_id: String,
    pub timestamp: DateTime<Utc>,
    pub current_objective_id: String,
    pub hypotheses_evaluated: Vec<String>,
    pub selected_capability: String,
    pub resolved_tool: String,
    pub justification: String,
    pub expected_outcome: String,
    pub confidence_score: f32,
}

impl ReasoningTrace {
    pub fn new(
        objective_id: &str,
        hypotheses: Vec<String>,
        capability: &str,
        tool: &str,
        justification: &str,
        expected_outcome: &str,
        confidence: f32,
    ) -> Self {
        Self {
            trace_id: format!("trace-{}", &Uuid::new_v4().to_string()[..8]),
            timestamp: Utc::now(),
            current_objective_id: objective_id.to_string(),
            hypotheses_evaluated: hypotheses,
            selected_capability: capability.to_string(),
            resolved_tool: tool.to_string(),
            justification: justification.to_string(),
            expected_outcome: expected_outcome.to_string(),
            confidence_score: confidence,
        }
    }
}
