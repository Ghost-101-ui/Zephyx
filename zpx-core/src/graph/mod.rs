pub mod knowledge;

use uuid::Uuid;
use crate::models::{AttackEdge, AttackNode, Finding, FindingKind};
pub use knowledge::{KnowledgeEdge, KnowledgeGraph, KnowledgeNode, KnowledgeNodeType};

pub struct AttackGraphStore;

impl AttackGraphStore {
    pub fn build_nodes_from_finding(target_ip: &str, finding: &Finding) -> (Vec<AttackNode>, Vec<AttackEdge>) {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let host_node_id = format!("host-{}", target_ip);
        nodes.push(AttackNode {
            id: host_node_id.clone(),
            node_type: "Host".into(),
            label: target_ip.to_string(),
            metadata_json: format!("{{\"ip\": \"{}\"}}", target_ip),
        });

        match &finding.kind {
            FindingKind::Port { port, service, version, .. } => {
                let service_node_id = format!("svc-{}-{}", target_ip, port);
                nodes.push(AttackNode {
                    id: service_node_id.clone(),
                    node_type: "Service".into(),
                    label: format!("{}:{} ({})", service, port, version.as_deref().unwrap_or("")),
                    metadata_json: format!("{{\"port\": {}, \"service\": \"{}\"}}", port, service),
                });
                edges.push(AttackEdge {
                    id: Uuid::new_v4().to_string(),
                    source_id: host_node_id,
                    target_id: service_node_id,
                    relationship: "exposes".into(),
                });
            }
            FindingKind::HttpEndpoint { url, status_code, .. } => {
                let ep_node_id = format!("endpoint-{}", Uuid::new_v4().to_string()[..8].to_string());
                nodes.push(AttackNode {
                    id: ep_node_id.clone(),
                    node_type: "Directory".into(),
                    label: format!("HTTP {} ({})", url, status_code),
                    metadata_json: format!("{{\"url\": \"{}\", \"status\": {}}}", url, status_code),
                });
                edges.push(AttackEdge {
                    id: Uuid::new_v4().to_string(),
                    source_id: host_node_id,
                    target_id: ep_node_id,
                    relationship: "contains".into(),
                });
            }
            FindingKind::Vulnerability { name, severity, cve, .. } => {
                let vuln_node_id = format!("vuln-{}", Uuid::new_v4().to_string()[..8].to_string());
                nodes.push(AttackNode {
                    id: vuln_node_id.clone(),
                    node_type: "Vulnerability".into(),
                    label: format!("[{}] {} ({})", severity, name, cve.as_deref().unwrap_or("N/A")),
                    metadata_json: format!("{{\"cve\": \"{}\", \"severity\": \"{}\"}}", cve.as_deref().unwrap_or(""), severity),
                });
                edges.push(AttackEdge {
                    id: Uuid::new_v4().to_string(),
                    source_id: host_node_id,
                    target_id: vuln_node_id,
                    relationship: "exploits".into(),
                });
            }
            FindingKind::Credential { username, service, .. } => {
                let cred_node_id = format!("cred-{}", username);
                nodes.push(AttackNode {
                    id: cred_node_id.clone(),
                    node_type: "Credential".into(),
                    label: format!("User: {} ({})", username, service),
                    metadata_json: format!("{{\"user\": \"{}\"}}", username),
                });
                edges.push(AttackEdge {
                    id: Uuid::new_v4().to_string(),
                    source_id: host_node_id,
                    target_id: cred_node_id,
                    relationship: "authenticates_with".into(),
                });
            }
            FindingKind::Flag { flag_type, value } => {
                let flag_node_id = format!("flag-{}", flag_type);
                nodes.push(AttackNode {
                    id: flag_node_id.clone(),
                    node_type: "Flag".into(),
                    label: format!("[{}] {}", flag_type, value),
                    metadata_json: format!("{{\"value\": \"{}\"}}", value),
                });
                edges.push(AttackEdge {
                    id: Uuid::new_v4().to_string(),
                    source_id: host_node_id,
                    target_id: flag_node_id,
                    relationship: "contains".into(),
                });
            }
            _ => {}
        }

        (nodes, edges)
    }

    pub fn export_dot(nodes: &[AttackNode], edges: &[AttackEdge]) -> String {
        let mut dot = String::from("digraph AttackGraph {\n  rankdir=LR;\n  node [shape=box, style=filled, fillcolor=lightblue];\n");
        for n in nodes {
            dot.push_str(&format!("  \"{}\" [label=\"{}: {}\"];\n", n.id, n.node_type, n.label));
        }
        for e in edges {
            dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"{}\"];\n", e.source_id, e.target_id, e.relationship));
        }
        dot.push_str("}\n");
        dot
    }
}
