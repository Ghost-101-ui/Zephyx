# Supported Platforms

Zephyx is designed to run on major operating systems. Support levels vary by platform.

---

## Support Tiers

| Tier | Description |
|---|---|
| ✅ **Tier 1** | Fully supported, tested in CI, binaries provided |
| ⚠️ **Tier 2** | Best-effort support, community tested |
| 🔮 **Tier 3** | Planned, not yet validated |

---

## Linux

| Distribution | Version | Tier | Notes |
|---|---|---|---|
| Kali Linux | 2023.x+ | ✅ Tier 1 | Primary development target |
| Parrot OS | 5.x+ | ✅ Tier 1 | Fully supported |
| Ubuntu | 22.04 LTS+ | ✅ Tier 1 | Tested in CI |
| Debian | 11 (Bullseye)+ | ✅ Tier 1 | Tested in CI |
| Fedora | 38+ | ⚠️ Tier 2 | Community tested |
| Arch Linux | Rolling | ⚠️ Tier 2 | Community tested |
| Alpine Linux | 3.18+ | ⚠️ Tier 2 | May require musl build |

---

## Windows

| Version | Tier | Notes |
|---|---|---|
| Windows 11 | ✅ Tier 1 | Natively supported |
| Windows 10 (1903+) | ✅ Tier 1 | Natively supported |
| Windows Server 2019+ | ⚠️ Tier 2 | Community tested |
| WSL2 (any distro) | ✅ Tier 1 | Recommended for full tool compatibility |

> **Note:** Many security tools orchestrated by Zephyx (nmap, ffuf, linpeas) are Linux-native. For Windows users, WSL2 is strongly recommended for the best experience.

---

## macOS

| Version | Tier | Notes |
|---|---|---|
| macOS 14 (Sonoma) | ✅ Tier 1 | Apple Silicon + Intel |
| macOS 13 (Ventura) | ✅ Tier 1 | Apple Silicon + Intel |
| macOS 12 (Monterey) | ⚠️ Tier 2 | Intel only tested |

---

## Architecture Support

| Architecture | Support |
|---|---|
| x86_64 (AMD64) | ✅ Fully supported |
| aarch64 (ARM64) | ✅ Fully supported (Apple Silicon, Raspberry Pi) |
| armv7 | ⚠️ Community tested |
| i686 (32-bit x86) | ❌ Not supported |

---

## Rust Version Requirement

Zephyx requires **Rust 1.75.0 or newer**. We track the latest stable Rust release.

```bash
rustup update stable
rustc --version
```

---

## Security Tool Compatibility

The tools Zephyx can orchestrate have their own platform requirements:

| Tool | Linux | Windows | macOS |
|---|---|---|---|
| nmap | ✅ | ✅ | ✅ |
| ffuf | ✅ | ✅ | ✅ |
| gobuster | ✅ | ✅ | ✅ |
| rustscan | ✅ | ⚠️ | ✅ |
| linpeas | ✅ | ❌ | ❌ |
| winpeas | ❌ | ✅ | ❌ |
| enum4linux | ✅ | ⚠️ WSL | ❌ |
| netexec | ✅ | ⚠️ | ⚠️ |
| sqlmap | ✅ | ✅ | ✅ |
| nikto | ✅ | ⚠️ | ✅ |
| whatweb | ✅ | ⚠️ | ✅ |

These tools are not bundled with Zephyx. See [docs/tool-manager.md](docs/tool-manager.md) for installation guidance.
