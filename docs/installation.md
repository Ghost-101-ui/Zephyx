# Installation Guide

See [INSTALL.md](../INSTALL.md) for the complete installation guide.

---

## Quick Links

- [Building from Source](../BUILDING.md)
- [Supported Platforms](../SUPPORTED_PLATFORMS.md)
- [Getting Started](getting-started.md)

---

## Minimal Quick Install

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/zephyx/zephyx.git
cd zephyx && cargo build --release
sudo cp target/release/zpx /usr/local/bin/

# Verify
zpx --version
zpx doctor
```
