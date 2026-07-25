# Workspace

The Zephyx workspace (`~/.zephyx/`) is the central hub for all platform data. This document explains every directory and its purpose.

---

## Full Structure

```
~/.zephyx/
├── bin/
├── plugins/
├── sessions/
├── artifacts/
├── knowledge/
├── rules/
├── database/
├── logs/
├── cache/
├── models/
├── templates/
└── reports/
```

---

## Directory Reference

### `bin/`

Managed tool binaries installed by Zephyx (not installed system-wide).

```
~/.zephyx/bin/
├── ffuf
├── feroxbuster
├── rustscan
└── ...
```

Zephyx resolves managed binaries here as a fallback when system tools are not found. Tools installed here are managed by Zephyx (installation, updates, verification).

---

### `plugins/`

Installed plugin directories. Each plugin has its own subdirectory:

```
~/.zephyx/plugins/
├── nmap/
│   ├── manifest.toml
│   └── config.toml
├── ffuf/
│   └── manifest.toml
└── custom-scanner/
    ├── manifest.toml
    └── wordlists/
```

---

### `sessions/`

All assessment sessions. Each session is isolated in its own subdirectory:

```
~/.zephyx/sessions/
├── session-a1b2c3d4/
│   ├── metadata.json    # Session metadata
│   ├── artifacts/       # Raw tool outputs
│   ├── evidence/        # Verified evidence
│   ├── reports/         # Generated reports
│   ├── logs/            # Execution logs
│   └── exports/         # Export bundles
└── session-b5c6d7e8/
    └── ...
```

---

### `artifacts/`

Global artifact index. References artifacts from all sessions for cross-session queries.

---

### `knowledge/`

Knowledge pack data — structured intelligence bases that inform the Decision Engine:

```
~/.zephyx/knowledge/
├── cve-patterns.toml
├── service-fingerprints.toml
└── tool-flags.toml
```

---

### `rules/`

Rule pack definitions used by the Decision Engine:

```
~/.zephyx/rules/
├── ctf-recon.toml
├── web-fuzzing.toml
├── privesc-linux.toml
├── active-directory.toml
└── api-security.toml
```

---

### `database/`

SQLite database for all persistent platform data:

```
~/.zephyx/database/
└── zephyx.db
```

Contains: findings, tasks, journal entries, snapshots, log entries, timeline events, workflow stats.

---

### `logs/`

Platform-level log files:

```
~/.zephyx/logs/
├── zpx-2026-07-25.log
└── zpx-2026-07-24.log
```

---

### `cache/`

Temporary cache files. Safe to delete — Zephyx will regenerate them:

```
~/.zephyx/cache/
├── tool-catalog.json
├── marketplace-index.json
└── rule-compiled.bin
```

Clear with: `zpx workspace clean`

---

### `models/`

Local AI model files (optional — only if using the AI layer):

```
~/.zephyx/models/
├── llama3-8b.gguf
└── mistral-7b.gguf
```

---

### `templates/`

Workflow and pipeline YAML templates:

```
~/.zephyx/templates/
├── workflows/
│   ├── htb-linux.yaml
│   └── htb-windows.yaml
└── pipelines/
    ├── default-recon.yaml
    └── web-full.yaml
```

---

### `reports/`

Global report archive — copies of all generated reports across all sessions:

```
~/.zephyx/reports/
├── htb-lame-writeup-2026-07-25.md
└── thm-room-report-2026-07-24.html
```

---

## Workspace Commands

```bash
# View workspace paths and stats
zpx workspace info

# Clear stale cache files
zpx workspace clean
```

---

## Workspace Initialization

The workspace is created on first run of any `zpx` command:

```bash
zpx init --name "MyTarget" --ip 10.10.10.1
# or just
zpx doctor
```

If you need to reset the workspace:
```bash
rm -rf ~/.zephyx
zpx init
```

> **Warning:** This deletes all sessions, findings, and artifacts. Back up important sessions first.
