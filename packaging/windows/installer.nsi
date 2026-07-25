; ══════════════════════════════════════════════════════════════════
; Zephyx Windows Installer — NSIS Script
; packaging/windows/installer.nsi
;
; Build:
;   makensis /DVERSION=0.6.2 installer.nsi
;
; Produces:
;   Zephyx-0.6.2-windows-x64-setup.exe
; ══════════════════════════════════════════════════════════════════

!define APPNAME     "Zephyx"
!define APPSLUG     "zephyx"
!define APPVER      "${VERSION}"
!define PUBLISHER   "Zephyx Core Team"
!define WEBSITE     "https://github.com/Ghost-101-ui/Zephyx"
!define REGKEY      "Software\Microsoft\Windows\CurrentVersion\Uninstall\Zephyx"

!include "MUI2.nsh"
!include "EnvVarUpdate.nsh"

; ─── Installer metadata ───────────────────────────────────────────
Name            "${APPNAME}"
BrandingText    "${APPNAME} v${APPVER}"
OutFile         "${APPNAME}-${APPVER}-windows-x64-setup.exe"
InstallDir      "$PROGRAMFILES64\${APPNAME}"
InstallDirRegKey HKLM "${REGKEY}" "InstallLocation"
RequestExecutionLevel admin
ShowInstDetails show
ShowUnInstDetails show

; ─── UI Pages ─────────────────────────────────────────────────────
!define MUI_WELCOMEFINISHPAGE_BITMAP         ""
!define MUI_ICON                              ""
!define MUI_UNICON                            ""

!define MUI_WELCOMEPAGE_TITLE                "Welcome to Zephyx ${APPVER} Setup"
!define MUI_WELCOMEPAGE_TEXT                 "Zephyx is a workflow-driven cybersecurity operating platform.$\n$\nThis will install Zephyx and add 'zpx' to your system PATH.$\n$\nClick Next to continue."
!define MUI_FINISHPAGE_TITLE                 "Zephyx Installation Complete"
!define MUI_FINISHPAGE_TEXT                  "Zephyx has been installed.$\n$\nOpen a new Command Prompt or PowerShell and run:$\n$\n    zpx init$\n    zpx doctor"
!define MUI_FINISHPAGE_RUN                   ""
!define MUI_FINISHPAGE_LINK                  "Open Zephyx documentation"
!define MUI_FINISHPAGE_LINK_LOCATION         "${WEBSITE}"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

; ─── Installation ─────────────────────────────────────────────────
Section "${APPNAME}" SecMain
  SectionIn RO   ; Cannot be deselected

  SetOutPath "$INSTDIR"
  File "zpx.exe"

  ; ── Add install directory to system PATH ──────────────────────
  ${EnvVarUpdate} $0 "PATH" "A" "HKLM" "$INSTDIR"

  ; ── Write uninstaller ─────────────────────────────────────────
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; ── Register in Add/Remove Programs ───────────────────────────
  WriteRegStr   HKLM "${REGKEY}" "DisplayName"          "${APPNAME}"
  WriteRegStr   HKLM "${REGKEY}" "DisplayVersion"       "${APPVER}"
  WriteRegStr   HKLM "${REGKEY}" "Publisher"            "${PUBLISHER}"
  WriteRegStr   HKLM "${REGKEY}" "URLInfoAbout"         "${WEBSITE}"
  WriteRegStr   HKLM "${REGKEY}" "InstallLocation"      "$INSTDIR"
  WriteRegStr   HKLM "${REGKEY}" "UninstallString"      '"$INSTDIR\Uninstall.exe"'
  WriteRegStr   HKLM "${REGKEY}" "QuietUninstallString" '"$INSTDIR\Uninstall.exe" /S'
  WriteRegDWORD HKLM "${REGKEY}" "NoModify"             1
  WriteRegDWORD HKLM "${REGKEY}" "NoRepair"             1

  ; ── Estimate installed size ───────────────────────────────────
  SectionGetSize ${SecMain} $0
  WriteRegDWORD HKLM "${REGKEY}" "EstimatedSize" "$0"

SectionEnd

; ─── Uninstallation ───────────────────────────────────────────────
Section "Uninstall"
  ; Remove binary
  Delete "$INSTDIR\zpx.exe"
  Delete "$INSTDIR\Uninstall.exe"

  ; Remove from PATH
  ${un.EnvVarUpdate} $0 "PATH" "R" "HKLM" "$INSTDIR"

  ; Remove registry entry
  DeleteRegKey HKLM "${REGKEY}"

  ; Remove install directory (only if empty)
  RMDir "$INSTDIR"

SectionEnd
