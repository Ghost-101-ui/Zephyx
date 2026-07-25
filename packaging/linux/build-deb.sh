#!/usr/bin/env bash
# packaging/linux/build-deb.sh
#
# Build a Debian .deb package for Zephyx.
#
# Usage:
#   ./build-deb.sh <binary_path> <version> <arch> <output.deb>
#
# Examples:
#   ./build-deb.sh target/release/zpx 0.6.2 amd64 Zephyx-0.6.2-linux-amd64.deb
#   ./build-deb.sh target/aarch64-unknown-linux-gnu/release/zpx 0.6.2 arm64 Zephyx-0.6.2-linux-arm64.deb

set -euo pipefail

BINARY="${1:?Usage: $0 <binary> <version> <arch> <output.deb>}"
VERSION="${2:?Missing version}"
ARCH="${3:?Missing arch (amd64|arm64)}"
OUTPUT="${4:?Missing output path}"

PKG_DIR=$(mktemp -d)
trap 'rm -rf "$PKG_DIR"' EXIT

echo "→ Building .deb: $OUTPUT"
echo "  Binary:  $BINARY"
echo "  Version: $VERSION"
echo "  Arch:    $ARCH"

# ─── Directory structure ──────────────────────────────────────────
mkdir -p "$PKG_DIR/DEBIAN"
mkdir -p "$PKG_DIR/usr/local/bin"
mkdir -p "$PKG_DIR/usr/share/doc/zephyx"
mkdir -p "$PKG_DIR/usr/share/man/man1"

# ─── Binary ──────────────────────────────────────────────────────
cp "$BINARY" "$PKG_DIR/usr/local/bin/zpx"
chmod 755 "$PKG_DIR/usr/local/bin/zpx"

# ─── control ─────────────────────────────────────────────────────
cat > "$PKG_DIR/DEBIAN/control" << EOF
Package: zephyx
Version: ${VERSION}
Architecture: ${ARCH}
Maintainer: Zephyx Core Team <team@zephyx.dev>
Installed-Size: $(du -sk "$BINARY" | cut -f1)
Depends: 
Section: net
Priority: optional
Homepage: https://github.com/Ghost-101-ui/Zephyx
Description: Extensible Cybersecurity Operating Platform
 Zephyx is a workflow-driven cybersecurity operating platform built in Rust.
 .
 It provides a structured, phase-aware methodology for security assessments
 through an intelligent CLI that orchestrates tools, tracks findings, builds
 knowledge graphs, and generates professional reports.
 .
 Designed for: CTF players, ethical hackers, red teamers, security students.
EOF

# ─── postinst ────────────────────────────────────────────────────
cat > "$PKG_DIR/DEBIAN/postinst" << 'POSTINST'
#!/bin/bash
set -e

if [ "$1" = "configure" ]; then
    echo ""
    echo "  ╔══════════════════════════════════════════════╗"
    echo "  ║                                              ║"
    echo "  ║   ⚡  Zephyx installed successfully!         ║"
    echo "  ║                                              ║"
    echo "  ║   Get started:                               ║"
    echo "  ║     zpx init                                 ║"
    echo "  ║     zpx doctor                               ║"
    echo "  ║                                              ║"
    echo "  ║   Documentation:                             ║"
    echo "  ║   https://github.com/Ghost-101-ui/Zephyx    ║"
    echo "  ║                                              ║"
    echo "  ╚══════════════════════════════════════════════╝"
    echo ""
fi
POSTINST
chmod 755 "$PKG_DIR/DEBIAN/postinst"

# ─── postrm ──────────────────────────────────────────────────────
cat > "$PKG_DIR/DEBIAN/postrm" << 'POSTRM'
#!/bin/bash
set -e

if [ "$1" = "purge" ]; then
    # Remove user workspace if purging (optional — commented out by default)
    # rm -rf ~/.zephyx
    :
fi
POSTRM
chmod 755 "$PKG_DIR/DEBIAN/postrm"

# ─── copyright ───────────────────────────────────────────────────
cat > "$PKG_DIR/usr/share/doc/zephyx/copyright" << EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: zephyx
Upstream-Contact: Zephyx Core Team <team@zephyx.dev>
Source: https://github.com/Ghost-101-ui/Zephyx

Files: *
Copyright: 2026 Zephyx Core Team
License: MIT OR Apache-2.0
EOF

# ─── Build ───────────────────────────────────────────────────────
dpkg-deb --build --root-owner-group "$PKG_DIR" "$OUTPUT"

echo "✓ Built: $OUTPUT ($(du -sh "$OUTPUT" | cut -f1))"
