# Troubleshooting

Common issues and their solutions.

---

## Installation Issues

### `cargo` is not found (Windows)

**Problem:** Running `cargo` in PowerShell gives "command not found".

**Solution:**
```powershell
# Use full path
& "$env:USERPROFILE\.cargo\bin\cargo.exe" --version

# Or permanently add to PATH:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
# Add to your PowerShell profile for persistence
```

---

### Build fails with `linker not found`

**Problem:** Compilation fails with "linker `cc` not found" on Linux.

**Solution:**
```bash
sudo apt install build-essential
# or on Fedora:
sudo dnf install gcc
```

---

### Build fails with `openssl` error

**Problem:** `openssl-sys` fails to compile.

**Solution:**
```bash
sudo apt install libssl-dev pkg-config
```

---

## Runtime Issues

### `zpx doctor` shows all tools as NOT INSTALLED

**Problem:** Tools you've installed aren't being found.

**Causes and fixes:**

1. **Tools not in PATH:**
   ```bash
   which nmap   # Should show a path
   echo $PATH   # Check PATH includes /usr/bin
   ```

2. **Wrong shell session:** Restart your terminal after installing Rust/tools.

3. **WSL vs Windows:** If running zpx in WSL but tools installed in Windows, they won't be found. Install them in WSL.

---

### Session not found on resume

**Problem:** `zpx session resume session-abc` gives "Session not found".

**Solution:**
```bash
# List available sessions
zpx session list
# Use the exact ID shown
zpx session resume session-a1b2c3d4
```

---

### TUI dashboard is blank or shows artifacts

**Problem:** The TUI (`zpx dashboard`) shows blank panels or garbled characters.

**Solutions:**
1. Ensure terminal is at least 80 columns wide
2. Set terminal type:
   ```bash
   TERM=xterm-256color zpx dashboard
   ```
3. Try a different terminal emulator (kitty, alacritty, wezterm recommended)

---

### Permission denied writing to `~/.zephyx/`

**Problem:** Zephyx can't write to the central workspace.

**Solution:**
```bash
# Check ownership
ls -la ~/.zephyx/
# Fix permissions
chmod -R u+rw ~/.zephyx/
```

---

## Performance Issues

### Scans are very slow

**Solution:** Switch to a faster execution profile:
```bash
zpx profile use aggressive
```

Or increase concurrency in your config:
```toml
[scheduler]
max_concurrency = 16
```

---

### `cargo build` is very slow

**Solution:** Use the mold linker for faster incremental builds:
```bash
sudo apt install mold
# Add to .cargo/config.toml:
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

---

## Database Issues

### `database is locked` error

**Problem:** Multiple Zephyx instances running simultaneously causing database contention.

**Solution:**
```bash
# Kill other zpx processes
pkill zpx
# Then restart
zpx session resume session-id
```

---

## Getting More Help

1. Run `zpx doctor` for a system health check
2. Enable debug logging: `RUST_LOG=debug zpx <command>`
3. Check [FAQ.md](../FAQ.md) for common questions
4. Open a [GitHub Issue](https://github.com/zephyx/zephyx/issues) with:
   - `zpx --version` output
   - Operating system and version
   - Full error message
   - Steps to reproduce
