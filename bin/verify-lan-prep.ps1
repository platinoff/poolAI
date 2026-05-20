# FM-027: LAN prep — health (+ optional discovery) before FM-003 §4 sign-off.
param(
    [string]$NodeAUrl = $(if ($env:POOLAI_NODE_A_URL) { $env:POOLAI_NODE_A_URL } else { "http://127.0.0.1:8080" }),
    [string]$NodeBUrl = $(if ($env:POOLAI_NODE_B_URL) { $env:POOLAI_NODE_B_URL } else { "http://127.0.0.1:8081" }),
    [int]$TimeoutSec = 5
)

$ErrorActionPreference = "Continue"
$NodeAUrl = $NodeAUrl.TrimEnd("/")
$NodeBUrl = $NodeBUrl.TrimEnd("/")
$TwoHost = [bool]($env:POOLAI_NODE_A_URL -and $env:POOLAI_NODE_B_URL)
$fail = 0

function Test-Health {
    param([string]$Name, [string]$Base, [bool]$Required = $true)
    try {
        $r = Invoke-WebRequest -Uri "$Base/api/v1/health" -TimeoutSec $TimeoutSec -UseBasicParsing
        if ($r.StatusCode -ge 200 -and $r.StatusCode -lt 300) {
            Write-Host "OK  $Name health -> $Base/api/v1/health" -ForegroundColor Green
        } else { throw "HTTP $($r.StatusCode)" }
    } catch {
        if ($Required) {
            Write-Host "FAIL $Name health -> $Base/api/v1/health ($($_.Exception.Message))" -ForegroundColor Red
            $script:fail = 1
        } else {
            Write-Host "SKIP $Name health -> $Base/api/v1/health" -ForegroundColor DarkYellow
        }
    }
}

function Test-Peers {
    param([string]$Name, [string]$Base)
    try {
        $r = Invoke-RestMethod -Uri "$Base/api/v1/discovery/peers" -TimeoutSec $TimeoutSec
        $count = @($r.peers).Count
        Write-Host "OK  $Name discovery/peers -> $count peer(s)" -ForegroundColor Green
    } catch {
        Write-Host "SKIP $Name discovery/peers (unavailable)" -ForegroundColor DarkYellow
    }
}

Write-Host "LAN prep (FM-027): A=$NodeAUrl B=$NodeBUrl two_host=$TwoHost"
Write-Host ""

Test-Health "node-A" $NodeAUrl $true
if ($TwoHost) {
    Test-Health "node-B" $NodeBUrl $true
} else {
    Test-Health "node-B (dual-port dev)" $NodeBUrl $false
}

Test-Peers "node-A" $NodeAUrl
if ($TwoHost) { Test-Peers "node-B" $NodeBUrl }

if ($fail -ne 0) {
    Write-Host ""
    Write-Host "LAN prep failed. Start: .\bin\run-lan-nodes.ps1 or set POOLAI_NODE_*_URL for two hosts." -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "LAN prep passed. Next: docs/performance/LAN_SIGNOFF_CHECKLIST.md" -ForegroundColor Cyan
