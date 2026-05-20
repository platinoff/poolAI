# FM-028: capture single-host dual-port metrics (run-lan-nodes + health_load + TQ01 snapshot).
param(
    [switch]$SkipBuild,
    [switch]$SkipLanStart,
    [int]$HealthSecs = 10,
    [int]$HealthWorkers = 50,
    [string]$HostLabel = "win10-local-26200-dual-stand"
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

$env:PATH = "$env:USERPROFILE\.cargo\bin;C:\msys64\ucrt64\bin;C:\msys64\usr\bin;" + $env:PATH
if (-not $env:K8S_OPENAPI_ENABLED_VERSION) { $env:K8S_OPENAPI_ENABLED_VERSION = "1.28" }

$Features = "enterprise,ml,cloud,test-utils"
$Stamp = Get-Date -Format "yyyyMMdd"
$OutJson = Join-Path $RepoRoot "data\lan-stand\metrics-fm028-$Stamp.json"
New-Item -ItemType Directory -Force -Path (Split-Path $OutJson) | Out-Null

function Stop-Poolai {
    Stop-Process -Name poolai -Force -ErrorAction SilentlyContinue
}

if (-not $SkipLanStart) {
    Stop-Poolai
    $buildArg = if ($SkipBuild) { "-SkipBuild" } else { @() }
    & (Join-Path $RepoRoot "bin\run-lan-nodes.ps1") @buildArg
    Start-Sleep -Seconds 18
}

& (Join-Path $RepoRoot "bin\verify-lan-prep.ps1")
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if (-not $SkipBuild) {
    cargo build --release --bin poolai_health_load --features $Features
    cargo build --bin poolai-p2b-tq01-snapshot --features ml
}

$ha = & cargo run --release --bin poolai_health_load --features $Features -- `
    --json "http://127.0.0.1:8080/api/v1/health" $HealthSecs $HealthWorkers | ConvertFrom-Json
$hb = & cargo run --release --bin poolai_health_load --features $Features -- `
    --json "http://127.0.0.1:8081/api/v1/health" $HealthSecs $HealthWorkers | ConvertFrom-Json
$tq = & cargo run --bin poolai-p2b-tq01-snapshot --features ml 2>$null | ConvertFrom-Json

$doc = [ordered]@{
    host_label = $HostLabel
    date       = $Stamp
    stand      = "single-host dual-port (run-lan-nodes)"
    health_load = @{
        node_a_8080 = $ha
        node_b_8081 = $hb
    }
    tq01_snapshot = $tq
}
($doc | ConvertTo-Json -Depth 8) | Set-Content -Encoding utf8 $OutJson
Write-Host "Wrote $OutJson" -ForegroundColor Cyan
Write-Host "FM-028: see docs/performance/BENCHMARKS.md P2b single-host table" -ForegroundColor Green

if (-not $SkipLanStart) { Stop-Poolai }
