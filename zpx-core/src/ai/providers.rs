use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPromptRequest {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPromptResponse {
    pub text: String,
    pub provider: String,
    pub model: String,
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    fn provider_name(&self) -> &str;
    fn is_available(&self) -> bool;
    async fn generate(&self, request: AiPromptRequest) -> Result<AiPromptResponse>;
}

pub struct MockAiProvider;

#[async_trait]
impl AiProvider for MockAiProvider {
    fn provider_name(&self) -> &str {
        "offline-mock"
    }

    fn is_available(&self) -> bool {
        true
    }

    async fn generate(&self, request: AiPromptRequest) -> Result<AiPromptResponse> {
        Ok(AiPromptResponse {
            text: format!("[Advisory AI Reasoning Response]: Analysis complete for prompt snippet '{}'", &request.prompt[..request.prompt.len().min(40)]),
            provider: "offline-mock".into(),
            model: "deterministic-fallback".into(),
        })
    }
}
