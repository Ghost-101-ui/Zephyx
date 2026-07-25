# Zephyx Roadmap

This document describes the planned and completed development milestones for the Zephyx platform.

---

## ✅ Completed Milestones

### v0.1.0 — Foundation
- Cargo workspace structure (`zpx-core`, `zpx-cli`, `zpx-tui`)
- Session manager with create/list/resume
- Central workspace initialization (`~/.zephyx/`)
- Tool manager and binary resolution
- Platform adapter (Linux/Windows/macOS detection)
- Basic configuration and execution profiles
- `zpx init`, `zpx doctor`, `zpx session`, `zpx workspace`

### v0.2.0 — Workflow Automation
- Phase-aware workflow state machine (9 phases)
- Built-in workflow templates (HTB, THM, AD, API, Web)
- Automation pipeline engine
- Tool pack installer (recon, web, ad, privesc)
- Execution profile management (stealth, aggressive, ctf, lab)

### v0.3.0 — Platform Core
- SQLite persistent database
- Artifact store with checksums and MIME tracking
- Typed finding models (Port, Vulnerability, Credential, Flag, etc.)
- Report generation engine (Markdown, HTML, JSON, CSV)
- Internal REST API server
- Recommendation queue

### v0.4.0 — Infrastructure
- Async task scheduler with concurrency control
- CPU/memory resource monitor with throttling
- Platform-wide event bus
- Task lifecycle management (pause, resume, cancel, retry)
- Evidence store with SHA-256 integrity
- Export engine and multi-format output

### v0.5.0 — Extensibility
- Plugin system with Manifest v2
- Plugin SDK for custom integrations
- Marketplace registry (search, install, publish, rate)
- Rule pack manager with TOML-defined deterministic rules
- Workspace snapshot management
- Session replayer and timeline
- Capability registry with fallback resolution

### v0.6.0 — Intelligence Layer *(Current)*
- Deterministic decision engine
- Knowledge graph (node-edge attack map)
- Context engine (aggregated target snapshot)
- Heuristic scoring engine
- Explainability engine (reasoning chains)
- Long-term memory system
- Workflow planner
- AI provider abstraction (advisory-only, no autonomous execution)
- Local LLM model management

---

## 🚧 In Progress

### v0.7.0 — Distributed Platform

**Theme:** Enable team-based assessments and remote agent orchestration.

Planned features:
- **Shared Sessions** — Multiple analysts working on the same session
- **Remote Agent Protocol** — Deploy Zephyx agents on remote jump hosts
- **Conflict Resolution** — Merge concurrent finding updates
- **Session Locking** — Optimistic locking for artifact writes
- **Activity Feed** — Real-time team activity stream via event bus
- **Role-Based Access** — Reader, Analyst, Lead roles per session

---

## 🔮 Future (v0.8+)

### v0.8.0 — Web Interface
- Self-hosted Zephyx web dashboard
- React + Axum server-side rendering
- Live workflow visualization
- Finding browser with filter/search
- Interactive knowledge graph explorer

### v0.9.0 — Cloud Synchronization
- Optional cloud session backup and sync
- End-to-end encrypted session storage
- Cross-device session resumption
- Team workspace sharing via cloud backend

### v1.0.0 — Stable Release
- Stable API guarantees for `zpx-core`
- Plugin ABI stability
- Full documentation coverage
- Performance benchmark suite
- Security audit

### Post-1.0 Vision
- 🌐 **SaaS Platform** — Hosted Zephyx with team workspaces
- 🤖 **Autonomous Mode** — User-gated autonomous pipeline execution
- 📦 **Package Manager** — `cargo install zpx`, APT/Homebrew packaging
- 🧩 **VS Code Extension** — Zephyx integration in the IDE
- 🔗 **Burp Suite Integration** — Import Burp findings directly
- 📱 **Mobile Companion** — Monitor sessions from a mobile app

---

## Tracking Progress

Issues and PRs related to each milestone are tracked on the [GitHub Project Board](https://github.com/Ghost-101-ui/Zephyx/projects).

To contribute to an upcoming milestone, check the [Contributing Guide](CONTRIBUTING.md) and look for issues labeled `milestone: v0.7` or `good first issue`.
