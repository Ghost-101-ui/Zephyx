# Changelog

All notable changes to Zephyx are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.5] — Autonomous Reasoning Runtime

### Added
- **`RuntimeCoordinator`** — Event-driven central reasoning loop connecting Observe -> Parse -> Persist -> Sync Context -> Update Graph -> Refine Hypotheses -> Evaluate Objectives -> Recalculate Strategies -> Select Capability -> Resolve Tool -> Execute -> Learn.
- **Runtime System Events** — Added full event set (`ToolStarted`, `ToolCompleted`, `ToolFailed`, `ParserCompleted`, `FindingUpdated`, `ContextUpdated`, `HypothesisUpdated`, `ObjectiveCompleted`, `StrategyUpdated`, `CapabilitySelected`, `RuntimeIdle`, `RuntimeCompleted`).
- **Capability Resolver** — Added `CapabilityResolver` to map decision engine requests to installed system tools.
- **Runtime REST API Endpoints** — `/api/v1/runtime`, `/api/v1/runtime/status`, `/api/v1/runtime/context`, `/api/v1/runtime/objectives`, `/api/v1/runtime/hypotheses`, `/api/v1/runtime/strategies`, `/api/v1/runtime/reasoning`.

---

## [0.6.4] — Cognitive Decision & Adaptive Reasoning Engine

### Added
- **Objective Engine** — Mission-driven objectives (Recon, Enumeration, Tech ID, Initial Access, PrivEsc, Flag Discovery) with automatic state progression.
- **Hypothesis Engine** — Probabilistic hypothesis evaluation over observed indicators (e.g. HTTP + PHP + robots.txt -> WordPress hypothesis @ 72% confidence -> WhatWeb tag -> Confirmed @ 99%).
- **Strategy Planner** — Competing attack vectors (Web, SMB, SSH, Network) ranked by probability, missing info, and cost.
- **Reasoning Trace Engine** — Full decision explainability with stored `ReasoningTrace` records.
- **Browser Intelligence Framework** — `BrowserIntelligence` for web app DOM, form, file upload, SPA framework, and API endpoint inspection.
- **Timeline & Collaboration Mode** — `TimelineRecord` and `CollaborationMode` for Interactive, Assisted, Explainable human decision making.

---

## [0.6.0] — Intelligence Layer

### Added
- **Decision Engine** — Deterministic rule-based recommendation system with confidence scoring
- **Knowledge Graph** — Live attack graph mapping hosts, services, credentials, vulnerabilities, and flags
- **Context Engine** — Aggregated target intelligence snapshot for decision and planning systems
- **Heuristic Engine** — Pattern-based scoring for port/service/technology combinations
- **Explainability Engine** — Full reasoning chain for every recommendation
- **Memory System** — Long-term storage of effective tool flags and execution patterns
- **Workflow Planner** — Generates sequenced action plans from target context
- **AI Provider Abstraction** — Pluggable AI layer (Ollama, OpenAI, mock) — advisory-only, never executes
- **Model Manager** — Local LLM model listing and health status
- `zpx plan` — Build dynamic workflow plan for a target
- `zpx decision inspect` — Inspect decision engine outcomes
- `zpx explain` — Explain findings, tools, or workflows
- `zpx ai` — AI provider diagnostics
- `zpx model` — Manage local LLM models
- `zpx context show` — Show aggregated target context
- `zpx graph show` — Display knowledge graph as Mermaid diagram
- `zpx memory list` — Browse long-term execution memory

### Changed
- `WorkflowEngine::evaluate_phase_transition` now uses `Context` snapshot for richer transition logic
- `Recommendation` struct now includes `priority` (Critical/High/Medium/Low) and `status` fields
- Plugin manifest updated to v2 format with capability declarations

### Fixed
- Session resume now correctly updates `updated_at` timestamp in metadata

---

## [0.5.0] — Extensibility

### Added
- **Plugin System** — Manifest v2 plugin architecture with capability declarations
- **Plugin SDK** — Rust SDK for building custom tool integrations
- **Marketplace** — Plugin registry with search, install, publish, and rating support
- **Rule Pack Manager** — TOML-defined deterministic rule sets
- **Snapshot Manager** — Create, list, restore, and delete workspace snapshots
- **Session Replayer** — Replay recorded workspace execution timelines
- **Capability Registry** — Abstraction layer mapping capabilities to available tools
- **Dependency Resolver** — Tool dependency resolution and verification
- `zpx plugin` — Full plugin lifecycle management (list, search, install, doctor, publish)
- `zpx rules` — Rule pack management
- `zpx snapshot` — Workspace snapshot management
- `zpx replay` — Replay execution timeline
- `zpx capability` — Capability resolution commands

### Changed
- Tool Manager now uses Capability Registry for tool selection
- Scheduler respects CPU and memory throttle limits from Resource Manager

---

## [0.4.0] — Infrastructure

### Added
- **Task Scheduler** — Async scheduler with configurable concurrency and priority queuing
- **Resource Manager** — CPU/memory monitoring with throttling support
- **Event Bus** — Platform-wide event publishing and subscription system
- **Execution Engine** — Task lifecycle management (pause, resume, cancel, retry)
- **Evidence Store** — SHA-256 checksummed evidence management
- **Export Engine** — Multi-format export (Markdown, HTML, JSON, CSV)
- **Learning System** — Records task outcomes for future optimization
- `zpx scheduler status` — Inspect task queue and concurrency
- `zpx resource status` — Monitor system resource usage
- `zpx tasks` — Full task lifecycle management commands
- `zpx pipeline` — Automation pipeline management

### Changed
- Session directory structure now includes `exports/` subdirectory
- Database schema extended with task state tracking tables

---

## [0.3.0] — Platform Core

### Added
- **SQLite Database** — Persistent storage for findings, tasks, snapshots, journal entries
- **Artifact Store** — Structured tool output storage with MIME types and checksums
- **Recommendation Queue** — Priority queue for decision engine recommendations
- **Report Engine** — Configurable report generation with template support
- **Statistics Engine** — Workflow completion metrics and analytics
- **Finding Models** — Typed findings: Port, HttpEndpoint, Vulnerability, Credential, Hash, Flag, SUID, Loot, SMB
- `zpx artifact` — Artifact management commands
- `zpx report` — Report generation commands
- `zpx api` — Internal REST API server

### Changed
- All findings now include confidence score and timestamp
- Workspace structure standardized across all sessions

---

## [0.2.0] — Workflow Automation

### Added
- **Workflow Engine** — Phase-aware state machine (9 phases: Recon through Reporting)
- **Workflow State Machine** — Validated transitions with terminal state protection
- **Built-in Workflow Templates** — HTB Linux/Windows, THM Web, PortSwigger, AD Assessment, API Assessment
- **Automation Pipeline** — YAML-defined pipeline steps with variable substitution
- **Tool Package Manager** — Bundled tool packs (recon, web, ad, privesc)
- **Execution Profiles** — Named profiles: default, stealth, aggressive, ctf, lab
- `zpx workflow` — Workflow lifecycle commands
- `zpx profile` — Profile management
- `zpx pack` — Tool pack installation

### Changed
- `zpx scan` now executes via the Pipeline engine
- `WorkflowEngine::calculate_progress` takes findings into account for bonus progress

---

## [0.1.0] — Foundation

### Added
- Initial project structure as Cargo workspace (`zpx-core`, `zpx-cli`, `zpx-tui`)
- **Session Manager** — Create, list, and resume named assessment sessions
- **Workspace Manager** — Central `~/.zephyx/` workspace initialization
- **Tool Manager** — Basic tool catalog and binary resolution
- **Platform Adapter** — OS detection and platform-specific tool management
- **Config Manager** — Execution profile configuration
- **TUI Dashboard** — Basic Ratatui terminal interface
- `zpx init` — Workspace initialization
- `zpx session` — Session management (create, list, resume)
- `zpx doctor` — System diagnostics
- `zpx tool` — Tool listing and verification
- `zpx workspace` — Workspace management
- `zpx update` — Catalog update
- `zpx config` — Configuration display
- `zpx dashboard` — Launch TUI

---

[0.6.2]: https://github.com/Ghost-101-ui/Zephyx/compare/v0.6.0...v0.6.2
[0.6.0]: https://github.com/Ghost-101-ui/Zephyx/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/Ghost-101-ui/Zephyx/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/Ghost-101-ui/Zephyx/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/Ghost-101-ui/Zephyx/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Ghost-101-ui/Zephyx/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Ghost-101-ui/Zephyx/releases/tag/v0.1.0
