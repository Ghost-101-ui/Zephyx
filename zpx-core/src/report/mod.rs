use anyhow::Result;
use crate::models::{Finding, FindingKind, TargetInfo};

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn generate_markdown(target: &TargetInfo, findings: &[Finding]) -> Result<String> {
        let mut report = String::new();

        report.push_str(&format!("# Zephyx Security Writeup - {}\n\n", target.name));
        report.push_str(&format!("**Target IP:** {}\n", target.ip));
        report.push_str(&format!("**Current Phase:** {}\n", target.phase));
        report.push_str(&format!("**Date:** {}\n\n", target.created_at.format("%Y-%m-%d %H:%M UTC")));

        report.push_str("## Executive Summary\n");
        report.push_str("Automated analysis conducted via Zephyx (`zpx`) workspace platform.\n\n");

        report.push_str("## Discovered Findings & Network Map\n\n");
        report.push_str("| ID | Source Tool | Type | Details |\n");
        report.push_str("| --- | --- | --- | --- |\n");

        for f in findings {
            let details = match &f.kind {
                FindingKind::Port { port, protocol, service, version } => {
                    format!("Port {}/{} ({}) - Version: {}", port, protocol, service, version.as_deref().unwrap_or("N/A"))
                }
                FindingKind::HttpEndpoint { url, status_code, content_length } => {
                    format!("Endpoint: {} (Status: {}, Length: {})", url, status_code, content_length)
                }
                FindingKind::Vulnerability { name, severity, details, .. } => {
                    format!("[{}] {} - {}", severity, name, details)
                }
                FindingKind::Credential { service, username, .. } => {
                    format!("Creds: {} ({})", username, service)
                }
                FindingKind::Hash { hash_type, hash_value, user } => {
                    format!("Hash [{}]: {} (User: {})", hash_type, hash_value, user.as_deref().unwrap_or("N/A"))
                }
                FindingKind::TokenOrJwt { token_type, value } => {
                    format!("Token [{}]: {}", token_type, value)
                }
                FindingKind::Flag { flag_type, value } => {
                    format!("Flag captured: [{}] {}", flag_type, value)
                }
                FindingKind::SmbShare { share_name, permissions, remark } => {
                    format!("SMB Share: {} [{}] ({})", share_name, permissions, remark.as_deref().unwrap_or("N/A"))
                }
                FindingKind::SuidBinary { path, owner } => {
                    format!("SUID Binary: {} (Owner: {})", path, owner)
                }
                FindingKind::Loot { name, description, .. } => {
                    format!("Loot: {} ({})", name, description)
                }
            };
            report.push_str(&format!("| {} | {} | {:?} | {} |\n", &f.id[..8], f.source_tool, f.kind, details));
        }

        report.push_str("\n## Methodology & Timeline\n");
        report.push_str("1. Initial Port Scanning & Service Identification\n");
        report.push_str("2. Targeted Protocol & Endpoint Enumeration\n");
        report.push_str("3. Vulnerability Verification & Evidence Gathering\n");

        Ok(report)
    }
}
