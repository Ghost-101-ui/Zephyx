# scripts/install.ps1
#
# Zephyx one-liner installer for Windows.
#
# Usage:
#   irm https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/install.ps1 | iex
#
# Options (set before running):
#   $env:ZEPHYX_VERSION       = "0.6.2"    # specific version (default: latest)
#   $env:ZEPHYX_INSTALL_DIR   = "C:\Tools" # custom install directory

#Requires -Version 5.1
[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# ── Config ──────────────────────────────────────────────────────
$REPO         = "Ghost-101-ui/Zephyx"
$BINARY_NAME  = "zpx.exe"
$INSTALL_DIR  = if ($env:ZEPHYX_INSTALL_DIR) { $env:ZEPHYX_INSTALL_DIR } `
                else { Join-Path $env:LOCALAPPDATA "Zephyx\bin" }
$VERSION_OVERRIDE = $env:ZEPHYX_VERSION

# ── UI helpers ──────────────────────────────────────────────────
function Write-Info  { param($msg) Write-Host "  → $msg"  -ForegroundColor Cyan }
function Write-Ok    { param($msg) Write-Host "  ✓ $msg"  -ForegroundColor Green }
function Write-Warn  { param($msg) Write-Host "  ⚠ $msg"  -ForegroundColor Yellow }
function Write-Fail  { param($msg) Write-Host "  ✗ $msg"  -ForegroundColor Red; exit 1 }

# ── Banner ──────────────────────────────────────────────────────
Write-Host ""
Write-Host "  ⚡ Zephyx Installer" -ForegroundColor Cyan -NoNewline
Write-Host " — Extensible Cybersecurity Operating Platform"
Write-Host ""

# ── Architecture detection ───────────────────────────────────────
$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
if ($arch -ne [System.Runtime.InteropServices.Architecture]::X64) {
    Write-Fail "Zephyx currently supports Windows x64 only. Detected: $arch"
}
Write-Ok "Architecture: Windows x64"

# ── Resolve version ──────────────────────────────────────────────
if ($VERSION_OVERRIDE) {
    $version = $VERSION_OVERRIDE
    Write-Info "Target version: $version (from ZEPHYX_VERSION)"
} else {
    Write-Info "Fetching latest release version..."
    try {
        $headers = @{ "User-Agent" = "zephyx-installer/1.0" }
        $release = Invoke-RestMethod `
            -Uri "https://api.github.com/repos/$REPO/releases/latest" `
            -Headers $headers
        $version = $release.tag_name -replace '^v', ''
        Write-Ok "Latest version: $version"
    } catch {
        Write-Fail "Failed to fetch latest version: $_"
    }
}

# ── Build URLs ───────────────────────────────────────────────────
$zipName      = "Zephyx-${version}-windows-x64.zip"
$zipUrl       = "https://github.com/$REPO/releases/download/v${version}/$zipName"
$checksumUrl  = "https://github.com/$REPO/releases/download/v${version}/checksums.txt"

# ── Create temp directory ────────────────────────────────────────
$tmpDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
$zipPath = Join-Path $tmpDir $zipName

# ── Download ─────────────────────────────────────────────────────
Write-Info "Downloading $zipName..."
try {
    $webClient = New-Object System.Net.WebClient
    $webClient.DownloadFile($zipUrl, $zipPath)
    Write-Ok "Downloaded: $zipName"
} catch {
    Write-Fail "Download failed: $_`n  URL: $zipUrl"
} finally {
    if ($webClient) { $webClient.Dispose() }
}

# ── Verify checksum ──────────────────────────────────────────────
try {
    $checksumPath = Join-Path $tmpDir "checksums.txt"
    (New-Object System.Net.WebClient).DownloadFile($checksumUrl, $checksumPath)

    $checksums = Get-Content $checksumPath
    $expected  = ($checksums | Where-Object { $_ -match [regex]::Escape($zipName) } `
                             | Select-Object -First 1) -split '\s+' | Select-Object -First 1

    if ($expected) {
        Write-Info "Verifying SHA256 checksum..."
        $actual = (Get-FileHash -Path $zipPath -Algorithm SHA256).Hash.ToLower()
        if ($actual -eq $expected.ToLower()) {
            Write-Ok "Checksum verified"
        } else {
            Write-Fail "Checksum mismatch!`n  Expected: $expected`n  Got:      $actual"
        }
    } else {
        Write-Warn "Checksum entry not found — skipping verification"
    }
} catch {
    Write-Warn "Checksum verification skipped: $_"
}

# ── Extract ───────────────────────────────────────────────────────
Write-Info "Extracting..."
Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force
Write-Ok "Extracted"

# ── Install ───────────────────────────────────────────────────────
Write-Info "Installing to: $INSTALL_DIR"
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null
Copy-Item -Path (Join-Path $tmpDir "zpx.exe") -Destination (Join-Path $INSTALL_DIR $BINARY_NAME) -Force
Write-Ok "Installed: $INSTALL_DIR\$BINARY_NAME"

# ── Add to PATH ──────────────────────────────────────────────────
$userPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$INSTALL_DIR*") {
    Write-Info "Adding $INSTALL_DIR to user PATH..."
    $newPath = "$userPath;$INSTALL_DIR"
    [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    $env:PATH += ";$INSTALL_DIR"
    Write-Ok "PATH updated (restart terminal to take effect)"
} else {
    Write-Ok "$INSTALL_DIR already in PATH"
}

# ── Cleanup ───────────────────────────────────────────────────────
Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue

# ── Verify ───────────────────────────────────────────────────────
$zpxPath = Join-Path $INSTALL_DIR $BINARY_NAME
try {
    $ver = & $zpxPath --version 2>&1
    Write-Ok "Binary works: $ver"
} catch {
    Write-Warn "Installed but could not run the binary."
}

# ── Done ─────────────────────────────────────────────────────────
Write-Host ""
Write-Host "  ╔══════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "  ║                                              ║" -ForegroundColor Green
Write-Host "  ║   ⚡  Zephyx v$version installed!            " -ForegroundColor Green
Write-Host "  ║                                              ║" -ForegroundColor Green
Write-Host "  ║   Open a NEW terminal window and run:        ║" -ForegroundColor Green
Write-Host "  ║     zpx init                                 ║" -ForegroundColor Green
Write-Host "  ║     zpx doctor                               ║" -ForegroundColor Green
Write-Host "  ║                                              ║" -ForegroundColor Green
Write-Host "  ╚══════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
