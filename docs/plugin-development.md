# Plugin Development Guide

This guide explains how to create a custom Zephyx plugin from scratch.

---

## Overview

Plugins in Zephyx are **tool integrations** that:
1. Declare what capabilities they provide
2. Describe how to verify the tool is installed
3. Provide an output parser that converts raw tool output into typed `Finding` objects
4. Register themselves in the capability registry

---

## Plugin Manifest v2

Every plugin starts with a `manifest.toml` file:

```toml
[plugin]
id = "my-custom-scanner"
name = "my-custom-scanner"
version = "1.0.0"
minimum_version = "0.6.0"          # Minimum Zephyx version
description = "Custom port scanner with JSON output"
author = "Your Name <you@example.com>"
category = "Reconnaissance"         # Reconnaissance | Enumeration | Exploitation | etc.
license = "MIT"
homepage = "https://github.com/yourname/my-custom-scanner"

# Platforms this plugin works on
supported_platforms = ["Linux", "macOS", "Windows"]

# Capabilities this plugin provides
capabilities = ["port_scanning", "service_detection"]

# Command to verify the tool is installed and working
verification_command = "my-scanner --version"

# If Zephyx manages this binary (optional)
managed_binary = "~/.zephyx/bin/my-scanner"

# Install command for auto-install (optional)
install_command = "apt install my-scanner"
```

---

## Capability Registration

When your plugin loads, Zephyx registers its capabilities in the Capability Registry:

```
Capability: port_scanning → my-custom-scanner (if nmap/rustscan not available)
Capability: service_detection → my-custom-scanner
```

This means when the Decision Engine needs `port_scanning`, it may select your plugin if it's the best available option.

---

## Output Parser

The parser converts raw tool output into typed `Finding` objects. Write a parser in Rust using the SDK:

```rust
// In your plugin's parser module
use zpx_core::models::{Finding, FindingKind};
use anyhow::Result;

pub fn parse_output(raw_output: &str, target_ip: &str) -> Result<Vec<Finding>> {
    let mut findings = Vec::new();

    // Parse your tool's output format (JSON, XML, text)
    for line in raw_output.lines() {
        if let Some(port_str) = line.strip_prefix("OPEN:") {
            let port: u16 = port_str.trim().parse()?;
            findings.push(Finding::new(
                target_ip,
                "my-custom-scanner",
                FindingKind::Port {
                    port,
                    protocol: "tcp".to_string(),
                    service: "unknown".to_string(),
                    version: None,
                },
            ));
        }
    }

    Ok(findings)
}
```

---

## Plugin Directory Structure

```
my-custom-scanner/
├── manifest.toml       # Required: plugin metadata and capabilities
├── parser.rs           # Output parser (converts raw → Finding)
├── config.toml         # Plugin-specific configuration (optional)
├── wordlists/          # Bundled wordlists (optional)
└── README.md           # Plugin documentation
```

---

## Built-in Plugin Examples

Study these built-in plugins to understand patterns:

- **nmap** (`zpx-core/src/plugin/parsers/nmap.rs`) — XML parsing, port findings
- Parsers convert tool-specific XML/JSON into the universal `Finding` type

---

## Testing Your Plugin

```bash
# Verify the plugin loads correctly
zpx plugin doctor

# List and confirm your plugin appears
zpx plugin list

# Check capability resolution includes your plugin
zpx capability resolve port_scanning

# Run a full test scan using your plugin
zpx scan --ip 127.0.0.1
```

---

## Publishing to the Marketplace

Once your plugin is ready:

```bash
# Publish to the marketplace
zpx plugin publish my-custom-scanner
```

Requirements for marketplace publication:
- Valid `manifest.toml` with all required fields
- `verification_command` must work
- Parser must produce valid `Finding` objects
- `README.md` with usage instructions
- Tests pass

---

## Plugin SDK

See [plugin-sdk.md](plugin-sdk.md) for the full SDK API reference, including:
- `FindingBuilder` — fluent API for building findings
- `ArtifactWriter` — write raw output to the artifact store
- `CapabilityResolver` — query the registry from within a plugin
- `EvidenceStore` — attach evidence to findings with checksums
