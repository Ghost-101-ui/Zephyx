use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models::Finding;
use crate::plugin::manifest::PluginManifest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub target_ip: String,
    pub target_port: Option<u16>,
    pub output_dir: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub findings: Vec<Finding>,
}

#[async_trait]
pub trait ZephyxPlugin: Send + Sync {
    fn manifest(&self) -> PluginManifest;
    async fn execute(&self, ctx: &PluginContext) -> Result<PluginResult>;
    fn verify(&self) -> bool {
        true
    }
}
