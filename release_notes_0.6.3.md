## What's fixed in v0.6.3

### Bug Fixes
- **Persistent CLI runtime state** - all `zpx` commands now use the shared SQLite database at `~/.zephyx/database/zephyx.db` instead of an in-memory database. Tasks, snapshots, reports, and other persisted records now survive between CLI invocations.
- **Cryptographic artifact checksums** - artifact integrity metadata now uses SHA-256 checksums, replacing the previous non-cryptographic value. Artifact output directories are also created automatically when needed.
- **Removed DB init log line** from all command output — the `INFO zpx_core::db: Database schema initialized...` line no longer appears on any `zpx` command run. It is now a `DEBUG`-level message only visible when `RUST_LOG=debug` is set.
- **Linux self-update permission fix** — `zpx update --self` now automatically falls back to `~/.local/bin/zpx` when it cannot write to the current binary location (e.g. `/usr/local/bin`). No more cryptic failure. A helpful message is shown guiding you to run `sudo zpx update --self` for system-wide installation.

### Linux Upgrade
If you're on Kali/Parrot/Debian running v0.6.2 and want to upgrade in-place:

```bash
sudo zpx update --self
```

Or without sudo (installs to ~/.local/bin):

```bash
zpx update --self
```
