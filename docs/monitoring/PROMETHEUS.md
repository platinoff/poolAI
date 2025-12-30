# Prometheus Monitoring Setup

## Overview

This guide covers setting up Prometheus to monitor PoolAI metrics and performance.

## Prerequisites

- Prometheus 2.40+
- PoolAI instance with metrics enabled
- Network access between Prometheus and PoolAI

## Configuration

### Basic Prometheus Configuration

Create `prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    cluster: 'poolai-production'
    environment: 'production'

scrape_configs:
  - job_name: 'poolai'
    static_configs:
      - targets: ['poolai:9090']
        labels:
          instance: 'poolai-main'
          service: 'poolai'
    metrics_path: '/metrics'
    scrape_interval: 15s
    scrape_timeout: 10s

  - job_name: 'poolai-raft'
    static_configs:
      - targets:
          - 'poolai-node1:9090'
          - 'poolai-node2:9090'
          - 'poolai-node3:9090'
        labels:
          service: 'poolai-raft'
    metrics_path: '/metrics'
    scrape_interval: 15s

rule_files:
  - 'poolai_alerts.yml'

alerting:
  alertmanagers:
    - static_configs:
        - targets:
            - 'alertmanager:9093'
```

### Docker Compose Setup

```yaml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    container_name: prometheus
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - ./poolai_alerts.yml:/etc/prometheus/poolai_alerts.yml
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/usr/share/prometheus/console_libraries'
      - '--web.console.templates=/usr/share/prometheus/consoles'
      - '--storage.tsdb.retention.time=30d'
      - '--web.enable-lifecycle'
    ports:
      - "9090:9090"
    networks:
      - monitoring

volumes:
  prometheus-data:

networks:
  monitoring:
    external: true
```

## Metrics Endpoints

PoolAI exposes metrics at `/metrics` endpoint:

### Available Metrics

- `poolai_http_requests_total` - Total HTTP requests
- `poolai_http_request_duration_seconds` - HTTP request duration
- `poolai_workers_total` - Total number of workers
- `poolai_workers_active` - Active workers
- `poolai_raid_artifacts_total` - Total RAID artifacts
- `poolai_raid_replication_operations_total` - Replication operations
- `poolai_vm_instances_total` - Total VM instances
- `poolai_vm_instances_running` - Running VM instances
- `poolai_resource_cpu_usage_percent` - CPU usage percentage
- `poolai_resource_memory_usage_mb` - Memory usage in MB
- `poolai_health_status` - Health status (1 = healthy, 0 = unhealthy)

## Alert Rules

Create `poolai_alerts.yml`:

```yaml
groups:
  - name: poolai_alerts
    interval: 30s
    rules:
      - alert: PoolAIDown
        expr: up{job="poolai"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "PoolAI instance is down"
          description: "PoolAI instance {{ $labels.instance }} has been down for more than 1 minute."

      - alert: HighCPUUsage
        expr: poolai_resource_cpu_usage_percent > 90
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage on {{ $labels.instance }}"
          description: "CPU usage is {{ $value }}% on {{ $labels.instance }}"

      - alert: HighMemoryUsage
        expr: poolai_resource_memory_usage_mb > 8192
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage on {{ $labels.instance }}"
          description: "Memory usage is {{ $value }}MB on {{ $labels.instance }}"

      - alert: UnhealthyInstance
        expr: poolai_health_status == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "PoolAI instance is unhealthy"
          description: "PoolAI instance {{ $labels.instance }} is reporting unhealthy status"

      - alert: HighRequestLatency
        expr: histogram_quantile(0.95, poolai_http_request_duration_seconds_bucket) > 1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High request latency on {{ $labels.instance }}"
          description: "95th percentile latency is {{ $value }}s on {{ $labels.instance }}"

      - alert: RaftNodeDown
        expr: up{job="poolai-raft"} == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Raft node is down"
          description: "Raft node {{ $labels.instance }} has been down for more than 2 minutes."

      - alert: LowWorkerCount
        expr: poolai_workers_active < 10
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Low worker count on {{ $labels.instance }}"
          description: "Only {{ $value }} active workers on {{ $labels.instance }}"

      - alert: ReplicationFailure
        expr: rate(poolai_raid_replication_operations_total{status="failed"}[5m]) > 0.1
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High replication failure rate"
          description: "Replication failure rate is {{ $value }} failures/second"
```

## Kubernetes ServiceMonitor

For Kubernetes deployments:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: poolai
  labels:
    app: poolai
spec:
  selector:
    matchLabels:
      app: poolai
  endpoints:
  - port: http
    path: /metrics
    interval: 15s
    scrapeTimeout: 10s
```

## Query Examples

### CPU Usage Over Time

```promql
rate(poolai_resource_cpu_usage_percent[5m])
```

### Request Rate

```promql
rate(poolai_http_requests_total[5m])
```

### Error Rate

```promql
rate(poolai_http_requests_total{status="error"}[5m]) / rate(poolai_http_requests_total[5m])
```

### Average Response Time

```promql
rate(poolai_http_request_duration_seconds_sum[5m]) / rate(poolai_http_request_duration_seconds_count[5m])
```

### Worker Utilization

```promql
poolai_workers_active / poolai_workers_total
```

## Recording Rules

Create `poolai_recording_rules.yml`:

```yaml
groups:
  - name: poolai_recording
    interval: 30s
    rules:
      - record: poolai:request_rate:5m
        expr: rate(poolai_http_requests_total[5m])

      - record: poolai:error_rate:5m
        expr: rate(poolai_http_requests_total{status="error"}[5m])

      - record: poolai:avg_response_time:5m
        expr: rate(poolai_http_request_duration_seconds_sum[5m]) / rate(poolai_http_request_duration_seconds_count[5m])

      - record: poolai:worker_utilization
        expr: poolai_workers_active / poolai_workers_total
```

## Retention and Storage

### Storage Configuration

```yaml
# prometheus.yml
global:
  storage.tsdb.retention.time: 30d
  storage.tsdb.retention.size: 50GB
```

### Backup

```bash
# Backup Prometheus data
docker exec prometheus promtool tsdb create-blocks-from openmetrics /backup/metrics.txt /prometheus
```

## Troubleshooting

### Check Targets

```bash
# Check if targets are being scraped
curl http://localhost:9090/api/v1/targets
```

### Check Metrics

```bash
# Query metrics
curl 'http://localhost:9090/api/v1/query?query=up{job="poolai"}'
```

### Reload Configuration

```bash
# Reload Prometheus config
curl -X POST http://localhost:9090/-/reload
```

