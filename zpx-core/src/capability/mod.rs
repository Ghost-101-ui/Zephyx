use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::tool_manager::ToolManager;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Capability {
    PortScanning,
    FastPortScan,
    WebDirectoryBruteforce,
    VhostEnumeration,
    DnsEnumeration,
    SmbEnumeration,
    TechnologyDetection,
    VulnerabilityScanning,
    PrivilegeEscalation,
    CredentialDumping,
    Custom(String),
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Capability::PortScanning => write!(f, "port_scanning"),
            Capability::FastPortScan => write!(f, "fast_port_scan"),
            Capability::WebDirectoryBruteforce => write!(f, "web_directory_bruteforce"),
            Capability::VhostEnumeration => write!(f, "vhost_enumeration"),
            Capability::DnsEnumeration => write!(f, "dns_enumeration"),
            Capability::SmbEnumeration => write!(f, "smb_enumeration"),
            Capability::TechnologyDetection => write!(f, "technology_detection"),
            Capability::VulnerabilityScanning => write!(f, "vulnerability_scanning"),
            Capability::PrivilegeEscalation => write!(f, "privilege_escalation"),
            Capability::CredentialDumping => write!(f, "credential_dumping"),
            Capability::Custom(name) => write!(f, "{}", name),
        }
    }
}

pub struct CapabilityRegistry;

impl CapabilityRegistry {
    pub fn get_candidate_tools(capability: &Capability) -> Vec<&'static str> {
        match capability {
            Capability::PortScanning => vec!["nmap", "rustscan"],
            Capability::FastPortScan => vec!["rustscan", "nmap"],
            Capability::WebDirectoryBruteforce => vec!["ffuf", "gobuster", "feroxbuster"],
            Capability::VhostEnumeration => vec!["gobuster", "ffuf"],
            Capability::DnsEnumeration => vec!["gobuster", "nmap"],
            Capability::SmbEnumeration => vec!["enum4linux", "netexec", "smbmap"],
            Capability::TechnologyDetection => vec!["whatweb", "nikto"],
            Capability::VulnerabilityScanning => vec!["searchsploit", "nikto", "nmap"],
            Capability::PrivilegeEscalation => vec!["linpeas", "winpeas", "privesccheck"],
            Capability::CredentialDumping => vec!["secretsdump", "mimikatz"],
            Capability::Custom(_) => vec!["nmap"],
        }
    }

    pub fn resolve_tool_for_capability(
        tool_manager: &ToolManager,
        capability: &Capability,
    ) -> Result<(String, String)> {
        let candidates = Self::get_candidate_tools(capability);
        for candidate in candidates {
            if let Ok(resolved_path) = tool_manager.resolve(candidate) {
                return Ok((candidate.to_string(), resolved_path));
            }
        }

        Err(anyhow!(
            "No installed tool satisfies capability '{}'. Attempting automatic installation...",
            capability
        ))
    }
}

pub struct CapabilityResolver;

impl CapabilityResolver {
    pub fn select_best_tool(
        tool_manager: &ToolManager,
        capability: &Capability,
    ) -> Result<(String, String)> {
        CapabilityRegistry::resolve_tool_for_capability(tool_manager, capability)
    }
}
