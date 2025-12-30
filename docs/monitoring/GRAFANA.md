# Grafana Dashboard Setup

## Overview

This guide covers setting up Grafana dashboards for PoolAI monitoring and visualization.

## Prerequisites

- Grafana 9.0+
- Prometheus data source configured
- PoolAI metrics being collected

## Data Source Configuration

### Add Prometheus Data Source

1. Go to Configuration → Data Sources
2. Add Prometheus data source
3. Configure:
   - URL: `http://prometheus:9090`
   - Access: Server (default)
   - Scrape interval: 15s

## Dashboard JSON

Create `poolai-dashboard.json`:

```json
{
  "dashboard": {
    "title": "PoolAI Monitoring",
    "tags": ["poolai", "monitoring"],
    "timezone": "browser",
    "panels": [
      {
        "id": 1,
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(poolai_http_requests_total[5m])",
            "legendFormat": "{{method}} {{status}}"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 0}
      },
      {
        "id": 2,
        "title": "Response Time (95th percentile)",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, poolai_http_request_duration_seconds_bucket)",
            "legendFormat": "95th percentile"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 0}
      },
      {
        "id": 3,
        "title": "CPU Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "poolai_resource_cpu_usage_percent",
            "legendFormat": "{{instance}}"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 8}
      },
      {
        "id": 4,
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "poolai_resource_memory_usage_mb",
            "legendFormat": "{{instance}}"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 8}
      },
      {
        "id": 5,
        "title": "Active Workers",
        "type": "stat",
        "targets": [
          {
            "expr": "poolai_workers_active",
            "legendFormat": "Active"
          }
        ],
        "gridPos": {"h": 4, "w": 6, "x": 0, "y": 16}
      },
      {
        "id": 6,
        "title": "Total Workers",
        "type": "stat",
        "targets": [
          {
            "expr": "poolai_workers_total",
            "legendFormat": "Total"
          }
        ],
        "gridPos": {"h": 4, "w": 6, "x": 6, "y": 16}
      },
      {
        "id": 7,
        "title": "VM Instances Running",
        "type": "stat",
        "targets": [
          {
            "expr": "poolai_vm_instances_running",
            "legendFormat": "Running"
          }
        ],
        "gridPos": {"h": 4, "w": 6, "x": 12, "y": 16}
      },
      {
        "id": 8,
        "title": "Health Status",
        "type": "stat",
        "targets": [
          {
            "expr": "poolai_health_status",
            "legendFormat": "{{instance}}"
          }
        ],
        "gridPos": {"h": 4, "w": 6, "x": 18, "y": 16}
      },
      {
        "id": 9,
        "title": "RAID Artifacts",
        "type": "graph",
        "targets": [
          {
            "expr": "poolai_raid_artifacts_total",
            "legendFormat": "Total Artifacts"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 0, "y": 20}
      },
      {
        "id": 10,
        "title": "Replication Operations",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(poolai_raid_replication_operations_total[5m])",
            "legendFormat": "{{status}}"
          }
        ],
        "gridPos": {"h": 8, "w": 12, "x": 12, "y": 20}
      }
    ],
    "refresh": "30s",
    "time": {
      "from": "now-1h",
      "to": "now"
    }
  }
}
```

## Docker Compose Setup

```yaml
version: '3.8'

services:
  grafana:
    image: grafana/grafana:latest
    container_name: grafana
    ports:
      - "3000:3000"
    volumes:
      - grafana-data:/var/lib/grafana
      - ./grafana/provisioning:/etc/grafana/provisioning
      - ./grafana/dashboards:/var/lib/grafana/dashboards
    environment:
      - GF_SECURITY_ADMIN_USER=admin
      - GF_SECURITY_ADMIN_PASSWORD=admin
      - GF_USERS_ALLOW_SIGN_UP=false
    networks:
      - monitoring

volumes:
  grafana-data:

networks:
  monitoring:
    external: true
```

## Dashboard Provisioning

Create `grafana/provisioning/dashboards/dashboard.yml`:

```yaml
apiVersion: 1

providers:
  - name: 'PoolAI'
    orgId: 1
    folder: 'PoolAI'
    type: file
    disableDeletion: false
    updateIntervalSeconds: 10
    allowUiUpdates: true
    options:
      path: /var/lib/grafana/dashboards
      foldersFromFilesStructure: true
```

## Alerting in Grafana

### Create Alert Rules

1. Go to Alerting → Alert Rules
2. Create new rule:
   - Name: "PoolAI High CPU Usage"
   - Query: `poolai_resource_cpu_usage_percent > 90`
   - Condition: `WHEN last() OF A IS ABOVE 90`
   - Evaluation: Every 1m, For 5m

### Notification Channels

Configure notification channels:

- Email
- Slack
- PagerDuty
- Webhook

## Custom Panels

### Worker Utilization Gauge

```json
{
  "type": "gauge",
  "targets": [
    {
      "expr": "poolai_workers_active / poolai_workers_total * 100",
      "legendFormat": "Utilization %"
    }
  ],
  "fieldConfig": {
    "defaults": {
      "min": 0,
      "max": 100,
      "unit": "percent"
    },
    "thresholds": {
      "mode": "absolute",
      "steps": [
        {"value": 0, "color": "green"},
        {"value": 70, "color": "yellow"},
        {"value": 90, "color": "red"}
      ]
    }
  }
}
```

### Error Rate Graph

```json
{
  "type": "graph",
  "targets": [
    {
      "expr": "rate(poolai_http_requests_total{status=\"error\"}[5m]) / rate(poolai_http_requests_total[5m]) * 100",
      "legendFormat": "Error Rate %"
    }
  ]
}
```

## Export and Import

### Export Dashboard

```bash
# Via API
curl -H "Authorization: Bearer $GRAFANA_API_KEY" \
  http://localhost:3000/api/dashboards/uid/poolai > dashboard.json
```

### Import Dashboard

```bash
# Via API
curl -X POST -H "Content-Type: application/json" \
  -H "Authorization: Bearer $GRAFANA_API_KEY" \
  -d @dashboard.json \
  http://localhost:3000/api/dashboards/db
```

## Best Practices

1. **Use Variables**: Create dashboard variables for instance selection
2. **Set Refresh Intervals**: Use appropriate refresh rates (15s-1m)
3. **Organize Panels**: Group related metrics together
4. **Use Annotations**: Mark important events
5. **Set Thresholds**: Configure visual thresholds for metrics
6. **Document Dashboards**: Add descriptions to panels

## Troubleshooting

### Dashboard Not Loading

- Check data source connection
- Verify query syntax
- Check time range

### Missing Metrics

- Verify Prometheus is scraping PoolAI
- Check metric names match
- Verify time range includes data

### Performance Issues

- Reduce panel count
- Increase refresh interval
- Use recording rules for complex queries

