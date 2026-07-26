use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::context::TargetContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub id: String,
    pub goal: String,
    pub vector_name: String,
    pub probability: f32,
    pub required_evidence: Vec<String>,
    pub blocked_conditions: Vec<String>,
    pub missing_info: Vec<String>,
    pub estimated_cost: u32,
    pub estimated_time_seconds: u32,
}

impl Strategy {
    pub fn new(goal: &str, vector_name: &str, probability: f32) -> Self {
        Self {
            id: format!("strat-{}", &Uuid::new_v4().to_string()[..8]),
            goal: goal.to_string(),
            vector_name: vector_name.to_string(),
            probability,
            required_evidence: Vec::new(),
            blocked_conditions: Vec::new(),
            missing_info: Vec::new(),
            estimated_cost: 1,
            estimated_time_seconds: 300,
        }
    }
}

pub struct StrategyPlanner;

impl StrategyPlanner {
    pub fn evaluate(ctx: &TargetContext) -> Result<Vec<Strategy>> {
        let mut strategies = Vec::new();

        if ctx.open_ports.contains(&80) || ctx.open_ports.contains(&443) || ctx.services.values().any(|s| s.contains("http")) {
            let mut web_strat = Strategy::new("Gain Initial Access", "Web Application Vector", 0.82);
            web_strat.missing_info.push("Full Directory Map".into());
            web_strat.missing_info.push("CMS Type & Version".into());
            strategies.push(web_strat);
        }

        if ctx.open_ports.contains(&445) || ctx.services.values().any(|s| s.contains("smb")) {
            let mut smb_strat = Strategy::new("Gain Initial Access", "SMB / Active Directory Vector", 0.65);
            smb_strat.missing_info.push("Anonymous Share Access".into());
            strategies.push(smb_strat);
        }

        if ctx.open_ports.contains(&22) || ctx.services.values().any(|s| s.contains("ssh")) {
            let mut ssh_strat = Strategy::new("Gain Initial Access", "SSH Credential Vector", 0.40);
            ssh_strat.missing_info.push("Valid Username / Private Key".into());
            strategies.push(ssh_strat);
        }

        if strategies.is_empty() {
            let mut recon_strat = Strategy::new("Discover Attack Surface", "Network Discovery Vector", 0.90);
            recon_strat.missing_info.push("All Open TCP/UDP Ports".into());
            strategies.push(recon_strat);
        }

        strategies.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap_or(std::cmp::Ordering::Equal));
        Ok(strategies)
    }
}
