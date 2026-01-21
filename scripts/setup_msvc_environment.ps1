# Setup MSVC Environment for Rust Development
# This script configures PATH, LIB, and INCLUDE for MSVC toolchain
# Run: .\scripts\setup_msvc_environment.ps1

Write-Host "🔧 Setting up MSVC environment for Rust..." -ForegroundColor Cyan

# Find Visual Studio installation
$vsPaths = @(
    "C:\Program Files\Microsoft Visual Studio\2022\Community",
    "C:\Program Files\Microsoft Visual Studio\2022\BuildTools",
    "C:\Program Files\Microsoft Visual Studio\2022\Professional",
    "C:\Program Files\Microsoft Visual Studio\2022\Enterprise"
)

$vsPath = $null
foreach ($path in $vsPaths) {
    if (Test-Path $path) {
        $vsPath = $path
        Write-Host "✅ Found Visual Studio at: $vsPath" -ForegroundColor Green
        break
    }
}

if (-not $vsPath) {
    Write-Host "⚠️  Visual Studio not found. Please install Visual Studio 2022 with C++ workload." -ForegroundColor Yellow
    Write-Host "   Download: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
    exit 1
}

# Find MSVC version
$msvcPath = "$vsPath\VC\Tools\MSVC"
if (-not (Test-Path $msvcPath)) {
    Write-Host "❌ MSVC Tools not found. Please install 'Desktop development with C++' workload." -ForegroundColor Red
    exit 1
}

$msvcVersions = Get-ChildItem $msvcPath | Sort-Object Name -Descending
if ($msvcVersions.Count -eq 0) {
    Write-Host "❌ No MSVC versions found." -ForegroundColor Red
    exit 1
}

$msvcVersion = $msvcVersions[0].Name
Write-Host "✅ Using MSVC version: $msvcVersion" -ForegroundColor Green

# Find Windows SDK
$sdkPath = "C:\Program Files (x86)\Windows Kits\10"
if (-not (Test-Path $sdkPath)) {
    Write-Host "⚠️  Windows SDK not found. Some features may not work." -ForegroundColor Yellow
}

$sdkVersions = @()
if (Test-Path "$sdkPath\Lib") {
    $sdkVersions = Get-ChildItem "$sdkPath\Lib" | Sort-Object Name -Descending | Select-Object -First 1
}

# Set up paths
$msvcBinPath = "$vsPath\VC\Tools\MSVC\$msvcVersion\bin\Hostx64\x64"
$msvcIncludePath = "$vsPath\VC\Tools\MSVC\$msvcVersion\include"
$msvcLibPath = "$vsPath\VC\Tools\MSVC\$msvcVersion\lib\x64"

# Build INCLUDE paths
$includePaths = @($msvcIncludePath)
if ($sdkVersions.Count -gt 0) {
    $sdkVersion = $sdkVersions[0].Name
    $includePaths += @(
        "$sdkPath\Include\$sdkVersion\shared",
        "$sdkPath\Include\$sdkVersion\ucrt",
        "$sdkPath\Include\$sdkVersion\um",
        "$sdkPath\Include\$sdkVersion\winrt"
    )
}

# Build LIB paths
$libPaths = @($msvcLibPath)
if ($sdkVersions.Count -gt 0) {
    $sdkVersion = $sdkVersions[0].Name
    $libPaths += @(
        "$sdkPath\Lib\$sdkVersion\ucrt\x64",
        "$sdkPath\Lib\$sdkVersion\um\x64"
    )
}

# Set environment variables
$env:PATH = "$msvcBinPath;$env:PATH"
$env:INCLUDE = ($includePaths -join ";") + ";$env:INCLUDE"
$env:LIB = ($libPaths -join ";") + ";$env:LIB"

# Ensure Cargo is in PATH (before MSYS2 to avoid conflicts)
$cargoPath = "$env:USERPROFILE\.cargo\bin"
if ($env:PATH -notlike "*$cargoPath*") {
    $env:PATH = "$cargoPath;$env:PATH"
}

# Remove MSYS2 from PATH temporarily (to avoid conflicts with MSVC)
$env:PATH = ($env:PATH -split ';' | Where-Object { $_ -notlike '*msys64*' }) -join ';'

Write-Host ""
Write-Host "✅ MSVC environment configured:" -ForegroundColor Green
Write-Host "   PATH includes: $msvcBinPath" -ForegroundColor Gray
Write-Host "   INCLUDE: $($includePaths.Count) paths" -ForegroundColor Gray
Write-Host "   LIB: $($libPaths.Count) paths" -ForegroundColor Gray
Write-Host ""

# Verify tools
Write-Host "🔍 Verifying tools..." -ForegroundColor Cyan

$tools = @("cl.exe", "link.exe", "rustc.exe", "cargo.exe")
foreach ($tool in $tools) {
    $cmd = Get-Command $tool -ErrorAction SilentlyContinue
    if ($cmd) {
        Write-Host "   ✅ $tool found at: $($cmd.Source)" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  $tool not found in PATH" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "💡 Note: These settings are only for the current PowerShell session." -ForegroundColor Yellow
Write-Host "   To make permanent, add to your PowerShell profile or use Developer PowerShell." -ForegroundColor Yellow
Write-Host ""
Write-Host "🚀 You can now run: cargo build" -ForegroundColor Cyan
