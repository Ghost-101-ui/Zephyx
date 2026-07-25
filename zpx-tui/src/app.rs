use sysinfo::{CpuRefreshKind, System};
use zpx_core::models::{
    AttackEdge, AttackNode, Finding, FindingKind, JournalEntry, LogEntry, Phase, Priority, Recommendation,
    RecommendationStatus, TargetInfo, Task, TaskState,
};
use zpx_core::pipeline::AutomationPipeline;
use zpx_core::plugin::manifest::PluginManifest;
use zpx_core::workflow::WorkflowTemplate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Dashboard,
    Logs,
    Findings,
    DecisionGraph,
    Knowledge,
    Tasks,
    Explorer,
    AttackGraph,
    WorkflowPipeline,
    Palette,
}

pub struct App {
    pub active_tab: ActiveTab,
    pub target: TargetInfo,
    pub findings: Vec<Finding>,
    pub recommendations: Vec<Recommendation>,
    pub journal_entries: Vec<JournalEntry>,
    pub tasks: Vec<Task>,
    pub plugins: Vec<PluginManifest>,
    pub workflow_templates: Vec<WorkflowTemplate>,
    pub attack_nodes: Vec<AttackNode>,
    pub attack_edges: Vec<AttackEdge>,
    pub active_pipeline: AutomationPipeline,
    pub logs: Vec<LogEntry>,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub palette_open: bool,
    pub palette_input: String,
    pub system_monitor: System,
}

impl App {
    pub fn new(target_name: String, target_ip: String) -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu();
        sys.refresh_memory();

        let sample_target = TargetInfo {
            name: target_name,
            ip: target_ip.clone(),
            hostname: Some("target-box.local".into()),
            os: Some("Linux 5.10".into()),
            phase: Phase::Enumeration,
            created_at: chrono::Utc::now(),
        };

        let sample_findings = vec![
            Finding::new(&target_ip, "nmap", FindingKind::Port {
                port: 80,
                protocol: "tcp".into(),
                service: "http".into(),
                version: Some("Apache/2.4.49".into()),
            }),
            Finding::new(&target_ip, "nmap", FindingKind::Port {
                port: 22,
                protocol: "tcp".into(),
                service: "ssh".into(),
                version: Some("OpenSSH 8.2p1".into()),
            }),
            Finding::new(&target_ip, "ffuf", FindingKind::HttpEndpoint {
                url: format!("http://{}/admin", target_ip),
                status_code: 200,
                content_length: 4210,
            }),
        ];

        let mut sample_recs = zpx_core::engine::RuleEngine::evaluate(&sample_findings, &target_ip);
        for rec in &mut sample_recs {
            rec.priority = Priority::High;
            rec.status = RecommendationStatus::Pending;
        }

        let sample_tasks = vec![
            Task {
                id: "task-1".into(),
                plugin_name: "nmap".into(),
                target_ip: target_ip.clone(),
                command: format!("nmap -sCV -oX - {}", target_ip),
                state: TaskState::Completed,
                progress_percentage: 100,
                current_operation: "Finished port scan".into(),
                elapsed_seconds: 14,
                estimated_seconds: 14,
                cpu_usage: 4.2,
                memory_mb: 32,
                started_at: Some(chrono::Utc::now()),
                finished_at: Some(chrono::Utc::now()),
            },
            Task {
                id: "task-2".into(),
                plugin_name: "ffuf".into(),
                target_ip: target_ip.clone(),
                command: format!("ffuf -u http://{}/FUZZ -w /usr/share/wordlists/dirb/common.txt", target_ip),
                state: TaskState::Running,
                progress_percentage: 45,
                current_operation: "Fuzzing web directories".into(),
                elapsed_seconds: 8,
                estimated_seconds: 20,
                cpu_usage: 18.5,
                memory_mb: 64,
                started_at: Some(chrono::Utc::now()),
                finished_at: None,
            },
        ];

        let sample_journal = vec![
            JournalEntry {
                id: "j-1".into(),
                timestamp: chrono::Utc::now(),
                decision: "HTTP Service Detected".into(),
                reason: "Port 80/tcp open with Apache web server".into(),
                confidence: 0.95,
                triggered_finding_ids: vec!["f-1".into()],
                generated_command: format!("ffuf -u http://{}/FUZZ -w wordlist.txt", target_ip),
                user_action: "ACCEPTED".into(),
            },
        ];

        let sample_nodes = vec![
            AttackNode {
                id: format!("host-{}", target_ip),
                node_type: "Host".into(),
                label: target_ip.clone(),
                metadata_json: format!("{{\"ip\": \"{}\"}}", target_ip),
            },
            AttackNode {
                id: format!("svc-{}-80", target_ip),
                node_type: "Service".into(),
                label: "HTTP:80 (Apache 2.4.49)".into(),
                metadata_json: "{}".into(),
            },
            AttackNode {
                id: format!("svc-{}-22", target_ip),
                node_type: "Service".into(),
                label: "SSH:22 (OpenSSH 8.2p1)".into(),
                metadata_json: "{}".into(),
            },
        ];

        let sample_edges = vec![
            AttackEdge {
                id: "edge-1".into(),
                source_id: format!("host-{}", target_ip),
                target_id: format!("svc-{}-80", target_ip),
                relationship: "exposes".into(),
            },
            AttackEdge {
                id: "edge-2".into(),
                source_id: format!("host-{}", target_ip),
                target_id: format!("svc-{}-22", target_ip),
                relationship: "exposes".into(),
            },
        ];

        Self {
            active_tab: ActiveTab::Dashboard,
            target: sample_target,
            findings: sample_findings,
            recommendations: sample_recs,
            journal_entries: sample_journal,
            tasks: sample_tasks,
            plugins: PluginManifest::get_builtins(),
            workflow_templates: WorkflowTemplate::get_builtins(),
            attack_nodes: sample_nodes,
            attack_edges: sample_edges,
            active_pipeline: AutomationPipeline::default_recon_pipeline(),
            logs: vec![
                LogEntry {
                    id: "1".into(),
                    timestamp: chrono::Utc::now(),
                    level: "INFO".into(),
                    source: "workspace".into(),
                    message: "Target workspace initialized".into(),
                },
                LogEntry {
                    id: "2".into(),
                    timestamp: chrono::Utc::now(),
                    level: "INFO".into(),
                    source: "workflow".into(),
                    message: "Workflow phase transitioned to Enumeration".into(),
                },
            ],
            cpu_usage: 12.5,
            memory_usage: sys.used_memory() / 1024 / 1024,
            palette_open: false,
            palette_input: String::new(),
            system_monitor: sys,
        }
    }

    pub fn tick(&mut self) {
        self.system_monitor.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system_monitor.refresh_memory();
        self.cpu_usage = self.system_monitor.global_cpu_info().cpu_usage();
        self.memory_usage = self.system_monitor.used_memory() / 1024 / 1024;
    }

    pub fn toggle_palette(&mut self) {
        self.palette_open = !self.palette_open;
        if !self.palette_open {
            self.palette_input.clear();
        }
    }
}
