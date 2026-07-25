use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::models::{Finding, FindingKind, Phase, WorkflowPhaseInfo};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowState {
    Created,
    TargetValidated,
    Recon,
    Enumeration,
    VulnerabilityDiscovery,
    Exploitation,
    PrivilegeEscalation,
    Persistence,
    LootCollection,
    Reporting,
    Completed,
    Failed,
    Cancelled,
    Paused,
}

impl WorkflowState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, WorkflowState::Completed | WorkflowState::Failed | WorkflowState::Cancelled)
    }

    pub fn validate_transition(&self, _next: &WorkflowState) -> Result<()> {
        if self.is_terminal() {
            return Err(anyhow!("Cannot transition out of terminal state '{:?}'", self));
        }
        Ok(())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled_plugins: Vec<String>,
    pub target_os: String,
    pub initial_phase: Phase,
}

impl WorkflowTemplate {
    pub fn get_builtins() -> Vec<Self> {
        vec![
            Self {
                id: "htb-linux".into(),
                name: "Hack The Box Linux Machine Workflow".into(),
                description: "Standard Linux CTF pipeline: port scan -> web/SMB enum -> vulnerability check -> SUID/sudo privesc -> flag collection.".into(),
                enabled_plugins: vec!["nmap".into(), "ffuf".into(), "feroxbuster".into(), "linpeas".into()],
                target_os: "Linux".into(),
                initial_phase: Phase::Recon,
            },
            Self {
                id: "htb-windows".into(),
                name: "Hack The Box Windows Machine Workflow".into(),
                description: "Standard Windows CTF pipeline: RustScan/Nmap -> SMB/RPC enum -> WinPEAS/service audit -> administrator escalation.".into(),
                enabled_plugins: vec!["rustscan".into(), "nmap".into(), "enum4linux".into(), "smbmap".into(), "winpeas".into()],
                target_os: "Windows".into(),
                initial_phase: Phase::Recon,
            },
            Self {
                id: "thm-web".into(),
                name: "TryHackMe Web Application Workflow".into(),
                description: "Web-focused assessment pipeline: WhatWeb fingerprinting -> Gobuster/FFUF directory fuzzing -> Nikto/SQLMap.".into(),
                enabled_plugins: vec!["whatweb".into(), "gobuster".into(), "ffuf".into(), "nikto".into(), "sqlmap".into()],
                target_os: "Any".into(),
                initial_phase: Phase::Enumeration,
            },
            Self {
                id: "portswigger".into(),
                name: "PortSwigger Academy Web Security Workflow".into(),
                description: "Deep web lab workflow focusing on HTTP parameter pollution, JWT manipulation, XSS, and SQL injection.".into(),
                enabled_plugins: vec!["ffuf".into(), "burpsuite".into(), "sqlmap".into(), "jwt-tool".into()],
                target_os: "Web".into(),
                initial_phase: Phase::Enumeration,
            },
            Self {
                id: "active-directory".into(),
                name: "Active Directory Domain Assessment".into(),
                description: "Enterprise AD assessment: SMB spray -> BloodHound graph collection -> Kerberoasting -> secretsdump.".into(),
                enabled_plugins: vec!["enum4linux".into(), "netexec".into(), "bloodhound".into(), "impacket".into()],
                target_os: "Windows".into(),
                initial_phase: Phase::Enumeration,
            },
            Self {
                id: "linux-privesc".into(),
                name: "Linux Privilege Escalation Workflow".into(),
                description: "Post-exploitation Linux audit: SUID/GUID binaries, sudo privileges, cron jobs, kernel exploits.".into(),
                enabled_plugins: vec!["linpeas".into(), "lse".into(), "pspy".into()],
                target_os: "Linux".into(),
                initial_phase: Phase::PrivilegeEscalation,
            },
            Self {
                id: "windows-privesc".into(),
                name: "Windows Privilege Escalation Workflow".into(),
                description: "Post-exploitation Windows audit: Unquoted service paths, token impersonation, WinPEAS, PrivescCheck.".into(),
                enabled_plugins: vec!["winpeas".into(), "privesccheck".into(), "chisel".into()],
                target_os: "Windows".into(),
                initial_phase: Phase::PrivilegeEscalation,
            },
            Self {
                id: "web-assessment".into(),
                name: "Comprehensive Web Application Audit".into(),
                description: "Complete web testing methodology: crawling, endpoint discovery, header analysis, vulnerability scanning.".into(),
                enabled_plugins: vec!["whatweb".into(), "nmap".into(), "ffuf".into(), "nikto".into(), "zap".into()],
                target_os: "Web".into(),
                initial_phase: Phase::Recon,
            },
            Self {
                id: "api-assessment".into(),
                name: "REST / GraphQL API Security Assessment".into(),
                description: "Targeted API auditing: OpenAPI schema parsing, endpoint fuzzing, auth bypass testing.".into(),
                enabled_plugins: vec!["kiterunner".into(), "ffuf".into(), "postman".into(), "jwt-tool".into()],
                target_os: "API".into(),
                initial_phase: Phase::Enumeration,
            },
        ]
    }
}

pub struct WorkflowEngine;

impl WorkflowEngine {
    pub fn get_phase_info(phase: &Phase) -> WorkflowPhaseInfo {
        match phase {
            Phase::Recon => WorkflowPhaseInfo {
                id: "phase_recon".into(),
                phase: Phase::Recon,
                display_name: "Reconnaissance".into(),
                description: "Network discovery, host reachability checks, and port scanning.".into(),
                prerequisites: vec![],
                completion_requirements: vec!["Reachability confirmed".into(), "Open ports cataloged".into()],
                supported_plugins: vec!["rustscan".into(), "nmap".into(), "ping".into()],
                expected_findings: vec!["Open TCP/UDP ports".into(), "IP address info".into()],
                recommended_actions: vec!["Launch fast port scan".into(), "Check host ping response".into()],
                next_phases: vec![Phase::Enumeration],
                estimated_duration_secs: 180,
                progress_percentage: 15.0,
            },
            Phase::Enumeration => WorkflowPhaseInfo {
                id: "phase_enumeration".into(),
                phase: Phase::Enumeration,
                display_name: "Service Enumeration".into(),
                description: "Service banner grabbing, web directory fuzzing, SMB share enumeration, SSH inspection.".into(),
                prerequisites: vec![Phase::Recon],
                completion_requirements: vec!["Web directories discovered".into(), "Services enumerated".into()],
                supported_plugins: vec!["nmap".into(), "ffuf".into(), "gobuster".into(), "enum4linux".into()],
                expected_findings: vec!["HTTP Endpoints".into(), "SMB Shares".into(), "SSH Banners".into()],
                recommended_actions: vec!["Fuzz web directories".into(), "Inspect SMB shares".into()],
                next_phases: vec![Phase::TechnologyDetection],
                estimated_duration_secs: 420,
                progress_percentage: 30.0,
            },
            Phase::TechnologyDetection => WorkflowPhaseInfo {
                id: "phase_tech_detection".into(),
                phase: Phase::TechnologyDetection,
                display_name: "Technology Detection".into(),
                description: "CMS identification, web framework detection, database version profiling.".into(),
                prerequisites: vec![Phase::Enumeration],
                completion_requirements: vec!["Web server version known".into(), "Frameworks identified".into()],
                supported_plugins: vec!["whatweb".into(), "wappalyzer".into(), "nikto".into()],
                expected_findings: vec!["Apache/Nginx Version".into(), "PHP/WordPress version".into()],
                recommended_actions: vec!["Analyze HTTP headers".into(), "Match CMS signatures".into()],
                next_phases: vec![Phase::VulnerabilityDiscovery],
                estimated_duration_secs: 240,
                progress_percentage: 45.0,
            },
            Phase::VulnerabilityDiscovery => WorkflowPhaseInfo {
                id: "phase_vuln_discovery".into(),
                phase: Phase::VulnerabilityDiscovery,
                display_name: "Vulnerability Discovery".into(),
                description: "Automated vulnerability scanning, CVE cross-referencing, exploit database lookup.".into(),
                prerequisites: vec![Phase::TechnologyDetection],
                completion_requirements: vec!["Known CVEs queried".into(), "Misconfigurations flagged".into()],
                supported_plugins: vec!["searchsploit".into(), "nikto".into(), "nmap".into()],
                expected_findings: vec!["CVE entries".into(), "Exploit PoCs".into()],
                recommended_actions: vec!["Run searchsploit for versions".into(), "Audit config files".into()],
                next_phases: vec![Phase::Exploitation],
                estimated_duration_secs: 600,
                progress_percentage: 60.0,
            },
            Phase::Exploitation => WorkflowPhaseInfo {
                id: "phase_exploitation".into(),
                phase: Phase::Exploitation,
                display_name: "Exploitation & Initial Access".into(),
                description: "Credential spraying, payload delivery, initial shell access acquisition.".into(),
                prerequisites: vec![Phase::VulnerabilityDiscovery],
                completion_requirements: vec!["User shell obtained".into(), "Valid credentials found".into()],
                supported_plugins: vec!["msfconsole".into(), "hydra".into(), "sqlmap".into()],
                expected_findings: vec!["Credentials".into(), "User Shell".into(), "User Flag".into()],
                recommended_actions: vec!["Attempt authentication with credentials".into(), "Trigger PoC script".into()],
                next_phases: vec![Phase::PrivilegeEscalation],
                estimated_duration_secs: 900,
                progress_percentage: 75.0,
            },
            Phase::PrivilegeEscalation => WorkflowPhaseInfo {
                id: "phase_privesc".into(),
                phase: Phase::PrivilegeEscalation,
                display_name: "Privilege Escalation".into(),
                description: "Local privilege escalation checks (SUID, sudo -l, unquoted service paths, tokens).".into(),
                prerequisites: vec![Phase::Exploitation],
                completion_requirements: vec!["Root / SYSTEM access obtained".into()],
                supported_plugins: vec!["linpeas".into(), "winpeas".into(), "lse".into()],
                expected_findings: vec!["SUID Binaries".into(), "Sudo permissions".into(), "Root Shell".into()],
                recommended_actions: vec!["Run LinPEAS / WinPEAS script".into(), "Check sudo rights".into()],
                next_phases: vec![Phase::PostExploitation],
                estimated_duration_secs: 600,
                progress_percentage: 88.0,
            },
            Phase::PostExploitation => WorkflowPhaseInfo {
                id: "phase_post_exploit".into(),
                phase: Phase::PostExploitation,
                display_name: "Post Exploitation & Looting".into(),
                description: "Gathering secrets, SAM/Shadow hashes, SSH keys, network pivoting.".into(),
                prerequisites: vec![Phase::PrivilegeEscalation],
                completion_requirements: vec!["Loot collected".into(), "Hashes dumped".into()],
                supported_plugins: vec!["mimikatz".into(), "secretsdump".into()],
                expected_findings: vec!["System Hashes".into(), "Private Keys".into()],
                recommended_actions: vec!["Dump password hashes".into(), "Collect sensitive loot".into()],
                next_phases: vec![Phase::FlagCollection],
                estimated_duration_secs: 300,
                progress_percentage: 95.0,
            },
            Phase::FlagCollection => WorkflowPhaseInfo {
                id: "phase_flags".into(),
                phase: Phase::FlagCollection,
                display_name: "Flag Collection".into(),
                description: "Locating user.txt and root.txt flags, validating flag hashes.".into(),
                prerequisites: vec![Phase::PrivilegeEscalation],
                completion_requirements: vec!["User flag captured".into(), "Root flag captured".into()],
                supported_plugins: vec!["flag-hunter".into()],
                expected_findings: vec!["user.txt".into(), "root.txt".into()],
                recommended_actions: vec!["Read /user.txt".into(), "Read /root/root.txt".into()],
                next_phases: vec![Phase::Reporting],
                estimated_duration_secs: 60,
                progress_percentage: 98.0,
            },
            Phase::Reporting => WorkflowPhaseInfo {
                id: "phase_reporting".into(),
                phase: Phase::Reporting,
                display_name: "Reporting & Writeup".into(),
                description: "Compiling attack timeline, evidence logs, findings table, and final writeup.".into(),
                prerequisites: vec![Phase::FlagCollection],
                completion_requirements: vec!["Markdown writeup generated".into()],
                supported_plugins: vec!["zpx-report".into()],
                expected_findings: vec!["Final Report".into()],
                recommended_actions: vec!["Generate Markdown writeup".into()],
                next_phases: vec![],
                estimated_duration_secs: 120,
                progress_percentage: 100.0,
            },
        }
    }

    pub fn evaluate_phase_transition(current_phase: &Phase, findings: &[Finding]) -> Phase {
        let mut has_ports = false;
        let mut has_web = false;
        let mut has_vuln = false;
        let mut has_creds = false;
        let mut has_root_cred = false;
        let mut has_flags = false;

        for f in findings {
            match &f.kind {
                FindingKind::Port { .. } => has_ports = true,
                FindingKind::HttpEndpoint { .. } => has_web = true,
                FindingKind::Vulnerability { .. } => has_vuln = true,
                FindingKind::Credential { username, .. } => {
                    has_creds = true;
                    if username.eq_ignore_ascii_case("root") || username.eq_ignore_ascii_case("administrator") {
                        has_root_cred = true;
                    }
                }
                FindingKind::Flag { .. } => has_flags = true,
                _ => {}
            }
        }

        match current_phase {
            Phase::Recon if has_ports => Phase::Enumeration,
            Phase::Enumeration if has_web => Phase::TechnologyDetection,
            Phase::TechnologyDetection => Phase::VulnerabilityDiscovery,
            Phase::VulnerabilityDiscovery if has_vuln || has_creds => Phase::Exploitation,
            Phase::Exploitation if has_creds => Phase::PrivilegeEscalation,
            Phase::PrivilegeEscalation if has_root_cred || has_flags => Phase::FlagCollection,
            Phase::FlagCollection if has_flags => Phase::Reporting,
            _ => current_phase.clone(),
        }
    }

    pub fn calculate_progress(current_phase: &Phase, findings: &[Finding]) -> f32 {
        let base = Self::get_phase_info(current_phase).progress_percentage;
        let bonus = (findings.len() as f32 * 1.5).min(10.0);
        (base + bonus).min(100.0)
    }

    pub fn rollback_phase(current_phase: &Phase) -> Phase {
        match current_phase {
            Phase::Enumeration => Phase::Recon,
            Phase::TechnologyDetection => Phase::Enumeration,
            Phase::VulnerabilityDiscovery => Phase::TechnologyDetection,
            Phase::Exploitation => Phase::VulnerabilityDiscovery,
            Phase::PrivilegeEscalation => Phase::Exploitation,
            Phase::PostExploitation => Phase::PrivilegeEscalation,
            Phase::FlagCollection => Phase::PostExploitation,
            Phase::Reporting => Phase::FlagCollection,
            Phase::Recon => Phase::Recon,
        }
    }
}
