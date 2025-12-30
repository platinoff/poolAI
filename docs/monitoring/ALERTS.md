# Alerting Configuration

## Overview

This guide covers setting up alerting for PoolAI using Prometheus Alertmanager and other alerting systems.

## Prometheus Alertmanager

### Basic Configuration

Create `alertmanager.yml`:

```yaml
global:
  resolve_timeout: 5m
  slack_api_url: 'https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK'

route:
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
  receiver: 'default'
  routes:
    - match:
        severity: critical
      receiver: 'critical-alerts'
      continue: true
    - match:
        severity: warning
      receiver: 'warning-alerts'

receivers:
  - name: 'default'
    email_configs:
      - to: 'admin@example.com'
        from: 'poolai-alerts@example.com'
        smarthost: 'smtp.example.com:587'
        auth_username: 'alerts@example.com'
        auth_password: 'password'
        headers:
          Subject: 'PoolAI Alert: {{ .GroupLabels.alertname }}'

  - name: 'critical-alerts'
    slack_configs:
      - channel: '#poolai-critical'
        title: '🚨 Critical Alert: {{ .GroupLabels.alertname }}'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'
    pagerduty_configs:
      - service_key: 'YOUR_PAGERDUTY_KEY'
        description: '{{ .GroupLabels.alertname }}'

  - name: 'warning-alerts'
    slack_configs:
      - channel: '#poolai-warnings'
        title: '⚠️ Warning: {{ .GroupLabels.alertname }}'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'
```

### Docker Compose Setup

```yaml
version: '3.8'

services:
  alertmanager:
    image: prom/alertmanager:latest
    container_name: alertmanager
    volumes:
      - ./alertmanager.yml:/etc/alertmanager/alertmanager.yml
      - alertmanager-data:/alertmanager
    command:
      - '--config.file=/etc/alertmanager/alertmanager.yml'
      - '--storage.path=/alertmanager'
    ports:
      - "9093:9093"
    networks:
      - monitoring

volumes:
  alertmanager-data:

networks:
  monitoring:
    external: true
```

## Alert Rules

### Critical Alerts

```yaml
groups:
  - name: poolai_critical
    interval: 30s
    rules:
      - alert: PoolAIDown
        expr: up{job="poolai"} == 0
        for: 1m
        labels:
          severity: critical
          team: platform
        annotations:
          summary: "PoolAI instance is down"
          description: "PoolAI instance {{ $labels.instance }} has been down for more than 1 minute."
          runbook_url: "https://docs.example.com/runbooks/poolai-down"

      - alert: RaftQuorumLost
        expr: count(up{job="poolai-raft"} == 1) < 2
        for: 2m
        labels:
          severity: critical
          team: platform
        annotations:
          summary: "Raft quorum lost"
          description: "Only {{ $value }} Raft nodes are up, quorum requires at least 2."

      - alert: DataReplicationFailure
        expr: rate(poolai_raid_replication_operations_total{status="failed"}[5m]) > 0.5
        for: 5m
        labels:
          severity: critical
          team: platform
        annotations:
          summary: "High replication failure rate"
          description: "Replication failure rate is {{ $value }} failures/second"
```

### Warning Alerts

```yaml
  - name: poolai_warnings
    interval: 30s
    rules:
      - alert: HighCPUUsage
        expr: poolai_resource_cpu_usage_percent > 80
        for: 10m
        labels:
          severity: warning
          team: platform
        annotations:
          summary: "High CPU usage on {{ $labels.instance }}"
          description: "CPU usage is {{ $value }}% on {{ $labels.instance }}"

      - alert: HighMemoryUsage
        expr: poolai_resource_memory_usage_mb > 6144
        for: 10m
        labels:
          severity: warning
          team: platform
        annotations:
          summary: "High memory usage on {{ $labels.instance }}"
          description: "Memory usage is {{ $value }}MB on {{ $labels.instance }}"

      - alert: HighRequestLatency
        expr: histogram_quantile(0.95, poolai_http_request_duration_seconds_bucket) > 0.5
        for: 10m
        labels:
          severity: warning
          team: platform
        annotations:
          summary: "High request latency on {{ $labels.instance }}"
          description: "95th percentile latency is {{ $value }}s"

      - alert: LowWorkerCount
        expr: poolai_workers_active < 20
        for: 15m
        labels:
          severity: warning
          team: platform
        annotations:
          summary: "Low worker count on {{ $labels.instance }}"
          description: "Only {{ $value }} active workers"
```

## Notification Channels

### Slack Integration

```yaml
receivers:
  - name: 'slack-alerts'
    slack_configs:
      - api_url: 'https://hooks.slack.com/services/YOUR/WEBHOOK/URL'
        channel: '#poolai-alerts'
        title: '{{ .GroupLabels.alertname }}'
        text: |
          {{ range .Alerts }}
          *Alert:* {{ .Annotations.summary }}
          *Description:* {{ .Annotations.description }}
          *Severity:* {{ .Labels.severity }}
          *Instance:* {{ .Labels.instance }}
          {{ end }}
        send_resolved: true
```

### Email Integration

```yaml
receivers:
  - name: 'email-alerts'
    email_configs:
      - to: 'ops-team@example.com'
        from: 'poolai-alerts@example.com'
        smarthost: 'smtp.example.com:587'
        auth_username: 'alerts@example.com'
        auth_password: 'password'
        headers:
          Subject: 'PoolAI Alert: {{ .GroupLabels.alertname }}'
        html: |
          <h2>PoolAI Alert</h2>
          <p><strong>Alert:</strong> {{ .GroupLabels.alertname }}</p>
          <p><strong>Description:</strong> {{ .Annotations.description }}</p>
          <p><strong>Severity:</strong> {{ .Labels.severity }}</p>
```

### PagerDuty Integration

```yaml
receivers:
  - name: 'pagerduty-critical'
    pagerduty_configs:
      - service_key: 'YOUR_PAGERDUTY_SERVICE_KEY'
        description: '{{ .GroupLabels.alertname }}: {{ .Annotations.summary }}'
        severity: 'critical'
        details:
          instance: '{{ .Labels.instance }}'
          description: '{{ .Annotations.description }}'
```

### Webhook Integration

```yaml
receivers:
  - name: 'webhook-alerts'
    webhook_configs:
      - url: 'https://your-webhook-endpoint.com/alerts'
        http_config:
          basic_auth:
            username: 'webhook-user'
            password: 'webhook-password'
        send_resolved: true
```

## Alert Grouping and Inhibition

### Grouping Configuration

```yaml
route:
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h
```

### Inhibition Rules

```yaml
inhibit_rules:
  - source_match:
      severity: 'critical'
    target_match:
      severity: 'warning'
    equal: ['alertname', 'instance']
  - source_match:
      alertname: 'PoolAIDown'
    target_match_re:
      alertname: '.*'
    equal: ['instance']
```

## Kubernetes Alertmanager

For Kubernetes deployments:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: alertmanager-config
data:
  alertmanager.yml: |
    global:
      resolve_timeout: 5m
    route:
      receiver: 'default'
    receivers:
      - name: 'default'
        webhook_configs:
          - url: 'http://webhook-service:8080/alerts'
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: alertmanager
spec:
  replicas: 1
  selector:
    matchLabels:
      app: alertmanager
  template:
    metadata:
      labels:
        app: alertmanager
    spec:
      containers:
      - name: alertmanager
        image: prom/alertmanager:latest
        volumeMounts:
        - name: config
          mountPath: /etc/alertmanager
        ports:
        - containerPort: 9093
      volumes:
      - name: config
        configMap:
          name: alertmanager-config
```

## Testing Alerts

### Test Alert Rule

```bash
# Send test alert
curl -X POST http://localhost:9093/api/v1/alerts \
  -H "Content-Type: application/json" \
  -d '[{
    "labels": {
      "alertname": "TestAlert",
      "severity": "warning",
      "instance": "test-instance"
    },
    "annotations": {
      "summary": "Test alert",
      "description": "This is a test alert"
    }
  }]'
```

### Verify Alert Configuration

```bash
# Check Alertmanager status
curl http://localhost:9093/api/v1/status

# Check active alerts
curl http://localhost:9093/api/v2/alerts
```

## Best Practices

1. **Use Appropriate Severities**: critical, warning, info
2. **Set Reasonable Thresholds**: Avoid alert fatigue
3. **Group Related Alerts**: Reduce notification noise
4. **Include Runbook URLs**: Help operators resolve issues
5. **Test Alert Rules**: Regularly test alert delivery
6. **Monitor Alertmanager**: Ensure alerts are being sent
7. **Use Inhibition Rules**: Prevent duplicate alerts
8. **Document Alert Rules**: Explain why each alert exists

## Troubleshooting

### Alerts Not Firing

- Check Prometheus alert rules are loaded
- Verify query returns results
- Check `for` duration is met
- Verify Alertmanager is connected to Prometheus

### Alerts Not Delivered

- Check Alertmanager configuration
- Verify notification channel credentials
- Check network connectivity
- Review Alertmanager logs

### Too Many Alerts

- Increase thresholds
- Adjust `repeat_interval`
- Use inhibition rules
- Group alerts more aggressively

