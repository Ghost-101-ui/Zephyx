use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use crate::models::{Evidence, Finding, FindingKind, Phase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetContext {
    pub target_ip: String,
    pub target_name: String,
    pub active_phase: Phase,
    pub open_ports: Vec<u16>,
    pub services: HashMap<u16, String>,
    pub technologies: HashSet<String>,
    pub credentials: Vec<(String, String)>,
    pub findings: Vec<Finding>,
    pub evidence: Vec<Evidence>,
    pub timeline: Vec<ContextTimelineEvent>,
    pub metadata: HashMap<String, String>,
}

impl TargetContext {
    pub fn new(target_ip: &str, target_name: &str) -> Self {
        Self {
            target_ip: target_ip.to_string(),
            target_name: target_name.to_string(),
            active_phase: Phase::Recon,
            open_ports: Vec::new(),
            services: HashMap::new(),
            technologies: HashSet::new(),
            credentials: Vec::new(),
            findings: Vec::new(),
            evidence: Vec::new(),
            timeline: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn record_finding(&mut self, finding: Finding) {
        self.timeline.push(ContextTimelineEvent {
            timestamp: Utc::now(),
            event_type: "finding_added".to_string(),
            summary: format!("Finding recorded from tool: {}", finding.source_tool),
        });
        self.findings.push(finding);
    }

    pub fn add_technology(&mut self, tech: &str) {
        if self.technologies.insert(tech.to_lowercase()) {
            self.timeline.push(ContextTimelineEvent {
                timestamp: Utc::now(),
                event_type: "technology_identified".to_string(),
                summary: format!("Technology identified: {}", tech),
            });
        }
    }

    pub fn add_port_service(&mut self, port: u16, service: &str) {
        if !self.open_ports.contains(&port) {
            self.open_ports.push(port);
        }
        self.services.insert(port, service.to_string());
        self.timeline.push(ContextTimelineEvent {
            timestamp: Utc::now(),
            event_type: "port_discovered".to_string(),
            summary: format!("Port {} ({}) exposed", port, service),
        });
    }

    pub fn add_credential(&mut self, user: &str, pass: &str) {
        self.credentials.push((user.to_string(), pass.to_string()));
        self.timeline.push(ContextTimelineEvent {
            timestamp: Utc::now(),
            event_type: "credential_found".to_string(),
            summary: format!("Credential acquired for user {}", user),
        });
    }

    pub fn update_from_finding(&mut self, finding: &Finding) {
        self.record_finding(finding.clone());
        match &finding.kind {
            FindingKind::Port { port, service, version, .. } => {
                let service_str = if let Some(v) = version {
                    format!("{} ({})", service, v)
                } else {
                    service.clone()
                };
                self.add_port_service(*port, &service_str);
            }
            FindingKind::HttpEndpoint { url, .. } => {
                self.add_technology("HTTP Server");
                self.metadata.insert("web_endpoint".into(), url.clone());
            }
            FindingKind::Credential { username, password_or_hash, .. } => {
                self.add_credential(username, password_or_hash);
            }
            FindingKind::Flag { flag_type, value } => {
                self.metadata.insert(format!("flag_{}", flag_type), value.clone());
            }
            FindingKind::SmbShare { share_name, .. } => {
                self.add_technology("SMB / Windows Share");
                self.metadata.insert(format!("share_{}", share_name), share_name.clone());
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTimelineEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub summary: String,
}

#[derive(Clone)]
pub struct ContextEngine {
    context: Arc<RwLock<TargetContext>>,
}

impl ContextEngine {
    pub fn new(target_ip: &str, target_name: &str) -> Self {
        Self {
            context: Arc::new(RwLock::new(TargetContext::new(target_ip, target_name))),
        }
    }

    pub fn get_snapshot(&self) -> TargetContext {
        self.context.read().unwrap().clone()
    }

    pub fn update<F>(&self, func: F) -> Result<()>
    where
        F: FnOnce(&mut TargetContext),
    {
        let mut guard = self.context.write().unwrap();
        func(&mut guard);
        Ok(())
    }
}
