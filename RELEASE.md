# Release Process

This document describes how Zephyx releases are prepared and published.

---

## Versioning

Zephyx follows [Semantic Versioning](https://semver.org/):

- `MAJOR.MINOR.PATCH`
- **MAJOR**: Breaking changes to the CLI, API, or plugin interface
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes and documentation updates

Current version is defined in `Cargo.toml` (workspace package version).

---

## Release Checklist

Before tagging a release:

- [ ] All tests pass: `cargo test --workspace`
- [ ] No Clippy warnings: `cargo clippy --workspace -- -D warnings`
- [ ] Code is formatted: `cargo fmt --all --check`
- [ ] `CHANGELOG.md` updated with all changes for this version
- [ ] `ROADMAP.md` updated to reflect completed milestones
- [ ] Version bumped in `Cargo.toml` workspace package section
- [ ] Documentation updated in `docs/` if behavior changed
- [ ] Security advisory reviewed (if applicable)

---

## Version Bump

```bash
# Edit the version in Cargo.toml
# [workspace.package]
# version = "0.7.0"

# Update Cargo.lock
cargo check

# Commit
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: bump version to 0.7.0"
```

---

## Tagging

```bash
git tag -a v0.7.0 -m "Release v0.7.0 — Distributed Platform"
git push origin v0.7.0
```

---

## Building Release Artifacts

### Linux (x86_64)

```bash
cargo build --release --bin zpx
strip target/release/zpx
tar -czf zpx-v0.7.0-linux-x86_64.tar.gz -C target/release zpx
```

### Linux (ARM64)

```bash
cross build --release --target aarch64-unknown-linux-gnu --bin zpx
tar -czf zpx-v0.7.0-linux-aarch64.tar.gz -C target/aarch64-unknown-linux-gnu/release zpx
```

### Windows (x86_64)

```powershell
cargo build --release --bin zpx
Compress-Archive -Path .\target\release\zpx.exe -DestinationPath zpx-v0.7.0-windows-x86_64.zip
```

### macOS (Universal Binary)

```bash
# Build for both Intel and Apple Silicon
cargo build --release --target x86_64-apple-darwin --bin zpx
cargo build --release --target aarch64-apple-darwin --bin zpx

# Create universal binary
lipo -create -output zpx \
    target/x86_64-apple-darwin/release/zpx \
    target/aarch64-apple-darwin/release/zpx

tar -czf zpx-v0.7.0-macos-universal.tar.gz zpx
```

---

## GitHub Release

1. Go to [GitHub Releases](https://github.com/Ghost-101-ui/Zephyx/releases/new)
2. Select the tag `v0.7.0`
3. Set title: `Zephyx v0.7.0 — Distributed Platform`
4. Copy the relevant section from `CHANGELOG.md` as the description
5. Upload all release artifacts:
   - `zpx-v0.7.0-linux-x86_64.tar.gz`
   - `zpx-v0.7.0-linux-aarch64.tar.gz`
   - `zpx-v0.7.0-windows-x86_64.zip`
   - `zpx-v0.7.0-macos-universal.tar.gz`
6. Publish the release

---

## Post-Release

- [ ] Announce on GitHub Discussions
- [ ] Update the `latest` release link in README if applicable
- [ ] Open the next milestone on GitHub
- [ ] Create tracking issues for the next version's roadmap items
