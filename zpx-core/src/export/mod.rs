use anyhow::Result;
use crate::models::{Finding, Recommendation};
use crate::db::DatabaseManager;

pub enum ExportFormat {
    Markdown,
    Json,
    Csv,
    Html,
}

pub struct ExportEngine;

impl ExportEngine {
    pub fn export_findings(findings: &[Finding], format: ExportFormat) -> Result<String> {
        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(findings)?;
                Ok(json)
            }
            ExportFormat::Csv => {
                let mut csv = String::from("id,target_ip,source_tool,confidence,timestamp\n");
                for f in findings {
                    csv.push_str(&format!("\"{}\",\"{}\",\"{}\",{},\"{}\"\n", f.id, f.target_ip, f.source_tool, f.confidence, f.timestamp.to_rfc3339()));
                }
                Ok(csv)
            }
            ExportFormat::Html => {
                let mut html = String::from("<html><head><title>Zephyx Findings Export</title></head><body><h1>Zephyx Findings</h1><table border='1'><tr><th>ID</th><th>Tool</th><th>Target</th></tr>");
                for f in findings {
                    html.push_str(&format!("<tr><td>{}</td><td>{}</td><td>{}</td></tr>", f.id, f.source_tool, f.target_ip));
                }
                html.push_str("</table></body></html>");
                Ok(html)
            }
            ExportFormat::Markdown => {
                let mut md = String::from("# Zephyx Findings Summary\n\n| ID | Tool | Target | Confidence |\n|---|---|---|---|\n");
                for f in findings {
                    md.push_str(&format!("| {} | {} | {} | {} |\n", f.id, f.source_tool, f.target_ip, f.confidence));
                }
                Ok(md)
            }
        }
    }

    pub fn export_recommendations(recs: &[Recommendation], format: ExportFormat) -> Result<String> {
        match format {
            ExportFormat::Json => Ok(serde_json::to_string_pretty(recs)?),
            ExportFormat::Csv => {
                let mut csv = String::from("id,title,tool,priority,status\n");
                for r in recs {
                    csv.push_str(&format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n", r.id, r.title, r.recommended_tool, r.priority, r.status));
                }
                Ok(csv)
            }
            ExportFormat::Html => {
                let mut html = String::from("<html><body><h1>Zephyx Recommendations</h1><ul>");
                for r in recs {
                    html.push_str(&format!("<li><b>{}</b> [{}] - {}</li>", r.title, r.priority, r.suggested_command));
                }
                html.push_str("</ul></body></html>");
                Ok(html)
            }
            ExportFormat::Markdown => {
                let mut md = String::from("# Zephyx Recommendations\n\n");
                for r in recs {
                    md.push_str(&format!("- **{}** (Priority: {}, Status: {})\n  - Tool: `{}`\n  - Command: `{}`\n\n", r.title, r.priority, r.status, r.recommended_tool, r.suggested_command));
                }
                Ok(md)
            }
        }
    }

    pub fn export_all(db: &DatabaseManager, format: ExportFormat) -> Result<String> {
        let findings = db.get_findings().unwrap_or_default();
        Self::export_findings(&findings, format)
    }
}
