use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::context::TargetContext;
use crate::models::Phase;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObjectiveType {
    Reconnaissance,
    Enumeration,
    AttackSurfaceDiscovery,
    TechnologyIdentification,
    AuthenticationDiscovery,
    CredentialDiscovery,
    InitialAccess,
    LocalEnumeration,
    PrivilegeEscalation,
    Persistence,
    FlagDiscovery,
    EvidenceCollection,
    Reporting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObjectiveState {
    Pending,
    Active,
    Completed,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub id: String,
    pub name: String,
    pub objective_type: ObjectiveType,
    pub priority: u8, // 1 (Highest) to 10
    pub completion_state: ObjectiveState,
    pub confidence: f32,
    pub dependencies: Vec<String>,
    pub parent_id: Option<String>,
    pub child_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Objective {
    pub fn new(name: &str, obj_type: ObjectiveType, priority: u8) -> Self {
        Self {
            id: format!("obj-{}", &Uuid::new_v4().to_string()[..8]),
            name: name.to_string(),
            objective_type: obj_type,
            priority,
            completion_state: ObjectiveState::Pending,
            confidence: 0.5,
            dependencies: Vec::new(),
            parent_id: None,
            child_ids: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub struct ObjectiveEngine;

impl ObjectiveEngine {
    pub fn get_default_tree() -> Vec<Objective> {
        let mut recon = Objective::new("Initial Target Reconnaissance", ObjectiveType::Reconnaissance, 1);
        recon.completion_state = ObjectiveState::Active;

        let mut enum_obj = Objective::new("Service & Port Enumeration", ObjectiveType::Enumeration, 2);
        enum_obj.dependencies.push(recon.id.clone());

        let mut tech_obj = Objective::new("Technology & CMS Identification", ObjectiveType::TechnologyIdentification, 3);
        tech_obj.dependencies.push(enum_obj.id.clone());

        let mut access_obj = Objective::new("Initial Access Acquisition", ObjectiveType::InitialAccess, 4);
        access_obj.dependencies.push(tech_obj.id.clone());

        let mut privesc_obj = Objective::new("Privilege Escalation", ObjectiveType::PrivilegeEscalation, 5);
        privesc_obj.dependencies.push(access_obj.id.clone());

        vec![recon, enum_obj, tech_obj, access_obj, privesc_obj]
    }

    pub fn evaluate(ctx: &TargetContext, objectives: &mut [Objective]) -> Result<Option<Objective>> {
        // Auto-advance objectives based on discovered context
        if !ctx.open_ports.is_empty() {
            if let Some(recon) = objectives.iter_mut().find(|o| o.objective_type == ObjectiveType::Reconnaissance) {
                recon.completion_state = ObjectiveState::Completed;
                recon.confidence = 1.0;
            }
            if let Some(enum_obj) = objectives.iter_mut().find(|o| o.objective_type == ObjectiveType::Enumeration) {
                if enum_obj.completion_state == ObjectiveState::Pending {
                    enum_obj.completion_state = ObjectiveState::Active;
                }
            }
        }

        if !ctx.services.is_empty() || !ctx.technologies.is_empty() {
            if let Some(enum_obj) = objectives.iter_mut().find(|o| o.objective_type == ObjectiveType::Enumeration) {
                enum_obj.completion_state = ObjectiveState::Completed;
                enum_obj.confidence = 0.95;
            }
            if let Some(tech) = objectives.iter_mut().find(|o| o.objective_type == ObjectiveType::TechnologyIdentification) {
                if tech.completion_state == ObjectiveState::Pending {
                    tech.completion_state = ObjectiveState::Active;
                }
            }
        }

        if ctx.active_phase == Phase::PrivilegeEscalation || ctx.credentials.iter().any(|(u, _)| u == "root" || u == "SYSTEM") {
            if let Some(access) = objectives.iter_mut().find(|o| o.objective_type == ObjectiveType::InitialAccess) {
                access.completion_state = ObjectiveState::Completed;
                access.confidence = 1.0;
            }
            if let Some(privesc) = objectives.iter_mut().find(|o| o.objective_type == ObjectiveType::PrivilegeEscalation) {
                privesc.completion_state = ObjectiveState::Active;
            }
        }

        // Active objective with highest priority
        let active = objectives
            .iter()
            .filter(|o| o.completion_state == ObjectiveState::Active)
            .min_by_key(|o| o.priority)
            .cloned();

        Ok(active)
    }
}
