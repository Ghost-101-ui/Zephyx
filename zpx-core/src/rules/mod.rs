pub mod pack;

use anyhow::{anyhow, Result};
use crate::models::RulePackInfo;
pub use pack::{RuleAction, RuleCondition, YamlRule, YamlRulePack};

pub struct RulePackManager;

impl RulePackManager {
    pub fn get_all_packs() -> Vec<RulePackInfo> {
        vec![
            RulePackInfo {
                id: "linux".into(),
                name: "Linux Security & Exploitation Rules".into(),
                version: "1.2.0".into(),
                description: "Rules for detecting Linux SUID/GUID binaries, sudo misconfigurations, cron jobs, and kernel exploits.".into(),
                enabled: true,
                rule_count: 42,
            },
            RulePackInfo {
                id: "windows".into(),
                name: "Windows & Active Directory Rules".into(),
                version: "1.4.1".into(),
                description: "Rules for unquoted service paths, token impersonation, SMB null sessions, and Kerberoasting.".into(),
                enabled: true,
                rule_count: 58,
            },
            RulePackInfo {
                id: "web".into(),
                name: "Web Application & API Security Rules".into(),
                version: "2.0.0".into(),
                description: "Rules targeting SQL Injection, XSS, CSRF, LFI/RFI, SSRF, and JWT vulnerabilities.".into(),
                enabled: true,
                rule_count: 85,
            },
            RulePackInfo {
                id: "active-directory".into(),
                name: "Enterprise Active Directory Audit Pack".into(),
                version: "1.1.0".into(),
                description: "BloodHound graph analysis, AS-REP roasting, DCSync, and domain trust enumeration rules.".into(),
                enabled: true,
                rule_count: 36,
            },
            RulePackInfo {
                id: "cloud".into(),
                name: "Cloud & Container Audit Rules".into(),
                version: "0.9.0".into(),
                description: "AWS/Azure IAM misconfigurations, Kubernetes pod escape, and S3 bucket exposure rules.".into(),
                enabled: false,
                rule_count: 24,
            },
            RulePackInfo {
                id: "oscp".into(),
                name: "OSCP Exam Methodology Rule Pack".into(),
                version: "1.0.0".into(),
                description: "OffSec Offensive Security Certified Professional exam rules for rapid initial enumeration.".into(),
                enabled: true,
                rule_count: 50,
            },
            RulePackInfo {
                id: "htb".into(),
                name: "Hack The Box Machine Rule Pack".into(),
                version: "1.3.0".into(),
                description: "Custom rules tailored for HTB machine patterns and realistic CTF challenges.".into(),
                enabled: true,
                rule_count: 45,
            },
            RulePackInfo {
                id: "thm".into(),
                name: "TryHackMe Learning Path Rules".into(),
                version: "1.0.0".into(),
                description: "Guided rules for TryHackMe walkthrough rooms and beginner penetration testing paths.".into(),
                enabled: true,
                rule_count: 30,
            },
            RulePackInfo {
                id: "portswigger".into(),
                name: "PortSwigger Web Security Academy Rules".into(),
                version: "1.1.0".into(),
                description: "Rules matching PortSwigger Academy lab solutions and exploit triggers.".into(),
                enabled: true,
                rule_count: 40,
            },
        ]
    }

    pub fn get_pack_info(pack_id: &str) -> Result<RulePackInfo> {
        Self::get_all_packs()
            .into_iter()
            .find(|p| p.id.eq_ignore_ascii_case(pack_id))
            .ok_or_else(|| anyhow!("Rule pack '{}' not found", pack_id))
    }
}
