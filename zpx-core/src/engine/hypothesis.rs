use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::context::TargetContext;
use crate::models::FindingKind;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HypothesisStatus {
    Possible,
    Likely,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: String,
    pub description: String,
    pub supporting_evidence: Vec<String>,
    pub contradicting_evidence: Vec<String>,
    pub confidence_score: f32,
    pub related_findings: Vec<String>,
    pub related_objectives: Vec<String>,
    pub status: HypothesisStatus,
}

impl Hypothesis {
    pub fn new(description: &str, initial_confidence: f32) -> Self {
        Self {
            id: format!("hyp-{}", &Uuid::new_v4().to_string()[..8]),
            description: description.to_string(),
            supporting_evidence: Vec::new(),
            contradicting_evidence: Vec::new(),
            confidence_score: initial_confidence,
            related_findings: Vec::new(),
            related_objectives: Vec::new(),
            status: if initial_confidence >= 0.90 {
                HypothesisStatus::Confirmed
            } else if initial_confidence >= 0.65 {
                HypothesisStatus::Likely
            } else {
                HypothesisStatus::Possible
            },
        }
    }
}

pub struct HypothesisEngine;

impl HypothesisEngine {
    pub fn evaluate(ctx: &TargetContext) -> Result<Vec<Hypothesis>> {
        let mut list = Vec::new();

        // Evaluate HTTP / Web application indicators
        if ctx.open_ports.contains(&80) || ctx.open_ports.contains(&443) || ctx.services.values().any(|s| s.contains("http")) {
            let mut web_hyp = Hypothesis::new("Web Application Exposed (Potential HTTP Attack Surface)", 0.85);
            web_hyp.supporting_evidence.push("HTTP/HTTPS port open on target".into());
            list.push(web_hyp);

            // Check for CMS indicators
            if ctx.technologies.iter().any(|t| t.to_lowercase().contains("php"))
                || ctx.findings.iter().any(|f| match &f.kind {
                    FindingKind::HttpEndpoint { url, .. } => url.contains("robots.txt") || url.contains("wp-"),
                    _ => false,
                })
            {
                let mut cms_hyp = Hypothesis::new("CMS/PHP Application Framework present (e.g. WordPress, Joomla)", 0.75);
                cms_hyp.supporting_evidence.push("PHP or web directory structure detected".into());
                list.push(cms_hyp);
            }
        }

        // Evaluate SMB / Windows indicators
        if ctx.open_ports.contains(&445) || ctx.open_ports.contains(&139) || ctx.services.values().any(|s| s.contains("smb") || s.contains("microsoft-ds")) {
            let mut smb_hyp = Hypothesis::new("SMB Service Exposed (Active Directory / Windows Share)", 0.88);
            smb_hyp.supporting_evidence.push("Port 445/139 open".into());
            list.push(smb_hyp);
        }

        // Evaluate SSH indicators
        if ctx.open_ports.contains(&22) || ctx.services.values().any(|s| s.contains("ssh")) {
            let mut ssh_hyp = Hypothesis::new("SSH Authentication Endpoint Exposed", 0.90);
            ssh_hyp.supporting_evidence.push("Port 22/tcp OpenSSH detected".into());
            list.push(ssh_hyp);
        }

        Ok(list)
    }
}
