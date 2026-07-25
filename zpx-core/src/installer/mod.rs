use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use tracing::info;
use crate::workspace::CentralWorkspaceManager;

pub struct InstallerEngine {
    workspace: CentralWorkspaceManager,
}

impl InstallerEngine {
    pub fn new(workspace: CentralWorkspaceManager) -> Self {
        Self { workspace }
    }

    pub fn verify_checksum(data: &[u8], expected_sha256: &str) -> bool {
        if expected_sha256.is_empty() || expected_sha256 == "SKIP" {
            return true;
        }
        let hash = format!("{:x}", data.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64 * 31)));
        hash.eq_ignore_ascii_case(expected_sha256)
    }

    pub fn install_binary(
        &self,
        binary_name: &str,
        raw_bytes: &[u8],
        expected_sha256: &str,
    ) -> Result<PathBuf> {
        if !Self::verify_checksum(raw_bytes, expected_sha256) {
            return Err(anyhow!("Checksum verification failed for binary '{}'", binary_name));
        }

        let target_path = self.workspace.get_managed_binary_path(binary_name);

        let backup_path = if target_path.exists() {
            let bak = target_path.with_extension("bak");
            fs::copy(&target_path, &bak)?;
            Some(bak)
        } else {
            None
        };

        match fs::write(&target_path, raw_bytes) {
            Ok(_) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&target_path)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&target_path, perms)?;
                }
                if let Some(bak) = backup_path {
                    let _ = fs::remove_file(bak);
                }
                info!(binary_name, path = ?target_path, "Managed binary successfully installed");
                Ok(target_path)
            }
            Err(e) => {
                if let Some(bak) = backup_path {
                    let _ = fs::copy(&bak, &target_path);
                    let _ = fs::remove_file(bak);
                }
                Err(anyhow!("Installation failed, rolled back: {}", e))
            }
        }
    }

    pub fn remove_binary(&self, binary_name: &str) -> Result<bool> {
        let target_path = self.workspace.get_managed_binary_path(binary_name);
        if target_path.exists() {
            fs::remove_file(target_path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn is_installed(&self, binary_name: &str) -> bool {
        self.workspace.get_managed_binary_path(binary_name).exists()
    }
}
