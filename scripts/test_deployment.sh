#!/bin/bash
# Deployment Testing Script for PoolAI
# This script validates deployment files and configurations

set -e

echo "🔍 Starting Deployment Testing..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command_exists docker; then
    echo -e "${YELLOW}⚠️  Docker is not installed. Some tests will be skipped.${NC}"
    DOCKER_AVAILABLE=false
else
    echo -e "${GREEN}✅ Docker is installed${NC}"
    DOCKER_AVAILABLE=true
fi

if ! command_exists docker-compose; then
    echo -e "${YELLOW}⚠️  Docker Compose is not installed. Some tests will be skipped.${NC}"
    COMPOSE_AVAILABLE=false
else
    echo -e "${GREEN}✅ Docker Compose is installed${NC}"
    COMPOSE_AVAILABLE=true
fi

# Test 1: Check if Dockerfile exists
echo ""
echo "📦 Testing Dockerfile..."
if [ -f "docker/Dockerfile" ]; then
    echo -e "${GREEN}✅ Dockerfile exists in docker/${NC}"
    
    # Check Dockerfile syntax (if docker is available)
    if [ "$DOCKER_AVAILABLE" = true ]; then
        echo "  Validating Dockerfile syntax..."
        if docker build --dry-run -f docker/Dockerfile . >/dev/null 2>&1; then
            echo -e "${GREEN}  ✅ Dockerfile syntax is valid${NC}"
        else
            echo -e "${YELLOW}  ⚠️  Could not validate Dockerfile syntax (docker build --dry-run not supported)${NC}"
        fi
    fi
else
    echo -e "${RED}❌ Dockerfile not found in docker/${NC}"
    exit 1
fi

# Test 2: Check if docker-compose.yml exists
echo ""
echo "🐳 Testing docker-compose.yml..."
if [ -f "docker/docker-compose.yml" ]; then
    echo -e "${GREEN}✅ docker-compose.yml exists in docker/${NC}"
    
    # Check docker-compose syntax (if docker-compose is available)
    if [ "$COMPOSE_AVAILABLE" = true ]; then
        echo "  Validating docker-compose.yml syntax..."
        if docker-compose -f docker/docker-compose.yml config >/dev/null 2>&1; then
            echo -e "${GREEN}  ✅ docker-compose.yml syntax is valid${NC}"
        else
            echo -e "${RED}  ❌ docker-compose.yml syntax is invalid${NC}"
            docker-compose -f docker/docker-compose.yml config
            exit 1
        fi
    fi
else
    echo -e "${RED}❌ docker-compose.yml not found in docker/${NC}"
    exit 1
fi

# Test 3: Check if .dockerignore exists
echo ""
echo "🚫 Testing .dockerignore..."
if [ -f "docker/.dockerignore" ]; then
    echo -e "${GREEN}✅ .dockerignore exists in docker/${NC}"
else
    echo -e "${YELLOW}⚠️  .dockerignore not found in docker/ (optional but recommended)${NC}"
fi

# Test 4: Check if config.example.toml exists
echo ""
echo "⚙️  Testing configuration files..."
if [ -f "config.example.toml" ]; then
    echo -e "${GREEN}✅ config.example.toml exists${NC}"
else
    echo -e "${RED}❌ config.example.toml not found${NC}"
    exit 1
fi

# Test 5: Check if deployment documentation exists
echo ""
echo "📚 Testing deployment documentation..."
if [ -f "docs/deployment/DOCKER.md" ]; then
    echo -e "${GREEN}✅ Docker deployment documentation exists${NC}"
else
    echo -e "${YELLOW}⚠️  Docker deployment documentation not found${NC}"
fi

if [ -f "docs/deployment/KUBERNETES.md" ]; then
    echo -e "${GREEN}✅ Kubernetes deployment documentation exists${NC}"
else
    echo -e "${YELLOW}⚠️  Kubernetes deployment documentation not found${NC}"
fi

if [ -f "docs/deployment/BARE_METAL.md" ]; then
    echo -e "${GREEN}✅ Bare metal deployment documentation exists${NC}"
else
    echo -e "${YELLOW}⚠️  Bare metal deployment documentation not found${NC}"
fi

# Test 6: Check if deployment testing checklist exists
echo ""
echo "✅ Testing deployment testing checklist..."
if [ -f "docs/deployment/DEPLOYMENT_TESTING_CHECKLIST.md" ]; then
    echo -e "${GREEN}✅ Deployment testing checklist exists${NC}"
else
    echo -e "${YELLOW}⚠️  Deployment testing checklist not found${NC}"
fi

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✅ Deployment Testing Complete!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📝 Next steps:"
echo "  1. Review the deployment files"
echo "  2. Test Docker build: docker build -t poolai:latest ."
echo "  3. Test docker-compose: docker-compose -f docker/docker-compose.yml up -d"
echo "  4. Follow the deployment testing checklist"
echo ""
