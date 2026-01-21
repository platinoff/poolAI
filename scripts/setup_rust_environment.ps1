# Setup Rust Environment Script
# Automatically detects and configures the correct Rust toolchain environment
# Run: .\scripts\setup_rust_environment.ps1

Write-Host "🔧 Setting up Rust development environment..." -ForegroundColor Cyan
Write-Host ""

# Check current toolchain
Write-Host "📋 Current Rust toolchain:" -ForegroundColor Cyan
rustup show
Write-Host ""

# Read rust-toolchain.toml
$toolchainFile = "rust-toolchain.toml"
if (Test-Path $toolchainFile) {
    Write-Host "📄 Reading $toolchainFile..." -ForegroundColor Cyan
    $content = Get-Content $toolchainFile -Raw
    
    if ($content -match 'channel\s*=\s*"([^"]+)"') {
        $channel = $matches[1]
        Write-Host "   Channel: $channel" -ForegroundColor Gray
        
        if ($channel -like "*msvc*") {
            Write-Host "   ✅ MSVC toolchain detected" -ForegroundColor Green
            Write-Host ""
            Write-Host "🔧 Setting up MSVC environment..." -ForegroundColor Cyan
            & "$PSScriptRoot\setup_msvc_environment.ps1"
        } elseif ($channel -like "*gnu*") {
            Write-Host "   ✅ GNU toolchain detected" -ForegroundColor Green
            Write-Host ""
            Write-Host "🔧 Setting up GNU/MSYS2 environment..." -ForegroundColor Cyan
            
            # Setup MSYS2 PATH
            $msys2Paths = @(
                "C:\msys64\ucrt64\bin",
                "C:\msys64\usr\bin"
            )
            
            $addedPaths = @()
            foreach ($msys2Path in $msys2Paths) {
                if (Test-Path $msys2Path) {
                    if ($env:PATH -notlike "*$msys2Path*") {
                        $env:PATH = "$msys2Path;$env:PATH"
                        $addedPaths += $msys2Path
                        Write-Host "   ✅ Added to PATH: $msys2Path" -ForegroundColor Green
                    }
                }
            }
            
            # Ensure Cargo is first
            $cargoPath = "$env:USERPROFILE\.cargo\bin"
            if ($env:PATH -notlike "*$cargoPath*") {
                $env:PATH = "$cargoPath;$env:PATH"
            } else {
                # Move Cargo to front
                $pathParts = $env:PATH -split ';'
                $pathParts = @($cargoPath) + ($pathParts | Where-Object { $_ -ne $cargoPath })
                $env:PATH = $pathParts -join ';'
            }
            
            # Set CC environment variables
            $env:CC = "gcc"
            $env:CXX = "g++"
            $env:CC_x86_64_pc_windows_gnu = "gcc"
            
            Write-Host ""
            Write-Host "✅ GNU/MSYS2 environment configured" -ForegroundColor Green
        }
    }
} else {
    Write-Host "⚠️  $toolchainFile not found. Using default toolchain." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🔍 Verifying Rust tools..." -ForegroundColor Cyan
$rustcVersion = rustc --version 2>&1
$cargoVersion = cargo --version 2>&1

if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✅ $rustcVersion" -ForegroundColor Green
    Write-Host "   ✅ $cargoVersion" -ForegroundColor Green
} else {
    Write-Host "   ❌ Rust tools not found. Please install Rust via rustup." -ForegroundColor Red
    Write-Host "      https://rustup.rs/" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "✅ Environment setup complete!" -ForegroundColor Green
Write-Host "   You can now run: cargo build" -ForegroundColor Cyan
