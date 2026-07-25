pub mod manifest;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models::Finding;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub required_binaries: Vec<String>,
}

#[async_trait]
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> PluginMetadata;
    fn build_command(&self, target_ip: &str, args: &[String]) -> (String, Vec<String>);
    fn parse_output(&self, raw_stdout: &str, target_ip: &str) -> Result<Vec<Finding>>;
}

pub mod parsers;
