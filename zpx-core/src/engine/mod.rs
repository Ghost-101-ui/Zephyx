use uuid::Uuid;

use crate::models::{Finding, FindingKind, Phase, Priority, Recommendation, RecommendationStatus};

pub struct RuleEngine;

impl RuleEngine {
    pub fn evaluate(findings: &[Finding], target_ip: &str) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        for finding in findings {
            match &finding.kind {
                FindingKind::Port { port, service, .. } => {
                    if service.contains("http") || *port == 80 || *port == 443 || *port == 8080 {
                        recommendations.push(Recommendation {
                            id: Uuid::new_v4().to_string(),
                            title: format!("Web Endpoint Enumeration on Port {}", port),
                            description: "Examine web applications and enumerate hidden directories/files.".to_string(),
                            recommended_tool: "ffuf".to_string(),
                            suggested_command: format!("ffuf -u http://{}:{}/FUZZ -w /usr/share/wordlists/dirb/common.txt", target_ip, port),
                            reasoning: vec![
                                format!("HTTP/HTTPS service detected on port {}.", port),
                                "Web endpoints frequently host admin panels, backup files, and vulnerable scripts.".to_string(),
                                "Automated fuzzing with FFUF provides high coverage.".to_string(),
                            ],
                            confidence: 0.95,
                            priority: Priority::High,
                            status: RecommendationStatus::Pending,
                            target_phase: Phase::Enumeration,
                        });
                    } else if service.contains("ssh") || *port == 22 {
                        recommendations.push(Recommendation {
                            id: Uuid::new_v4().to_string(),
                            title: "SSH Protocol Verification & Service Audit".to_string(),
                            description: "Audit SSH authentication methods and banner versions.".to_string(),
                            recommended_tool: "nmap".to_string(),
                            suggested_command: format!("nmap -p 22 --script ssh-auth-methods,ssh2-enum-algos {}", target_ip),
                            reasoning: vec![
                                "SSH port 22 discovered open.".to_string(),
                                "Auditing supported auth algorithms identifies weak configurations.".to_string(),
                            ],
                            confidence: 0.85,
                            priority: Priority::Medium,
                            status: RecommendationStatus::Pending,
                            target_phase: Phase::Enumeration,
                        });
                    } else if service.contains("smb") || *port == 445 {
                        recommendations.push(Recommendation {
                            id: Uuid::new_v4().to_string(),
                            title: "SMB Share Enumeration".to_string(),
                            description: "Enumerate guest/null SMB shares and domain users.".to_string(),
                            recommended_tool: "enum4linux".to_string(),
                            suggested_command: format!("enum4linux -a {}", target_ip),
                            reasoning: vec![
                                "SMB service detected on port 445.".to_string(),
                                "Null session queries often leak shares, user lists, and password policies.".to_string(),
                            ],
                            confidence: 0.90,
                            priority: Priority::High,
                            status: RecommendationStatus::Pending,
                            target_phase: Phase::Enumeration,
                        });
                    }
                }
                FindingKind::Credential { username, .. } => {
                    recommendations.push(Recommendation {
                        id: Uuid::new_v4().to_string(),
                        title: format!("Attempt Privileged SSH Login for '{}'", username),
                        description: "Use newly discovered credentials to test SSH shell access.".to_string(),
                        recommended_tool: "ssh".to_string(),
                        suggested_command: format!("ssh {}@{}", username, target_ip),
                        reasoning: vec![
                            format!("Valid credential for user '{}' captured in vault.", username),
                            "SSH login allows direct shell execution and privilege escalation auditing.".to_string(),
                        ],
                        confidence: 0.98,
                        priority: Priority::Critical,
                        status: RecommendationStatus::Pending,
                        target_phase: Phase::Exploitation,
                    });
                }
                _ => {}
            }
        }

        recommendations
    }
}
