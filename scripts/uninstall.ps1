# scripts/uninstall.ps1
#
# Zephyx one-liner uninstaller for Windows.
#
# Usage:
#   irm https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/uninstall.ps1 | iex
#
# Options:
#   $env:ZEPHYX_INSTALL_DIR = "C:\Tools"  # custom install directory
#   $env:ZEPHYX_PURGE_DATA   = "1"         # set to 1 to purge config & data

#Requires -Version 5.1
[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Continue"

# ── Config ──────────────────────────────────────────────────────
$BINARY_NAME = "zpx.exe"
$INSTALL_DIR = if ($env:ZEPHYX_INSTALL_DIR) { $env:ZEPHYX_INSTALL_DIR } `
                else { Join-Path $env:LOCALAPPDATA "Zephyx\bin" }

function Write-Info  { param($msg) Write-Host "  → $msg"  -ForegroundColor Cyan }
function Write-Ok    { param($msg) Write-Host "  ✓ $msg"  -ForegroundColor Green }
function Write-Warn  { param($msg) Write-Host "  ⚠ $msg"  -ForegroundColor Yellow }
function Write-Fail  { param($msg) Write-Host "  ✗ $msg"  -ForegroundColor Red }

Write-Host ""
Write-Host "  ⚡ Zephyx Uninstaller" -ForegroundColor Cyan -NoNewline
Write-Host " — Removing Zephyx from Windows"
Write-Host ""

# ── Remove Binary ───────────────────────────────────────────────
$binaryPath = Join-Path $INSTALL_DIR $BINARY_NAME

if (Test-Path $binaryPath) {
    Write-Info "Removing $binaryPath..."
    try {
        Remove-Item -Path $binaryPath -Force
        Write-Ok "Removed binary: $binaryPath"
    } catch {
        Write-Fail "Failed to remove binary: $_"
    }
} else {
    Write-Warn "Binary not found at $binaryPath (already uninstalled?)"
}

# ── Remove from PATH ─────────────────────────────────────────────
$userPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -like "*$INSTALL_DIR*") {
    Write-Info "Removing $INSTALL_DIR from user PATH..."
    $pathParts = $userPath -split ';' | Where-Object { $_ -and $_ -ne $INSTALL_DIR }
    $newPath = $pathParts -join ';'
    [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    Write-Ok "PATH updated."
}

# ── Purge Data if requested ──────────────────────────────────────
if ($env:ZEPHYX_PURGE_DATA -eq "1") {
    Write-Info "Purging Zephyx configuration and data..."
    $configDir = Join-Path $env:APPDATA "Zephyx"
    $dataDir   = Join-Path $env:LOCALAPPDATA "Zephyx"
    if (Test-Path $configDir) { Remove-Item -Recurse -Force $configDir -ErrorAction SilentlyContinue }
    if (Test-Path $dataDir)   { Remove-Item -Recurse -Force $dataDir   -ErrorAction SilentlyContinue }
    Write-Ok "Purged data and configuration."
}

Write-Host ""
Write-Host "  ✓ Zephyx has been successfully uninstalled." -ForegroundColor Green
Write-Host "  To reinstall, run:"
Write-Host "    irm https://raw.githubusercontent.com/Ghost-101-ui/Zephyx/main/scripts/install.ps1 | iex" -ForegroundColor Yellow
Write-Host ""
