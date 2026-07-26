#!/usr/bin/env sh
# scripts/uninstall.sh
#
# Zephyx one-liner uninstaller for Linux and macOS.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/uninstall.sh | sh
#
# Options:
#   ZEPHYX_INSTALL_DIR — directory where zpx is installed (default: /usr/local/bin)
#   ZEPHYX_PURGE_DATA   — set to 1 to also delete configuration and local data

set -eu

INSTALL_DIR="${ZEPHYX_INSTALL_DIR:-/usr/local/bin}"
BINARY_NAME="zpx"

# ── Colors ──────────────────────────────────────────────────────
if [ -t 1 ]; then
    RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
    CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'
else
    RED=''; GREEN=''; YELLOW=''; CYAN=''; BOLD=''; RESET=''
fi

info()  { printf "${CYAN}  →${RESET} %s\n" "$1"; }
ok()    { printf "${GREEN}  ✓${RESET} %s\n" "$1"; }
warn()  { printf "${YELLOW}  ⚠${RESET} %s\n" "$1"; }
error() { printf "${RED}  ✗${RESET} %s\n" "$1" >&2; exit 1; }

printf "\n"
printf "${BOLD}  ⚡ Zephyx Uninstaller${RESET}\n"
printf "  Removing Zephyx binary and configuration...\n"
printf "\n"

# ── Remove Binary ───────────────────────────────────────────────
TARGET_BIN="$INSTALL_DIR/$BINARY_NAME"

if [ -f "$TARGET_BIN" ]; then
    info "Removing binary from $TARGET_BIN..."
    if [ -w "$TARGET_BIN" ] || [ -w "$INSTALL_DIR" ]; then
        rm -f "$TARGET_BIN"
    else
        if command -v sudo >/dev/null 2>&1; then
            sudo rm -f "$TARGET_BIN"
        else
            error "Permission denied to remove $TARGET_BIN. Please run with sudo."
        fi
    fi
    ok "Removed binary: $TARGET_BIN"
else
    warn "Binary not found at $TARGET_BIN (already uninstalled?)"
fi

# ── Remove Data / Config (if requested or by default) ────────────
if [ "${ZEPHYX_PURGE_DATA:-0}" = "1" ]; then
    info "Purging Zephyx data and configuration directories..."
    rm -rf "$HOME/.config/zephyx" "$HOME/.local/share/zephyx" "$HOME/.zephyx"
    ok "Purged user configuration and data."
fi

printf "\n"
printf "${BOLD}${GREEN}  ✓ Zephyx has been successfully uninstalled.${RESET}\n"
printf "  To reinstall, run:\n"
printf "    ${BOLD}curl -fsSL https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/install.sh | sh${RESET}\n\n"
