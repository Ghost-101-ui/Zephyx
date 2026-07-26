use anyhow::Result;
use crate::db::DatabaseManager;
use crate::models::{
    AttackEdge, AttackNode, Evidence, Finding, JournalEntry, Recommendation, Snapshot, TargetInfo, Task,
};

#[derive(Clone)]
pub struct RepositoryManager {
    db: DatabaseManager,
}

impl RepositoryManager {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseManager {
        &self.db
    }
}

#[derive(Clone)]
pub struct SessionRepository {
    db: DatabaseManager,
}

impl SessionRepository {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn save_target(&self, target: &TargetInfo) -> Result<()> {
        self.db.save_target(target)
    }

    pub fn get_target(&self, ip: &str) -> Result<Option<TargetInfo>> {
        self.db.get_target(ip)
    }
}

#[derive(Clone)]
pub struct FindingRepository {
    db: DatabaseManager,
}

impl FindingRepository {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn insert_finding(&self, finding: &Finding) -> Result<()> {
        self.db.insert_finding(finding)
    }

    pub fn get_findings(&self) -> Result<Vec<Finding>> {
        self.db.get_findings()
    }
}

#[derive(Clone)]
pub struct ArtifactRepository {
    db: DatabaseManager,
}

impl ArtifactRepository {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn save_evidence(&self, evidence: &Evidence) -> Result<()> {
        self.db.save_evidence(evidence)
    }

    pub fn get_evidence(&self) -> Result<Vec<Evidence>> {
        self.db.get_evidence()
    }
}

#[derive(Clone)]
pub struct GraphRepository {
    db: DatabaseManager,
}

impl GraphRepository {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn insert_node(&self, node: &AttackNode) -> Result<()> {
        self.db.insert_attack_node(node)
    }

    pub fn insert_edge(&self, edge: &AttackEdge) -> Result<()> {
        self.db.insert_attack_edge(edge)
    }

    pub fn get_nodes(&self) -> Result<Vec<AttackNode>> {
        self.db.get_attack_nodes()
    }

    pub fn get_edges(&self) -> Result<Vec<AttackEdge>> {
        self.db.get_attack_edges()
    }
}

#[derive(Clone)]
pub struct TaskRepository {
    db: DatabaseManager,
}

impl TaskRepository {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn save_task(&self, task: &Task) -> Result<()> {
        self.db.save_task(task)
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        self.db.get_tasks()
    }
}

#[derive(Clone)]
pub struct ReportRepository {
    db: DatabaseManager,
}

impl ReportRepository {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn save_snapshot(&self, snapshot: &Snapshot) -> Result<()> {
        self.db.save_snapshot(snapshot)
    }

    pub fn get_snapshots(&self) -> Result<Vec<Snapshot>> {
        self.db.get_snapshots()
    }
}

#[derive(Clone)]
pub struct DecisionRepository {
    db: DatabaseManager,
}

impl DecisionRepository {
    pub fn new(db: DatabaseManager) -> Self {
        Self { db }
    }

    pub fn save_recommendations(&self, recs: &[Recommendation]) -> Result<()> {
        self.db.save_recommendations(recs)
    }

    pub fn get_recommendations(&self) -> Result<Vec<Recommendation>> {
        self.db.get_recommendations()
    }

    pub fn save_journal_entry(&self, entry: &JournalEntry) -> Result<()> {
        self.db.save_journal_entry(entry)
    }

    pub fn get_journal_entries(&self) -> Result<Vec<JournalEntry>> {
        self.db.get_journal_entries()
    }
}
