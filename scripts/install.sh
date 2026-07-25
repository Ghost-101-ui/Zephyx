#!/usr/bin/env sh
# scripts/install.sh
#
# Zephyx one-liner installer for Linux and macOS.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/install.sh | sh
#
# Options (environment variables):
#   ZEPHYX_INSTALL_DIR   — where to install zpx binary (default: /usr/local/bin)
#   ZEPHYX_VERSION       — version to install (default: latest)

set -eu

# ── Constants ───────────────────────────────────────────────────
REPO="Ghost-101-ui/Zephyx"
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

# ── Banner ──────────────────────────────────────────────────────
printf "\n"
printf "${BOLD}  ⚡ Zephyx Installer${RESET}\n"
printf "  Extensible Cybersecurity Operating Platform\n"
printf "\n"

# ── Detect OS and Architecture ──────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)  OS_NAME="linux"  ;;
    Darwin) OS_NAME="macos"  ;;
    *)      error "Unsupported OS: $OS. Please download manually from https://github.com/$REPO/releases" ;;
esac

case "$ARCH" in
    x86_64|amd64)   ARCH_NAME="amd64" ;;
    aarch64|arm64)  ARCH_NAME="arm64" ;;
    *)              error "Unsupported architecture: $ARCH" ;;
esac

# macOS arch suffix differs
if [ "$OS_NAME" = "macos" ]; then
    case "$ARCH_NAME" in
        amd64) PKG_SUFFIX="macos-intel" ;;
        arm64) PKG_SUFFIX="macos-arm"   ;;
    esac
else
    PKG_SUFFIX="${OS_NAME}-${ARCH_NAME}"
fi

info "Detected: $OS $ARCH ($OS_NAME-$ARCH_NAME)"

# ── Resolve version ─────────────────────────────────────────────
if [ -n "${ZEPHYX_VERSION:-}" ]; then
    VERSION="$ZEPHYX_VERSION"
    info "Target version: $VERSION (from ZEPHYX_VERSION)"
else
    info "Fetching latest release version..."
    if command -v curl >/dev/null 2>&1; then
        VERSION=$(curl -fsSL \
            "https://api.github.com/repos/${REPO}/releases/latest" \
            | grep '"tag_name"' \
            | sed 's/.*"tag_name": *"v\?\([^"]*\)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        VERSION=$(wget -qO- \
            "https://api.github.com/repos/${REPO}/releases/latest" \
            | grep '"tag_name"' \
            | sed 's/.*"tag_name": *"v\?\([^"]*\)".*/\1/')
    else
        error "curl or wget is required to download Zephyx."
    fi
    [ -n "$VERSION" ] || error "Failed to determine latest version."
    ok "Latest version: $VERSION"
fi

# ── Build download URL ──────────────────────────────────────────
ARCHIVE="Zephyx-${VERSION}-${PKG_SUFFIX}.tar.gz"
URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ARCHIVE}"
CHECKSUM_URL="https://github.com/${REPO}/releases/download/v${VERSION}/checksums.txt"

info "Downloading $ARCHIVE..."

# ── Download ─────────────────────────────────────────────────────
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

if command -v curl >/dev/null 2>&1; then
    curl -fsSL --progress-bar "$URL" -o "$TMP_DIR/$ARCHIVE"
    curl -fsSL "$CHECKSUM_URL" -o "$TMP_DIR/checksums.txt" 2>/dev/null || true
elif command -v wget >/dev/null 2>&1; then
    wget -q --show-progress "$URL" -O "$TMP_DIR/$ARCHIVE"
    wget -q "$CHECKSUM_URL" -O "$TMP_DIR/checksums.txt" 2>/dev/null || true
fi
ok "Downloaded $ARCHIVE"

# ── Verify checksum ──────────────────────────────────────────────
if [ -f "$TMP_DIR/checksums.txt" ] && command -v sha256sum >/dev/null 2>&1; then
    info "Verifying SHA256 checksum..."
    EXPECTED=$(grep "$ARCHIVE" "$TMP_DIR/checksums.txt" | awk '{print $1}')
    if [ -n "$EXPECTED" ]; then
        ACTUAL=$(sha256sum "$TMP_DIR/$ARCHIVE" | awk '{print $1}')
        if [ "$EXPECTED" = "$ACTUAL" ]; then
            ok "Checksum verified"
        else
            error "Checksum mismatch! Expected: $EXPECTED, Got: $ACTUAL"
        fi
    else
        warn "Checksum entry not found in checksums.txt — skipping verification"
    fi
elif [ -f "$TMP_DIR/checksums.txt" ] && command -v shasum >/dev/null 2>&1; then
    info "Verifying SHA256 checksum..."
    EXPECTED=$(grep "$ARCHIVE" "$TMP_DIR/checksums.txt" | awk '{print $1}')
    if [ -n "$EXPECTED" ]; then
        ACTUAL=$(shasum -a 256 "$TMP_DIR/$ARCHIVE" | awk '{print $1}')
        if [ "$EXPECTED" = "$ACTUAL" ]; then
            ok "Checksum verified"
        else
            error "Checksum mismatch! Expected: $EXPECTED, Got: $ACTUAL"
        fi
    fi
else
    warn "SHA256 verification skipped (sha256sum/shasum not available)"
fi

# ── Extract ───────────────────────────────────────────────────────
info "Extracting..."
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"
ok "Extracted"

# ── Install ───────────────────────────────────────────────────────
info "Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

if [ -w "$INSTALL_DIR" ]; then
    cp "$TMP_DIR/zpx" "$INSTALL_DIR/$BINARY_NAME"
    chmod 755 "$INSTALL_DIR/$BINARY_NAME"
else
    # Need sudo
    if command -v sudo >/dev/null 2>&1; then
        sudo cp "$TMP_DIR/zpx" "$INSTALL_DIR/$BINARY_NAME"
        sudo chmod 755 "$INSTALL_DIR/$BINARY_NAME"
    else
        error "Cannot write to $INSTALL_DIR and sudo is not available."
    fi
fi

ok "Installed to $INSTALL_DIR/$BINARY_NAME"

# ── Verify installation ──────────────────────────────────────────
if "$INSTALL_DIR/$BINARY_NAME" --version >/dev/null 2>&1; then
    INSTALLED_VER=$("$INSTALL_DIR/$BINARY_NAME" --version 2>&1 || true)
    ok "Binary works: $INSTALLED_VER"
else
    warn "Installed but could not run the binary."
fi

# ── Check PATH ───────────────────────────────────────────────────
if ! command -v "$BINARY_NAME" >/dev/null 2>&1; then
    warn "$INSTALL_DIR is not in your PATH."
    printf "\n  Add this to your shell profile:\n"
    printf "    ${BOLD}export PATH=\"\$PATH:$INSTALL_DIR\"${RESET}\n\n"
fi

# ── Done ─────────────────────────────────────────────────────────
printf "\n"
printf "${BOLD}${GREEN}  ╔══════════════════════════════════════════════╗${RESET}\n"
printf "${BOLD}${GREEN}  ║   ⚡  Zephyx v${VERSION} installed!${RESET}\n"
printf "${BOLD}${GREEN}  ║${RESET}\n"
printf "${BOLD}${GREEN}  ║   Get started:${RESET}\n"
printf "${BOLD}${GREEN}  ║     zpx init${RESET}\n"
printf "${BOLD}${GREEN}  ║     zpx doctor${RESET}\n"
printf "${BOLD}${GREEN}  ╚══════════════════════════════════════════════╝${RESET}\n"
printf "\n"
