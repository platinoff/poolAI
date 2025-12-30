# Performance Tuning Guide

## Overview

This guide covers performance optimization techniques for PoolAI deployments to achieve maximum throughput and efficiency.

## System-Level Optimization

### CPU Optimization

#### CPU Affinity

Pin PoolAI processes to specific CPU cores:

```bash
# Linux: Use taskset
taskset -c 0-3 poolai

# Or in systemd service
[Service]
CPUAffinity=0-3
```

#### CPU Governor

Set CPU governor to performance mode:

```bash
# Check current governor
cat /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Set to performance
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Make permanent
sudo apt-get install cpufrequtils
echo 'GOVERNOR="performance"' | sudo tee /etc/default/cpufrequtils
```

### Memory Optimization

#### Huge Pages

Enable huge pages for better memory performance:

```bash
# Check current huge pages
cat /proc/meminfo | grep Huge

# Set huge pages (example: 1024 pages of 2MB = 2GB)
echo 1024 | sudo tee /proc/sys/vm/nr_hugepages

# Make permanent
echo 'vm.nr_hugepages=1024' | sudo tee -a /etc/sysctl.conf
```

#### Swappiness

Reduce swap usage:

```bash
# Set swappiness to 10 (default is 60)
echo 10 | sudo tee /proc/sys/vm/swappiness

# Make permanent
echo 'vm.swappiness=10' | sudo tee -a /etc/sysctl.conf
```

### Network Optimization

#### TCP Tuning

Optimize TCP settings:

```bash
# Increase TCP buffer sizes
echo 'net.core.rmem_max = 16777216' | sudo tee -a /etc/sysctl.conf
echo 'net.core.wmem_max = 16777216' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv4.tcp_rmem = 4096 87380 16777216' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv4.tcp_wmem = 4096 65536 16777216' | sudo tee -a /etc/sysctl.conf

# Enable TCP fast open
echo 'net.ipv4.tcp_fastopen = 3' | sudo tee -a /etc/sysctl.conf

# Increase connection tracking
echo 'net.netfilter.nf_conntrack_max = 262144' | sudo tee -a /etc/sysctl.conf

# Apply changes
sudo sysctl -p
```

#### Connection Limits

Increase file descriptor limits:

```bash
# Edit /etc/security/limits.conf
* soft nofile 65536
* hard nofile 65536

# Edit /etc/systemd/system.conf
DefaultLimitNOFILE=65536

# Reload systemd
sudo systemctl daemon-reload
```

### I/O Optimization

#### I/O Scheduler

Set optimal I/O scheduler:

```bash
# Check current scheduler
cat /sys/block/sda/queue/scheduler

# Set to deadline or noop (SSD) or mq-deadline (NVMe)
echo deadline | sudo tee /sys/block/sda/queue/scheduler

# Make permanent (add to /etc/rc.local or systemd service)
```

#### Filesystem Options

Mount with performance options:

```bash
# For ext4
mount -o noatime,nodiratime /dev/sda1 /data

# For XFS (recommended for high I/O)
mount -o noatime /dev/sda1 /data

# Make permanent in /etc/fstab
/dev/sda1 /data xfs noatime 0 2
```

## Application-Level Optimization

### Configuration Tuning

#### Worker Threads

Match worker threads to CPU cores:

```toml
[server]
workers = 8  # Match number of CPU cores
```

#### Connection Pooling

Optimize connection pool:

```toml
[server]
max_connections = 10000
keep_alive_seconds = 60
```

#### RAID Cache

Increase RAID cache for better performance:

```toml
[raid]
cache_size_mb = 2048  # 2GB cache
cache_ttl_seconds = 3600
```

### Async Runtime Tuning

#### Tokio Runtime

Configure Tokio runtime for optimal performance:

```rust
// In main.rs or lib.rs
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(8)  // Match CPU cores
    .max_blocking_threads(512)
    .thread_name("poolai-worker")
    .thread_stack_size(3 * 1024 * 1024)  // 3MB stack
    .enable_all()
    .build()?;
```

### Memory Management

#### Buffer Sizes

Optimize buffer sizes:

```toml
[pool]
task_queue_size = 10000
worker_pool_size = 100

[raid]
read_buffer_size_kb = 64
write_buffer_size_kb = 64
```

#### Garbage Collection

Tune GC settings:

```toml
[raid]
gc_interval_seconds = 3600
gc_threshold_mb = 10000
gc_batch_size = 100
```

## Database/Storage Optimization

### RAID Configuration

#### Replication Strategy

Choose appropriate replication strategy:

```toml
[replication]
strategy = "asynchronous"  # For better write performance
# or
strategy = "synchronous"   # For consistency
```

#### Consistency Level

Balance consistency and performance:

```toml
[raid]
consistency_level = "quorum"  # Balance between performance and consistency
# or
consistency_level = "eventual"  # Best performance
```

### Disk Layout

#### Separate Data and Logs

```bash
# Separate disks for data and logs
/data     -> /dev/sdb (data)
/logs     -> /dev/sdc (logs)
/tmp      -> tmpfs (in-memory)
```

## Network Performance

### Load Balancing

#### Nginx Configuration

```nginx
upstream poolai {
    least_conn;
    server poolai1:8080;
    server poolai2:8080;
    server poolai3:8080;
    keepalive 32;
}

server {
    listen 80;
    
    location / {
        proxy_pass http://poolai;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
        proxy_buffering off;
        proxy_read_timeout 300s;
    }
}
```

### Compression

Enable compression:

```toml
[server]
compression_enabled = true
compression_level = 6  # Balance between CPU and size
```

## Monitoring Performance

### Key Metrics

Monitor these metrics:

- Request rate (requests/second)
- Response time (p50, p95, p99)
- CPU usage percentage
- Memory usage
- I/O wait time
- Network throughput
- Error rate

### Performance Baselines

Establish baselines:

```bash
# Baseline request rate
curl http://localhost:8080/api/v1/metrics | grep poolai_http_requests_total

# Baseline response time
curl -w "@curl-format.txt" -o /dev/null -s http://localhost:8080/api/v1/health
```

## Benchmarking

### Load Testing

Use tools like `wrk` or `ab`:

```bash
# Install wrk
sudo apt-get install wrk

# Run benchmark
wrk -t12 -c400 -d30s http://localhost:8080/api/v1/health

# With custom script
wrk -t12 -c400 -d30s -s script.lua http://localhost:8080/api/v1/artifacts
```

### Stress Testing

```bash
# Multiple concurrent requests
for i in {1..100}; do
    curl http://localhost:8080/api/v1/health &
done
wait
```

## Optimization Checklist

### System Level
- [ ] CPU governor set to performance
- [ ] CPU affinity configured
- [ ] Huge pages enabled
- [ ] Swappiness reduced
- [ ] TCP buffers increased
- [ ] File descriptor limits increased
- [ ] I/O scheduler optimized
- [ ] Filesystem mounted with noatime

### Application Level
- [ ] Worker threads match CPU cores
- [ ] Connection pool optimized
- [ ] Cache sizes tuned
- [ ] Buffer sizes optimized
- [ ] GC settings tuned

### Network Level
- [ ] Load balancer configured
- [ ] Compression enabled
- [ ] Keep-alive connections enabled
- [ ] Connection pooling optimized

### Storage Level
- [ ] RAID replication strategy chosen
- [ ] Consistency level balanced
- [ ] Disk layout optimized
- [ ] Separate data and logs

## Performance Troubleshooting

### High CPU Usage

1. Check worker thread count
2. Profile with `perf` or `flamegraph`
3. Check for CPU-intensive operations
4. Consider horizontal scaling

### High Memory Usage

1. Check cache sizes
2. Review buffer sizes
3. Check for memory leaks
4. Enable memory profiling

### High I/O Wait

1. Check disk I/O with `iostat`
2. Optimize I/O scheduler
3. Consider faster storage (SSD/NVMe)
4. Separate data and logs

### Network Bottlenecks

1. Check network throughput
2. Optimize TCP settings
3. Enable compression
4. Use connection pooling

## Best Practices

1. **Profile First**: Always profile before optimizing
2. **Measure**: Establish baselines and measure improvements
3. **Incremental**: Make one change at a time
4. **Test**: Test changes in staging before production
5. **Monitor**: Continuously monitor performance metrics
6. **Document**: Document all changes and their impact

