# Playwright E2E via MSYS2 bash (PowerShell-safe; no WSL bash stub).
param(
    [switch]$Start,
    [switch]$UpdateSnapshots
)

$playwrightArgs = @("bin/e2e-playwright.sh")
if ($Start) { $playwrightArgs += "--start" }
if ($UpdateSnapshots) { $playwrightArgs += "--update-snapshots" }

& (Join-Path $PSScriptRoot "poolai-msys.ps1") @playwrightArgs
exit $LASTEXITCODE
