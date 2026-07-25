# Reporting

Zephyx can generate professional assessment reports in multiple formats from a single command.

---

## Overview

The Report Engine aggregates:
- All findings discovered during the session
- The attack timeline (journal entries)
- Recommendations accepted and completed
- Evidence references with checksums
- Workflow phase summary

---

## Generating a Report

```bash
# Markdown (default — great for GitHub / HackMD writeups)
zpx report --output writeup.md --format markdown

# HTML (standalone, shareable)
zpx report --output report.html --format html

# JSON (machine-readable, for integrations)
zpx report --output findings.json --format json

# CSV (spreadsheet-friendly)
zpx report --output data.csv --format csv
```

---

## Report Sections

### Markdown Report Structure

```markdown
# Zephyx Security Assessment Report
## Target: [name] ([ip])
## Date: [timestamp]
## Assessor: [session name]

---

## Executive Summary
[Phase completion, total findings, critical findings count]

## Findings

| # | Type | Details | Severity | Confidence | Tool | Timestamp |
|---|---|---|---|---|---|---|
| 1 | Port | 80/tcp — http (Apache 2.4.7) | Medium | 90% | nmap | 2026-07-25 |
| 2 | Vulnerability | CVE-2011-2523 vsftpd Backdoor | Critical | 95% | searchsploit | 2026-07-25 |
| 3 | Credential | root via backdoor exploit | Critical | 100% | manual | 2026-07-25 |

## Attack Timeline

[Chronological list of all executed tasks and decisions]

## Recommendations

[All accepted recommendations with reasoning]

## Evidence

[SHA-256 checksums for all collected artifacts]

## Workflow Summary

[Phases completed, progress percentage, total time]
```

---

## Finding Types in Reports

| Finding Kind | Report Column | Description |
|---|---|---|
| `Port` | Open port with service/version | Network exposure |
| `HttpEndpoint` | URL, status code, content length | Web surface |
| `Vulnerability` | CVE, name, severity, details | Exploitable weakness |
| `Credential` | Service, username, password/hash | Authentication data |
| `Hash` | Hash type, value, username | Captured hashes |
| `Flag` | Type, value | CTF flag captured |
| `SmbShare` | Share name, permissions | SMB exposure |
| `SuidBinary` | Binary path, owner | Privesc vector |
| `Loot` | Name, path, description | Collected data |

---

## Evidence Integrity

Every artifact in the report includes its SHA-256 checksum:

```
Evidence:
  artifact-a1b2c3d4: nmap_scan_output.xml (SHA256: abc123...)
  artifact-e5f6g7h8: ffuf_output.json (SHA256: def456...)
```

This allows a third party to verify that reported findings match the raw tool output.

---

## Report Location

Reports are saved to:
1. The path you specify with `--output`
2. Automatically copied to `~/.zephyx/sessions/<session-id>/reports/`
3. Archived in `~/.zephyx/reports/` (global archive)
