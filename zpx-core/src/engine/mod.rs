pub mod hypothesis;
pub mod objective;
pub mod reasoning;
pub mod strategy;

pub use hypothesis::{Hypothesis, HypothesisEngine, HypothesisStatus};
pub use objective::{Objective, ObjectiveEngine, ObjectiveState, ObjectiveType};
pub use reasoning::ReasoningTrace;
pub use strategy::{Strategy, StrategyPlanner};

use crate::models::{Finding, Priority, Recommendation, RecommendationStatus};

pub struct RuleEngine;

impl RuleEngine {
    pub fn evaluate(findings: &[Finding], target_ip: &str) -> Vec<Recommendation> {
        let mut recs = Vec::new();
        let has_port_80 = findings.iter().any(|f| match &f.kind {
            crate::models::FindingKind::Port { port, .. } => *port == 80 || *port == 443,
            _ => false,
        });

        if has_port_80 {
            recs.push(Recommendation {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Run Web Directory Fuzzing".into(),
                description: "Exposed HTTP port detected; fuzz common endpoints".into(),
                recommended_tool: "ffuf".into(),
                suggested_command: format!("ffuf -u http://{}/FUZZ -w /usr/share/wordlists/dirb/common.txt", target_ip),
                reasoning: vec!["HTTP/HTTPS port open".into()],
                confidence: 0.90,
                priority: Priority::High,
                status: RecommendationStatus::Pending,
                target_phase: crate::models::Phase::Enumeration,
            });
        }
        recs
    }
}
