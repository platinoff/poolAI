# RAID admin metrics (BurstRAID / SmallWorld)

**PH-S18 (2026-05-24)** — polish for `/ui/admin/raid` and admin API contracts.

## API

| Endpoint | Response | When unavailable |
|----------|----------|------------------|
| `GET /api/v1/raid/admin/metrics/burst` | `{ "metrics": BurstRaidMetrics }` | `503` — BurstRAID not active |
| `GET /api/v1/raid/admin/metrics/smallworld` | `{ "metrics": SmallWorldMetrics }` | `503` — SmallWorld not active |

### BurstRaidMetrics keys (UI + contracts)

- `total_artifacts`, `artifacts_in_burst`, `total_requests`, `burst_threshold_rps`
- `base_replication_factor`, `max_replication_factor`

### SmallWorldMetrics keys

- `total_artifacts`, `total_nodes`, `avg_clustering_coefficient`, `target_clustering_coefficient`
- `base_replication_factor`

## Admin UI

- Cards always visible: active metrics or muted «strategy not active».
- Progress bars: burst load %, clustering vs target.
- Sparklines (`admin_charts.js`): rolling history on 10s poll (`loadRaidData`).
- **Rebalance** — `POST /api/v1/raid/admin/rebalance` (Admin/Operator).

## Verification

```bash
cargo test --test admin_ui_api_contracts raid_admin_burst_metrics raid_admin_smallworld_metrics
# Full CI slice:
export K8S_OPENAPI_ENABLED_VERSION=1.28
cargo test-ci
```

Contract reference: [`ADMIN_UI_JSON_CONTRACTS.md`](./ADMIN_UI_JSON_CONTRACTS.md).
