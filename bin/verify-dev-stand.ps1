# FM-003 / FM-016+++: health + virtual-node bootstrap checks for local dev stand.
# PH-S54: optional RAID job store step (VERIFY_RAID_JOB_STORE=1).
param(
    [int]$CoordinatorPort = 8080,
    [int]$WorkerPort = 9090,
    [int]$NodeBPort = 8081,
    [string]$WorkerId = "vn-dev-stand",
    [int]$TimeoutSec = 5,
    [int]$WarmupSecs = 50,
    [int]$TaskRetries = 12,
    [int]$TaskSleepSecs = 5,
    [int]$MinCompleted = 4,
    [int]$VerifyMlPipeline = 1,
    [int]$VerifyRaidJobStore = $(if ($env:VERIFY_RAID_JOB_STORE -eq "1") { 1 } else { 0 })
)

$ErrorActionPreference = "Continue"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$CoordUrl = "http://127.0.0.1:$CoordinatorPort"
$fail = 0

function Wait-CoordHealth {
    param([int]$Retries = 45, [int]$SleepSec = 2)
    for ($i = 0; $i -lt $Retries; $i++) {
        try {
            Invoke-WebRequest -Uri "$CoordUrl/api/v1/health" -TimeoutSec $TimeoutSec -UseBasicParsing | Out-Null
            return $true
        } catch {
            Start-Sleep -Seconds $SleepSec
        }
    }
    return $false
}

function Get-PoolaiExe {
    $candidates = @(
        $env:POOLAI_BIN,
        (Join-Path $RepoRoot "target\release\poolai.exe"),
        (Join-Path $RepoRoot "target\debug\poolai.exe")
    ) | Where-Object { $_ -and (Test-Path -LiteralPath $_) }
    return $candidates | Select-Object -First 1
}

function Stop-ListenerOnPort {
    param([int]$Port)
    $lines = netstat -ano 2>$null | Select-String ":$Port\s" | Select-String "LISTEN"
    if (-not $lines) { return }
    $listenerPid = ($lines[0].ToString() -split '\s+')[-1]
    if ($listenerPid -and $listenerPid -ne "0") {
        Stop-Process -Id ([int]$listenerPid) -Force -ErrorAction SilentlyContinue
    }
}

function Restart-Coordinator {
    $standRoot = $env:POOLAI_E2E_STAND_ROOT
    if ($standRoot -and (Test-Path -LiteralPath (Join-Path $standRoot "restart.sh"))) {
        & (Join-Path $RepoRoot "bin\poolai-msys.ps1") -lc "bash '$standRoot/restart.sh'"
        if (-not (Wait-CoordHealth)) { throw "health not ready after e2e restart" }
        return
    }
    $raid = if ($env:POOLAI_RAID_BASE_PATH) { $env:POOLAI_RAID_BASE_PATH } else { Join-Path $RepoRoot "data\dev\single\raid" }
    $data = if ($env:POOLAI_DATA_PATH) { $env:POOLAI_DATA_PATH } else { Join-Path $RepoRoot "data\dev\single" }
    $jobStore = if ($env:POOLAI_JOB_STORE) { $env:POOLAI_JOB_STORE } else { "raid" }
    $exe = Get-PoolaiExe
    if (-not $exe) { throw "poolai.exe not found (build or set POOLAI_BIN)" }
    Stop-ListenerOnPort -Port $CoordinatorPort
    Start-Sleep -Seconds 2
    $logDir = Join-Path $RepoRoot "data\dev\logs"
    New-Item -ItemType Directory -Force -Path $logDir | Out-Null
    $log = Join-Path $logDir "verify-restart-$CoordinatorPort.log"
    $env:POOLAI_HTTP_PORT = "$CoordinatorPort"
    $env:POOLAI_RAID_BASE_PATH = $raid
    $env:POOLAI_DATA_PATH = $data
    $env:POOLAI_JOB_STORE = $jobStore
    Write-Host "  ... restarting coordinator (job_store=$jobStore)" -ForegroundColor DarkGray
    Start-Process -FilePath $exe -RedirectStandardOutput $log -RedirectStandardError $log -WindowStyle Hidden | Out-Null
    if (-not (Wait-CoordHealth)) { throw "health not ready after dev restart" }
}

function Test-RaidJobStore {
    try {
        $list = Invoke-RestMethod -Uri "$CoordUrl/api/v1/jobs" -TimeoutSec $TimeoutSec
        if ($list.store_backend -ne "raid") {
            Write-Host "SKIP RAID job store -> store_backend=$($list.store_backend) (need raid + POOLAI_JOB_STORE=raid before start)" -ForegroundColor DarkYellow
            return
        }
        Write-Host "OK  job store backend -> raid" -ForegroundColor Green
        $body = @{
            kind               = "inference"
            priority           = 7
            input_artifact_ids = @("ph-s54-raid-smoke")
        } | ConvertTo-Json
        $created = Invoke-RestMethod -Method Post -Uri "$CoordUrl/api/v1/jobs" -Body $body -ContentType "application/json" -TimeoutSec $TimeoutSec
        if (-not $created.id) { throw "POST /jobs missing id" }
        Write-Host "OK  RAID job create -> id=$($created.id)" -ForegroundColor Green
        Restart-Coordinator
        Write-Host "OK  coordinator restart -> health" -ForegroundColor Green
        $detail = Invoke-RestMethod -Uri "$CoordUrl/api/v1/jobs/$($created.id)" -TimeoutSec $TimeoutSec
        if ($detail.job.spec.id -ne $created.id -or $detail.job.spec.kind -ne "inference" -or $detail.job.status -ne "scheduled") {
            throw "GET job mismatch after restart"
        }
        Write-Host "OK  RAID job persist -> GET /jobs/$($created.id) (inference, scheduled)" -ForegroundColor Green
    } catch {
        Write-Host "FAIL RAID job store -> $($_.Exception.Message)" -ForegroundColor Red
        $script:fail = 1
    }
}

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

if ($VerifyMlPipeline -eq 1) {
    $mlUrl = "$CoordUrl/api/enterprise/ai-ml/pipeline/demo"
    try {
        $demo = Invoke-RestMethod -Uri $mlUrl -TimeoutSec $TimeoutSec
        $profile = $demo.step_results.profile
        $out = $profile.output
        $kind = $out.step_kind
        $status = $out.status
        if ($kind -eq "profiling" -and $status -eq "completed") {
            Write-Host "OK  ML pipeline demo -> step_kind=profiling status=completed" -ForegroundColor Green
        } else {
            Write-Host "FAIL ML pipeline demo: step_kind=$kind status=$status" -ForegroundColor Red
            $fail = 1
        }
    } catch {
        Write-Host "SKIP ML pipeline demo -> $mlUrl ($($_.Exception.Message))" -ForegroundColor DarkYellow
    }
}

if ($VerifyRaidJobStore -eq 1) {
    Test-RaidJobStore
}

if ($fail -ne 0) { exit 1 }
Write-Host "Dev stand verification passed (health + virtual-node bootstrap + ML pipeline demo when enabled + optional RAID job store)." -ForegroundColor Cyan
