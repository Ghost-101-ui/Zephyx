# Configuration

This document explains how to configure Zephyx for your environment.

---

## Configuration File

Zephyx stores its main configuration at:

```
~/.zephyx/config/config.toml
```

The file is created automatically on first run with sensible defaults.

---

## Full Configuration Reference

```toml
# ~/.zephyx/config/config.toml

[general]
# Automatically install missing tools when needed
auto_install_missing_tools = true

# Which execution profile to use by default
default_execution_profile = "default"

# Internal event bus buffer capacity
event_bus_capacity = 1024

# Platform adapter (auto-detected)
# Values: "Linux", "Windows", "macOS"
platform_adapter = "Linux"

[binary_resolution]
# Order to search for tools
# Values: "System" (PATH), "Managed" (~/.zephyx/bin), "AutoInstall"
order = ["System", "Managed", "AutoInstall"]

# Directory where Zephyx manages its own tool binaries
bin_dir = "~/.zephyx/bin"

[ai]
# Enable optional AI advisory layer
enabled = false

# AI provider
# Values: "ollama", "openai_compatible", "none"
provider = "none"

# Model name (for Ollama: "llama3", "mistral", etc.)
model = ""

# Ollama base URL (if using Ollama)
ollama_url = "http://localhost:11434"

# AI is ALWAYS advisory-only — this cannot be changed
advisory_only = true

[scheduler]
# Maximum concurrent tool executions
max_concurrency = 4

# Default task timeout in seconds
default_timeout_secs = 300

# Stop scheduling new tasks above this CPU usage threshold
cpu_throttle_percent = 80.0

# Stop scheduling new tasks above this memory usage (MB)
memory_limit_mb = 2048

[reporting]
# Default output format
# Values: "markdown", "html", "json", "csv"
default_format = "markdown"

# Default output filename
default_output = "writeup.md"

# Include evidence checksums in report
include_evidence_checksums = true

[workspace]
# Path to the central workspace root
root_dir = "~/.zephyx"

# Maximum age of cached items before they expire (hours)
cache_ttl_hours = 24
```

---

## Execution Profiles

Profiles configure how aggressively tools run. Switch profiles with:

```bash
zpx profile list
zpx profile use aggressive
```

### Built-in Profiles

#### `default`
Balanced for most use cases. Safe for lab environments.
```
Threads: 4 | Timeout: 300s | Rate limit: moderate
```

#### `stealth`
Minimal noise. Useful when you want to avoid detection in realistic environments.
```
Threads: 1 | Timeout: 600s | Rate limit: very low | Delay: 500ms between tasks
```

#### `aggressive`
Maximum speed. Use in dedicated lab environments where noise doesn't matter.
```
Threads: 16 | Timeout: 120s | Rate limit: none
```

#### `ctf`
Optimized for CTF speed without being completely loud.
```
Threads: 8 | Timeout: 240s | Rate limit: low
```

#### `lab`
For local lab environments (VirtualBox, Docker, local VMs).
```
Threads: 4 | Timeout: 180s | Rate limit: low
```

---

## Workspace Configuration

The workspace path can be overridden with an environment variable:

```bash
export ZEPHYX_HOME=/custom/path/to/workspace
zpx doctor
```

---

## Rule Pack Configuration

Enable or disable rule packs:

```bash
zpx rules list
zpx rules enable ctf-recon
zpx rules disable web-aggressive
```

Rule packs are stored in `~/.zephyx/rules/`.

---

## Plugin Configuration

Plugin settings are managed per-plugin in `~/.zephyx/plugins/<plugin-name>/config.toml`.

---

## Environment Variables

| Variable | Description | Default |
|---|---|---|
| `ZEPHYX_HOME` | Override workspace root | `~/.zephyx` |
| `ZEPHYX_PROFILE` | Override active profile | `default` |
| `ZEPHYX_LOG_LEVEL` | Log verbosity (`trace`,`debug`,`info`,`warn`,`error`) | `info` |
| `RUST_LOG` | Standard Rust log filter | — |

---

## Viewing Current Config

```bash
zpx config
```

Output:
```
Zephyx Configuration (~/.zephyx/config/config.toml):
  auto_install_missing_tools: true
  binary_resolution_order:   [System, Managed, AutoInstall]
  default_execution_profile:  default
  event_bus_capacity:         1024
  platform_adapter:           Linux (APT)
```
