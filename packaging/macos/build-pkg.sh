#!/usr/bin/env bash
# packaging/macos/build-pkg.sh
#
# Build a macOS .pkg installer for Zephyx.
#
# Usage:
#   ./build-pkg.sh <binary_path> <version> <output.pkg>
#
# Examples:
#   ./build-pkg.sh target/x86_64-apple-darwin/release/zpx 0.6.2 Zephyx-0.6.2-macos-intel.pkg
#   ./build-pkg.sh target/aarch64-apple-darwin/release/zpx  0.6.2 Zephyx-0.6.2-macos-arm.pkg

set -euo pipefail

BINARY="${1:?Usage: $0 <binary> <version> <output.pkg>}"
VERSION="${2:?Missing version}"
OUTPUT="${3:?Missing output path}"

PKG_STAGE=$(mktemp -d)
SCRIPTS_DIR=$(mktemp -d)
trap 'rm -rf "$PKG_STAGE" "$SCRIPTS_DIR"' EXIT

echo "→ Building macOS .pkg: $OUTPUT"
echo "  Binary:  $BINARY"
echo "  Version: $VERSION"

# ─── Payload ─────────────────────────────────────────────────────
mkdir -p "$PKG_STAGE/usr/local/bin"
cp "$BINARY" "$PKG_STAGE/usr/local/bin/zpx"
chmod 755 "$PKG_STAGE/usr/local/bin/zpx"

# ─── Post-install script ─────────────────────────────────────────
cat > "$SCRIPTS_DIR/postinstall" << 'EOF'
#!/bin/bash
echo ""
echo "  ╔══════════════════════════════════════════════╗"
echo "  ║                                              ║"
echo "  ║   ⚡  Zephyx installed successfully!         ║"
echo "  ║                                              ║"
echo "  ║   Open a new Terminal and run:               ║"
echo "  ║     zpx init                                 ║"
echo "  ║     zpx doctor                               ║"
echo "  ║                                              ║"
echo "  ╚══════════════════════════════════════════════╝"
echo ""
EOF
chmod +x "$SCRIPTS_DIR/postinstall"

# ─── Build component .pkg ────────────────────────────────────────
pkgbuild \
    --root "$PKG_STAGE" \
    --identifier dev.zephyx.zpx \
    --version "$VERSION" \
    --install-location "/" \
    --scripts "$SCRIPTS_DIR" \
    "$OUTPUT"

echo "✓ Built: $OUTPUT ($(du -sh "$OUTPUT" | cut -f1))"
