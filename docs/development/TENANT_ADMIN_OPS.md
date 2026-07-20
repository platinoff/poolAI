# Tenant Admin/Ops Glue — Enterprise Phase A (Band 54)

Canonical doc: [`TENANT_ADMIN_OPS.md`](./TENANT_ADMIN_OPS.md) (band 54, PH-S1186).

## Overview

Band 54 wires **admin UI + ops hooks** for tenant store/usage/quota surfaces
already verified as HTTP contracts in band 53 ([`TENANT_API.md`](./TENANT_API.md)).

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire strip | `/ui/admin/tenants` `#tenant-store-badge` | `GET /api/enterprise/tenants/store` |
| Usage refresh | row action **Usage** (`refreshTenantUsage`) | `GET /api/enterprise/tenants/{id}/usage` |
| Quota probe | row action **Quota** (`probeTenantQuota`) | `POST /api/enterprise/tenants/{id}/quota` |
| Verify / quick | `VERIFY_TENANT_ADMIN_OPS` / `--tenant-admin-ops` | loc-audit gate |

**Boundary:** band 54 = admin/ops glue + verify/loc-audit; restart-safe SQLite CRUD
remains a later phase-A band.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --tenant-admin-ops
cargo run --bin poolai-loc-audit -- --tenant-admin-ops --advisory --min-ratio 0.95
VERIFY_TENANT_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-admin-ops
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_admin_ops_mode` | `true` when `--tenant-admin-ops` (PH-S1185) |
| `tenant_admin_ops_criteria_total` | Registry size (10) |
| `tenant_admin_ops_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/tenant_admin_ops_depth.rs)
- admin UI: [`tenants.rs`](../../src/ui/admin/tenants.rs)
- tests: `tenant_admin_ops_integration.rs`, `galaxy_horizon_s1179_integration.rs`
- API contracts (band 53): [`TENANT_API.md`](./TENANT_API.md)
- store wire (band 52): [`TENANT_STORE.md`](./TENANT_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.35 band 54 journal
- OpenAPI tag `EnterpriseTenants` + schema `TenantStoreWire`
