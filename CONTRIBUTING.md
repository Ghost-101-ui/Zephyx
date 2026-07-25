# Contributing to Zephyx

Thank you for your interest in contributing to Zephyx! This document explains how to get involved, the standards we follow, and the processes for submitting code, documentation, and bug reports.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Workspace Layout](#workspace-layout)
- [Coding Standards](#coding-standards)
- [Commit Convention](#commit-convention)
- [Pull Request Process](#pull-request-process)
- [Issue Templates](#issue-templates)
- [Feature Requests](#feature-requests)
- [Running Tests](#running-tests)

---

## Code of Conduct

All contributors must follow our [Code of Conduct](CODE_OF_CONDUCT.md). We are committed to a welcoming, respectful community.

---

## Getting Started

### 1. Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/zephyx.git
cd zephyx
```

### 2. Set Up Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup component add clippy rustfmt
```

### 3. Build and Test

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

### 4. Create a Branch

```bash
git checkout -b feat/my-feature-name
```

---

## Workspace Layout

```
zephyx/
├── zpx-core/       # Platform core library (all business logic)
│   └── src/
│       ├── ai/             # AI provider abstractions
│       ├── artifact/       # Artifact store
│       ├── capability/     # Capability registry
│       ├── context.rs      # Context engine
│       ├── db.rs           # SQLite persistence
│       ├── decision.rs     # Decision engine
│       ├── engine/         # Workflow engine
│       ├── graph/          # Knowledge graph
│       ├── models.rs       # Shared data models
│       ├── plugin/         # Plugin system
│       ├── scheduler/      # Task scheduler
│       ├── session/        # Session manager
│       ├── workflow/       # Workflow state machine
│       └── workspace/      # Workspace manager
├── zpx-cli/        # CLI binary (clap-based)
│   └── src/main.rs
├── zpx-tui/        # TUI dashboard (ratatui-based)
│   └── src/
├── docs/           # Documentation
└── Cargo.toml      # Workspace manifest
```

**Rule:** All business logic lives in `zpx-core`. The CLI and TUI are thin consumers.

---

## Coding Standards

### Rust Style

- Follow standard Rust idioms and the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Format all code with `cargo fmt`
- Fix all Clippy warnings: `cargo clippy -- -D warnings`
- Prefer `anyhow::Result` for error propagation in application code
- Use `thiserror` for library error types
- Derive `Debug`, `Clone`, `Serialize`, `Deserialize` on public data types where appropriate

### Documentation

- All public functions, structs, and modules must have `///` doc comments
- Include examples in doc comments where the API is non-obvious
- Update `docs/` when adding new features or changing behavior

### No Unsafe

- Do not introduce `unsafe` code without a compelling reason and explicit review
- If `unsafe` is necessary, document exactly why it is sound

### Tests

- Add unit tests in the same file using `#[cfg(test)]` modules
- Add integration tests in `zpx-core/tests/`
- New features should include at least one test

---

## Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

### Types

| Type | Description |
|---|---|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `refactor` | Refactoring without behavior change |
| `test` | Adding or fixing tests |
| `chore` | Build, CI, or tooling changes |
| `perf` | Performance improvement |
| `style` | Code style / formatting |

### Examples

```
feat(workflow): add phase rollback support
fix(session): handle missing metadata.json gracefully
docs(readme): update quick start commands
refactor(capability): extract resolver into separate module
test(decision): add unit tests for rule evaluation
```

---

## Pull Request Process

1. **Ensure your branch is up to date** with `main`
2. **Run the full check suite** before opening a PR:
   ```bash
   cargo fmt --check
   cargo clippy --workspace -- -D warnings
   cargo test --workspace
   ```
3. **Write a clear PR description** explaining:
   - What problem this solves
   - How it was implemented
   - Any trade-offs or open questions
4. **Reference any related issues** with `Closes #123`
5. **Request a review** from a maintainer
6. **Address all review feedback** before merge
7. PRs are merged using **squash merge** to keep history clean

### PR Title Format

Follow the same convention as commits:
```
feat(plugin): add manifest v3 support
```

---

## Issue Templates

### Bug Report

When reporting a bug, include:

- Zephyx version (`zpx --version`)
- Operating system and version
- Steps to reproduce
- Expected behavior
- Actual behavior
- Relevant error output or logs

### Feature Request

When requesting a feature, include:

- Problem you're trying to solve
- Proposed solution or API
- Alternatives you considered
- Whether you're willing to implement it

---

## Feature Requests

We welcome feature requests! Please open a GitHub Issue using the Feature Request template. Large features should be discussed as an issue before a PR is opened.

---

## Running Tests

```bash
# Unit tests only
cargo test --package zpx-core

# All tests including integration
cargo test --workspace

# Run with output (useful for debugging)
cargo test --workspace -- --nocapture

# Run a specific test
cargo test --package zpx-core session_create

# Check without running tests
cargo check --workspace --all-targets
```

---

## Security Vulnerabilities

Do **not** open public GitHub issues for security vulnerabilities. Instead, see [SECURITY.md](SECURITY.md) for the responsible disclosure process.

---

Thank you for making Zephyx better! 🦀
