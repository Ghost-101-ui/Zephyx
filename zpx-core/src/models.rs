use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Phase {
    Recon,
    Enumeration,
    TechnologyDetection,
    VulnerabilityDiscovery,
    Exploitation,
    PrivilegeEscalation,
    PostExploitation,
    FlagCollection,
    Reporting,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Recon => write!(f, "Reconnaissance"),
            Phase::Enumeration => write!(f, "Enumeration"),
            Phase::TechnologyDetection => write!(f, "Technology Detection"),
            Phase::VulnerabilityDiscovery => write!(f, "Vulnerability Discovery"),
            Phase::Exploitation => write!(f, "Exploitation"),
            Phase::PrivilegeEscalation => write!(f, "Privilege Escalation"),
            Phase::PostExploitation => write!(f, "Post Exploitation"),
            Phase::FlagCollection => write!(f, "Flag Collection"),
            Phase::Reporting => write!(f, "Reporting"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInfo {
    pub name: String,
    pub ip: String,
    pub hostname: Option<String>,
    pub os: Option<String>,
    pub phase: Phase,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingKind {
    Port {
        port: u16,
        protocol: String,
        service: String,
        version: Option<String>,
    },
    HttpEndpoint {
        url: String,
        status_code: u16,
        content_length: usize,
    },
    Vulnerability {
        cve: Option<String>,
        name: String,
        severity: String,
        details: String,
    },
    Credential {
        service: String,
        username: String,
        password_or_hash: String,
    },
    Hash {
        hash_type: String,
        hash_value: String,
        user: Option<String>,
    },
    TokenOrJwt {
        token_type: String,
        value: String,
    },
    Flag {
        flag_type: String,
        value: String,
    },
    SmbShare {
        share_name: String,
        permissions: String,
        remark: Option<String>,
    },
    SuidBinary {
        path: String,
        owner: String,
    },
    Loot {
        name: String,
        path: String,
        description: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub target_ip: String,
    pub source_tool: String,
    pub kind: FindingKind,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
}

impl Finding {
    pub fn new(target_ip: impl Into<String>, source_tool: impl Into<String>, kind: FindingKind) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            target_ip: target_ip.into(),
            source_tool: source_tool.into(),
            kind,
            confidence: 0.9,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Critical => write!(f, "CRITICAL"),
            Priority::High => write!(f, "HIGH"),
            Priority::Medium => write!(f, "MEDIUM"),
            Priority::Low => write!(f, "LOW"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecommendationStatus {
    Pending,
    Accepted,
    Running,
    Completed,
    Ignored,
    Rejected,
    Expired,
}

impl std::fmt::Display for RecommendationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecommendationStatus::Pending => write!(f, "PENDING"),
            RecommendationStatus::Accepted => write!(f, "ACCEPTED"),
            RecommendationStatus::Running => write!(f, "RUNNING"),
            RecommendationStatus::Completed => write!(f, "COMPLETED"),
            RecommendationStatus::Ignored => write!(f, "IGNORED"),
            RecommendationStatus::Rejected => write!(f, "REJECTED"),
            RecommendationStatus::Expired => write!(f, "EXPIRED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub recommended_tool: String,
    pub suggested_command: String,
    pub reasoning: Vec<String>,
    pub confidence: f32,
    pub priority: Priority,
    pub status: RecommendationStatus,
    pub target_phase: Phase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackNode {
    pub id: String,
    pub node_type: String, // Host, Service, Credential, Vulnerability, Flag
    pub label: String,
    pub metadata_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackEdge {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relationship: String, // runs, exposes, exploits, authenticates_with
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub finding_id: String,
    pub tool_name: String,
    pub raw_output_path: String,
    pub checksum_sha256: String,
    pub mime_type: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub target_name: String,
    pub created_at: DateTime<Utc>,
    pub file_path: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskState {
    Queued,
    Starting,
    Running,
    Paused,
    Cancelled,
    Failed,
    Completed,
    Timeout,
    Skipped,
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskState::Queued => write!(f, "QUEUED"),
            TaskState::Starting => write!(f, "STARTING"),
            TaskState::Running => write!(f, "RUNNING"),
            TaskState::Paused => write!(f, "PAUSED"),
            TaskState::Cancelled => write!(f, "CANCELLED"),
            TaskState::Failed => write!(f, "FAILED"),
            TaskState::Completed => write!(f, "COMPLETED"),
            TaskState::Timeout => write!(f, "TIMEOUT"),
            TaskState::Skipped => write!(f, "SKIPPED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub plugin_name: String,
    pub target_ip: String,
    pub command: String,
    pub state: TaskState,
    pub progress_percentage: u8,
    pub current_operation: String,
    pub elapsed_seconds: u64,
    pub estimated_seconds: u64,
    pub cpu_usage: f32,
    pub memory_mb: u64,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub decision: String,
    pub reason: String,
    pub confidence: f32,
    pub triggered_finding_ids: Vec<String>,
    pub generated_command: String,
    pub user_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub source: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPhaseInfo {
    pub id: String,
    pub phase: Phase,
    pub display_name: String,
    pub description: String,
    pub prerequisites: Vec<Phase>,
    pub completion_requirements: Vec<String>,
    pub supported_plugins: Vec<String>,
    pub expected_findings: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub next_phases: Vec<Phase>,
    pub estimated_duration_secs: u64,
    pub progress_percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePackInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    pub rule_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub phase: Phase,
    pub event_type: String,
    pub summary: String,
    pub details_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStats {
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub failed_tasks: u32,
    pub recommendations_generated: u32,
    pub recommendations_accepted: u32,
    pub recommendations_ignored: u32,
    pub workflow_completion_percentage: f32,
    pub average_execution_time_secs: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayRecord {
    pub step: usize,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub plugin: String,
    pub command: String,
    pub status: String,
    pub duration_ms: u64,
    pub summary: String,
}

