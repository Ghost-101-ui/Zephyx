# Complete CLI Reference

Full documentation for every `zpx` command and subcommand.

---

## Global Options

```bash
zpx --help                  # Show help
zpx --version               # Show version (0.6.0)
```

---

## `zpx init`

Initialize a target workspace and the central `~/.zephyx/` environment.

```bash
zpx init [OPTIONS]

OPTIONS:
  -n, --name <NAME>    Target machine name [default: TargetBox]
  -i, --ip <IP>        Target IP address [default: 127.0.0.1]
```

**Example:**
```bash
zpx init --name "HTB-Lame" --ip 10.10.10.3
```

**Output:**
```
Initializing Zephyx central workspace at /home/user/.zephyx
Target workspace successfully created at: ./HTB-Lame
```

---

## `zpx doctor`

Run system self-diagnostics. Checks tool availability, workspace health, and database connectivity.

```bash
zpx doctor
```

**Output:**
```
Running Zephyx System Doctor (v0.6.0):
  [✓] Central workspace initialized
  [✓] nmap — INSTALLED at /usr/bin/nmap
  [✗] ffuf — NOT INSTALLED
  [✓] Scheduler: ready (max 4 concurrent)
```

---

## `zpx scan`

Run automated recon and enumeration against a target.

```bash
zpx scan --ip <IP>

OPTIONS:
  -i, --ip <IP>    Target IP address (required)
```

**Example:**
```bash
zpx scan --ip 10.10.10.3
```

---

## `zpx session`

Manage CTF and pentest sessions.

### `zpx session create`

```bash
zpx session create [OPTIONS]

OPTIONS:
  -n, --name <NAME>      Session name [default: CTF-Assessment]
  -t, --target <TARGET>  Target IP address [default: 10.10.10.123]
```

**Example:**
```bash
zpx session create --name "HTB-Lame" --target 10.10.10.3
```

**Output:**
```
Created new Zephyx session!
  ID:        session-a1b2c3d4
  Name:      HTB-Lame
  Target:    10.10.10.3
  Directory: ~/.zephyx/sessions/session-a1b2c3d4
```

### `zpx session list`

```bash
zpx session list
```

**Output:**
```
Zephyx Recorded Sessions (2)
  • session-a1b2c3d4  [Active ] Target: 10.10.10.3     Created: 2026-07-25 17:00
  • session-b5c6d7e8  [Completed] Target: 10.10.10.5   Created: 2026-07-24 09:30
```

### `zpx session resume`

```bash
zpx session resume <SESSION_ID>
```

**Example:**
```bash
zpx session resume session-a1b2c3d4
```

---

## `zpx workflow`

Manage the CTF workflow state machine.

### `zpx workflow list`

```bash
zpx workflow list
```

**Output:**
```
Built-in CTF Workflow Templates (9)
  • htb-linux             [Linux  ] Hack The Box Linux Machine Workflow
  • htb-windows           [Windows] Hack The Box Windows Machine Workflow
  • thm-web               [Any    ] TryHackMe Web Application Workflow
  • portswigger           [Web    ] PortSwigger Academy Web Security Workflow
  • active-directory      [Windows] Active Directory Domain Assessment
  • linux-privesc         [Linux  ] Linux Privilege Escalation Workflow
  • windows-privesc       [Windows] Windows Privilege Escalation Workflow
  • web-assessment        [Web    ] Comprehensive Web Application Audit
  • api-assessment        [API    ] REST / GraphQL API Security Assessment
```

### `zpx workflow start <template>`

```bash
zpx workflow start htb-linux
```

### `zpx workflow status`

```bash
zpx workflow status
```

**Output:**
```
Active Phase: Service Enumeration (30%)
Description:  Service banner grabbing, web directory fuzzing, SMB enumeration
Prerequisites: [Reconnaissance]
Supported Plugins: [nmap, ffuf, gobuster, enum4linux]
Next Phases: [TechnologyDetection]
Estimated Duration: ~7 minutes
```

### `zpx workflow pause | resume | reset`

```bash
zpx workflow pause
zpx workflow resume
zpx workflow reset
```

---

## `zpx tool`

Manage security tools in the Zephyx catalog.

### `zpx tool list`

```bash
zpx tool list
```

**Output:**
```
Zephyx Tool Manager Catalog (12)
  • nmap           [Installed    ] /usr/bin/nmap
  • ffuf           [Installed    ] ~/.zephyx/bin/ffuf
  • gobuster       [NOT INSTALLED] NOT INSTALLED
  • rustscan       [Installed    ] /usr/local/bin/rustscan
```

### `zpx tool verify <name>`

```bash
zpx tool verify nmap
# nmap is verified and accessible.
```

### `zpx tool install <name>`

```bash
zpx tool install ffuf
# Successfully installed tool 'ffuf' to ~/.zephyx/bin/ffuf
```

### `zpx tool update <name>`

```bash
zpx tool update gobuster
# Tool 'gobuster' updated.
```

---

## `zpx pack`

Install bundled tool packs.

### `zpx pack list`

```bash
zpx pack list
```

**Output:**
```
Available Tool Packs:
  • recon     - Network Discovery & Fingerprinting (nmap, rustscan, httpx, whatweb)
  • web       - Web Application & Fuzzing (gobuster, ffuf, feroxbuster, sqlmap)
  • ad        - Active Directory & Domain Audit (netexec, bloodhound, crackmapexec, kerbrute)
  • privesc   - Privilege Escalation Scripts (linpeas, winpeas, privesccheck, pspy)
```

### `zpx pack install <name>`

```bash
zpx pack install recon
# Installing tool pack 'recon'...
# [✓] Installed nmap
# [✓] Installed rustscan
# Pack 'recon' installation complete.
```

---

## `zpx plugin`

Manage plugins and the marketplace.

| Subcommand | Description |
|---|---|
| `zpx plugin list` | List all registered plugins |
| `zpx plugin search <query>` | Search the marketplace |
| `zpx plugin info <name>` | Show plugin manifest details |
| `zpx plugin install <name>` | Install from marketplace |
| `zpx plugin uninstall <name>` | Uninstall a plugin |
| `zpx plugin enable <name>` | Enable a disabled plugin |
| `zpx plugin disable <name>` | Disable without uninstalling |
| `zpx plugin update` | Update all plugins |
| `zpx plugin verify` | Verify all plugin manifests |
| `zpx plugin reload` | Reload plugins dynamically |
| `zpx plugin doctor` | Run plugin health checks |
| `zpx plugin publish <name>` | Publish plugin to marketplace |

---

## `zpx artifact`

Manage output artifacts from tool executions.

```bash
zpx artifact list                           # List artifacts in active session
zpx artifact export <id> <output_dir>       # Export an artifact
```

---

## `zpx report`

Generate assessment reports.

```bash
zpx report [OPTIONS]

OPTIONS:
  -o, --output <FILE>    Output filename [default: writeup.md]
  -f, --format <FORMAT>  Output format: markdown, json, csv, html [default: markdown]
```

**Examples:**
```bash
zpx report --output writeup.md --format markdown
zpx report --output findings.json --format json
zpx report --output report.html --format html
zpx report --output data.csv --format csv
```

---

## `zpx rules`

Manage deterministic rule packs.

```bash
zpx rules list                  # List all rule packs (enabled/disabled status)
zpx rules enable <pack>         # Enable a rule pack
zpx rules disable <pack>        # Disable a rule pack
zpx rules info <pack>           # Show rule pack details
```

**Output of `zpx rules list`:**
```
Zephyx Deterministic Rule Packs (5)
  • ctf-recon         [ENABLED ] v1.2.0  (12 rules) - CTF Reconnaissance Rules
  • web-fuzzing       [ENABLED ] v1.0.0  (8 rules)  - Web Fuzzing Triggers
  • privesc-linux     [ENABLED ] v2.1.0  (15 rules) - Linux Privesc Patterns
  • active-directory  [DISABLED] v1.0.0  (20 rules) - AD Assessment Rules
  • api-security      [ENABLED ] v1.0.0  (6 rules)  - API Security Patterns
```

---

## `zpx workspace`

Manage the central `~/.zephyx/` workspace.

```bash
zpx workspace info      # Show workspace paths and stats
zpx workspace clean     # Clear stale cache files
```

**Output of `zpx workspace info`:**
```
Zephyx Central Workspace Info:
  Root Directory:    ~/.zephyx
  Managed Binaries:  ~/.zephyx/bin
  SQLite Master DB:  ~/.zephyx/database/zephyx.db
```

---

## `zpx profile`

Manage execution profiles.

```bash
zpx profile list            # List available profiles
zpx profile use <name>      # Switch active profile
```

---

## `zpx scheduler`

Inspect the task scheduler.

```bash
zpx scheduler status
```

**Output:**
```
Zephyx Task Scheduler Status:
  Queued Tasks:       2
  Running Tasks:      1
  Completed Tasks:    14
  Failed Tasks:       0
  Max Concurrency:    4
```

---

## `zpx resource`

Monitor system resource usage.

```bash
zpx resource status
```

**Output:**
```
Zephyx Resource Monitor:
  CPU Usage:    12.4%
  Memory Usage: 342 MB / 8192 MB
  Active Scans: 1
  Throttled:    false
```

---

## `zpx capability`

Work with the capability registry.

```bash
zpx capability list                    # List all capability → tool mappings
zpx capability resolve <name>          # Find the best installed tool for a capability
```

**Example:**
```bash
zpx capability resolve web_directory_bruteforce
# Capability 'web_directory_bruteforce' resolved to candidate 'ffuf' at path: ~/.zephyx/bin/ffuf
```

---

## `zpx tasks`

Manage background task execution.

```bash
zpx tasks list                  # List all tasks
zpx tasks pause <id>            # Pause a running task
zpx tasks resume <id>           # Resume a paused task
zpx tasks cancel <id>           # Cancel a task
zpx tasks retry <id>            # Retry a failed task
zpx tasks logs <id>             # Show task stdout/stderr logs
```

---

## `zpx pipeline`

Manage automation pipelines.

```bash
zpx pipeline create <name>      # Create a new pipeline
zpx pipeline list               # List available pipelines
zpx pipeline run <name>         # Execute a pipeline
zpx pipeline validate <path>    # Validate a pipeline YAML file
zpx pipeline info <name>        # Show pipeline details
zpx pipeline export <name>      # Export pipeline as YAML
```

---

## `zpx snapshot`

Manage workspace state snapshots.

```bash
zpx snapshot create             # Create a snapshot of current workspace
zpx snapshot list               # List all snapshots
zpx snapshot restore <id>       # Restore a snapshot
zpx snapshot delete <id>        # Delete a snapshot
```

---

## `zpx api`

Start the internal REST API server.

```bash
zpx api --port 8080
# Starting Zephyx Internal REST API on port 8080...
# Press Ctrl+C to stop API server.
```

---

## `zpx replay`

Replay recorded workspace execution timeline.

```bash
zpx replay <workspace>
```

**Output:**
```
Replaying workspace execution history for 'TargetBox':
  [ 1] User       nmap        Running  -> nmap -sV 10.10.10.3 (completed)
  [ 2] Engine     ffuf        Running  -> ffuf -u http://10.10.10.3/FUZZ (completed)
```

---

## `zpx graph`

Show or export the knowledge graph.

```bash
zpx graph show
# Outputs Mermaid diagram of current attack graph
```

---

## `zpx context`

Show aggregated target context.

```bash
zpx context show
```

**Output:**
```
Zephyx Target Context Engine:
  Target:     TargetBox (10.10.10.3)
  Open Ports: [21, 22, 80, 139, 445]
```

---

## `zpx memory`

Manage long-term execution memory.

```bash
zpx memory list
```

**Output:**
```
Zephyx Long-Term Knowledge Memory:
  - Tool: nmap         | Flags: -sV -sC -p-           | Time: 45s
  - Tool: ffuf         | Flags: -w common.txt -mc 200  | Time: 30s
```

---

## `zpx plan`

Build a dynamic workflow plan for a target.

```bash
zpx plan --ip 10.10.10.3
```

**Output:**
```
Zephyx Workflow Planner for Target '10.10.10.3':
  Plan: Automated CTF Assessment
  [Step 1] Fast Port Scan           -> rustscan --addresses 10.10.10.3 -r 1-65535
  [Step 2] Service Version Scan     -> nmap -sV -sC -p 21,22,80,445 10.10.10.3
  [Step 3] Web Directory Fuzz       -> ffuf -w /usr/share/wordlists/common.txt -u http://10.10.10.3/FUZZ
```

---

## `zpx explain`

Explain findings, tools, or workflows using the explainability engine.

```bash
zpx explain <target>

TARGETS:
  finding    Explain the most recent finding
  workflow   Explain the current workflow state
  tool       Explain a specific tool
```

**Example output:**
```
Zephyx Explainability Engine for 'finding':
  Title:       Web Directory Bruteforce Recommended
  Reason:      HTTP 80/443 exposed; Apache/PHP banner detected
  Confidence:  95%
  Rule:        RulePackMatch::WebDirectoryBruteforce
```

---

## `zpx ai`

AI provider diagnostics.

```bash
zpx ai doctor
```

---

## `zpx model`

Manage local LLM models.

```bash
zpx model list
```

---

## `zpx update`

Update Zephyx tool catalog and registry.

```bash
zpx update
# Updating Zephyx tool catalog and central registry...
# All catalogs up to date.
```

---

## `zpx config`

View Zephyx system configuration.

```bash
zpx config
```

---

## `zpx dashboard`

Launch the interactive Ratatui TUI dashboard.

```bash
zpx dashboard [OPTIONS]

OPTIONS:
  -n, --name <NAME>    Target name [default: TargetBox]
  -i, --ip <IP>        Target IP [default: 10.10.10.123]
```

**Example:**
```bash
zpx dashboard --name "HTB-Lame" --ip 10.10.10.3
```

---

## `zpx decision`

Inspect decision engine outcomes.

```bash
zpx decision inspect
```
