use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrowserIntelligence {
    pub page_title: Option<String>,
    pub dom_elements: Vec<String>,
    pub forms: Vec<FormIntelligence>,
    pub admin_panels: Vec<String>,
    pub cookies: HashMap<String, String>,
    pub session_tokens: Vec<String>,
    pub jwts: Vec<String>,
    pub javascript_frameworks: Vec<String>,
    pub api_endpoints: Vec<String>,
    pub has_graphql: bool,
    pub has_websocket: bool,
    pub is_spa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormIntelligence {
    pub action_url: String,
    pub method: String,
    pub inputs: Vec<String>,
    pub form_type: FormType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FormType {
    Login,
    Registration,
    FileUpload,
    Search,
    Generic,
}

impl BrowserIntelligence {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inspect_html(html_content: &str) -> Self {
        let mut intel = Self::new();
        let lower = html_content.to_lowercase();

        if lower.contains("<title>") {
            if let Some(start) = lower.find("<title>") {
                if let Some(end) = lower[start..].find("</title>") {
                    intel.page_title = Some(html_content[start + 7..start + end].trim().to_string());
                }
            }
        }

        if lower.contains("type=\"file\"") {
            intel.forms.push(FormIntelligence {
                action_url: "/upload".into(),
                method: "POST".into(),
                inputs: vec!["file".into()],
                form_type: FormType::FileUpload,
            });
        }

        if lower.contains("type=\"password\"") || lower.contains("login") {
            intel.forms.push(FormIntelligence {
                action_url: "/login".into(),
                method: "POST".into(),
                inputs: vec!["username".into(), "password".into()],
                form_type: FormType::Login,
            });
        }

        if lower.contains("react") || lower.contains("vue") || lower.contains("angular") {
            intel.is_spa = true;
            intel.javascript_frameworks.push("SPA Framework Detected".into());
        }

        if lower.contains("/graphql") {
            intel.has_graphql = true;
            intel.api_endpoints.push("/graphql".into());
        }

        intel
    }
}
