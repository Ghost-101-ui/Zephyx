use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::models::{Finding, LogEntry, Recommendation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    // Legacy v0.3 / v0.4 events (preserved for backward compatibility)
    FindingDiscovered(Finding),
    RecommendationGenerated(Recommendation),
    LogEmitted(LogEntry),
    ProcessStarted { tool_name: String, command: String },
    ProcessFinished { tool_name: String, exit_code: i32 },

    // v0.5 Extensibility Events
    ToolInstalled { tool: String, path: String },
    ToolUpdated { tool: String },
    ToolRemoved { tool: String },
    PluginLoaded { plugin: String },
    PluginUnloaded { plugin: String },
    WorkflowStarted { id: String, target: String },
    WorkflowStateChanged { id: String, state: String },
    WorkflowCompleted { id: String },
    WorkflowFailed { id: String, error: String },
    ScanStarted { target: String, tool: String },
    ScanCompleted { target: String, tool: String },
    FindingCreated { finding_id: String, kind: String },
    EvidenceAdded { evidence_id: String, finding_id: String },
    ReportGenerated { path: String },
    SessionStarted { session_id: String, name: String },
    SessionEnded { session_id: String },
    ArtifactCreated { artifact_id: String, name: String, path: String },
    ResourceAlert { message: String },

    // v0.6.4 Cognitive Reasoning Events
    ObjectiveActivated { objective_id: String, name: String },
    ObjectiveCompleted { objective_id: String, name: String },
    HypothesisCreated { hypothesis_id: String, description: String, confidence: f32 },
    HypothesisUpdated { hypothesis_id: String, status: String, confidence: f32 },
    DecisionMade { decision_id: String, capability: String, tool: String },
    CapabilitySelected { capability: String, tool: String },
    StrategyChanged { strategy_id: String, vector: String, probability: f32 },
    ReasoningGenerated { trace_id: String, justification: String },
    TimelineUpdated { record_id: String },
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn global() -> Self {
        Self::new(1024)
    }

    pub fn publish(&self, event: SystemEvent) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }
}
