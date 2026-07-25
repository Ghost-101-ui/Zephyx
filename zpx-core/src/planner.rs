use serde::{Deserialize, Serialize};
use crate::context::TargetContext;
use crate::decision::{DecisionAction, DecisionEngine};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_number: usize,
    pub name: String,
    pub command: String,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicWorkflowPlan {
    pub target_ip: String,
    pub plan_name: String,
    pub steps: Vec<ExecutionStep>,
}

pub struct WorkflowPlanner;

impl WorkflowPlanner {
    pub fn build_plan(ctx: &TargetContext) -> DynamicWorkflowPlan {
        let mut steps = Vec::new();

        steps.push(ExecutionStep {
            step_number: 1,
            name: "Initial Port & Service Scan".into(),
            command: format!("nmap -sCV -p- {}", ctx.target_ip),
            expected_outcome: "Discover exposed TCP services".into(),
        });

        if let Ok(outcome) = DecisionEngine::evaluate(ctx) {
            if let DecisionAction::SelectCapability { capability_name, command, .. } = outcome.action {
                steps.push(ExecutionStep {
                    step_number: 2,
                    name: format!("Targeted Capability Execution ({})", capability_name),
                    command,
                    expected_outcome: format!("Execute {}", capability_name),
                });
            }
        }

        steps.push(ExecutionStep {
            step_number: steps.len() + 1,
            name: "Artifact & Evidence Synchronization".into(),
            command: format!("zpx export --target {}", ctx.target_ip),
            expected_outcome: "Export final session assessment report".into(),
        });

        DynamicWorkflowPlan {
            target_ip: ctx.target_ip.clone(),
            plan_name: format!("Adaptive Workflow Plan for {}", ctx.target_ip),
            steps,
        }
    }
}
