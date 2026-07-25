use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub target_fingerprint: String,
    pub successful_tool: String,
    pub effective_flags: String,
    pub execution_time_secs: f32,
}

pub struct MemorySystem;

impl MemorySystem {
    pub fn get_insights(fingerprint: &str) -> Vec<MemoryRecord> {
        vec![
            MemoryRecord {
                target_fingerprint: fingerprint.to_string(),
                successful_tool: "ffuf".to_string(),
                effective_flags: "-u http://TARGET/FUZZ -w common.txt".to_string(),
                execution_time_secs: 12.4,
            },
            MemoryRecord {
                target_fingerprint: fingerprint.to_string(),
                successful_tool: "enum4linux".to_string(),
                effective_flags: "-a TARGET".to_string(),
                execution_time_secs: 45.1,
            },
        ]
    }
}
