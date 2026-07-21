# Tenant Stand Smoke — Enterprise Phase A (Band 55)

Canonical doc: [`TENANT_STAND_SMOKE.md`](./TENANT_STAND_SMOKE.md) (band 55, PH-S1196).

## Overview

Band 55 adds **live HTTP stand smoke** for tenant store / CRUD / usage+quota
against a running coordinator (`POOLAI_BASE_URL`), plus loc-audit / verify hooks.
In-process CI canon remains `tests/tenant_stand_smoke_integration.rs` (no stand).

| Surface | Where | Notes |
|---------|-------|-------|
| Store wire | `GET /api/enterprise/tenants/store` | `{mode,durable_path,configured}` |
| CRUD | list → create → get → delete | admin Bearer via `/api/v1/login` |
| Usage + quota | `GET …/usage`, `POST …/quota` | allow/deny + foreign UUID → 404 |
| CLI | `--tenant-stand-smoke` / `POOLAI_STAND_SMOKE_TENANT=1` | live suite |
| Verify / quick | `VERIFY_TENANT_STAND_SMOKE` / `--tenant-stand-smoke` | live + loc-audit |

**Boundary:** band 55 = live stand smoke + ops gates; restart-safe SQLite CRUD
remains a later phase-A band. Prior: [`TENANT_ADMIN_OPS.md`](./TENANT_ADMIN_OPS.md)
(band 54), [`TENANT_API.md`](./TENANT_API.md) (band 53).

## Live smoke / loc-audit / verify

```bash
export POOLAI_BASE_URL=http://127.0.0.1:8080
cargo run --bin poolai-http-stand-smoke -- --tenant-stand-smoke
# or: POOLAI_STAND_SMOKE_TENANT=1

cargo run --bin poolai-loc-audit -- --tenant-stand-smoke
cargo run --bin poolai-loc-audit -- --tenant-stand-smoke --advisory --min-ratio 0.95
VERIFY_TENANT_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-stand-smoke
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_stand_smoke_mode` | `true` when `--tenant-stand-smoke` (PH-S1194) |
| `tenant_stand_smoke_criteria_total` | Registry size (10) |
| `tenant_stand_smoke_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/tenant_stand_smoke_depth.rs)
- live runners: [`poolai_http_stand_smoke.rs`](../../src/bin/poolai_http_stand_smoke.rs) (`smoke_tenants_*`)
- tests: `tenant_stand_smoke_integration.rs`, `galaxy_horizon_s1189_integration.rs`
- admin/ops (band 54): [`TENANT_ADMIN_OPS.md`](./TENANT_ADMIN_OPS.md)
- API contracts (band 53): [`TENANT_API.md`](./TENANT_API.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.36 band 55 journal
- OpenAPI tag `EnterpriseTenants` + schema `TenantStoreWire`
