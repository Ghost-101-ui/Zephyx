# Security Policy

## Supported Versions

We accept security reports for the following versions of Zephyx:

| Version | Supported |
|---|---|
| 0.6.x (latest) | ✅ Yes |
| 0.5.x | ⚠️ Critical fixes only |
| < 0.5 | ❌ No |

---

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in Zephyx, please report it responsibly:

### Option 1: GitHub Private Security Advisory (Preferred)

Use GitHub's [Private Security Advisory](https://github.com/Ghost-101-ui/Zephyx/security/advisories/new) feature to report the vulnerability confidentially.

### Option 2: Email

Send a detailed report to **security@zephyx.dev** (placeholder address) with the subject line:
```
[SECURITY] Brief description of the vulnerability
```

### What to Include

Please include as much of the following as possible:

- Type of vulnerability (e.g., path traversal, command injection, privilege escalation)
- Affected component and version
- Steps to reproduce
- Proof of concept (if available)
- Potential impact assessment
- Any suggested mitigations

---

## Response Timeline

| Stage | Timeline |
|---|---|
| Acknowledgement | Within 48 hours |
| Initial assessment | Within 5 business days |
| Status update | Within 10 business days |
| Patch / disclosure | Coordinated with reporter |

---

## Disclosure Policy

We follow **coordinated disclosure**:

1. Reporter submits the vulnerability privately
2. We confirm and assess the issue
3. We develop and test a fix
4. We release the fix and credit the reporter (if desired)
5. We publish a security advisory

We ask that reporters give us a reasonable amount of time (typically 90 days) to address the issue before any public disclosure.

---

## Scope

### In Scope

- Command injection or arbitrary code execution vulnerabilities in `zpx-core` or `zpx-cli`
- Path traversal vulnerabilities in session/artifact/workspace management
- Authentication bypasses in the REST API
- Privilege escalation within the platform itself
- Dependency vulnerabilities with a direct impact on Zephyx users

### Out of Scope

- Vulnerabilities in the security tools that Zephyx orchestrates (nmap, ffuf, etc.) — report those to their respective projects
- Issues requiring physical access to the machine
- Social engineering
- Denial of service attacks on a single-user local tool

---

## Security by Design

Zephyx is designed with security-conscious defaults:

- **AI is advisory-only** — Zephyx AI providers never execute commands autonomously
- **No network calls by default** — Zephyx operates fully offline
- **Local-only data** — All session data stays in `~/.zephyx/` on your machine
- **No telemetry** — Zephyx does not collect or transmit usage data

---

## Acknowledgements

We thank all security researchers who responsibly disclose vulnerabilities. Credit will be given in the security advisory and CHANGELOG unless the reporter prefers to remain anonymous.
