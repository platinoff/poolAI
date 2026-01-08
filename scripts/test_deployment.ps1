# Deployment Testing Script for PoolAI (PowerShell)
# This script validates deployment files and configurations

$ErrorActionPreference = "Stop"

Write-Host "🔍 Starting Deployment Testing..." -ForegroundColor Cyan

# Function to check if command exists
function Test-Command {
    param([string]$Command)
    $null = Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

# Check prerequisites
Write-Host "`n📋 Checking prerequisites..." -ForegroundColor Cyan

$dockerAvailable = Test-Command "docker"
$composeAvailable = Test-Command "docker-compose"

if (-not $dockerAvailable) {
    Write-Host "⚠️  Docker is not installed. Some tests will be skipped." -ForegroundColor Yellow
} else {
    Write-Host "✅ Docker is installed" -ForegroundColor Green
}

if (-not $composeAvailable) {
    Write-Host "⚠️  Docker Compose is not installed. Some tests will be skipped." -ForegroundColor Yellow
} else {
    Write-Host "✅ Docker Compose is installed" -ForegroundColor Green
}

# Test 1: Check if Dockerfile exists
Write-Host "`n📦 Testing Dockerfile..." -ForegroundColor Cyan
if (Test-Path "Dockerfile") {
    Write-Host "✅ Dockerfile exists" -ForegroundColor Green
    
    # Check Dockerfile content
    $dockerfileContent = Get-Content "Dockerfile" -Raw
    if ($dockerfileContent -match "FROM rust:") {
        Write-Host "  ✅ Contains Rust builder stage" -ForegroundColor Green
    }
    if ($dockerfileContent -match "EXPOSE 8080") {
        Write-Host "  ✅ Exposes port 8080" -ForegroundColor Green
    }
    if ($dockerfileContent -match "EXPOSE 8443") {
        Write-Host "  ✅ Exposes port 8443" -ForegroundColor Green
    }
    if ($dockerfileContent -match "USER ") {
        Write-Host "  ✅ Uses non-root user" -ForegroundColor Green
    }
    if ($dockerfileContent -match "HEALTHCHECK") {
        Write-Host "  ✅ Has health check" -ForegroundColor Green
    }
} else {
    Write-Host "❌ Dockerfile not found" -ForegroundColor Red
    exit 1
}

# Test 2: Check if docker-compose.yml exists
Write-Host "`n🐳 Testing docker-compose.yml..." -ForegroundColor Cyan
if (Test-Path "docker-compose.yml") {
    Write-Host "✅ docker-compose.yml exists" -ForegroundColor Green
    
    # Check docker-compose syntax (if docker-compose is available)
    if ($composeAvailable) {
        Write-Host "  Validating docker-compose.yml syntax..." -ForegroundColor Gray
        try {
            docker-compose config | Out-Null
            Write-Host "  ✅ docker-compose.yml syntax is valid" -ForegroundColor Green
        } catch {
            Write-Host "  ❌ docker-compose.yml syntax is invalid" -ForegroundColor Red
            docker-compose config
            exit 1
        }
    }
} else {
    Write-Host "❌ docker-compose.yml not found" -ForegroundColor Red
    exit 1
}

# Test 3: Check if .dockerignore exists
Write-Host "`n🚫 Testing .dockerignore..." -ForegroundColor Cyan
if (Test-Path ".dockerignore") {
    Write-Host "✅ .dockerignore exists" -ForegroundColor Green
} else {
    Write-Host "⚠️  .dockerignore not found (optional but recommended)" -ForegroundColor Yellow
}

# Test 4: Check if config.example.toml exists
Write-Host "`n⚙️  Testing configuration files..." -ForegroundColor Cyan
if (Test-Path "config.example.toml") {
    Write-Host "✅ config.example.toml exists" -ForegroundColor Green
} else {
    Write-Host "❌ config.example.toml not found" -ForegroundColor Red
    exit 1
}

# Test 5: Check if deployment documentation exists
Write-Host "`n📚 Testing deployment documentation..." -ForegroundColor Cyan
$docs = @(
    "docs/deployment/DOCKER.md",
    "docs/deployment/KUBERNETES.md",
    "docs/deployment/BARE_METAL.md"
)

foreach ($doc in $docs) {
    if (Test-Path $doc) {
        $docName = Split-Path $doc -Leaf
        Write-Host "✅ $docName exists" -ForegroundColor Green
    } else {
        $docName = Split-Path $doc -Leaf
        Write-Host "⚠️  $docName not found" -ForegroundColor Yellow
    }
}

# Test 6: Check if deployment testing checklist exists
Write-Host "`n✅ Testing deployment testing checklist..." -ForegroundColor Cyan
if (Test-Path "docs/deployment/DEPLOYMENT_TESTING_CHECKLIST.md") {
    Write-Host "✅ Deployment testing checklist exists" -ForegroundColor Green
} else {
    Write-Host "⚠️  Deployment testing checklist not found" -ForegroundColor Yellow
}

# Summary
Write-Host "`n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "✅ Deployment Testing Complete!" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host ""
Write-Host "📝 Next steps:" -ForegroundColor Cyan
Write-Host "  1. Review the deployment files"
Write-Host "  2. Test Docker build: docker build -t poolai:latest ."
Write-Host "  3. Test docker-compose: docker-compose up -d"
Write-Host "  4. Follow the deployment testing checklist"
Write-Host ""
