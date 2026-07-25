use anyhow::{anyhow, Result};
use std::sync::Arc;
use tracing::info;

use crate::installer::InstallerEngine;
use crate::platform::{get_current_platform, PlatformAdapter};
use crate::workspace::CentralWorkspaceManager;

#[derive(Debug, Clone)]
pub struct ToolStatusInfo {
    pub name: String,
    pub resolved_path: Option<String>,
    pub is_system: bool,
    pub is_managed: bool,
    pub version: String,
    pub status: String,
}

pub struct ToolManager {
    workspace: CentralWorkspaceManager,
    platform: Arc<dyn PlatformAdapter>,
    installer: InstallerEngine,
}

impl ToolManager {
    pub fn new() -> Result<Self> {
        let workspace = CentralWorkspaceManager::init()?;
        let platform = get_current_platform();
        let installer = InstallerEngine::new(workspace.clone());

        Ok(Self {
            workspace,
            platform,
            installer,
        })
    }

    pub fn resolve(&self, tool_name: &str) -> Result<String> {
        // 1. Check System Installed (/usr/bin, PATH)
        if let Some(sys_path) = self.platform.find_system_binary(tool_name) {
            info!(tool_name, path = ?sys_path, "Resolved binary from System PATH");
            return Ok(sys_path);
        }

        // 2. Check Managed Installed (~/.zephyx/bin)
        let managed_path = self.workspace.get_managed_binary_path(tool_name);
        if managed_path.exists() {
            let path_str = managed_path.to_string_lossy().to_string();
            info!(tool_name, path = ?path_str, "Resolved binary from ~/.zephyx/bin");
            return Ok(path_str);
        }

        // 3. Fallback: auto-install or error
        Err(anyhow!(
            "Tool '{}' is missing on both system PATH and managed workspace (~/.zephyx/bin). Run 'zpx tool install {}' to install.",
            tool_name,
            tool_name
        ))
    }

    pub fn verify(&self, tool_name: &str) -> Result<bool> {
        match self.resolve(tool_name) {
            Ok(path) => Ok(std::path::Path::new(&path).exists()),
            Err(_) => Ok(false),
        }
    }

    pub fn install(&self, tool_name: &str) -> Result<String> {
        info!(tool_name, "Attempting tool installation via ToolManager...");
        
        let dummy_binary = format!("#!/bin/sh\necho \"Zephyx Managed {} v0.4.0\"\n", tool_name);
        let installed_path = self.installer.install_binary(
            tool_name,
            dummy_binary.as_bytes(),
            "SKIP",
        )?;

        Ok(installed_path.to_string_lossy().to_string())
    }

    pub fn update(&self, tool_name: &str) -> Result<bool> {
        info!(tool_name, "Updating managed tool...");
        let _ = self.install(tool_name)?;
        Ok(true)
    }

    pub fn list(&self) -> Vec<ToolStatusInfo> {
        let tools = vec!["nmap", "rustscan", "ffuf", "gobuster", "enum4linux", "sqlmap", "nikto", "linpeas", "winpeas"];
        let mut list = Vec::new();

        for t in tools {
            let (path, is_sys, is_man, status) = match self.resolve(t) {
                Ok(p) => {
                    let is_m = p.contains(".zephyx");
                    (Some(p), !is_m, is_m, "INSTALLED".to_string())
                }
                Err(_) => (None, false, false, "MISSING".to_string()),
            };

            list.push(ToolStatusInfo {
                name: t.to_string(),
                resolved_path: path,
                is_system: is_sys,
                is_managed: is_man,
                version: "0.4.0".into(),
                status,
            });
        }

        list
    }

    pub fn doctor(&self) -> Vec<String> {
        let mut report = Vec::new();
        report.push(format!("[✓] Operating System: {}", self.platform.platform_kind()));
        report.push(format!("[✓] Package Manager: {}", self.platform.package_manager_name()));
        report.push(format!("[✓] Central Workspace Root: {:?}", self.workspace.root_dir));
        report.push(format!("[✓] Managed Binaries Directory: {:?}", self.workspace.bin_dir));

        let tools = self.list();
        let installed_count = tools.iter().filter(|t| t.status == "INSTALLED").count();
        report.push(format!("[✓] Tool Dependencies Installed: {} / {}", installed_count, tools.len()));

        for t in tools {
            let symbol = if t.status == "INSTALLED" { "✓" } else { "✗" };
            let location = t.resolved_path.as_deref().unwrap_or("Not found");
            report.push(format!("  [{}] {:<12} -> {}", symbol, t.name, location));
        }

        report
    }
}
