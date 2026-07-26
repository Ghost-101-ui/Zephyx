pub mod ffuf;
pub mod nmap;

use anyhow::Result;
use crate::models::Finding;

pub fn parse_tool_output(plugin_name: &str, raw_content: &str, target_ip: &str) -> Result<Vec<Finding>> {
    match plugin_name.to_lowercase().as_str() {
        "nmap" => nmap::parse_nmap_xml(raw_content, target_ip),
        "ffuf" => ffuf::parse_ffuf_json(raw_content, target_ip),
        _ => Ok(Vec::new()),
    }
}
