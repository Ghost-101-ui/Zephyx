use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::context::TargetContext;
use crate::db::DatabaseManager;
use crate::engine::{
    HypothesisEngine, ObjectiveEngine, ReasoningTrace, StrategyPlanner,
};
use crate::events::{EventBus, SystemEvent};
use crate::repository::RepositoryManager;

pub struct RuntimeCoordinator {
    pub db: DatabaseManager,
    pub repo: RepositoryManager,
    pub event_bus: EventBus,
    pub target_ip: String,
    pub target_name: String,
    pub ctx: Arc<RwLock<TargetContext>>,
}

impl RuntimeCoordinator {
    pub fn new(db: DatabaseManager, target_name: String, target_ip: String) -> Self {
        let repo = RepositoryManager::new(db.clone());
        let event_bus = EventBus::global();
        let ctx = Arc::new(RwLock::new(TargetContext::new(&target_ip, &target_name)));

        Self {
            db,
            repo,
            event_bus,
            target_ip,
            target_name,
            ctx,
        }
    }

    pub async fn run_reasoning_cycle(&self) -> Result<()> {
        info!("Beginning autonomous reasoning cycle for target: {}", self.target_ip);

        // 1. Observe & Sync Context
        let findings = self.repo.findings().get_findings().unwrap_or_default();
        {
            let mut ctx = self.ctx.write().await;
            for f in &findings {
                ctx.update_from_finding(f);
            }
        }

        let ctx_guard = self.ctx.read().await;

        // 2. Evaluate Hypotheses
        let hypotheses = HypothesisEngine::evaluate(&ctx_guard)?;
        for h in &hypotheses {
            self.event_bus.publish(SystemEvent::HypothesisCreated {
                hypothesis_id: h.id.clone(),
                description: h.description.clone(),
                confidence: h.confidence_score,
            });
        }

        // 3. Evaluate Objectives
        let mut objectives = ObjectiveEngine::get_default_tree();
        let active_obj = ObjectiveEngine::evaluate(&ctx_guard, &mut objectives)?;
        if let Some(ref obj) = active_obj {
            self.event_bus.publish(SystemEvent::ObjectiveActivated {
                objective_id: obj.id.clone(),
                name: obj.name.clone(),
            });
        }

        // 4. Recalculate Strategies
        let strategies = StrategyPlanner::evaluate(&ctx_guard)?;
        if let Some(top_strat) = strategies.first() {
            self.event_bus.publish(SystemEvent::StrategyChanged {
                strategy_id: top_strat.id.clone(),
                vector: top_strat.vector_name.clone(),
                probability: top_strat.probability,
            });
        }

        // 5. Generate Reasoning Trace
        let active_obj_id = active_obj.as_ref().map(|o| o.id.as_str()).unwrap_or("obj-none");
        let top_capability = if ctx_guard.open_ports.is_empty() {
            "port_scanning"
        } else if ctx_guard.open_ports.contains(&80) || ctx_guard.open_ports.contains(&443) {
            "web_directory_bruteforce"
        } else {
            "technology_detection"
        };

        let trace = ReasoningTrace::new(
            active_obj_id,
            hypotheses.iter().map(|h| h.description.clone()).collect(),
            top_capability,
            "nmap/ffuf",
            "Highest probability vector identified based on live findings",
            "Expand target attack surface and discover vulnerabilities",
            0.88,
        );

        self.event_bus.publish(SystemEvent::ReasoningGenerated {
            trace_id: trace.trace_id,
            justification: trace.justification,
        });

        self.event_bus.publish(SystemEvent::RuntimeIdle);
        Ok(())
    }
}
