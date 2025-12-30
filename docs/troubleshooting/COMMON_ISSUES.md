# Common Issues and Troubleshooting

## Overview

This guide covers common issues encountered when deploying and operating PoolAI, along with troubleshooting steps and solutions.

## Startup Issues

### Application Won't Start

#### Issue: Port Already in Use

**Symptoms:**
```
Error: Address already in use (os error 98)
```

**Solution:**
```bash
# Find process using port
sudo lsof -i :8080
# or
sudo netstat -tulpn | grep 8080

# Kill process
sudo kill -9 <PID>

# Or change port in config
[server]
port = 8081
```

#### Issue: Permission Denied

**Symptoms:**
```
Error: Permission denied (os error 13)
```

**Solution:**
```bash
# Check file permissions
ls -la /opt/poolai

# Fix ownership
sudo chown -R poolai:poolai /opt/poolai

# Fix permissions
sudo chmod -R 755 /opt/poolai
```

#### Issue: Configuration File Not Found

**Symptoms:**
```
Error: No such file or directory (os error 2)
```

**Solution:**
```bash
# Check config path
poolai --config /path/to/config.toml

# Or set environment variable
export POOLAI_CONFIG_PATH=/path/to/config.toml
```

## Runtime Issues

### High CPU Usage

#### Symptoms
- CPU usage consistently above 80%
- Slow response times
- System becomes unresponsive

#### Diagnosis
```bash
# Check CPU usage
top -p $(pgrep poolai)

# Profile with perf
sudo perf record -p $(pgrep poolai) -g sleep 30
sudo perf report

# Check worker threads
curl http://localhost:8080/api/v1/metrics | grep poolai_workers
```

#### Solutions
1. **Reduce worker threads**:
```toml
[server]
workers = 4  # Reduce from 8
```

2. **Enable compression**:
```toml
[server]
compression_enabled = true
```

3. **Optimize cache**:
```toml
[raid]
cache_size_mb = 512  # Reduce from 2048
```

### High Memory Usage

#### Symptoms
- Memory usage above 90%
- OOM (Out of Memory) kills
- System swapping

#### Diagnosis
```bash
# Check memory usage
free -h
ps aux | grep poolai

# Check for memory leaks
valgrind --leak-check=full poolai
```

#### Solutions
1. **Reduce cache size**:
```toml
[raid]
cache_size_mb = 256
```

2. **Limit instances**:
```toml
[vm]
max_instances = 20  # Reduce from 100
```

3. **Enable GC**:
```toml
[raid]
gc_interval_seconds = 1800  # More frequent GC
```

### Network Issues

#### Issue: Connection Timeouts

**Symptoms:**
- Requests timing out
- Intermittent connectivity
- High latency

**Diagnosis:**
```bash
# Check network connectivity
ping poolai.example.com

# Check DNS resolution
nslookup poolai.example.com

# Check firewall
sudo ufw status
sudo iptables -L
```

**Solutions:**
1. **Increase timeouts**:
```toml
[server]
request_timeout_seconds = 60
```

2. **Check firewall rules**:
```bash
sudo ufw allow 8080/tcp
sudo ufw allow 8443/tcp
```

3. **Check network policies** (Kubernetes):
```bash
kubectl get networkpolicies
kubectl describe networkpolicy poolai-netpol
```

#### Issue: Slow Replication

**Symptoms:**
- Replication operations taking too long
- High replication latency
- Replication failures

**Diagnosis:**
```bash
# Check replication metrics
curl http://localhost:8080/api/v1/metrics | grep replication

# Check network between nodes
ping node2.example.com
traceroute node2.example.com
```

**Solutions:**
1. **Switch to async replication**:
```toml
[replication]
strategy = "asynchronous"
```

2. **Increase timeout**:
```toml
[replication]
timeout_seconds = 30
```

3. **Check network bandwidth**:
```bash
iperf3 -c node2.example.com
```

## Storage Issues

### Disk Space

#### Issue: Disk Full

**Symptoms:**
```
Error: No space left on device (os error 28)
```

**Solution:**
```bash
# Check disk usage
df -h

# Clean up old data
poolai --gc --force

# Increase disk space or add new volume
```

### I/O Errors

#### Issue: Read/Write Failures

**Symptoms:**
- I/O errors in logs
- Data corruption
- Slow I/O operations

**Diagnosis:**
```bash
# Check disk health
sudo smartctl -a /dev/sda

# Check I/O wait
iostat -x 1

# Check filesystem
sudo fsck /dev/sda1
```

**Solutions:**
1. **Check disk health**:
```bash
sudo smartctl -a /dev/sda
```

2. **Optimize I/O scheduler**:
```bash
echo deadline | sudo tee /sys/block/sda/queue/scheduler
```

3. **Use faster storage** (SSD/NVMe)

## Distributed RAID Issues

### Raft Cluster Issues

#### Issue: No Leader

**Symptoms:**
- Write operations failing
- "No leader" errors
- Cluster split-brain

**Diagnosis:**
```bash
# Check Raft status
curl http://localhost:8080/api/v1/raft/status

# Check node connectivity
curl http://node2:8080/api/v1/raft/status
```

**Solutions:**
1. **Check quorum**:
```bash
# Need majority of nodes (3 nodes = 2 nodes minimum)
```

2. **Restart nodes**:
```bash
# Restart all nodes in cluster
sudo systemctl restart poolai
```

3. **Check network connectivity**:
```bash
# Ensure all nodes can communicate
ping node2
ping node3
```

#### Issue: Replication Failures

**Symptoms:**
- Replication operations failing
- Data inconsistency
- High error rates

**Diagnosis:**
```bash
# Check replication metrics
curl http://localhost:8080/api/v1/metrics | grep replication_failures

# Check node health
curl http://localhost:8080/api/v1/health
```

**Solutions:**
1. **Check node health**:
```bash
# Ensure all nodes are healthy
for node in node1 node2 node3; do
    curl http://$node:8080/api/v1/health
done
```

2. **Increase timeout**:
```toml
[replication]
timeout_seconds = 60
```

3. **Check network**:
```bash
# Test connectivity between nodes
```

## Authentication Issues

### JWT Token Issues

#### Issue: Token Expired

**Symptoms:**
```
Error: Token expired
```

**Solution:**
```bash
# Refresh token
curl -X POST http://localhost:8080/api/v1/auth/refresh \
  -H "Authorization: Bearer $REFRESH_TOKEN"
```

#### Issue: Invalid Token

**Symptoms:**
```
Error: Invalid token
```

**Solution:**
1. **Check token format**:
```bash
# Token should be in format: header.payload.signature
```

2. **Verify secret**:
```toml
[security]
jwt_secret = "correct_secret_here"
```

3. **Regenerate token**:
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -d '{"username":"user","password":"pass"}'
```

## Performance Issues

### Slow API Responses

#### Diagnosis
```bash
# Check response times
curl -w "@curl-format.txt" -o /dev/null -s http://localhost:8080/api/v1/health

# Check metrics
curl http://localhost:8080/api/v1/metrics | grep response_time
```

#### Solutions
1. **Enable caching**:
```toml
[server]
cache_enabled = true
cache_ttl_seconds = 300
```

2. **Optimize queries**:
```toml
[raid]
index_enabled = true
```

3. **Scale horizontally**:
```bash
# Add more nodes
```

### High Latency

#### Diagnosis
```bash
# Check network latency
ping node2.example.com

# Check disk I/O
iostat -x 1
```

#### Solutions
1. **Optimize network**:
```bash
# Use faster network (10Gbps)
```

2. **Optimize storage**:
```bash
# Use SSD/NVMe
```

3. **Reduce replication factor**:
```toml
[raid]
replication_factor = 2  # Reduce from 3
```

## Logging and Debugging

### Enable Debug Logging

```toml
[logging]
level = "debug"
format = "pretty"
```

### Check Logs

```bash
# Linux (systemd)
sudo journalctl -u poolai -f

# Docker
docker logs -f poolai

# Kubernetes
kubectl logs -f deployment/poolai
```

### Common Log Patterns

#### Error Patterns
- `ERROR`: Critical errors requiring attention
- `WARN`: Warnings that may indicate issues
- `INFO`: Informational messages

#### Debug Information
```bash
# Enable debug logging
export RUST_LOG=debug

# Check specific component
export RUST_LOG=poolai::raid=debug
```

## Health Checks

### Check Application Health

```bash
# Health endpoint
curl http://localhost:8080/api/v1/health

# Metrics endpoint
curl http://localhost:8080/api/v1/metrics

# Status endpoint
curl http://localhost:8080/api/v1/status
```

### Health Check Failures

#### Symptoms
- Health checks failing
- Auto-restart triggered
- Service marked as unhealthy

#### Diagnosis
```bash
# Check health endpoint
curl http://localhost:8080/api/v1/health

# Check logs
sudo journalctl -u poolai -n 100
```

#### Solutions
1. **Check dependencies**:
```bash
# Ensure all dependencies are available
```

2. **Check resources**:
```bash
# Ensure sufficient CPU/memory
```

3. **Review configuration**:
```bash
# Check for configuration errors
poolai --check-config config.toml
```

## Getting Help

### Collecting Information

Before seeking help, collect:

1. **Logs**:
```bash
sudo journalctl -u poolai > poolai.log
```

2. **Configuration**:
```bash
cat /opt/poolai/config/config.toml
```

3. **System Information**:
```bash
uname -a
free -h
df -h
```

4. **Metrics**:
```bash
curl http://localhost:8080/api/v1/metrics > metrics.txt
```

### Reporting Issues

When reporting issues, include:

- PoolAI version
- Operating system
- Configuration (sanitized)
- Logs
- Steps to reproduce
- Expected vs actual behavior

