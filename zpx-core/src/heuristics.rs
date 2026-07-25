use serde::{Deserialize, Serialize};
use crate::context::TargetContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicResult {
    pub name: String,
    pub confidence: f32,
    pub reasoning: String,
    pub recommended_capability: String,
}

#[derive(Debug, Clone)]
pub struct HeuristicEngine;

impl HeuristicEngine {
    pub fn evaluate(ctx: &TargetContext) -> Vec<HeuristicResult> {
        let mut results = Vec::new();

        let has_port_80 = ctx.open_ports.contains(&80) || ctx.open_ports.contains(&443);
        let has_apache = ctx.technologies.contains("apache");
        let has_php = ctx.technologies.contains("php");
        let has_wordpress_indicator = ctx.technologies.contains("wordpress")
            || ctx.findings.iter().any(|f| match &f.kind {
                crate::models::FindingKind::HttpEndpoint { url, .. } => {
                    url.to_lowercase().contains("wp-login") || url.to_lowercase().contains("wp-content")
                }
                _ => false,
            });

        if has_port_80 && (has_apache || has_php) && has_wordpress_indicator {
            results.push(HeuristicResult {
                name: "WordPress CMS Detected".to_string(),
                confidence: 0.95,
                reasoning: "HTTP service combined with Apache/PHP fingerprint and wp-login/wp-content indicator".to_string(),
                recommended_capability: "web_wordpress_enumeration".to_string(),
            });
        }

        let has_smb = ctx.open_ports.contains(&445) || ctx.open_ports.contains(&139);
        let has_kerberos = ctx.open_ports.contains(&88);
        let has_ldap = ctx.open_ports.contains(&389) || ctx.open_ports.contains(&636);

        if has_smb && has_kerberos && has_ldap {
            results.push(HeuristicResult {
                name: "Active Directory Domain Controller".to_string(),
                confidence: 0.98,
                reasoning: "Co-location of SMB (445), Kerberos (88), and LDAP (389) ports indicates an Active Directory Domain Controller".to_string(),
                recommended_capability: "ad_kerberoast_enumeration".to_string(),
            });
        } else if has_smb {
            results.push(HeuristicResult {
                name: "SMB File Server".to_string(),
                confidence: 0.85,
                reasoning: "Exposed SMB port (445) available for share inspection and null session testing".to_string(),
                recommended_capability: "smb_share_enumeration".to_string(),
            });
        }

        let has_ssh = ctx.open_ports.contains(&22);
        if has_ssh {
            results.push(HeuristicResult {
                name: "SSH Management Interface".to_string(),
                confidence: 0.90,
                reasoning: "Exposed SSH service (port 22) detected".to_string(),
                recommended_capability: "ssh_auth_audit".to_string(),
            });
        }

        results
    }
}
