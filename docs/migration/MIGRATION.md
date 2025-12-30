# Migration Guide

## Overview

This guide covers migrating PoolAI deployments between different environments, versions, and configurations.

## Version Migration

### Upgrading PoolAI

#### Pre-Migration Checklist

- [ ] Backup all data
- [ ] Review release notes
- [ ] Test in staging environment
- [ ] Document current configuration
- [ ] Plan downtime window

#### Backup Procedure

```bash
# Backup data directory
tar czf poolai-data-backup-$(date +%Y%m%d).tar.gz /opt/poolai/data

# Backup configuration
tar czf poolai-config-backup-$(date +%Y%m%d).tar.gz /opt/poolai/config

# Backup database (if applicable)
# Database-specific backup commands
```

#### Upgrade Steps

1. **Stop service**:
```bash
sudo systemctl stop poolai
```

2. **Backup current version**:
```bash
cp /usr/local/bin/poolai /usr/local/bin/poolai.old
```

3. **Install new version**:
```bash
# Build from source
git pull
cargo build --release
sudo cp target/release/poolai /usr/local/bin/
```

4. **Verify configuration**:
```bash
poolai --check-config /opt/poolai/config/config.toml
```

5. **Start service**:
```bash
sudo systemctl start poolai
sudo systemctl status poolai
```

6. **Verify functionality**:
```bash
curl http://localhost:8080/api/v1/health
```

#### Rollback Procedure

If upgrade fails:

```bash
# Stop service
sudo systemctl stop poolai

# Restore old binary
sudo cp /usr/local/bin/poolai.old /usr/local/bin/poolai

# Restore data (if needed)
tar xzf poolai-data-backup-YYYYMMDD.tar.gz -C /

# Start service
sudo systemctl start poolai
```

## Environment Migration

### Local to Production

#### Preparation

1. **Export configuration**:
```bash
# Export current config
poolai --export-config > production-config.toml
```

2. **Export data**:
```bash
# Export artifacts
poolai --export-data > data-export.json
```

#### Migration Steps

1. **Set up production environment**:
```bash
# Follow deployment guide
# docs/deployment/BARE_METAL.md
```

2. **Import configuration**:
```bash
# Copy config to production
scp production-config.toml production-server:/opt/poolai/config/
```

3. **Import data**:
```bash
# Import data
poolai --import-data < data-export.json
```

4. **Verify migration**:
```bash
# Check data integrity
poolai --verify-data

# Test functionality
curl http://production-server:8080/api/v1/health
```

### Single Node to Cluster

#### Preparation

1. **Backup current node**:
```bash
# Full backup
tar czf node1-backup.tar.gz /opt/poolai
```

2. **Prepare new nodes**:
```bash
# Set up nodes 2 and 3
# Follow deployment guide
```

#### Migration Steps

1. **Configure cluster**:
```toml
# node1, node2, node3 config.toml
[raid]
mode = "distributed"
node_id = "node1"  # or node2, node3
raft_cluster = "node1:8080,node2:8080,node3:8080"
```

2. **Start nodes in order**:
```bash
# Start node1 first (becomes leader)
sudo systemctl start poolai@node1

# Wait for leader election
sleep 10

# Start other nodes
sudo systemctl start poolai@node2
sudo systemctl start poolai@node3
```

3. **Verify cluster**:
```bash
# Check cluster status
curl http://node1:8080/api/v1/raft/status
curl http://node2:8080/api/v1/raft/status
curl http://node3:8080/api/v1/raft/status
```

4. **Migrate data**:
```bash
# Data will replicate automatically
# Monitor replication status
curl http://node1:8080/api/v1/metrics | grep replication
```

## Configuration Migration

### Changing Storage Backend

#### From Local to Distributed

1. **Backup data**:
```bash
tar czf local-data-backup.tar.gz /opt/poolai/data
```

2. **Update configuration**:
```toml
[raid]
mode = "distributed"
node_id = "node1"
raft_cluster = "node1:8080,node2:8080,node3:8080"
```

3. **Restart service**:
```bash
sudo systemctl restart poolai
```

4. **Verify migration**:
```bash
# Check data is accessible
curl http://localhost:8080/api/v1/raid/artifacts
```

### Changing Replication Strategy

#### From Synchronous to Asynchronous

1. **Update configuration**:
```toml
[replication]
strategy = "asynchronous"
```

2. **Restart service**:
```bash
sudo systemctl restart poolai
```

3. **Monitor replication**:
```bash
# Check replication metrics
curl http://localhost:8080/api/v1/metrics | grep replication
```

## Data Migration

### Artifact Migration

#### Export Artifacts

```bash
# Export all artifacts
poolai --export-artifacts > artifacts.json

# Export specific artifacts
poolai --export-artifacts --filter "name:test*" > test-artifacts.json
```

#### Import Artifacts

```bash
# Import artifacts
poolai --import-artifacts < artifacts.json

# Verify import
curl http://localhost:8080/api/v1/raid/artifacts
```

### Worker Migration

#### Export Workers

```bash
# Export worker configurations
poolai --export-workers > workers.json
```

#### Import Workers

```bash
# Import workers
poolai --import-workers < workers.json

# Verify import
curl http://localhost:8080/api/v1/workers
```

## Platform Migration

### Docker to Kubernetes

#### Export from Docker

```bash
# Export data volume
docker run --rm -v poolai-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/poolai-data.tar.gz /data

# Export configuration
docker run --rm -v poolai-config:/config -v $(pwd):/backup \
  alpine tar czf /backup/poolai-config.tar.gz /config
```

#### Import to Kubernetes

```bash
# Create PersistentVolumeClaim
kubectl apply -f pvc.yaml

# Copy data to PVC
kubectl cp poolai-data.tar.gz poolai-pod:/tmp/
kubectl exec poolai-pod -- tar xzf /tmp/poolai-data.tar.gz -C /data

# Apply configuration as ConfigMap
kubectl create configmap poolai-config --from-file=config.toml
```

### Bare Metal to Cloud

#### Preparation

1. **Export all data**:
```bash
# Full system backup
tar czf full-backup.tar.gz /opt/poolai
```

2. **Document configuration**:
```bash
# Export config
poolai --export-config > cloud-config.toml
```

#### Migration Steps

1. **Set up cloud infrastructure**:
```bash
# Follow cloud provider deployment guide
```

2. **Import data**:
```bash
# Upload backup to cloud
scp full-backup.tar.gz cloud-server:/tmp/

# Extract on cloud server
ssh cloud-server "cd /opt/poolai && tar xzf /tmp/full-backup.tar.gz"
```

3. **Update configuration**:
```toml
# Update network settings for cloud
[server]
host = "0.0.0.0"  # Allow external access
```

4. **Verify migration**:
```bash
# Test from local machine
curl http://cloud-server-ip:8080/api/v1/health
```

## Migration Best Practices

### Planning

1. **Test in staging first**
2. **Create detailed migration plan**
3. **Schedule maintenance window**
4. **Prepare rollback procedure**

### Execution

1. **Backup everything**
2. **Verify backups**
3. **Execute migration steps**
4. **Verify functionality**
5. **Monitor for issues**

### Post-Migration

1. **Verify data integrity**
2. **Test all functionality**
3. **Monitor performance**
4. **Document any issues**
5. **Update documentation**

## Troubleshooting Migration

### Data Loss

If data is lost during migration:

1. **Stop service immediately**
2. **Restore from backup**
3. **Verify data integrity**
4. **Investigate root cause**

### Configuration Errors

If configuration errors occur:

1. **Check configuration syntax**
2. **Validate against schema**
3. **Review migration steps**
4. **Consult documentation**

### Performance Degradation

If performance degrades after migration:

1. **Compare before/after metrics**
2. **Check resource allocation**
3. **Review configuration changes**
4. **Optimize as needed**

## Migration Checklist

### Pre-Migration

- [ ] Backup all data
- [ ] Document current state
- [ ] Test migration in staging
- [ ] Prepare rollback plan
- [ ] Schedule maintenance window

### During Migration

- [ ] Stop services
- [ ] Export/backup data
- [ ] Update configuration
- [ ] Deploy new version
- [ ] Import/restore data
- [ ] Start services
- [ ] Verify functionality

### Post-Migration

- [ ] Verify data integrity
- [ ] Test all features
- [ ] Monitor performance
- [ ] Update documentation
- [ ] Remove old backups (after verification)

