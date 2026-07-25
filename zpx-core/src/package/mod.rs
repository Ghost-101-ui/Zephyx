use anyhow::{anyhow, Result};
use crate::tool_manager::ToolManager;

#[derive(Debug, Clone)]
pub struct ToolPack {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tools: Vec<String>,
}

pub struct ToolPackManager;

impl ToolPackManager {
    pub fn get_all_packs() -> Vec<ToolPack> {
        vec![
            ToolPack {
                id: "recon".into(),
                name: "Recon & Network Discovery Pack".into(),
                description: "Essential network scanners and HTTP fingerprinting utilities.".into(),
                tools: vec!["nmap".into(), "rustscan".into(), "httpx".into(), "whatweb".into()],
            },
            ToolPack {
                id: "web".into(),
                name: "Web Application & Fuzzing Pack".into(),
                description: "Directory brute forcers, vhost fuzzers, and SQL injection auditors.".into(),
                tools: vec!["gobuster".into(), "ffuf".into(), "feroxbuster".into(), "sqlmap".into()],
            },
            ToolPack {
                id: "ad".into(),
                name: "Active Directory & Enterprise Audit Pack".into(),
                description: "SMB sprayers, BloodHound graph collectors, and Kerberos auditors.".into(),
                tools: vec!["netexec".into(), "bloodhound".into(), "crackmapexec".into(), "kerbrute".into()],
            },
            ToolPack {
                id: "privesc".into(),
                name: "Privilege Escalation & Post-Exploitation Pack".into(),
                description: "Linux & Windows privilege escalation scripts and process monitors.".into(),
                tools: vec!["linpeas".into(), "winpeas".into(), "privesccheck".into(), "pspy".into()],
            },
        ]
    }

    pub fn install_pack(pack_id: &str, tool_manager: &ToolManager) -> Result<Vec<String>> {
        let packs = Self::get_all_packs();
        let pack = packs
            .into_iter()
            .find(|p| p.id.eq_ignore_ascii_case(pack_id))
            .ok_or_else(|| anyhow!("Tool pack '{}' not found", pack_id))?;

        let mut installed = Vec::new();
        for tool in &pack.tools {
            let path = tool_manager.install(tool)?;
            installed.push(path);
        }

        Ok(installed)
    }
}
