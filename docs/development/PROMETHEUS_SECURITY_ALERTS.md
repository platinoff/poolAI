# Prometheus security alerting (PH-S28)

Example alert rules for PoolAI coordinators: [`../../deploy/prometheus/poolai-alerts.yml`](../../deploy/prometheus/poolai-alerts.yml).

## Metrics used

| Metric | When |
|--------|------|
| `poolai_http_requests_total{status}` | HTTP error rate |
| `poolai_secret_rotations_total{kind,success}` | PH-S29 rotation hooks |
| `poolai_workers_active` | Worker pool health |

Enable metrics: `cargo build --features prometheus,enterprise` — see [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md).

## Install

```yaml
# prometheus.yml fragment
rule_files:
  - /etc/prometheus/poolai-alerts.yml
scrape_configs:
  - job_name: poolai
    metrics_path: /metrics
    static_configs:
      - targets: ["poolai-coordinator:8080"]
```

## Related

- [`../security/PEN_TEST_CHECKLIST.md`](../security/PEN_TEST_CHECKLIST.md)
- [`../security/SECRETS_MANAGEMENT.md`](../security/SECRETS_MANAGEMENT.md)
