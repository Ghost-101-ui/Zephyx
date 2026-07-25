# Getting Started with Zephyx

Welcome to Zephyx! This guide will walk you through your first assessment from zero to a generated report.

---

## What You'll Do

1. Install Zephyx
2. Run system diagnostics
3. Initialize your workspace
4. Create an assessment session
5. Run a workflow
6. Review findings and recommendations
7. Generate a report

---

## Step 1: Install Zephyx

See [INSTALL.md](../INSTALL.md) for platform-specific instructions. The quick path:

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build Zephyx
git clone https://github.com/Ghost-101-ui/Zephyx.git
cd Zephyx
cargo build --release --bin zpx
sudo cp target/release/zpx /usr/local/bin/
```

---

## Step 2: Run System Diagnostics

```bash
zpx doctor
```

Expected output:
```
Running Zephyx System Doctor (v0.6.0):
  [✓] Central workspace initialized at ~/.zephyx
  [✓] SQLite database accessible
  [✓] nmap — INSTALLED at /usr/bin/nmap
  [✗] ffuf — NOT INSTALLED
  [✗] gobuster — NOT INSTALLED
  [✓] Scheduler: ready (max 4 concurrent)
  [✓] Event bus: ready (capacity 1024)
```

If tools are missing, install them:

```bash
zpx tool install ffuf
zpx pack install recon    # Install nmap, rustscan, httpx, whatweb
```

---

## Step 3: Initialize Your Workspace

```bash
zpx init --name "HTB-Lame" --ip 10.10.10.3
```

This creates:
- `~/.zephyx/` — central workspace
- A local target workspace directory
- Initial database tables

---

## Step 4: Create an Assessment Session

```bash
zpx session create --name "Lame-Assessment" --target 10.10.10.3
```

Output:
```
Created new Zephyx session!
  ID:        session-a1b2c3d4
  Name:      Lame-Assessment
  Target:    10.10.10.3
  Directory: ~/.zephyx/sessions/session-a1b2c3d4
```

---

## Step 5: Start a Workflow

```bash
# List available workflow templates
zpx workflow list

# Start the HTB Linux workflow
zpx workflow start htb-linux

# Check current phase
zpx workflow status
```

Output:
```
Active Phase: Reconnaissance (Network discovery, host reachability checks)
Phase Progress: 15%
Prerequisites: []
Supported Plugins: [rustscan, nmap, ping]
```

---

## Step 6: Run a Scan

```bash
zpx scan --ip 10.10.10.3
```

This runs the configured recon pipeline and stores findings in the session artifact store.

---

## Step 7: Review Recommendations

```bash
zpx decision inspect
```

Output:
```
Zephyx Deterministic Decision Engine:
  Decision: Run web directory enumeration
  Confidence: 92%
  Reason: HTTP port 80 detected, Apache banner identified
  Rule: RulePackMatch::WebDirectoryBruteforce
  Suggested: ffuf -w /usr/share/wordlists/dirb/common.txt -u http://10.10.10.3/FUZZ
```

---

## Step 8: Generate a Report

```bash
zpx report --output lame-writeup.md --format markdown
```

Your report is generated at `lame-writeup.md` with:
- Target summary
- Findings table
- Attack timeline
- Recommendations
- Evidence references

---

## What's Next?

- [Configuration Guide](configuration.md) — Customize profiles and settings
- [CLI Reference](cli-reference.md) — All available commands
- [Plugin Development](plugin-development.md) — Build your own tool plugins
- [Architecture](architecture.md) — Understand how Zephyx works internally
