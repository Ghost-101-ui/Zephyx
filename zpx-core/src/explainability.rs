use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionExplanation {
    pub decision_title: String,
    pub primary_reason: String,
    pub confidence_score: f32,
    pub supporting_evidence: Vec<String>,
    pub deterministic_rule: String,
}

pub struct ExplainabilityEngine;

impl ExplainabilityEngine {
    pub fn explain(
        title: &str,
        reason: &str,
        confidence: f32,
        evidence: &[&str],
        rule: &str,
    ) -> DecisionExplanation {
        DecisionExplanation {
            decision_title: title.to_string(),
            primary_reason: reason.to_string(),
            confidence_score: confidence,
            supporting_evidence: evidence.iter().map(|s| s.to_string()).collect(),
            deterministic_rule: rule.to_string(),
        }
    }
}
