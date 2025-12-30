# Bare Metal Deployment Guide

## Overview

This guide covers deploying PoolAI directly on physical servers or virtual machines without containerization.

## Prerequisites

- Linux (Ubuntu 22.04+, Debian 12+, RHEL 9+) or Windows Server 2019+
- Rust 1.75+ (for building from source)
- OpenSSL 3.0+ (for HTTPS support)
- 4GB+ RAM
- 10GB+ free disk space
- Network access for distributed RAID (if using)

## Installation

### Linux

#### Build from Source

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    git

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone repository
git clone https://github.com/platinoff/poolAI.git
cd poolAI

# Build
cargo build --release

# Install
sudo cp target/release/poolai /usr/local/bin/
sudo chmod +x /usr/local/bin/poolai
```

#### Systemd Service

Create `/etc/systemd/system/poolai.service`:

```ini
[Unit]
Description=PoolAI - AI Mining Pool Management System
After=network.target

[Service]
Type=simple
User=poolai
Group=poolai
WorkingDirectory=/opt/poolai
ExecStart=/usr/local/bin/poolai
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/poolai/data /opt/poolai/config

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

[Install]
WantedBy=multi-user.target
```

Create user and directories:

```bash
sudo useradd -r -s /bin/false poolai
sudo mkdir -p /opt/poolai/{data,config,certs}
sudo chown -R poolai:poolai /opt/poolai
```

Start service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable poolai
sudo systemctl start poolai
sudo systemctl status poolai
```

### Windows

#### Build from Source

```powershell
# Install Rust
# Download and run rustup-init.exe from https://rustup.rs/

# Install Visual Studio Build Tools
# Download from https://visualstudio.microsoft.com/downloads/

# Clone repository
git clone https://github.com/platinoff/poolAI.git
cd poolAI

# Build
cargo build --release

# Install (copy to desired location)
Copy-Item target\release\poolai.exe C:\Program Files\PoolAI\
```

#### Windows Service

Use NSSM (Non-Sucking Service Manager):

```powershell
# Download NSSM from https://nssm.cc/download
# Install service
nssm install PoolAI "C:\Program Files\PoolAI\poolai.exe"
nssm set PoolAI AppDirectory "C:\Program Files\PoolAI"
nssm set PoolAI AppStdout "C:\Program Files\PoolAI\logs\stdout.log"
nssm set PoolAI AppStderr "C:\Program Files\PoolAI\logs\stderr.log"
nssm set PoolAI Start SERVICE_AUTO_START
nssm start PoolAI
```

## Configuration

### Configuration File

Create `/opt/poolai/config/config.toml` (Linux) or `C:\Program Files\PoolAI\config.toml` (Windows):

```toml
[server]
host = "0.0.0.0"
port = 8080
https_port = 8443

[raid]
mode = "local"  # or "distributed"
data_dir = "/opt/poolai/data"  # or "C:\\Program Files\\PoolAI\\data"

[security]
jwt_enabled = true
https_enabled = true
cert_path = "/opt/poolai/certs/cert.pem"
key_path = "/opt/poolai/certs/key.pem"

[logging]
level = "info"
```

See `docs/configuration/` for detailed configuration examples.

## Firewall Configuration

### Linux (UFW)

```bash
sudo ufw allow 8080/tcp
sudo ufw allow 8443/tcp
```

### Linux (firewalld)

```bash
sudo firewall-cmd --permanent --add-port=8080/tcp
sudo firewall-cmd --permanent --add-port=8443/tcp
sudo firewall-cmd --reload
```

### Windows Firewall

```powershell
New-NetFirewallRule -DisplayName "PoolAI HTTP" -Direction Inbound -LocalPort 8080 -Protocol TCP -Action Allow
New-NetFirewallRule -DisplayName "PoolAI HTTPS" -Direction Inbound -LocalPort 8443 -Protocol TCP -Action Allow
```

## SSL/TLS Certificates

### Let's Encrypt (Linux)

```bash
# Install certbot
sudo apt-get install certbot

# Obtain certificate
sudo certbot certonly --standalone -d poolai.example.com

# Copy certificates
sudo cp /etc/letsencrypt/live/poolai.example.com/fullchain.pem /opt/poolai/certs/cert.pem
sudo cp /etc/letsencrypt/live/poolai.example.com/privkey.pem /opt/poolai/certs/key.pem
sudo chown poolai:poolai /opt/poolai/certs/*.pem

# Auto-renewal (add to crontab)
0 0 * * * certbot renew --quiet --deploy-hook "systemctl reload poolai"
```

### Self-Signed Certificate

```bash
# Generate self-signed certificate
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
```

## Distributed RAID Setup

For multi-node deployment:

1. Configure each node with unique `NODE_ID`
2. Set `RAFT_CLUSTER` environment variable:

```bash
export RAFT_CLUSTER="node1:8080,node2:8080,node3:8080"
export NODE_ID="node1"
```

Or in systemd service:

```ini
[Service]
Environment="NODE_ID=node1"
Environment="RAFT_CLUSTER=node1:8080,node2:8080,node3:8080"
```

## Monitoring

### Logs

Linux (systemd):

```bash
# View logs
sudo journalctl -u poolai -f

# View last 100 lines
sudo journalctl -u poolai -n 100
```

Windows:

```powershell
# View logs
Get-Content "C:\Program Files\PoolAI\logs\stdout.log" -Tail 100 -Wait
```

### Health Check

```bash
# Check health endpoint
curl http://localhost:8080/api/v1/health

# Check metrics
curl http://localhost:8080/metrics
```

## Performance Tuning

### Linux

#### Increase file descriptor limits

Edit `/etc/security/limits.conf`:

```
poolai soft nofile 65536
poolai hard nofile 65536
```

#### CPU affinity

```bash
# Pin to specific CPUs
taskset -c 0-3 /usr/local/bin/poolai
```

#### Memory limits

Edit systemd service:

```ini
[Service]
MemoryLimit=4G
```

### Windows

#### Process priority

```powershell
# Set high priority
(Get-Process poolai).PriorityClass = "High"
```

## Backup

### Linux

```bash
# Backup data directory
tar czf poolai-backup-$(date +%Y%m%d).tar.gz /opt/poolai/data

# Backup configuration
tar czf poolai-config-backup-$(date +%Y%m%d).tar.gz /opt/poolai/config
```

### Windows

```powershell
# Backup data directory
Compress-Archive -Path "C:\Program Files\PoolAI\data" -DestinationPath "poolai-backup-$(Get-Date -Format 'yyyyMMdd').zip"
```

## Troubleshooting

### Service won't start

```bash
# Check service status
sudo systemctl status poolai

# Check logs
sudo journalctl -u poolai -n 50

# Check permissions
ls -la /opt/poolai
```

### Port already in use

```bash
# Find process using port
sudo lsof -i :8080
sudo netstat -tulpn | grep 8080

# Kill process
sudo kill -9 <PID>
```

### Permission denied

```bash
# Fix ownership
sudo chown -R poolai:poolai /opt/poolai

# Fix permissions
sudo chmod -R 755 /opt/poolai
```

## Updating

```bash
# Stop service
sudo systemctl stop poolai

# Backup
tar czf backup-$(date +%Y%m%d).tar.gz /opt/poolai

# Update binary
sudo cp target/release/poolai /usr/local/bin/

# Start service
sudo systemctl start poolai
```

## Security Hardening

1. **Run as non-root user** (already configured in systemd service)
2. **Use firewall** (configured above)
3. **Enable HTTPS** (certificates configured above)
4. **Regular updates**:
```bash
# Set up automatic security updates
sudo apt-get install unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
```
5. **Disable unnecessary services**
6. **Use fail2ban** for brute force protection:
```bash
sudo apt-get install fail2ban
sudo systemctl enable fail2ban
```

