# Docker Deployment Guide

## Overview

This guide covers deploying PoolAI using Docker containers. Docker provides a consistent environment across different platforms and simplifies deployment.

## Prerequisites

- Docker Engine 20.10+ or Docker Desktop
- Docker Compose 2.0+ (optional, for multi-container setups)
- At least 4GB RAM available for Docker
- 10GB+ free disk space

## Quick Start

### Single Container Deployment

```bash
# Build the Docker image
docker build -t poolai:latest .

# Run the container
docker run -d \
  --name poolai \
  -p 8080:8080 \
  -p 8443:8443 \
  -v poolai-data:/data \
  -v poolai-config:/config \
  poolai:latest
```

### Using Docker Compose

A `docker-compose.yml` file is provided in the project root. To use it:

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f poolai

# Stop all services
docker-compose down
```

The `docker-compose.yml` file includes:
- PoolAI service with health checks
- Volume management for data and configuration
- Network configuration
- Optional Prometheus and Grafana services (commented out)

**Note**: The `docker-compose.yml` file is already present in the project root. You can customize it as needed.
      - poolai-certs:/certs
    environment:
      - RUST_LOG=info
      - POOLAI_CONFIG_PATH=/config/config.toml
    networks:
      - poolai-network

volumes:
  poolai-data:
    driver: local
  poolai-config:
    driver: local
  poolai-certs:
    driver: local

networks:
  poolai-network:
    driver: bridge
```

Start with:

```bash
docker-compose up -d
```

## Dockerfile

Create a `Dockerfile` in the project root:

```dockerfile
# Build stage
FROM rust:1.75-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/poolai /app/poolai

# Create directories
RUN mkdir -p /data /config /certs

# Set permissions
RUN chmod +x /app/poolai

# Expose ports
EXPOSE 8080 8443

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/api/v1/health || exit 1

# Run the application
CMD ["/app/poolai"]
```

## Configuration

### Environment Variables

- `RUST_LOG`: Log level (default: `info`)
- `POOLAI_CONFIG_PATH`: Path to config file (default: `/config/config.toml`)
- `POOLAI_DATA_DIR`: Data directory (default: `/data`)
- `POOLAI_CERTS_DIR`: Certificates directory (default: `/certs`)

### Volume Mounts

- `/data`: Persistent data storage (artifacts, libraries, etc.)
- `/config`: Configuration files
- `/certs`: SSL/TLS certificates (if using HTTPS)

## Production Considerations

### Resource Limits

```yaml
services:
  poolai:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G
        reservations:
          cpus: '1'
          memory: 2G
```

### Security

1. **Run as non-root user**:
```dockerfile
RUN useradd -r -s /bin/false poolai
USER poolai
```

2. **Read-only root filesystem** (where possible):
```yaml
read_only: true
tmpfs:
  - /tmp
```

3. **Network isolation**:
```yaml
networks:
  poolai-network:
    driver: bridge
    internal: false  # Set to true for internal-only network
```

### Health Checks

The Dockerfile includes a health check. Monitor with:

```bash
docker ps --format "table {{.Names}}\t{{.Status}}"
```

### Logging

Configure logging driver:

```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```

## Multi-Container Setup

For distributed RAID deployment:

```yaml
version: '3.8'

services:
  poolai-node1:
    build: .
    environment:
      - NODE_ID=node1
      - RAFT_CLUSTER=node1:8080,node2:8080,node3:8080
    networks:
      - poolai-cluster

  poolai-node2:
    build: .
    environment:
      - NODE_ID=node2
      - RAFT_CLUSTER=node1:8080,node2:8080,node3:8080
    networks:
      - poolai-cluster

  poolai-node3:
    build: .
    environment:
      - NODE_ID=node3
      - RAFT_CLUSTER=node1:8080,node2:8080,node3:8080
    networks:
      - poolai-cluster

networks:
  poolai-cluster:
    driver: bridge
```

## Troubleshooting

### Container won't start

```bash
# Check logs
docker logs poolai

# Check container status
docker ps -a

# Inspect container
docker inspect poolai
```

### Permission issues

```bash
# Fix volume permissions
docker run --rm -v poolai-data:/data alpine chown -R 1000:1000 /data
```

### Network issues

```bash
# Check network connectivity
docker exec poolai ping -c 3 8.8.8.8

# Inspect network
docker network inspect poolai-network
```

## Updating

```bash
# Pull latest changes
git pull

# Rebuild image
docker build -t poolai:latest .

# Restart container
docker-compose down
docker-compose up -d
```

## Backup and Restore

### Backup

```bash
# Backup data volume
docker run --rm -v poolai-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/poolai-data-backup.tar.gz /data

# Backup config
docker run --rm -v poolai-config:/config -v $(pwd):/backup \
  alpine tar czf /backup/poolai-config-backup.tar.gz /config
```

### Restore

```bash
# Restore data
docker run --rm -v poolai-data:/data -v $(pwd):/backup \
  alpine tar xzf /backup/poolai-data-backup.tar.gz -C /

# Restore config
docker run --rm -v poolai-config:/config -v $(pwd):/backup \
  alpine tar xzf /backup/poolai-config-backup.tar.gz -C /
```

