# PoolAI — локальний лаунчер (PowerShell, без WSL).
# Документація: docs/development/RUN_LOCAL.md
#Requires -Version 5.1
param(
    [Parameter(Position = 0)]
    [ValidateSet("single", "quick", "lan", "virtual-node", "vn", "docker", "build", "stop", "status", "help")]
    [string]$Command = "single",

    [switch]$Background,
    [switch]$SkipBuild,
    [switch]$Light,
    [switch]$RaidJobs,
    [switch]$StandSmoke,
    [switch]$MigrationAdvisory,
    [switch]$StableTouchup,
    [switch]$EdgeVerification,
    [switch]$PrePushCanon,
    [int]$Port = 8080,
    [string]$Features = "enterprise,ml,cloud,test-utils",
    [ValidateSet("json", "sqlite", "raid")]
    [string]$JobStore = ""
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

# Do not prepend MSYS to PATH here — breaks PowerShell `cargo` (GNU `link` vs MSVC).
# Builds use poolai-msys.ps1 (GNU toolchain per rust-toolchain.toml).
if (-not $env:K8S_OPENAPI_ENABLED_VERSION) { $env:K8S_OPENAPI_ENABLED_VERSION = "1.28" }
if (-not $env:RUST_LOG) { $env:RUST_LOG = "info" }
$LightFeatures = "enterprise,test-utils"
$LastRunPath = Join-Path $RepoRoot "data\dev\last_run.json"

$MsysWrapper = Join-Path $PSScriptRoot "poolai-msys.ps1"

function Show-Help {
    @"
PoolAI run (PowerShell) - no WSL, no bare 'bash' command.

Usage:
  .\bin\run-poolai.ps1 [-Command] <name> [-Background] [-Port N] [-SkipBuild]
                       [-RaidJobs] [-JobStore json|sqlite|raid]

Commands:
  single (default)  one coordinator on :8080
  quick             light build + single -Background + health wait (PH-S1012)
  stop              kill poolai.exe / poolai-worker.exe
  status            health on 8080, 8081, 9090
  build             cargo build
  lan, virtual-node, docker, help

Examples:
  .\bin\run-poolai.ps1 build
  .\bin\run-poolai.ps1 single -Background -SkipBuild
  .\bin\run-poolai.ps1 single -Background -SkipBuild -RaidJobs
  .\bin\run-poolai.ps1 stop
  .\bin\run-poolai.ps1 status

UI:     http://127.0.0.1:8080/ui/login  then  /ui/admin/jobs
Login:  admin / admin123

For bash scripts (e2e, verify-dev-stand, git):
  .\bin\poolai-msys.ps1 bin/e2e-playwright.sh --start
"@
}

function Get-PoolaiExe {
    $candidates = @(
        (Join-Path $RepoRoot "target\release\poolai.exe"),
        (Join-Path $RepoRoot "target\debug\poolai.exe"),
        (Join-Path $RepoRoot "target\x86_64-pc-windows-gnu\release\poolai.exe"),
        (Join-Path $RepoRoot "target\x86_64-pc-windows-gnu\debug\poolai.exe")
    )
    foreach ($path in $candidates) {
        if (Test-Path -LiteralPath $path) { return $path }
    }
    return $null
}

function Save-LastRunSnapshot {
    param(
        [string]$Preset,
        [int]$ListenPort,
        [string]$Feat,
        [string]$Store = "",
        [int]$ProcId = 0
    )
    $dir = Split-Path -Parent $LastRunPath
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $obj = @{
        preset    = $Preset
        port      = $ListenPort
        features  = $Feat
        job_store = if ($Store) { $Store } else { $null }
        pid       = if ($ProcId -gt 0) { $ProcId } else { $null }
        saved_at  = [string][int][double]::Parse((Get-Date -UFormat %s))
    }
    ($obj | ConvertTo-Json -Depth 3) | Set-Content -LiteralPath $LastRunPath -Encoding UTF8
}

function Get-LastRunPort {
    if (-not (Test-Path -LiteralPath $LastRunPath)) { return $null }
    try {
        $json = Get-Content -LiteralPath $LastRunPath -Raw -Encoding UTF8 | ConvertFrom-Json
        if ($null -ne $json.port) { return [int]$json.port }
    } catch { }
    return $null
}

function Wait-Health {
    param([int]$ListenPort, [int]$Tries = 30)
    for ($i = 0; $i -lt $Tries; $i++) {
        try {
            Invoke-WebRequest -Uri "http://127.0.0.1:$ListenPort/api/v1/health" -TimeoutSec 3 -UseBasicParsing | Out-Null
            Write-Host "Health OK http://127.0.0.1:$ListenPort/api/v1/health" -ForegroundColor Green
            return $true
        } catch {
            Start-Sleep -Seconds 1
        }
    }
    Write-Host "Health wait timeout on port $ListenPort" -ForegroundColor Red
    return $false
}

function Invoke-Build {
    if (-not (Test-Path -LiteralPath $MsysWrapper)) {
        throw "Missing poolai-msys.ps1. Install MSYS2 UCRT64 at C:\msys64"
    }
    $feat = if ($Light) { $LightFeatures } else { $Features }
    Write-Host "Building poolai via MSYS2 GNU (features: $feat)..." -ForegroundColor Cyan
    $buildCmd = 'cargo build --features ' + $feat + ' && (cargo build --bin poolai-worker 2>/dev/null || true)'
    & $MsysWrapper -lc $buildCmd
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Hint: build in MSYS2 UCRT64: /usr/bin/bash bin/run-poolai.sh build" -ForegroundColor Yellow
        exit $LASTEXITCODE
    }
}

function Invoke-Stop {
    Write-Host "Stopping PoolAI processes..." -ForegroundColor Yellow
    Save-LastRunSnapshot -Preset "stop" -ListenPort $Port -Feat $Features -Store $JobStore
    Get-Process -Name poolai, poolai-worker -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
    Write-Host "Done."
}

function Invoke-Status {
    foreach ($p in @(8080, 8081, 9090)) {
        try {
            Invoke-WebRequest -Uri "http://127.0.0.1:$p/api/v1/health" -TimeoutSec 3 -UseBasicParsing | Out-Null
            Write-Host "UP   http://127.0.0.1:$p/api/v1/health" -ForegroundColor Green
        } catch {
            Write-Host "DOWN http://127.0.0.1:$p/api/v1/health" -ForegroundColor DarkGray
        }
    }
}

function Invoke-Single {
    $data = Join-Path $RepoRoot "data\dev\single"
    $raid = Join-Path $data "raid"
    $logs = Join-Path $RepoRoot "data\dev\logs"
    New-Item -ItemType Directory -Force -Path $raid, $logs | Out-Null

    if (-not $SkipBuild) { Invoke-Build }

    $exe = Get-PoolaiExe
    if (-not $exe) {
        throw "Missing poolai.exe. Run: .\bin\run-poolai.ps1 build"
    }

    $env:POOLAI_HTTP_PORT = "$Port"
    $env:POOLAI_DATA_PATH = $data
    $env:POOLAI_RAID_BASE_PATH = $raid
    if ($RaidJobs) {
        $JobStore = "raid"
    }
    if ($JobStore) {
        $env:POOLAI_JOB_STORE = $JobStore
    }

    $url = "http://127.0.0.1:$Port"
    Write-Host "PoolAI single node"
    Write-Host "  API:   $url/api/v1/health"
    Write-Host "  Login: $url/ui/login"
    Write-Host "  Admin: $url/ui/admin/jobs  (admin / admin123)"
    Write-Host "  Data:  $data"
    $jobStoreLabel = if ($env:POOLAI_JOB_STORE) { $env:POOLAI_JOB_STORE } else { "json" }
    Write-Host "  Job store: $jobStoreLabel"
    if ($env:POOLAI_JOB_STORE -eq "raid") {
        Write-Host "  RAID path: $env:POOLAI_RAID_BASE_PATH"
    }

    if ($Background) {
        $log = Join-Path $logs "single-$Port.log"
        $errLog = Join-Path $logs "single-$Port.err.log"
        $proc = Start-Process -FilePath $exe -RedirectStandardOutput $log -RedirectStandardError $errLog -PassThru
        Save-LastRunSnapshot -Preset "single" -ListenPort $Port -Feat $(if ($Light) { $LightFeatures } else { $Features }) -Store $JobStore -ProcId $proc.Id
        Write-Host "Background PID $($proc.Id) - log: $log"
        Write-Host "Stop: .\bin\run-poolai.ps1 stop"
        Start-Sleep -Seconds 3
        Invoke-Status
    } else {
        Write-Host "Foreground (Ctrl+C to stop)..."
        & $exe
    }
}

function Invoke-Quick {
    $restored = Get-LastRunPort
    if ($restored) { $Port = $restored }
    $Light = $true
    $Background = $true
    Invoke-Single
    if (-not (Wait-Health -ListenPort $Port)) { exit 1 }
    if ($StandSmoke) {
        $env:POOLAI_BASE_URL = "http://127.0.0.1:$Port"
        Write-Host "Running poolai-http-stand-smoke --run-local-smoke (PH-S1095)..."
        & $MsysWrapper cargo run --quiet --bin poolai-http-stand-smoke -- --run-local-smoke
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }
    if ($MigrationAdvisory) {
        Write-Host "Running poolai-loc-audit --migration-advisory (PH-S1104)..."
        & $MsysWrapper cargo run --quiet --bin poolai-loc-audit -- --migration-advisory
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }
    if ($StableTouchup) {
        Write-Host "Running poolai-loc-audit --stable-touchup (PH-S1114)..."
        & $MsysWrapper cargo run --quiet --bin poolai-loc-audit -- --stable-touchup
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }
    if ($EdgeVerification) {
        Write-Host "Running poolai-loc-audit --edge-verification-advisory (PH-S1125)..."
        & $MsysWrapper cargo run --quiet --bin poolai-loc-audit -- --edge-verification-advisory
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }
    if ($PrePushCanon) {
        Write-Host "Running poolai-loc-audit --pre-push-canon (PH-S1134)..."
        & $MsysWrapper cargo run --quiet --bin poolai-loc-audit -- --pre-push-canon
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }
}

switch ($Command) {
    "help" { Show-Help }
    "build" { Invoke-Build }
    "stop" { Invoke-Stop }
    "status" { Invoke-Status }
    "single" { Invoke-Single }
    "quick" { Invoke-Quick }
    "lan" { & (Join-Path $RepoRoot "bin\run-lan-nodes.ps1") }
    "virtual-node" { & (Join-Path $RepoRoot "bin\run-virtual-node-dev.ps1") }
    "vn" { & (Join-Path $RepoRoot "bin\run-virtual-node-dev.ps1") }
    "docker" {
        Push-Location (Join-Path $RepoRoot "docker")
        docker compose up -d --build
        Pop-Location
    }
    default { Show-Help; exit 1 }
}
