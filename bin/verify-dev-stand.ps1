# FM-003 / FM-016+++: health + virtual-node bootstrap checks for local dev stand.
param(
    [int]$CoordinatorPort = 8080,
    [int]$WorkerPort = 9090,
    [int]$NodeBPort = 8081,
    [string]$WorkerId = "vn-dev-stand",
    [int]$TimeoutSec = 5,
    [int]$WarmupSecs = 50,
    [int]$TaskRetries = 12,
    [int]$TaskSleepSecs = 5,
    [int]$MinCompleted = 4
)

$ErrorActionPreference = "Continue"
$CoordUrl = "http://127.0.0.1:$CoordinatorPort"
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

Test-Endpoint "coordinator" "$CoordUrl/api/v1/health"
Test-Endpoint "node-B" "http://127.0.0.1:$NodeBPort/api/v1/health" -Optional
Test-Endpoint "virtual worker" "http://127.0.0.1:$WorkerPort/health"

if ($fail -ne 0) { exit 1 }

Write-Host "Waiting ${WarmupSecs}s for worker bootstrap tasks..." -ForegroundColor Cyan
Start-Sleep -Seconds $WarmupSecs

try {
    $vn = Invoke-RestMethod -Uri "$CoordUrl/api/v1/discovery/virtual-nodes" -TimeoutSec $TimeoutSec
    $found = @($vn.nodes | Where-Object { $_.peer.peer_id -eq $WorkerId }).Count -gt 0
    if ($found) {
        Write-Host "OK  discovery virtual-node -> $WorkerId" -ForegroundColor Green
    } else {
        Write-Host "FAIL discovery virtual-node missing $WorkerId" -ForegroundColor Red
        $fail = 1
    }
} catch {
    Write-Host "FAIL discovery virtual-nodes ($($_.Exception.Message))" -ForegroundColor Red
    $fail = 1
}

try {
    $workers = Invoke-RestMethod -Uri "$CoordUrl/api/v1/workers" -TimeoutSec $TimeoutSec
    $inPool = @($workers | Where-Object { $_.id -eq $WorkerId }).Count -gt 0
    if ($inPool) {
        Write-Host "OK  pool join -> worker $WorkerId in /workers" -ForegroundColor Green
    } else {
        Write-Host "FAIL pool join: $WorkerId not in /workers" -ForegroundColor Red
        $fail = 1
    }
} catch {
    Write-Host "FAIL /workers ($($_.Exception.Message))" -ForegroundColor Red
    $fail = 1
}

$completed = 0
for ($i = 0; $i -lt $TaskRetries; $i++) {
    try {
        $status = Invoke-RestMethod -Uri "$CoordUrl/api/v1/virtual-nodes/$WorkerId/tasks/status" -TimeoutSec $TimeoutSec
        $completed = [int]$status.completed
        $pending = $status.pending
        if ($completed -ge $MinCompleted) {
            Write-Host "OK  bootstrap tasks -> completed=$completed pending=$pending" -ForegroundColor Green
            break
        }
        if ($i -lt ($TaskRetries - 1)) {
            Write-Host "  ... tasks completed=$completed/$MinCompleted, retry in ${TaskSleepSecs}s" -ForegroundColor DarkGray
            Start-Sleep -Seconds $TaskSleepSecs
        }
    } catch {
        Write-Host "  ... tasks status error: $($_.Exception.Message)" -ForegroundColor DarkYellow
        Start-Sleep -Seconds $TaskSleepSecs
    }
}

if ($completed -lt $MinCompleted) {
    Write-Host "FAIL bootstrap tasks: completed=$completed (need >= $MinCompleted)" -ForegroundColor Red
    $fail = 1
}

try {
    $health = Invoke-RestMethod -Uri "http://127.0.0.1:$WorkerPort/health" -TimeoutSec $TimeoutSec
    if ([int]$health.cached_artifacts -ge 1) {
        Write-Host "OK  worker cache -> cached_artifacts=$($health.cached_artifacts)" -ForegroundColor Green
    } else {
        Write-Host "FAIL worker cache: cached_artifacts=$($health.cached_artifacts)" -ForegroundColor Red
        $fail = 1
    }
} catch {
    Write-Host "FAIL worker /health for cache check ($($_.Exception.Message))" -ForegroundColor Red
    $fail = 1
}

if ($fail -ne 0) { exit 1 }
Write-Host "Dev stand verification passed (health + virtual-node bootstrap)." -ForegroundColor Cyan
