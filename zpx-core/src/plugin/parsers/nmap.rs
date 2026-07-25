use anyhow::{Context, Result};
use quick_xml::de::from_str;
use serde::Deserialize;

use crate::models::{Finding, FindingKind};

#[derive(Debug, Deserialize)]
struct NmapRun {
    host: Option<NmapHost>,
}

#[derive(Debug, Deserialize)]
struct NmapHost {
    ports: Option<NmapPorts>,
}

#[derive(Debug, Deserialize)]
struct NmapPorts {
    port: Vec<NmapPort>,
}

#[derive(Debug, Deserialize)]
struct NmapPort {
    #[serde(rename = "@portid")]
    port_id: u16,
    #[serde(rename = "@protocol")]
    protocol: String,
    state: NmapState,
    service: Option<NmapService>,
}

#[derive(Debug, Deserialize)]
struct NmapState {
    #[serde(rename = "@state")]
    state: String,
}

#[derive(Debug, Deserialize)]
struct NmapService {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@product")]
    product: Option<String>,
    #[allow(dead_code)]
    #[serde(rename = "@version")]
    version: Option<String>,
}

pub fn parse_nmap_xml(xml_content: &str, target_ip: &str) -> Result<Vec<Finding>> {
    let run: NmapRun = from_str(xml_content).context("Failed to deserialize Nmap XML output")?;
    let mut findings = Vec::new();

    if let Some(host) = run.host {
        if let Some(ports) = host.ports {
            for p in ports.port {
                if p.state.state == "open" {
                    let service_name = p.service.as_ref().map(|s| s.name.clone()).unwrap_or_else(|| "unknown".into());
                    let ver = p.service.as_ref().and_then(|s| s.product.clone());

                    findings.push(Finding::new(
                        target_ip,
                        "nmap",
                        FindingKind::Port {
                            port: p.port_id,
                            protocol: p.protocol,
                            service: service_name,
                            version: ver,
                        },
                    ));
                }
            }
        }
    }

    Ok(findings)
}
