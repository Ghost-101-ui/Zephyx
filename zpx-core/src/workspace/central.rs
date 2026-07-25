use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CentralWorkspaceManager {
    pub root_dir: PathBuf,
    pub bin_dir: PathBuf,
    pub plugins_dir: PathBuf,
    pub knowledge_dir: PathBuf,
    pub wordlists_dir: PathBuf,
    pub reports_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub sessions_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub database_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl CentralWorkspaceManager {
    pub fn get_default_root() -> PathBuf {
        if let Some(user_home) = dirs_home() {
            user_home.join(".zephyx")
        } else {
            PathBuf::from(".zephyx")
        }
    }

    pub fn init() -> Result<Self> {
        let root = Self::get_default_root();
        Self::init_at(root)
    }

    pub fn init_at(root_dir: impl AsRef<Path>) -> Result<Self> {
        let root = root_dir.as_ref().to_path_buf();

        let manager = Self {
            root_dir: root.clone(),
            bin_dir: root.join("bin"),
            plugins_dir: root.join("plugins"),
            knowledge_dir: root.join("knowledge"),
            wordlists_dir: root.join("wordlists"),
            reports_dir: root.join("reports"),
            logs_dir: root.join("logs"),
            sessions_dir: root.join("sessions"),
            cache_dir: root.join("cache"),
            database_dir: root.join("database"),
            templates_dir: root.join("templates"),
            config_dir: root.join("config"),
        };

        manager.ensure_directories()?;
        Ok(manager)
    }

    pub fn ensure_directories(&self) -> Result<()> {
        let dirs = [
            &self.root_dir,
            &self.bin_dir,
            &self.plugins_dir,
            &self.knowledge_dir,
            &self.wordlists_dir,
            &self.reports_dir,
            &self.logs_dir,
            &self.sessions_dir,
            &self.cache_dir,
            &self.database_dir,
            &self.templates_dir,
            &self.config_dir,
        ];

        for d in &dirs {
            fs::create_dir_all(d).with_context(|| format!("Failed to create directory {:?}", d))?;
        }

        Ok(())
    }

    pub fn get_database_path(&self) -> PathBuf {
        self.database_dir.join("zephyx.db")
    }

    pub fn get_managed_binary_path(&self, binary_name: &str) -> PathBuf {
        let exe_name = if cfg!(target_os = "windows") && !binary_name.ends_with(".exe") {
            format!("{}.exe", binary_name)
        } else {
            binary_name.to_string()
        };
        self.bin_dir.join(exe_name)
    }

    pub fn clean_cache(&self) -> Result<usize> {
        let mut count = 0;
        if self.cache_dir.exists() {
            for entry in fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(path)?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}

fn dirs_home() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home))
    } else if let Ok(userprofile) = std::env::var("USERPROFILE") {
        Some(PathBuf::from(userprofile))
    } else {
        None
    }
}
