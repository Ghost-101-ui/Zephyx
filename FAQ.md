# Frequently Asked Questions

---

## General

**Q: What is Zephyx?**

A: Zephyx is a workflow-driven cybersecurity operating platform built in Rust. It orchestrates security tools, manages assessment sessions, tracks findings, and generates professional reports from a unified CLI.

---

**Q: Is Zephyx a hacking tool?**

A: Zephyx itself does not perform attacks. It orchestrates and organizes other security tools (like `nmap`, `ffuf`, `linpeas`, etc.) that you explicitly choose to run, within authorized environments. Think of it as a project manager for security assessments, not an attack tool.

---

**Q: Who is Zephyx for?**

A: CTF players, security students, ethical hackers, red teamers, security researchers, and anyone doing authorized security work. It is designed for legal, authorized engagements only.

---

**Q: Is Zephyx free?**

A: Yes. Zephyx is open-source and free under the MIT / Apache 2.0 dual license.

---

**Q: Does Zephyx work offline?**

A: Yes. Zephyx is fully offline by default. The AI layer is optional and also supports local LLMs via Ollama (no internet required). No telemetry or usage data is collected.

---

## Installation

**Q: What do I need to install Zephyx?**

A: Rust 1.75+ and Cargo. See [INSTALL.md](INSTALL.md) for full instructions.

---

**Q: Does Zephyx work on Windows?**

A: Yes. Zephyx compiles and runs on Windows. However, many security tools it can orchestrate (nmap, ffuf, linpeas, etc.) are primarily Linux-native. For best experience on Windows, use WSL2.

---

**Q: Can I use Zephyx on Kali Linux?**

A: Absolutely. Kali is one of the primary target environments. Install Rust via rustup and build from source.

---

## Usage

**Q: How do I start my first assessment?**

A:
```bash
zpx init --name "MyTarget" --ip 10.10.10.123
zpx session create --name "First-Session" --target 10.10.10.123
zpx scan --ip 10.10.10.123
zpx workflow status
zpx decision inspect
```

---

**Q: Where is my data stored?**

A: All data is stored locally in `~/.zephyx/`. Each session has its own isolated subdirectory. Nothing leaves your machine.

---

**Q: How do I generate a report?**

A:
```bash
zpx report --output writeup.md --format markdown
zpx report --output findings.json --format json
zpx report --output report.html --format html
```

---

**Q: What is the difference between Managed and System tools?**

A: **System tools** are already installed on your machine (resolved from `$PATH`). **Managed tools** are installed and maintained by Zephyx itself in `~/.zephyx/bin/`. Zephyx tries system tools first, then managed, then auto-installs if configured.

---

**Q: What are Workflow Templates?**

A: Pre-defined methodology configurations for common engagement types (HTB Linux, HTB Windows, TryHackMe Web, Active Directory, etc.). They set the initial phase, enabled plugins, and scope.

---

## Plugins

**Q: How do I install a plugin?**

A:
```bash
zpx plugin search "my-tool"
zpx plugin install my-tool
```

---

**Q: Can I write my own plugin?**

A: Yes. See [docs/plugin-development.md](docs/plugin-development.md) for the full guide.

---

**Q: What is the Plugin Marketplace?**

A: A built-in registry where community plugins can be discovered, installed, rated, and published. It is currently a foundational feature — a fully hosted marketplace is planned for a future release.

---

## AI

**Q: Does Zephyx require AI or an internet connection?**

A: No. Zephyx is fully functional offline with zero AI. The decision engine and recommendations are 100% deterministic.

---

**Q: What does the AI layer do?**

A: It provides optional advisory hints to the decision engine. AI **never** executes commands autonomously — all execution flows through the Scheduler and requires user action.

---

**Q: Which AI providers are supported?**

A: Currently: local Ollama models (offline) and a mock provider for testing. OpenAI-compatible APIs are planned.

---

## Troubleshooting

**Q: `cargo` is not found on Windows**

A: Use the full path: `& "$env:USERPROFILE\.cargo\bin\cargo.exe"`. Or add `%USERPROFILE%\.cargo\bin` to your system PATH.

---

**Q: A tool says "NOT INSTALLED" in `zpx tool list`**

A: The tool is not found in your PATH or `~/.zephyx/bin/`. Either install it manually or run:
```bash
zpx tool install <tool-name>
```

---

**Q: The TUI dashboard is blank**

A: Ensure your terminal supports 256 colors and is at least 80x24. Try:
```bash
TERM=xterm-256color zpx dashboard
```

---

**Q: I got a "session not found" error on resume**

A: Run `zpx session list` to see available sessions and their IDs. Session IDs look like `session-a1b2c3d4`.

---

For more help, see [docs/troubleshooting.md](docs/troubleshooting.md) or open a [GitHub Issue](https://github.com/Ghost-101-ui/Zephyx/issues).
