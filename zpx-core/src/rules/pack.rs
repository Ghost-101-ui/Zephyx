use serde::{Deserialize, Serialize};
use crate::context::TargetContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub technologies: Option<Vec<String>>,
    pub ports: Option<Vec<u16>>,
    pub services: Option<Vec<String>>,
    pub min_confidence: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub capability: String,
    pub description: String,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlRule {
    pub id: String,
    pub name: String,
    pub when: RuleCondition,
    pub then: RuleAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlRulePack {
    pub pack_id: String,
    pub name: String,
    pub version: String,
    pub rules: Vec<YamlRule>,
}

impl YamlRulePack {
    pub fn evaluate(&self, ctx: &TargetContext) -> Vec<RuleAction> {
        let mut matched_actions = Vec::new();

        for rule in &self.rules {
            let mut matches = true;

            if let Some(ref tech_list) = rule.when.technologies {
                let tech_match = tech_list.iter().any(|t| ctx.technologies.contains(&t.to_lowercase()));
                if !tech_match {
                    matches = false;
                }
            }

            if let Some(ref port_list) = rule.when.ports {
                let port_match = port_list.iter().any(|p| ctx.open_ports.contains(p));
                if !port_match {
                    matches = false;
                }
            }

            if let Some(ref service_list) = rule.when.services {
                let service_match = service_list.iter().any(|s| {
                    ctx.services.values().any(|v| v.eq_ignore_ascii_case(s))
                });
                if !service_match {
                    matches = false;
                }
            }

            if matches {
                matched_actions.push(rule.then.clone());
            }
        }

        matched_actions
    }
}
