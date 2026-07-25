use zpx_core::db::DatabaseManager;
use zpx_core::evidence::EvidenceManager;
use zpx_core::graph::AttackGraphStore;
use zpx_core::models::{Finding, FindingKind, Phase};
use zpx_core::pipeline::AutomationPipeline;
use zpx_core::snapshot::SnapshotManager;
use zpx_core::workflow::WorkflowEngine;
use tempfile::tempdir;

#[test]
fn test_database_manager_in_memory() {
    let db = DatabaseManager::in_memory().expect("In-memory SQLite should initialize");
    let finding = Finding::new("10.10.10.123", "nmap", FindingKind::Port {
        port: 80,
        protocol: "tcp".into(),
        service: "http".into(),
        version: Some("Apache/2.4.49".into()),
    });

    db.insert_finding(&finding).expect("Finding insertion should succeed");
    let findings = db.get_findings().expect("Fetching findings should succeed");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].target_ip, "10.10.10.123");
}

#[test]
fn test_workflow_phase_transition() {
    let findings = vec![Finding::new("10.10.10.123", "nmap", FindingKind::Port {
        port: 80,
        protocol: "tcp".into(),
        service: "http".into(),
        version: None,
    })];

    let next_phase = WorkflowEngine::evaluate_phase_transition(&Phase::Recon, &findings);
    assert_eq!(next_phase, Phase::Enumeration);
}

#[test]
fn test_attack_graph_node_generation() {
    let finding = Finding::new("10.10.10.123", "nmap", FindingKind::Port {
        port: 80,
        protocol: "tcp".into(),
        service: "http".into(),
        version: Some("Apache 2.4.49".into()),
    });

    let (nodes, edges) = AttackGraphStore::build_nodes_from_finding("10.10.10.123", &finding);
    assert_eq!(nodes.len(), 2);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].relationship, "exposes");
}

#[test]
fn test_evidence_recording() {
    let dir = tempdir().expect("Failed to create tempdir");
    let ev = EvidenceManager::record_evidence("f-100", "nmap", "<nmaprun></nmaprun>", dir.path())
        .expect("Recording evidence should succeed");

    assert_eq!(ev.tool_name, "nmap");
    assert_eq!(ev.mime_type, "application/xml");
}

#[test]
fn test_snapshot_creation() {
    let dir = tempdir().expect("Failed to create tempdir");
    let snap = SnapshotManager::create_snapshot("TargetBox", dir.path())
        .expect("Snapshot creation should succeed");

    assert_eq!(snap.target_name, "TargetBox");
}

#[test]
fn test_automation_pipeline_default() {
    let pipe = AutomationPipeline::default_recon_pipeline();
    assert_eq!(pipe.steps.len(), 5);
    assert_eq!(pipe.steps[0].plugin, "nmap");
}
