# Production Configuration Guide

## Overview

This guide provides production-ready configuration examples for PoolAI deployments.

## Basic Production Configuration

Create `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080
https_port = 8443
workers = 4  # Number of async worker threads

[security]
jwt_enabled = true
https_enabled = true
cert_path = "/opt/poolai/certs/cert.pem"
key_path = "/opt/poolai/certs/key.pem"
token_expiry_seconds = 3600  # 1 hour

[raid]
mode = "distributed"  # Use distributed RAID for production
data_dir = "/opt/poolai/data"
replication_factor = 3
consistency_level = "quorum"

[logging]
level = "info"
format = "json"  # JSON format for log aggregation
file = "/var/log/poolai/poolai.log"
max_size_mb = 100
max_files = 10

[monitoring]
metrics_enabled = true
metrics_port = 9090
health_check_interval_seconds = 30

[pool]
max_workers = 1000
worker_timeout_seconds = 300
health_check_interval_seconds = 60

[vm]
max_instances = 100
default_cpu_cores = 2
default_memory_mb = 2048
auto_recovery_enabled = true
max_restart_attempts = 5
```

## High Availability Configuration

For multi-node deployment:

### Node 1

```toml
[server]
host = "0.0.0.0"
port = 8080

[raid]
mode = "distributed"
node_id = "node1"
raft_cluster = "node1:8080,node2:8080,node3:8080"
raft_data_dir = "/opt/poolai/raft-data"

[replication]
strategy = "synchronous"
quorum_size = 2
timeout_seconds = 5
```

### Node 2

```toml
[server]
host = "0.0.0.0"
port = 8080

[raid]
mode = "distributed"
node_id = "node2"
raft_cluster = "node1:8080,node2:8080,node3:8080"
raft_data_dir = "/opt/poolai/raft-data"

[replication]
strategy = "synchronous"
quorum_size = 2
timeout_seconds = 5
```

### Node 3

```toml
[server]
host = "0.0.0.0"
port = 8080

[raid]
mode = "distributed"
node_id = "node3"
raft_cluster = "node1:8080,node2:8080,node3:8080"
raft_data_dir = "/opt/poolai/raft-data"

[replication]
strategy = "synchronous"
quorum_size = 2
timeout_seconds = 5
```

## Performance Tuning

### High Performance Configuration

```toml
[server]
workers = 8  # Match CPU cores
max_connections = 10000
keep_alive_seconds = 60

[raid]
cache_size_mb = 1024  # 1GB cache
gc_interval_seconds = 3600
gc_threshold_mb = 10000

[pool]
worker_pool_size = 100
task_queue_size = 10000

[vm]
resource_monitoring_interval_seconds = 10
resource_history_limit = 1000
```

### Memory-Optimized Configuration

```toml
[server]
workers = 2

[raid]
cache_size_mb = 256
gc_threshold_mb = 1000

[pool]
worker_pool_size = 50
task_queue_size = 1000

[vm]
max_instances = 20
resource_history_limit = 100
```

## Security Hardening

### Secure Configuration

```toml
[security]
jwt_enabled = true
https_enabled = true
cert_path = "/opt/poolai/certs/cert.pem"
key_path = "/opt/poolai/certs/key.pem"
token_expiry_seconds = 1800  # 30 minutes
require_https = true
allowed_origins = ["https://poolai.example.com"]
cors_enabled = true

[api]
rate_limit_per_minute = 60
max_request_size_mb = 10
timeout_seconds = 30

[logging]
level = "warn"  # Reduce log verbosity
sanitize_logs = true  # Remove sensitive data
```

## Monitoring Configuration

### With Prometheus

```toml
[monitoring]
metrics_enabled = true
metrics_port = 9090
metrics_path = "/metrics"

[prometheus]
enabled = true
scrape_interval_seconds = 15
```

### With Custom Monitoring

```toml
[monitoring]
metrics_enabled = true
health_check_interval_seconds = 10
resource_usage_tracking = true
alert_thresholds = { cpu_percent = 90.0, memory_mb = 8192 }
```

## Environment-Specific Configurations

### Development

```toml
[server]
port = 8080
https_enabled = false

[security]
jwt_enabled = false

[logging]
level = "debug"
format = "pretty"

[raid]
mode = "local"
```

### Testing

```toml
[server]
port = 8080

[security]
jwt_enabled = true
token_expiry_seconds = 60  # Short expiry for tests

[logging]
level = "info"

[raid]
mode = "local"
data_dir = "/tmp/poolai-test"
```

### Staging

```toml
[server]
port = 8080
https_port = 8443

[security]
jwt_enabled = true
https_enabled = true

[logging]
level = "info"
format = "json"

[raid]
mode = "distributed"
replication_factor = 2  # Lower for staging
```

## Configuration Validation

Validate configuration:

```bash
poolai --check-config config.toml
```

## Environment Variables

Override configuration with environment variables:

```bash
export POOLAI_SERVER_PORT=8080
export POOLAI_RAID_MODE=distributed
export POOLAI_SECURITY_JWT_ENABLED=true
export RUST_LOG=info
```

## Best Practices

1. **Use environment variables for secrets**:
```bash
export POOLAI_JWT_SECRET=$(openssl rand -hex 32)
```

2. **Separate configs by environment**:
   - `config.production.toml`
   - `config.staging.toml`
   - `config.development.toml`

3. **Use configuration management tools**:
   - Ansible
   - Puppet
   - Chef
   - Terraform

4. **Version control configuration**:
   - Store in Git (without secrets)
   - Use secrets management (Vault, AWS Secrets Manager)

5. **Regular configuration reviews**:
   - Audit security settings
   - Review performance parameters
   - Update certificates

