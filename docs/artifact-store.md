# Artifact Store

The Artifact Store manages all raw tool outputs produced during an assessment.

---

## What Is an Artifact?

An artifact is any file produced by a security tool:
- Nmap XML scan output
- FFUF JSON directory enumeration results
- LinPEAS HTML output
- Screenshots
- Downloaded files

Each artifact is:
- Stored with a unique ID
- Linked to the finding it produced
- Checksummed (SHA-256) for integrity
- Tagged with MIME type and source tool

---

## Artifact Data Model

```rust
pub struct Evidence {
    pub id: String,
    pub finding_id: String,       // Which finding this evidence supports
    pub tool_name: String,        // Which tool produced it
    pub raw_output_path: String,  // Path in session artifacts/ dir
    pub checksum_sha256: String,  // Integrity verification
    pub mime_type: String,        // "application/xml", "application/json", etc.
    pub timestamp: DateTime<Utc>,
}
```

---

## Artifact Location

Artifacts are stored in the session directory:

```
~/.zephyx/sessions/session-{id}/
├── artifacts/
│   ├── art-a1b2c3d4_nmap_scan.xml
│   ├── art-e5f6g7h8_ffuf_dirs.json
│   └── art-i9j0k1l2_linpeas.html
└── evidence/
    ├── art-a1b2c3d4.evidence.json   # Metadata + checksum
    ├── art-e5f6g7h8.evidence.json
    └── art-i9j0k1l2.evidence.json
```

---

## Commands

```bash
# List all artifacts in the active session
zpx artifact list

# Export an artifact to a directory
zpx artifact export art-a1b2c3d4 ./output/
```

**Output of `zpx artifact list`:**
```
Managed Artifacts in Active Session:
  • art-a1b2c3d4  [XML]  nmap_scan_output.xml (14.2 KB)
  • art-e5f6g7h8  [JSON] ffuf_directories.json (8.1 KB)
```

---

## Integrity Verification

Every artifact has a SHA-256 checksum. Reports include these checksums so findings can be verified against the original tool output:

```
Evidence:
  art-a1b2c3d4: nmap_scan_output.xml
    SHA256: abc123def456...
    Tool:   nmap
    Time:   2026-07-25 17:05:00 UTC
```
