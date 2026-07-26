use anyhow::Result;

use crate::dependency::DependencyResolver;
use crate::export::{ExportEngine, ExportFormat};
use crate::models::{Finding, Recommendation};
use crate::package::ToolPackManager;
use crate::plugin::manifest::PluginManifest;
use crate::recommendation::RecommendationQueue;
use crate::tool_manager::{ToolManager, ToolStatusInfo};
use crate::workflow::WorkflowTemplate;
use crate::workspace::CentralWorkspaceManager;

pub struct ToolService {
    tool_manager: ToolManager,
}

impl ToolService {
    pub fn new() -> Result<Self> {
        let tool_manager = ToolManager::new()?;
        Ok(Self { tool_manager })
    }

    pub fn resolve_tool(&self, tool: &str) -> Result<String> {
        self.tool_manager.resolve(tool)
    }

    pub fn install_tool(&self, tool: &str) -> Result<String> {
        self.tool_manager.install(tool)
    }

    pub fn verify_tool(&self, tool: &str) -> Result<bool> {
        self.tool_manager.verify(tool)
    }

    pub fn update_tool(&self, tool: &str) -> Result<bool> {
        self.tool_manager.update(tool)
    }

    pub fn list_tools(&self) -> Vec<ToolStatusInfo> {
        self.tool_manager.list()
    }

    pub fn doctor_report(&self) -> Vec<String> {
        self.tool_manager.doctor()
    }

    pub fn install_pack(&self, pack_id: &str) -> Result<Vec<String>> {
        ToolPackManager::install_pack(pack_id, &self.tool_manager)
    }
}

pub struct WorkflowService {
    dependency_resolver: DependencyResolver,
}

impl WorkflowService {
    pub fn new() -> Result<Self> {
        let dependency_resolver = DependencyResolver::new()?;
        Ok(Self { dependency_resolver })
    }

    pub fn list_templates(&self) -> Vec<WorkflowTemplate> {
        WorkflowTemplate::get_builtins()
    }

    pub fn prepare_workflow(&self, template: &WorkflowTemplate) -> Result<Vec<String>> {
        self.dependency_resolver.resolve_workflow_dependencies(template)
    }
}

pub struct WorkspaceService {
    central: CentralWorkspaceManager,
}

impl WorkspaceService {
    pub fn new() -> Result<Self> {
        let central = CentralWorkspaceManager::init()?;
        Ok(Self { central })
    }

    pub fn clean_workspace_cache(&self) -> Result<usize> {
        self.central.clean_cache()
    }

    pub fn get_central(&self) -> &CentralWorkspaceManager {
        &self.central
    }
}

pub struct PluginService;

impl PluginService {
    pub fn list_plugins() -> Vec<PluginManifest> {
        PluginManifest::get_builtins()
    }
}

pub struct ReportService;

impl ReportService {
    pub fn export_findings(findings: &[Finding], format: ExportFormat) -> Result<String> {
        ExportEngine::export_findings(findings, format)
    }
}

pub struct RecommendationService;

impl RecommendationService {
    pub fn generate(target_ip: &str, finding: &Finding, phase: &crate::models::Phase) -> Recommendation {
        RecommendationQueue::generate_from_finding(target_ip, finding, phase)
    }
}

pub struct SessionService {
    session_manager: crate::session::SessionManager,
    db: crate::db::DatabaseManager,
}

impl SessionService {
    pub fn new(db: crate::db::DatabaseManager) -> Result<Self> {
        let session_manager = crate::session::SessionManager::new()?;
        Ok(Self { session_manager, db })
    }

    pub fn create_session(&self, name: &str, target_ip: &str) -> Result<crate::session::Session> {
        let session = self.session_manager.create_session(name, target_ip)?;
        let target = crate::models::TargetInfo {
            name: name.to_string(),
            ip: target_ip.to_string(),
            hostname: None,
            os: None,
            phase: crate::models::Phase::Recon,
            created_at: chrono::Utc::now(),
        };
        self.db.save_target(&target)?;
        Ok(session)
    }

    pub fn list_sessions(&self) -> Result<Vec<crate::session::SessionMetadata>> {
        self.session_manager.list_sessions()
    }

    pub fn resume_session(&self, session_id: &str) -> Result<crate::session::Session> {
        self.session_manager.resume_session(session_id)
    }
}

pub struct TaskService {
    db: crate::db::DatabaseManager,
}

impl TaskService {
    pub fn new(db: crate::db::DatabaseManager) -> Self {
        Self { db }
    }

    pub fn save_task(&self, task: &crate::models::Task) -> Result<()> {
        self.db.save_task(task)
    }

    pub fn get_tasks(&self) -> Result<Vec<crate::models::Task>> {
        self.db.get_tasks()
    }
}

pub struct PipelineService;

impl PipelineService {
    pub fn default_pipeline() -> crate::pipeline::AutomationPipeline {
        crate::pipeline::AutomationPipeline::default_recon_pipeline()
    }
}

pub struct ArtifactService {
    db: crate::db::DatabaseManager,
}

impl ArtifactService {
    pub fn new(db: crate::db::DatabaseManager) -> Self {
        Self { db }
    }

    pub fn save_evidence(&self, evidence: &crate::models::Evidence) -> Result<()> {
        self.db.save_evidence(evidence)
    }

    pub fn get_evidence(&self) -> Result<Vec<crate::models::Evidence>> {
        self.db.get_evidence()
    }
}

pub struct GraphService {
    db: crate::db::DatabaseManager,
}

impl GraphService {
    pub fn new(db: crate::db::DatabaseManager) -> Self {
        Self { db }
    }

    pub fn get_nodes(&self) -> Result<Vec<crate::models::AttackNode>> {
        self.db.get_attack_nodes()
    }

    pub fn get_edges(&self) -> Result<Vec<crate::models::AttackEdge>> {
        self.db.get_attack_edges()
    }
}

pub struct DecisionService {
    db: crate::db::DatabaseManager,
}

impl DecisionService {
    pub fn new(db: crate::db::DatabaseManager) -> Self {
        Self { db }
    }

    pub fn evaluate(&self, ctx: &crate::context::TargetContext) -> Result<crate::decision::DecisionOutcome> {
        let outcome = crate::decision::DecisionEngine::evaluate(ctx)?;
        self.db.save_recommendations(&[outcome.recommendation.clone()])?;
        Ok(outcome)
    }
}
