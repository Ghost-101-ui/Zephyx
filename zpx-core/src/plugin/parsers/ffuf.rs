use anyhow::{Context, Result};
use serde::Deserialize;

use crate::models::{Finding, FindingKind};

#[derive(Debug, Deserialize)]
struct FfufOutput {
    results: Vec<FfufResult>,
}

#[derive(Debug, Deserialize)]
struct FfufResult {
    url: String,
    status: u16,
    length: usize,
}

pub fn parse_ffuf_json(json_content: &str, target_ip: &str) -> Result<Vec<Finding>> {
    let output: FfufOutput = serde_json::from_str(json_content).context("Failed to deserialize FFUF JSON output")?;
    let mut findings = Vec::new();

    for res in output.results {
        findings.push(Finding::new(
            target_ip,
            "ffuf",
            FindingKind::HttpEndpoint {
                url: res.url,
                status_code: res.status,
                content_length: res.length,
            },
        ));
    }

    Ok(findings)
}
