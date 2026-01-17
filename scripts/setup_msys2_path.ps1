# Setup MSYS2 PATH for Rust compilation
# This script adds MSYS2 bin directory to PATH for dlltool.exe and other tools

$msys2Path = "C:\msys64\usr\bin"

if (Test-Path $msys2Path) {
    if ($env:PATH -notlike "*$msys2Path*") {
        $env:PATH = "$msys2Path;$env:PATH"
        Write-Host "Added MSYS2 to PATH: $msys2Path" -ForegroundColor Green
    } else {
        Write-Host "MSYS2 already in PATH: $msys2Path" -ForegroundColor Yellow
    }

    # Verify dlltool.exe is accessible
    $dlltool = Get-Command dlltool -ErrorAction SilentlyContinue
    if ($dlltool) {
        Write-Host "dlltool.exe found at: $($dlltool.Source)" -ForegroundColor Green
        dlltool --version 2>&1 | Select-Object -First 1
    } else {
        Write-Host "Warning: dlltool.exe not found in PATH" -ForegroundColor Red
    }
} else {
    Write-Host "MSYS2 not found at: $msys2Path" -ForegroundColor Red
    Write-Host "Please install MSYS2 or update the path in this script" -ForegroundColor Yellow
}
