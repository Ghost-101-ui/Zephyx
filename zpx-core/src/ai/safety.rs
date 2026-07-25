use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedAiOperation {
    Explain,
    Summarize,
    Recommend,
    Compare,
    Reason,
}

pub struct AiSafetyLayer;

impl AiSafetyLayer {
    pub fn is_allowed(op: &AllowedAiOperation) -> bool {
        match op {
            AllowedAiOperation::Explain
            | AllowedAiOperation::Summarize
            | AllowedAiOperation::Recommend
            | AllowedAiOperation::Compare
            | AllowedAiOperation::Reason => true,
        }
    }

    pub fn sanitize_prompt(raw: &str) -> String {
        raw.trim().to_string()
    }
}
