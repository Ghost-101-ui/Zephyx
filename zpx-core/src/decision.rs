use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::TargetContext;
use crate::explainability::DecisionExplanation;
use crate::heuristics::HeuristicEngine;
use crate::models::{Phase, Priority, Recommendation, RecommendationStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionAction {
    ContinueWorkflow,
    RetryStep { step_id: String },
    SelectCapability { capability_name: String, target_ip: String, command: String },
    SwitchTools { from_tool: String, to_tool: String },
    PauseWorkflow { reason: String },
    RequestUserConfirmation { prompt: String },
    GenerateExplanation { explanation: DecisionExplanation },
    Escalate { target_phase: Phase },
    TerminateWorkflow { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOutcome {
    pub decision_id: String,
    pub action: DecisionAction,
    pub explanation: DecisionExplanation,
    pub recommendation: Recommendation,
}

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn evaluate(ctx: &TargetContext) -> Result<DecisionOutcome> {
        let heuristics = HeuristicEngine::evaluate(ctx);

        let (action, title, desc, tool, cmd, priority, reasoning, confidence, rule) = if let Some(top) = heuristics.first() {
            let action = DecisionAction::SelectCapability {
                capability_name: top.recommended_capability.clone(),
                target_ip: ctx.target_ip.clone(),
                command: format!("zpx run {} --target {}", top.recommended_capability, ctx.target_ip),
            };

            (
                action,
                top.name.clone(),
                top.reasoning.clone(),
                top.recommended_capability.clone(),
                format!("zpx run {} --target {}", top.recommended_capability, ctx.target_ip),
                Priority::High,
                vec![top.reasoning.clone()],
                top.confidence,
                "HeuristicRuleMatch".to_string(),
            )
        } else {
            let action = DecisionAction::ContinueWorkflow;
            (
                action,
                "Initial Target Discovery".to_string(),
                "Perform baseline network port and service scanning".to_string(),
                "nmap".to_string(),
                format!("nmap -sCV -p- {}", ctx.target_ip),
                Priority::Medium,
                vec!["No specific web/SMB indicators yet; run full TCP scan".to_string()],
                0.80,
                "DefaultReconStrategy".to_string(),
            )
        };

        let explanation = DecisionExplanation {
            decision_title: title.clone(),
            primary_reason: desc.clone(),
            confidence_score: confidence,
            supporting_evidence: reasoning.clone(),
            deterministic_rule: rule,
        };

        let rec = Recommendation {
            id: Uuid::new_v4().to_string(),
            title,
            description: desc,
            recommended_tool: tool,
            suggested_command: cmd,
            reasoning,
            confidence,
            priority,
            status: RecommendationStatus::Pending,
            target_phase: ctx.active_phase.clone(),
        };

        Ok(DecisionOutcome {
            decision_id: Uuid::new_v4().to_string(),
            action,
            explanation,
            recommendation: rec,
        })
    }
}
