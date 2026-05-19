# PoolAI — локальний лаунчер (PowerShell). Документація: docs/development/RUN_LOCAL.md
param(
    [Parameter(Position = 0)]
    [ValidateSet("single", "lan", "virtual-node", "vn", "docker", "build", "stop", "status", "help")]
    [string]$Command = "single",

    [switch]$Background,
    [switch]$SkipBuild,
    [int]$Port = 8080,
    [string]$Features = "enterprise,ml,cloud,test-utils"
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

$env:PATH = "$env:USERPROFILE\.cargo\bin;C:\msys64\ucrt64\bin;C:\msys64\usr\bin;" + $env:PATH
if (-not $env:K8S_OPENAPI_ENABLED_VERSION) { $env:K8S_OPENAPI_ENABLED_VERSION = "1.28" }
if (-not $env:RUST_LOG) { $env:RUST_LOG = "info" }

function Show-Help {
    @"
Usage: .\bin\run-poolai.ps1 [-Command] <name> [-Background] [-Port N] [-SkipBuild]

Commands: single (default), lan, virtual-node, docker, build, stop, status, help
Examples:
  .\bin\run-poolai.ps1 single
  .\bin\run-poolai.ps1 virtual-node
  .\bin\run-poolai.ps1 -Command stop
"@
}

function Invoke-Build {
    Write-Host "Building poolai (--features $Features)..." -ForegroundColor Cyan
    & cargo build --features $Features
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & cargo build --bin poolai-worker 2>$null
}

function Invoke-Stop {
    Write-Host "Stopping PoolAI processes..." -ForegroundColor Yellow
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

    $exe = Join-Path $RepoRoot "target\debug\poolai.exe"
    if (-not (Test-Path $exe)) { throw "Missing $exe — run build first" }

    $env:POOLAI_HTTP_PORT = "$Port"
    $env:POOLAI_DATA_PATH = $data
    $env:POOLAI_RAID_BASE_PATH = $raid

    $url = "http://127.0.0.1:$Port"
    Write-Host "PoolAI single — UI $url/ui  Admin $url/ui/admin  (admin / admin123)"

    if ($Background) {
        $log = Join-Path $logs "single-$Port.log"
        Start-Process -FilePath $exe -RedirectStandardOutput $log -RedirectStandardError $log -NoNewWindow
        Start-Sleep -Seconds 3
        Invoke-Status
    } else {
        & $exe
    }
}

switch ($Command) {
    "help" { Show-Help }
    "build" { Invoke-Build }
    "stop" { Invoke-Stop }
    "status" { Invoke-Status }
    "single" { Invoke-Single }
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
