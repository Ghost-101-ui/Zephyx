use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModelInfo {
    pub name: String,
    pub provider: String,
    pub size_gb: f32,
    pub memory_req_gb: f32,
    pub is_installed: bool,
    pub health_status: String,
}

pub struct ModelManager;

impl ModelManager {
    pub fn list_models() -> Vec<LocalModelInfo> {
        vec![
            LocalModelInfo {
                name: "llama3:8b-instruct-q4_K_M".to_string(),
                provider: "Ollama".to_string(),
                size_gb: 4.7,
                memory_req_gb: 8.0,
                is_installed: true,
                health_status: "Ready".to_string(),
            },
            LocalModelInfo {
                name: "mistral-7b-instruct-v0.2".to_string(),
                provider: "llama.cpp".to_string(),
                size_gb: 4.1,
                memory_req_gb: 6.5,
                is_installed: false,
                health_status: "Not Installed".to_string(),
            },
            LocalModelInfo {
                name: "codellama:7b".to_string(),
                provider: "LM Studio".to_string(),
                size_gb: 3.8,
                memory_req_gb: 6.0,
                is_installed: false,
                health_status: "Not Installed".to_string(),
            },
        ]
    }
}
