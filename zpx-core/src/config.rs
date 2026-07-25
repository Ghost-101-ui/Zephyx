use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_target_dir: String,
    pub auto_install_missing_tools: bool,
    pub max_parallel_scans: usize,
    pub default_user_mode: UserExperienceLevel,
    pub logging_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserExperienceLevel {
    Beginner,
    Intermediate,
    Expert,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_target_dir: "./workspace".to_string(),
            auto_install_missing_tools: true,
            max_parallel_scans: 4,
            default_user_mode: UserExperienceLevel::Intermediate,
            logging_level: "info".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProfileConfig {
    pub name: String,
    pub description: String,
    pub thread_count: usize,
    pub timeout_seconds: u64,
    pub rate_limit_rps: Option<u32>,
    pub scan_depth: u8,
    pub custom_wordlist: Option<String>,
}

pub struct ProfileManager;

impl ProfileManager {
    pub fn get_builtins() -> Vec<ExecutionProfileConfig> {
        vec![
            ExecutionProfileConfig {
                name: "default".into(),
                description: "Balanced configuration for general assessments.".into(),
                thread_count: 20,
                timeout_seconds: 600,
                rate_limit_rps: None,
                scan_depth: 2,
                custom_wordlist: None,
            },
            ExecutionProfileConfig {
                name: "fast".into(),
                description: "High-concurrency rapid initial port scanning and fuzzing.".into(),
                thread_count: 100,
                timeout_seconds: 180,
                rate_limit_rps: None,
                scan_depth: 1,
                custom_wordlist: Some("/usr/share/wordlists/dirb/common.txt".into()),
            },
            ExecutionProfileConfig {
                name: "stealth".into(),
                description: "Low-noise rate-limited scan to evade IDS/WAF triggers.".into(),
                thread_count: 5,
                timeout_seconds: 1800,
                rate_limit_rps: Some(10),
                scan_depth: 3,
                custom_wordlist: None,
            },
            ExecutionProfileConfig {
                name: "deep".into(),
                description: "Comprehensive recursive scanning and exhaustive vulnerability audits.".into(),
                thread_count: 50,
                timeout_seconds: 3600,
                rate_limit_rps: None,
                scan_depth: 5,
                custom_wordlist: Some("/usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt".into()),
            },
            ExecutionProfileConfig {
                name: "ctf".into(),
                description: "Tailored for time-sensitive CTF competitions.".into(),
                thread_count: 80,
                timeout_seconds: 300,
                rate_limit_rps: None,
                scan_depth: 2,
                custom_wordlist: None,
            },
            ExecutionProfileConfig {
                name: "oscp".into(),
                description: "Strict OffSec exam methodology profile.".into(),
                thread_count: 30,
                timeout_seconds: 900,
                rate_limit_rps: None,
                scan_depth: 3,
                custom_wordlist: None,
            },
            ExecutionProfileConfig {
                name: "ad".into(),
                description: "Active Directory domain controller enumeration profile.".into(),
                thread_count: 15,
                timeout_seconds: 1200,
                rate_limit_rps: None,
                scan_depth: 3,
                custom_wordlist: None,
            },
            ExecutionProfileConfig {
                name: "web".into(),
                description: "Dedicated web application crawling and directory discovery.".into(),
                thread_count: 60,
                timeout_seconds: 600,
                rate_limit_rps: None,
                scan_depth: 4,
                custom_wordlist: None,
            },
        ]
    }
}

