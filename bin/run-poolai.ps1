# PoolAI — локальний лаунчер (PowerShell, без WSL).
# Документація: docs/development/RUN_LOCAL.md
#Requires -Version 5.1
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

# Do not prepend MSYS to PATH here — breaks PowerShell `cargo` (GNU `link` vs MSVC).
# Builds use poolai-msys.ps1 (GNU toolchain per rust-toolchain.toml).
if (-not $env:K8S_OPENAPI_ENABLED_VERSION) { $env:K8S_OPENAPI_ENABLED_VERSION = "1.28" }
if (-not $env:RUST_LOG) { $env:RUST_LOG = "info" }

$MsysWrapper = Join-Path $PSScriptRoot "poolai-msys.ps1"

function Show-Help {
    @"
PoolAI run (PowerShell) - no WSL, no bare 'bash' command.

Usage:
  .\bin\run-poolai.ps1 [-Command] <name> [-Background] [-Port N] [-SkipBuild]

Commands:
  single (default)  one coordinator on :8080
  stop              kill poolai.exe / poolai-worker.exe
  status            health on 8080, 8081, 9090
  build             cargo build
  lan, virtual-node, docker, help

Examples:
  .\bin\run-poolai.ps1 build
  .\bin\run-poolai.ps1 single -Background -SkipBuild
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

function Invoke-Build {
    if (-not (Test-Path -LiteralPath $MsysWrapper)) {
        throw "Missing poolai-msys.ps1. Install MSYS2 UCRT64 at C:\msys64"
    }
    Write-Host "Building poolai via MSYS2 GNU (features: $Features)..." -ForegroundColor Cyan
    $buildCmd = 'cargo build --features ' + $Features + ' && (cargo build --bin poolai-worker 2>/dev/null || true)'
    & $MsysWrapper -lc $buildCmd
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Hint: build in MSYS2 UCRT64: /usr/bin/bash bin/run-poolai.sh build" -ForegroundColor Yellow
        exit $LASTEXITCODE
    }
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

    $exe = Get-PoolaiExe
    if (-not $exe) {
        throw "Missing poolai.exe. Run: .\bin\run-poolai.ps1 build"
    }

    $env:POOLAI_HTTP_PORT = "$Port"
    $env:POOLAI_DATA_PATH = $data
    $env:POOLAI_RAID_BASE_PATH = $raid

    $url = "http://127.0.0.1:$Port"
    Write-Host "PoolAI single node"
    Write-Host "  API:   $url/api/v1/health"
    Write-Host "  Login: $url/ui/login"
    Write-Host "  Admin: $url/ui/admin/jobs  (admin / admin123)"
    Write-Host "  Data:  $data"

    if ($Background) {
        $log = Join-Path $logs "single-$Port.log"
        $errLog = Join-Path $logs "single-$Port.err.log"
        $proc = Start-Process -FilePath $exe -RedirectStandardOutput $log -RedirectStandardError $errLog -PassThru
        Write-Host "Background PID $($proc.Id) - log: $log"
        Write-Host "Stop: .\bin\run-poolai.ps1 stop"
        Start-Sleep -Seconds 3
        Invoke-Status
    } else {
        Write-Host "Foreground (Ctrl+C to stop)..."
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
