# Run PoolAI with enterprise features
# Automatically stops any running poolai process before starting

param(
    [switch]$Enterprise,
    [switch]$Debug,
    [string]$Features = ""
)

Write-Host "Checking for running poolai processes..." -ForegroundColor Yellow
$processes = Get-Process -Name "poolai" -ErrorAction SilentlyContinue
if ($processes) {
    Write-Host "Stopping $($processes.Count) running poolai process(es)..." -ForegroundColor Yellow
    $processes | Stop-Process -Force
    Start-Sleep -Seconds 2
}

$featuresArg = if ($Enterprise) {
    "--features enterprise"
} elseif ($Features) {
    "--features $Features"
} else {
    ""
}

$releaseArg = if (-not $Debug) {
    "--release"
} else {
    ""
}

Write-Host "Starting PoolAI..." -ForegroundColor Green
if ($featuresArg -and $releaseArg) {
    cargo run $releaseArg $featuresArg
} elseif ($featuresArg) {
    cargo run $featuresArg
} elseif ($releaseArg) {
    cargo run $releaseArg
} else {
    cargo run
}
