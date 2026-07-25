use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionProfile {
    Fast,
    Balanced,
    Deep,
    Stealth,
    Aggressive,
}

impl ExecutionProfile {
    pub fn get_arguments(&self, tool: &str) -> Vec<String> {
        match (tool.to_lowercase().as_str(), self) {
            ("nmap", ExecutionProfile::Fast) => vec!["-F".into(), "-T4".into(), "--top-ports".into(), "100".into()],
            ("nmap", ExecutionProfile::Stealth) => vec!["-sS".into(), "-T2".into(), "-Pn".into(), "-n".into()],
            ("nmap", ExecutionProfile::Aggressive) => vec!["-A".into(), "-T4".into(), "-p-".into()],
            ("nmap", ExecutionProfile::Deep) => vec!["-sCV".into(), "-p-".into(), "--script=vuln".into()],
            ("ffuf", ExecutionProfile::Fast) => vec!["-t".into(), "100".into(), "-mc".into(), "200,204,301,302,307".into()],
            ("ffuf", ExecutionProfile::Stealth) => vec!["-t".into(), "5".into(), "-p".into(), "0.2".into()],
            ("ffuf", ExecutionProfile::Aggressive) => vec!["-t".into(), "200".into(), "-recursion".into()],
            ("gobuster", ExecutionProfile::Fast) => vec!["dir".into(), "-t".into(), "50".into()],
            ("gobuster", ExecutionProfile::Stealth) => vec!["dir".into(), "-t".into(), "5".into(), "--delay".into(), "500ms".into()],
            _ => vec!["-sCV".into()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    pub name: String,
    pub plugin: String,
    pub profile: ExecutionProfile,
    pub timeout_seconds: u64,
    pub retry_count: u8,
    #[serde(default)]
    pub conditions: Vec<String>,
    #[serde(default)]
    pub expected_outputs: Vec<String>,
    #[serde(default)]
    pub rollback_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationPipeline {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub variables: Vec<(String, String)>,
    pub steps: Vec<PipelineStep>,
}

impl AutomationPipeline {
    pub fn default_recon_pipeline() -> Self {
        Self {
            id: "default-recon".into(),
            name: "Default Network & Web Recon Pipeline".into(),
            description: "Initial port scan followed by web directory discovery.".into(),
            variables: vec![("TARGET_IP".into(), "127.0.0.1".into())],
            steps: vec![
                PipelineStep {
                    name: "Initial Port Scan".into(),
                    plugin: "nmap".into(),
                    profile: ExecutionProfile::Balanced,
                    timeout_seconds: 300,
                    retry_count: 1,
                    conditions: vec!["host_online".into()],
                    expected_outputs: vec!["ports.xml".into()],
                    rollback_command: None,
                },
                PipelineStep {
                    name: "Web Directory Discovery".into(),
                    plugin: "ffuf".into(),
                    profile: ExecutionProfile::Balanced,
                    timeout_seconds: 600,
                    retry_count: 0,
                    conditions: vec!["port_80_or_443_open".into()],
                    expected_outputs: vec!["web_directories.json".into()],
                    rollback_command: None,
                },
            ],
        }
    }

    pub fn from_yaml(yaml_str: &str) -> Result<Self> {
        let pipe: Self = serde_yaml::from_str(yaml_str)?;
        pipe.validate()?;
        Ok(pipe)
    }

    pub fn to_yaml(&self) -> Result<String> {
        let yaml = serde_yaml::to_string(self)?;
        Ok(yaml)
    }

    pub fn validate(&self) -> Result<()> {
        if self.id.trim().is_empty() {
            return Err(anyhow!("Pipeline ID cannot be empty"));
        }
        if self.steps.is_empty() {
            return Err(anyhow!("Pipeline must contain at least one step"));
        }
        for step in &self.steps {
            if step.plugin.trim().is_empty() {
                return Err(anyhow!("Pipeline step '{}' must specify a plugin", step.name));
            }
        }
        Ok(())
    }
}
