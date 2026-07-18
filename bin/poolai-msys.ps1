# Invoke MSYS2 UCRT64 bash with PoolAI PATH (avoids WSL "bash" stub in PowerShell).
# Usage:
#   .\bin\poolai-msys.ps1 bin/run-poolai.sh single --bg --skip-build
#   .\bin\poolai-msys.ps1 -lc "cargo test-ci"
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Rest
)

$ErrorActionPreference = "Stop"

if (-not $Rest -or $Rest.Count -eq 0) {
    Write-Host @"
Usage: .\bin\poolai-msys.ps1 <command> [args...]

Examples:
  .\bin\poolai-msys.ps1 bin/run-poolai.sh status
  .\bin\poolai-msys.ps1 bin/run-poolai.sh single --bg --skip-build
  .\bin\poolai-msys.ps1 -lc "export K8S_OPENAPI_ENABLED_VERSION=1.28; cargo test-ci"

Prefer native PowerShell for run/stop only:
  .\bin\run-poolai.ps1 single -Background -SkipBuild
"@
    exit 0
}

$MsysBash = if ($env:POOLAI_MSYS_BASH) { $env:POOLAI_MSYS_BASH } else { "C:\msys64\usr\bin\bash.exe" }
if (-not (Test-Path -LiteralPath $MsysBash)) {
    throw "MSYS2 bash not found at $MsysBash. Install MSYS2 UCRT64 or set POOLAI_MSYS_BASH."
}

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$Drive = $RepoRoot.Substring(0, 1).ToLowerInvariant()
$UnixRepo = "/" + $Drive + ($RepoRoot.Substring(2) -replace '\\', '/')

$k8s = $env:K8S_OPENAPI_ENABLED_VERSION
if (-not $k8s) { $k8s = "1.28" }

function ConvertTo-BashSingleQuoted {
    param([string]$Text)
    return $Text.Replace("'", "'\''")
}

if ($Rest[0] -eq "-lc") {
    if ($Rest.Count -eq 2) {
        $userCmd = $Rest[1]
    } else {
        $userCmd = (($Rest | Select-Object -Skip 1) | ForEach-Object { ConvertTo-BashSingleQuoted $_ }) -join " "
    }
} else {
    $userCmd = ($Rest | ForEach-Object { ConvertTo-BashSingleQuoted $_ }) -join " "
}
$bashScript = "export PATH=`"`$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:`$PATH`"; " +
    "export K8S_OPENAPI_ENABLED_VERSION=$k8s; " +
    "cd '$UnixRepo' || cd 'S:/rust/poolAI' || exit 1; " +
    $userCmd

& $MsysBash -lc $bashScript
exit $LASTEXITCODE
