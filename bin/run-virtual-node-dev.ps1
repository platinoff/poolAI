# FM-003 + FM-016: coordinator (8080) + virtual-node worker (9090) on one machine.
#
# Usage (repo root):
#   .\bin\run-virtual-node-dev.ps1
#   .\bin\run-virtual-node-dev.ps1 -SkipBuild

param(
    [switch]$SkipBuild,
    [int]$CoordinatorPort = 8080,
    [int]$WorkerPort = 9090,
    [string]$Features = "enterprise,ml,cloud,test-utils"
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

$env:PATH = "$env:USERPROFILE\.cargo\bin;C:\msys64\ucrt64\bin;C:\msys64\usr\bin;" + $env:PATH
if (-not $env:K8S_OPENAPI_ENABLED_VERSION) { $env:K8S_OPENAPI_ENABLED_VERSION = "1.28" }
if (-not $env:RUST_LOG) { $env:RUST_LOG = "info" }

$PoolaiExe = Join-Path $RepoRoot "target\debug\poolai.exe"
$WorkerExe = Join-Path $RepoRoot "target\debug\poolai-worker.exe"

if (-not $SkipBuild) {
    Write-Host "Building poolai + poolai-worker (--features $Features)..." -ForegroundColor Cyan
    & cargo build --features $Features
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & cargo build --bin poolai-worker
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

foreach ($p in @($PoolaiExe, $WorkerExe)) {
    if (-not (Test-Path $p)) { throw "Missing binary: $p" }
}

$StandRoot = Join-Path $RepoRoot "data\lan-stand\virtual-node"
$CoordData = Join-Path $StandRoot "coordinator"
$RaidPath = Join-Path $CoordData "raid"
New-Item -ItemType Directory -Force -Path $RaidPath | Out-Null
$WorkerCache = Join-Path $StandRoot "worker-cache"
$LogDir = Join-Path $StandRoot "logs"
New-Item -ItemType Directory -Force -Path $WorkerCache | Out-Null
New-Item -ItemType Directory -Force -Path $LogDir | Out-Null

$coordUrl = "http://127.0.0.1:$CoordinatorPort"
$coordLog = Join-Path $LogDir "coordinator-$CoordinatorPort.log"
$workerLog = Join-Path $LogDir "worker-$WorkerPort.log"

$coordCmd = @"
Set-Location '$RepoRoot'
`$env:POOLAI_HTTP_PORT='$CoordinatorPort'
`$env:POOLAI_RAID_BASE_PATH='$RaidPath'
`$env:POOLAI_DATA_PATH='$CoordData'
`$env:POOLAI_VIRTUAL_NODE_DATA_DIR='$StandRoot\vn-store'
`$env:K8S_OPENAPI_ENABLED_VERSION='$($env:K8S_OPENAPI_ENABLED_VERSION)'
`$env:RUST_LOG='$($env:RUST_LOG)'
& '$PoolaiExe' *>&1 | Tee-Object -FilePath '$coordLog'
"@

$workerCmd = @"
Set-Location '$RepoRoot'
`$env:POOLAI_COORDINATOR_URL='$coordUrl'
`$env:POOLAI_WORKER_ADDRESS='127.0.0.1'
`$env:POOLAI_WORKER_PORT='$WorkerPort'
`$env:POOLAI_WORKER_CHANNEL='dev'
`$env:POOLAI_TELEGRAM_ID='dev-stand-user'
`$env:POOLAI_WORKER_CACHE_DIR='$WorkerCache'
& '$WorkerExe' --worker-id vn-dev-stand *>&1 | Tee-Object -FilePath '$workerLog'
"@

Write-Host "Starting coordinator -> $coordUrl" -ForegroundColor Green
$coordProc = Start-Process powershell.exe -ArgumentList @(
    "-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", $coordCmd
) -PassThru -WindowStyle Hidden

Start-Sleep -Seconds 12

Write-Host "Starting worker -> http://127.0.0.1:$WorkerPort/health" -ForegroundColor Green
$workerProc = Start-Process powershell.exe -ArgumentList @(
    "-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", $workerCmd
) -PassThru -WindowStyle Hidden

Write-Host ""
Write-Host "Coordinator PID: $($coordProc.Id)  Worker PID: $($workerProc.Id)" -ForegroundColor Cyan
Write-Host "Verify after bootstrap:" -ForegroundColor Cyan
Write-Host "  .\bin\verify-dev-stand.ps1"
Write-Host "Stop:" -ForegroundColor Yellow
Write-Host "  Stop-Process -Id $($coordProc.Id),$($workerProc.Id) -Force -ErrorAction SilentlyContinue"
Write-Host "  Stop-Process -Name poolai,poolai-worker -Force -ErrorAction SilentlyContinue"
