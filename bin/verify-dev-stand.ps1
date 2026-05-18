# FM-003: health checks for local dev stand (after run-lan-nodes or run-virtual-node-dev).
param(
    [int]$CoordinatorPort = 8080,
    [int]$WorkerPort = 9090,
    [int]$NodeBPort = 8081,
    [int]$TimeoutSec = 5
)

$ErrorActionPreference = "Continue"
$fail = 0

function Test-Endpoint {
    param([string]$Name, [string]$Url, [switch]$Optional)
    try {
        $r = Invoke-WebRequest -Uri $Url -TimeoutSec $TimeoutSec -UseBasicParsing
        if ($r.StatusCode -ge 200 -and $r.StatusCode -lt 300) {
            Write-Host "OK  $Name -> $Url" -ForegroundColor Green
        } else {
            throw "HTTP $($r.StatusCode)"
        }
    } catch {
        if ($Optional) {
            Write-Host "SKIP $Name -> $Url ($($_.Exception.Message))" -ForegroundColor DarkYellow
        } else {
            Write-Host "FAIL $Name -> $Url ($($_.Exception.Message))" -ForegroundColor Red
            $script:fail = 1
        }
    }
}

Test-Endpoint "coordinator" "http://127.0.0.1:$CoordinatorPort/api/v1/health"
Test-Endpoint "node-B" "http://127.0.0.1:$NodeBPort/api/v1/health" -Optional
Test-Endpoint "virtual worker" "http://127.0.0.1:$WorkerPort/health"

if ($fail -ne 0) { exit 1 }
Write-Host "Dev stand health checks passed." -ForegroundColor Cyan
