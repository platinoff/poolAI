# Start N PoolAI nodes on one machine (FM-003 / LAN dev stand).
# Requires POOLAI_HTTP_PORT and POOLAI_RAID_BASE_PATH (src/main.rs, src/raid/mod.rs).
#
# Usage (from repo root):
#   .\bin\run-lan-nodes.ps1
#   .\bin\run-lan-nodes.ps1 -SkipBuild -NodeCount 3

param(
    [switch]$SkipBuild,
    [int]$NodeCount = 2,
    [int]$BasePort = 8080,
    [string]$Features = "enterprise,ml,cloud,test-utils"
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

$env:PATH = "$env:USERPROFILE\.cargo\bin;C:\msys64\ucrt64\bin;C:\msys64\usr\bin;" + $env:PATH
if (-not $env:K8S_OPENAPI_ENABLED_VERSION) { $env:K8S_OPENAPI_ENABLED_VERSION = "1.28" }
if (-not $env:RUST_LOG) { $env:RUST_LOG = "info" }

$Exe = Join-Path $RepoRoot "target\debug\poolai.exe"
if (-not $SkipBuild) {
    Write-Host "Building poolai (--features $Features)..." -ForegroundColor Cyan
    & cargo build --features $Features
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

if (-not (Test-Path $Exe)) {
    throw "Binary not found: $Exe"
}

$LogDir = Join-Path $RepoRoot "data\lan-stand\logs"
New-Item -ItemType Directory -Force -Path $LogDir | Out-Null

$started = @()
for ($i = 0; $i -lt $NodeCount; $i++) {
    $port = $BasePort + $i
    $nodeName = "node-$([char](65 + $i))"
    $dataRoot = Join-Path $RepoRoot "data\lan-stand\$nodeName"
    $raidPath = Join-Path $dataRoot "raid"
    New-Item -ItemType Directory -Force -Path $raidPath | Out-Null

    $logOut = Join-Path $LogDir "$nodeName-$port.log"
    $logErr = Join-Path $LogDir "$nodeName-$port.err.log"

    $cmd = @"
Set-Location '$RepoRoot'
`$env:POOLAI_HTTP_PORT='$port'
`$env:POOLAI_RAID_BASE_PATH='$raidPath'
`$env:POOLAI_DATA_PATH='$dataRoot'
`$env:K8S_OPENAPI_ENABLED_VERSION='$($env:K8S_OPENAPI_ENABLED_VERSION)'
`$env:RUST_LOG='$($env:RUST_LOG)'
& '$Exe' > '$logOut' 2> '$logErr'
"@

    Write-Host "Starting $nodeName -> http://127.0.0.1:$port" -ForegroundColor Green
    $p = Start-Process -FilePath "powershell.exe" -ArgumentList @(
        "-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", $cmd
    ) -PassThru -WindowStyle Hidden
    $started += [PSCustomObject]@{ Name = $nodeName; Port = $port; Pid = $p.Id; Log = $logOut; ErrLog = $logErr }
    Start-Sleep -Seconds 3
}

Write-Host ""
$started | Format-Table -AutoSize
Write-Host "Wait ~15s, then health:" -ForegroundColor Cyan
foreach ($n in $started) {
    Write-Host "  Invoke-WebRequest http://127.0.0.1:$($n.Port)/api/v1/health"
}
Write-Host "Stop: Stop-Process -Name poolai -Force -ErrorAction SilentlyContinue" -ForegroundColor Yellow
