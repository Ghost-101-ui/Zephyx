use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_percent: f32,
    pub max_memory_mb: u64,
    pub max_concurrent_scans: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_percent: 85.0,
            max_memory_mb: 4096,
            max_concurrent_scans: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    pub cpu_usage_percent: f32,
    pub used_memory_mb: u64,
    pub total_memory_mb: u64,
    pub active_scans: usize,
    pub is_throttled: bool,
}

pub struct ResourceManager {
    limits: ResourceLimits,
    sys: System,
}

impl ResourceManager {
    pub fn new(limits: ResourceLimits) -> Self {
        let sys = System::new_all();
        Self { limits, sys }
    }

    pub fn default_manager() -> Self {
        Self::new(ResourceLimits::default())
    }

    pub fn snapshot(&mut self, active_scans: usize) -> ResourceSnapshot {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let cpu = self.sys.global_cpu_info().cpu_usage();
        let used_mem = self.sys.used_memory() / (1024 * 1024);
        let total_mem = self.sys.total_memory() / (1024 * 1024);

        let throttled = cpu > self.limits.max_cpu_percent || active_scans >= self.limits.max_concurrent_scans;

        ResourceSnapshot {
            cpu_usage_percent: cpu,
            used_memory_mb: used_mem,
            total_memory_mb: total_mem,
            active_scans,
            is_throttled: throttled,
        }
    }

    pub fn can_spawn_task(&mut self, current_active: usize) -> bool {
        let snap = self.snapshot(current_active);
        !snap.is_throttled
    }
}
