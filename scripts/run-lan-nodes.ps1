# DEPRECATED: canonical script is bin/run-lan-nodes.ps1 (see docs/development/REPOSITORY_LAYOUT.md).
& (Join-Path (Split-Path $PSScriptRoot -Parent) "bin\run-lan-nodes.ps1") @args
if ($LASTEXITCODE -ne $null) { exit $LASTEXITCODE }
