# Setup MSYS2 PATH for Rust compilation
# This script adds MSYS2 bin directories to PATH for gcc.exe, dlltool.exe and other tools

$msys2Paths = @(
    "C:\msys64\ucrt64\bin",  # UCRT64 toolchain (gcc, g++, etc.)
    "C:\msys64\usr\bin"      # MSYS2 tools (dlltool, etc.)
)

$addedPaths = @()

foreach ($msys2Path in $msys2Paths) {
    if (Test-Path $msys2Path) {
        if ($env:PATH -notlike "*$msys2Path*") {
            $env:PATH = "$msys2Path;$env:PATH"
            $addedPaths += $msys2Path
            Write-Host "Added MSYS2 to PATH: $msys2Path" -ForegroundColor Green
        } else {
            Write-Host "MSYS2 already in PATH: $msys2Path" -ForegroundColor Yellow
        }
    } else {
        Write-Host "MSYS2 path not found: $msys2Path" -ForegroundColor Yellow
    }
}

if ($addedPaths.Count -gt 0) {
    Write-Host ""
    Write-Host "MSYS2 paths added to PATH for this session" -ForegroundColor Green
    Write-Host "Note: This is only for the current PowerShell session" -ForegroundColor Yellow
    Write-Host "For permanent setup, add these paths to System Environment Variables" -ForegroundColor Yellow
}

# Verify tools are accessible
Write-Host ""
Write-Host "Verifying tools..." -ForegroundColor Cyan

$tools = @("gcc", "dlltool")
foreach ($tool in $tools) {
    $cmd = Get-Command $tool -ErrorAction SilentlyContinue
    if ($cmd) {
        Write-Host "  $tool.exe found at: $($cmd.Source)" -ForegroundColor Green
    } else {
        Write-Host "  Warning: $tool.exe not found in PATH" -ForegroundColor Red
    }
}

# Set CC environment variables for cargo
$env:CC = "gcc"
$env:CC_x86_64_pc_windows_gnu = "gcc"
Write-Host ""
Write-Host "CC environment variables set:" -ForegroundColor Green
Write-Host "  CC=$env:CC"
Write-Host "  CC_x86_64_pc_windows_gnu=$env:CC_x86_64_pc_windows_gnu"
