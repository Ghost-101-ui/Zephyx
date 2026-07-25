use std::env;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformKind {
    Linux,
    Windows,
    MacOS,
    Unknown,
}

impl std::fmt::Display for PlatformKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformKind::Linux => write!(f, "Linux"),
            PlatformKind::Windows => write!(f, "Windows"),
            PlatformKind::MacOS => write!(f, "macOS"),
            PlatformKind::Unknown => write!(f, "Unknown"),
        }
    }
}

pub trait PlatformAdapter: Send + Sync {
    fn platform_kind(&self) -> PlatformKind;
    fn os_release_info(&self) -> String;
    fn package_manager_name(&self) -> String;
    fn find_system_binary(&self, binary_name: &str) -> Option<String>;
    fn install_package_cmd(&self, package: &str) -> Option<String>;
}

pub struct LinuxPlatform;

impl PlatformAdapter for LinuxPlatform {
    fn platform_kind(&self) -> PlatformKind {
        PlatformKind::Linux
    }

    fn os_release_info(&self) -> String {
        if Path::new("/etc/os-release").exists() {
            std::fs::read_to_string("/etc/os-release").unwrap_or_else(|_| "Linux (Kali / Debian)".into())
        } else {
            "Linux Generic".into()
        }
    }

    fn package_manager_name(&self) -> String {
        "apt".into()
    }

    fn find_system_binary(&self, binary_name: &str) -> Option<String> {
        let candidate_dirs = ["/usr/bin", "/usr/local/bin", "/bin", "/usr/sbin", "/sbin"];
        for dir in &candidate_dirs {
            let path = Path::new(dir).join(binary_name);
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
        if let Ok(path_var) = env::var("PATH") {
            for p in path_var.split(':') {
                let candidate = Path::new(p).join(binary_name);
                if candidate.exists() {
                    return Some(candidate.to_string_lossy().to_string());
                }
            }
        }
        None
    }

    fn install_package_cmd(&self, package: &str) -> Option<String> {
        Some(format!("sudo apt update && sudo apt install -y {}", package))
    }
}

pub struct WindowsPlatform;

impl PlatformAdapter for WindowsPlatform {
    fn platform_kind(&self) -> PlatformKind {
        PlatformKind::Windows
    }

    fn os_release_info(&self) -> String {
        "Windows NT".into()
    }

    fn package_manager_name(&self) -> String {
        "winget".into()
    }

    fn find_system_binary(&self, binary_name: &str) -> Option<String> {
        let exe_name = if binary_name.ends_with(".exe") {
            binary_name.to_string()
        } else {
            format!("{}.exe", binary_name)
        };

        if let Ok(path_var) = env::var("PATH") {
            for p in path_var.split(';') {
                let candidate = Path::new(p).join(&exe_name);
                if candidate.exists() {
                    return Some(candidate.to_string_lossy().to_string());
                }
            }
        }
        None
    }

    fn install_package_cmd(&self, package: &str) -> Option<String> {
        Some(format!("winget install --exact --id {}", package))
    }
}

pub struct MacOSPlatform;

impl PlatformAdapter for MacOSPlatform {
    fn platform_kind(&self) -> PlatformKind {
        PlatformKind::MacOS
    }

    fn os_release_info(&self) -> String {
        "macOS Darwin".into()
    }

    fn package_manager_name(&self) -> String {
        "homebrew".into()
    }

    fn find_system_binary(&self, binary_name: &str) -> Option<String> {
        let candidate_dirs = ["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin", "/bin"];
        for dir in &candidate_dirs {
            let path = Path::new(dir).join(binary_name);
            if path.exists() {
                return Some(path.to_string_lossy().to_string());
            }
        }
        None
    }

    fn install_package_cmd(&self, package: &str) -> Option<String> {
        Some(format!("brew install {}", package))
    }
}

pub fn get_current_platform() -> Arc<dyn PlatformAdapter> {
    if cfg!(target_os = "windows") {
        Arc::new(WindowsPlatform)
    } else if cfg!(target_os = "macos") {
        Arc::new(MacOSPlatform)
    } else {
        Arc::new(LinuxPlatform)
    }
}
