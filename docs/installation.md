# Installation Guide

See [INSTALL.md](../INSTALL.md) for the complete installation guide.

---

## Quick Links

- [Building from Source](../BUILDING.md)
- [Supported Platforms](../SUPPORTED_PLATFORMS.md)
- [Getting Started](getting-started.md)

---

## One-Liner Quick Install / Reinstall / Uninstall

### Linux & macOS
```bash
# Install / Reinstall
curl -fsSL https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/install.sh | sh

# Uninstall / Remove
curl -fsSL https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/uninstall.sh | sh
```

### Windows (PowerShell)
```powershell
# Install / Reinstall
irm https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/install.ps1 | iex

# Uninstall / Remove
irm https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/uninstall.ps1 | iex
```

## Minimal Quick Build from Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/Ghost-101-ui/Zephyx.git
cd Zephyx && cargo build --release
sudo cp target/release/zpx /usr/local/bin/

# Verify
zpx --version
zpx doctor
```
