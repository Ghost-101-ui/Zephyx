use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::context::TargetContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeNodeType {
    Target,
    Port,
    Service,
    Technology,
    Credential,
    User,
    Group,
    Share,
    Hash,
    Flag,
    Exploit,
    CVE,
    Tool,
    Capability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub node_type: KnowledgeNodeType,
    pub label: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub source_id: String,
    pub target_id: String,
    pub relationship: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_context(ctx: &TargetContext) -> Self {
        let mut graph = KnowledgeGraph::new();

        let target_node_id = format!("target-{}", ctx.target_ip);
        graph.nodes.push(KnowledgeNode {
            id: target_node_id.clone(),
            node_type: KnowledgeNodeType::Target,
            label: format!("Target: {}", ctx.target_ip),
            properties: HashMap::from([("name".into(), ctx.target_name.clone())]),
        });

        for port in &ctx.open_ports {
            let port_id = format!("port-{}-{}", ctx.target_ip, port);
            let service_name = ctx.services.get(port).cloned().unwrap_or_else(|| "unknown".into());

            graph.nodes.push(KnowledgeNode {
                id: port_id.clone(),
                node_type: KnowledgeNodeType::Port,
                label: format!("Port {} ({})", port, service_name),
                properties: HashMap::from([("port".into(), port.to_string()), ("service".into(), service_name.clone())]),
            });

            graph.edges.push(KnowledgeEdge {
                source_id: target_node_id.clone(),
                target_id: port_id.clone(),
                relationship: "EXPOSES".into(),
            });
        }

        for tech in &ctx.technologies {
            let tech_id = format!("tech-{}", tech);
            graph.nodes.push(KnowledgeNode {
                id: tech_id.clone(),
                node_type: KnowledgeNodeType::Technology,
                label: format!("Tech: {}", tech),
                properties: HashMap::new(),
            });

            graph.edges.push(KnowledgeEdge {
                source_id: target_node_id.clone(),
                target_id: tech_id,
                relationship: "RUNS".into(),
            });
        }

        for (user, pass) in &ctx.credentials {
            let cred_id = format!("cred-{}", user);
            graph.nodes.push(KnowledgeNode {
                id: cred_id.clone(),
                node_type: KnowledgeNodeType::Credential,
                label: format!("User: {}", user),
                properties: HashMap::from([("user".into(), user.clone()), ("pass".into(), pass.clone())]),
            });

            graph.edges.push(KnowledgeEdge {
                source_id: target_node_id.clone(),
                target_id: cred_id,
                relationship: "HAS_CREDENTIAL".into(),
            });
        }

        graph
    }

    pub fn export_mermaid(&self) -> String {
        let mut mermaid = String::from("graph TD\n");
        for n in &self.nodes {
            mermaid.push_str(&format!("    {}[\"{}\"]\n", n.id.replace('-', "_"), n.label));
        }
        for e in &self.edges {
            mermaid.push_str(&format!(
                "    {} -->|{}| {}\n",
                e.source_id.replace('-', "_"),
                e.relationship,
                e.target_id.replace('-', "_")
            ));
        }
        mermaid
    }
}
