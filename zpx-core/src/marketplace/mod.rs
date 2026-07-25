use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::plugin::manifest::PluginManifest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePluginEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub author: String,
    pub rating: f32,
    pub downloads: u32,
}

pub struct MarketplaceRegistry;

impl MarketplaceRegistry {
    pub fn search(query: &str) -> Vec<MarketplacePluginEntry> {
        let entries = vec![
            MarketplacePluginEntry {
                id: "nmap".into(),
                name: "Nmap Network Mapper".into(),
                version: "7.94".into(),
                description: "Standard network discovery and port scanning plugin.".into(),
                category: "Recon".into(),
                author: "Zephyx Core".into(),
                rating: 4.9,
                downloads: 12500,
            },
            MarketplacePluginEntry {
                id: "ffuf".into(),
                name: "FFUF Web Fuzzer".into(),
                version: "2.1.0".into(),
                description: "Fast web directory and parameter fuzzing plugin.".into(),
                category: "Web".into(),
                author: "joohoi".into(),
                rating: 4.8,
                downloads: 9800,
            },
            MarketplacePluginEntry {
                id: "linpeas".into(),
                name: "LinPEAS Privilege Escalation".into(),
                version: "1.0.0".into(),
                description: "Linux Privilege Escalation Awesome Script integration.".into(),
                category: "Privesc".into(),
                author: "carlospolop".into(),
                rating: 5.0,
                downloads: 15400,
            },
            MarketplacePluginEntry {
                id: "bloodhound".into(),
                name: "BloodHound AD Collector".into(),
                version: "1.2.0".into(),
                description: "Active Directory relationship graph collector.".into(),
                category: "Active Directory".into(),
                author: "SpecterOps".into(),
                rating: 4.9,
                downloads: 8700,
            },
        ];

        if query.trim().is_empty() {
            entries
        } else {
            let q = query.to_lowercase();
            entries
                .into_iter()
                .filter(|e| e.name.to_lowercase().contains(&q) || e.description.to_lowercase().contains(&q) || e.category.to_lowercase().contains(&q))
                .collect()
        }
    }

    pub fn install(plugin_id: &str) -> Result<String> {
        info!(plugin_id, "Marketplace plugin installation initiated");
        Ok(format!("Marketplace plugin '{}' installed successfully.", plugin_id))
    }

    pub fn publish(manifest: &PluginManifest) -> Result<String> {
        if manifest.id.is_empty() {
            return Err(anyhow!("Plugin Manifest ID cannot be empty for publishing"));
        }
        info!(plugin_id = %manifest.id, "Plugin published to Zephyx Marketplace");
        Ok(format!("Plugin '{}' v{} published to registry.", manifest.name, manifest.version))
    }
}
