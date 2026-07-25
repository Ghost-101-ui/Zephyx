use anyhow::Result;
use tracing::info;

use crate::pipeline::AutomationPipeline;
use crate::tool_manager::ToolManager;
use crate::workflow::WorkflowTemplate;

pub struct DependencyResolver {
    tool_manager: ToolManager,
}

impl DependencyResolver {
    pub fn new() -> Result<Self> {
        let tool_manager = ToolManager::new()?;
        Ok(Self { tool_manager })
    }

    pub fn check_and_install_tool(&self, tool: &str) -> Result<String> {
        match self.tool_manager.resolve(tool) {
            Ok(path) => {
                info!(tool, path = ?path, "Dependency verified");
                Ok(path)
            }
            Err(_) => {
                info!(tool, "Dependency missing, triggering automatic managed installation...");
                let path = self.tool_manager.install(tool)?;
                Ok(path)
            }
        }
    }

    pub fn resolve_workflow_dependencies(&self, template: &WorkflowTemplate) -> Result<Vec<String>> {
        info!(template_id = %template.id, "Resolving workflow tool dependencies...");
        let mut resolved_paths = Vec::new();

        for tool in &template.enabled_plugins {
            let path = self.check_and_install_tool(tool)?;
            resolved_paths.push(path);
        }

        Ok(resolved_paths)
    }

    pub fn resolve_pipeline_dependencies(&self, pipeline: &AutomationPipeline) -> Result<Vec<String>> {
        info!(pipeline_id = %pipeline.id, "Resolving pipeline tool dependencies...");
        let mut resolved_paths = Vec::new();

        for step in &pipeline.steps {
            let path = self.check_and_install_tool(&step.plugin)?;
            resolved_paths.push(path);
        }

        Ok(resolved_paths)
    }
}
