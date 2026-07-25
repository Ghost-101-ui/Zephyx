# Installing Zephyx

This document covers all installation methods for every supported platform.

For platform support details, see [SUPPORTED_PLATFORMS.md](SUPPORTED_PLATFORMS.md).

---

## Prerequisites

All installation methods require:

- **Rust 1.75+** and **Cargo** — Install via [rustup](https://rustup.rs/)
- **Git** — For cloning the repository
- **C build toolchain** — `build-essential` on Linux, Xcode CLI tools on macOS, MSVC on Windows

---

## Installing Rust

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows — download rustup-init.exe from:
# https://rustup.rs/
```

---

## Method 1: Install via Cargo (Recommended)

```bash
cargo install --git https://github.com/Ghost-101-ui/Zephyx.git --bin zpx
```

This compiles and installs the `zpx` binary to `~/.cargo/bin/zpx`.

---

## Method 2: Build from Source

```bash
# Clone
git clone https://github.com/Ghost-101-ui/Zephyx.git
cd Zephyx

# Build release binary
cargo build --release --bin zpx

# The binary is at:
# Linux/macOS: ./target/release/zpx
# Windows:     .\target\release\zpx.exe
```

### Install to system PATH

```bash
# Linux / macOS
sudo cp target/release/zpx /usr/local/bin/
# or
cp target/release/zpx ~/.local/bin/

# Windows — copy to a directory in your PATH
```

---

## Method 3: Portable Binary (Pre-built)

Download the pre-built binary for your platform from the [GitHub Releases](https://github.com/Ghost-101-ui/Zephyx/releases) page.

```bash
# Linux (example)
wget https://github.com/Ghost-101-ui/Zephyx/releases/latest/download/Zephyx-0.6.2-linux-amd64.tar.gz
tar -xzf Zephyx-0.6.2-linux-amd64.tar.gz
chmod +x zpx
sudo mv zpx /usr/local/bin/
```

---

## Linux — Detailed Guide

### Kali Linux / Parrot OS / Debian / Ubuntu

```bash
# Install build dependencies
sudo apt update
sudo apt install -y build-essential curl git pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/Ghost-101-ui/Zephyx.git
cd Zephyx
cargo build --release --bin zpx

# Install
sudo cp target/release/zpx /usr/local/bin/
zpx --version
```

---

## Windows — Detailed Guide

### Option A: Native Windows

1. Install [Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or Visual Studio with the "Desktop development with C++" workload
2. Install Rust from https://rustup.rs/
3. Open PowerShell:

```powershell
git clone https://github.com/Ghost-101-ui/Zephyx.git
cd Zephyx
cargo build --release --bin zpx
# Binary: .\target\release\zpx.exe

# Add to PATH (permanent)
$env:PATH += ";$PWD\target\release"
```

### Option B: WSL2 (Recommended for full tool compatibility)

```bash
# Inside WSL2
sudo apt update && sudo apt install -y build-essential curl git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
git clone https://github.com/Ghost-101-ui/Zephyx.git
cd Zephyx && cargo build --release
sudo cp target/release/zpx /usr/local/bin/
```

---

## macOS — Detailed Guide

```bash
# Install Xcode CLI tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/Ghost-101-ui/Zephyx.git
cd Zephyx
cargo build --release --bin zpx

# Install
sudo cp target/release/zpx /usr/local/bin/

# Apple Silicon (M1/M2/M3) — natively supported
```

---

## Verify Installation

```bash
zpx --version
# Expected: zpx 0.6.2

zpx doctor
# Expected: System health report with tool status
```

---

## Future Package Manager Installations

> These are planned and not yet available:

```bash
# Homebrew (macOS / Linux)
brew install Ghost-101-ui/tap/zpx

# Cargo (crates.io)
cargo install zpx

# Debian/Ubuntu PPA
sudo add-apt-repository ppa:zephyx/zephyx
sudo apt install zpx

# Arch AUR
yay -S zpx
```

---

## Post-Installation

After installing, run the setup wizard:

```bash
zpx doctor          # Check system health
zpx init --name "MyBox" --ip 10.10.10.1   # Initialize workspace
```

See [docs/getting-started.md](docs/getting-started.md) to begin your first assessment.
