use anyhow::Result;
use uuid::Uuid;

use crate::models::{
    Finding, FindingKind, JournalEntry, Phase, Priority, Recommendation, RecommendationStatus,
};
use crate::db::DatabaseManager;

pub struct PriorityEngine;

impl PriorityEngine {
    pub fn calculate_priority(
        severity: &str,
        confidence: f32,
        phase: &Phase,
        has_evidence: bool,
    ) -> Priority {
        let mut score = match severity.to_lowercase().as_str() {
            "critical" => 40.0,
            "high" => 30.0,
            "medium" => 20.0,
            _ => 10.0,
        };

        score += confidence * 20.0;

        if has_evidence {
            score += 15.0;
        }

        match phase {
            Phase::Exploitation | Phase::PrivilegeEscalation => score += 25.0,
            Phase::VulnerabilityDiscovery => score += 15.0,
            _ => score += 5.0,
        }

        if score >= 80.0 {
            Priority::Critical
        } else if score >= 60.0 {
            Priority::High
        } else if score >= 35.0 {
            Priority::Medium
        } else {
            Priority::Low
        }
    }
}

pub struct RecommendationQueue;

impl RecommendationQueue {
    pub fn generate_from_finding(target_ip: &str, finding: &Finding, phase: &Phase) -> Recommendation {
        let (title, desc, tool, cmd, priority) = match &finding.kind {
            FindingKind::Port { port: 80, .. } | FindingKind::Port { port: 443, .. } => (
                format!("Web Directory Fuzzing on port 80/443"),
                format!("HTTP service detected on port 80/443 of target {}. Perform web directory discovery.", target_ip),
                "ffuf".to_string(),
                format!("ffuf -u http://{}/FUZZ -w /usr/share/wordlists/dirb/common.txt -mc 200,301,302", target_ip),
                PriorityEngine::calculate_priority("High", finding.confidence, phase, true),
            ),
            FindingKind::Port { port: 445, .. } | FindingKind::Port { port: 139, .. } => (
                format!("SMB Share & User Enumeration"),
                format!("SMB service exposed on port 445. Inspect shares and null session access."),
                "enum4linux".to_string(),
                format!("enum4linux -a {}", target_ip),
                PriorityEngine::calculate_priority("High", finding.confidence, phase, true),
            ),
            FindingKind::Vulnerability { name, cve, severity, .. } => (
                format!("Exploit Vulnerability: {}", name),
                format!("Critical/High vulnerability detected ({}). Attempt targeted exploit.", cve.as_deref().unwrap_or("N/A")),
                "searchsploit".to_string(),
                format!("searchsploit {}", name),
                PriorityEngine::calculate_priority(severity, finding.confidence, phase, true),
            ),
            FindingKind::SuidBinary { path, owner } => (
                format!("Escalate Privileges via SUID Binary: {}", path),
                format!("Found SUID binary owned by {}: {}. Check GTFOBins for privesc vector.", owner, path),
                "linpeas".to_string(),
                format!("linpeas.sh -k {}", path),
                PriorityEngine::calculate_priority("Critical", finding.confidence, phase, true),
            ),
            _ => (
                format!("General Enumeration for {}", target_ip),
                format!("Follow up finding from source tool {}.", finding.source_tool),
                "nmap".to_string(),
                format!("nmap -sCV -p- {}", target_ip),
                PriorityEngine::calculate_priority("Medium", finding.confidence, phase, false),
            ),
        };

        Recommendation {
            id: Uuid::new_v4().to_string(),
            title,
            description: desc,
            recommended_tool: tool,
            suggested_command: cmd,
            reasoning: vec![format!("Triggered by finding {}", finding.id), format!("Confidence: {}", finding.confidence)],
            confidence: finding.confidence,
            priority,
            status: RecommendationStatus::Pending,
            target_phase: phase.clone(),
        }
    }

    pub fn process_action(
        db: &DatabaseManager,
        rec_id: &str,
        action: &str,
    ) -> Result<JournalEntry> {
        let new_status = match action.to_lowercase().as_str() {
            "accept" => RecommendationStatus::Accepted,
            "reject" => RecommendationStatus::Rejected,
            "ignore" => RecommendationStatus::Ignored,
            "postpone" => RecommendationStatus::Pending,
            _ => RecommendationStatus::Pending,
        };

        db.update_recommendation_status(rec_id, new_status.clone())?;

        let journal = JournalEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            decision: format!("Recommendation Action: {}", action),
            reason: format!("User processed recommendation {} with action {}", rec_id, action),
            confidence: 0.95,
            triggered_finding_ids: vec![rec_id.to_string()],
            generated_command: format!("Action -> {}", action),
            user_action: action.to_string(),
        };

        db.save_journal_entry(&journal)?;
        Ok(journal)
    }
}
