# Building Zephyx from Source

This guide explains how to build Zephyx from source for development and production use.

---

## Prerequisites

- **Rust** 1.75+ (`rustup update stable`)
- **Cargo** (bundled with Rust)
- **Git**
- **C toolchain**: `build-essential` (Linux), Xcode CLI (macOS), MSVC (Windows)

---

## Clone the Repository

```bash
git clone https://github.com/zephyx/zephyx.git
cd zephyx
```

---

## Workspace Structure

```
zephyx/
├── Cargo.toml          # Workspace manifest
├── zpx-core/           # Core library (all business logic)
├── zpx-cli/            # CLI binary (zpx)
└── zpx-tui/            # TUI library
```

---

## Development Build

Fast compilation for development (no optimizations):

```bash
cargo build --workspace
# Output: target/debug/zpx (Linux/macOS) or target\debug\zpx.exe (Windows)
```

---

## Release Build

Optimized production binary:

```bash
cargo build --release --bin zpx
# Output: target/release/zpx
```

---

## Run Without Installing

```bash
# Run the CLI directly from cargo
cargo run --bin zpx -- --help
cargo run --bin zpx -- doctor
cargo run --bin zpx -- session list
```

---

## Check (Faster Than Build)

Verify code compiles without producing a binary:

```bash
cargo check --workspace
cargo check --workspace --all-targets
```

---

## Linting and Formatting

```bash
# Format all code
cargo fmt --all

# Check format without modifying (CI)
cargo fmt --all --check

# Clippy lints (treat warnings as errors)
cargo clippy --workspace -- -D warnings
```

---

## Running Tests

```bash
# All tests
cargo test --workspace

# Core library tests only
cargo test --package zpx-core

# With output
cargo test --workspace -- --nocapture

# Specific test
cargo test --package zpx-core session_create
```

---

## Building with Features

```bash
# Build with all features enabled
cargo build --workspace --all-features

# Build without default features
cargo build --package zpx-core --no-default-features
```

---

## Cross-Compilation

### Linux → Windows (requires mingw-w64 or cross)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu --bin zpx
```

### Linux → macOS (requires cross)

```bash
cargo install cross
cross build --release --target x86_64-apple-darwin --bin zpx
```

### Linux → ARM64 (e.g., Raspberry Pi)

```bash
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu --bin zpx
```

---

## Generating Documentation

```bash
# Generate and open API documentation
cargo doc --workspace --no-deps --open

# Include private items
cargo doc --workspace --no-deps --document-private-items
```

---

## CI Checks (Full Suite)

Run the same checks that CI performs:

```bash
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo check --workspace --all-targets
```

---

## Installing from Local Build

```bash
# Install to ~/.cargo/bin/
cargo install --path zpx-cli

# Verify
zpx --version
```

---

## Dependency Updates

```bash
# Check for outdated dependencies
cargo install cargo-outdated
cargo outdated

# Update all dependencies
cargo update

# Audit for security vulnerabilities
cargo install cargo-audit
cargo audit
```

---

## Cleaning Build Artifacts

```bash
# Remove target/ directory
cargo clean

# Clean only one package
cargo clean --package zpx-core
```

---

## Troubleshooting Build Issues

### `linker not found` on Linux

```bash
sudo apt install build-essential
```

### `openssl` errors

```bash
sudo apt install libssl-dev pkg-config
```

### Slow compilation

```bash
# Use the mold linker for faster builds
sudo apt install mold
# Add to .cargo/config.toml:
# [target.x86_64-unknown-linux-gnu]
# linker = "clang"
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

### Out of disk space

```bash
# Clean all build artifacts across all Rust projects
cargo clean
du -sh ~/.cargo/registry/  # Check registry size
```
