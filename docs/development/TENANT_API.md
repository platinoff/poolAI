# Tenant HTTP API Contracts — Enterprise Phase A (Band 53)

Canonical doc: [`TENANT_API.md`](./TENANT_API.md) (band 53, PH-S1177).

## Overview

Band 53 verifies **HTTP** contracts for `/api/enterprise/tenants*`
(CRUD lifecycle, quota/usage, cross-tenant isolation, store-wire read).
Complements band 52 [`TENANT_STORE.md`](./TENANT_STORE.md) (durable-path stub)
and band 51 [`TENANT_PERSIST.md`](./TENANT_PERSIST.md) (store env scaffold).

| Surface | Method | Auth | Notes |
|---------|--------|------|-------|
| `/tenants` | GET | — | List |
| `/tenants` | POST | JWT | Create |
| `/tenants/{id}` | GET | — | Get |
| `/tenants/{id}` | POST | JWT + `admin:all` | Update |
| `/tenants/{id}` | DELETE | JWT | Delete |
| `/tenants/{id}/usage` | GET | — | `TenantResourceUsage` |
| `/tenants/{id}/quota` | POST | — | `QuotaCheckResult` allow/deny |
| `/tenants/store` | GET | — | `TenantStoreWire` `{mode,durable_path,configured}` |

**Boundary:** band 53 = HTTP contract matrix + OpenAPI schemas; restart-safe
SQLite CRUD remains a later phase-A band.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --tenant-api
VERIFY_TENANT_API=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-api
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_api_mode` | `true` when `--tenant-api` (PH-S1176) |
| `tenant_api_criteria_total` | Registry size (10) |
| `tenant_api_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/tenant_api_contracts_depth.rs)
- handlers: [`tenants.rs`](../../src/network/enterprise_api/tenants.rs)
- tests: `tenant_api_contracts_integration.rs`, `galaxy_horizon_s1169_integration.rs`
- store wire (band 52): [`TENANT_STORE.md`](./TENANT_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.34 band 53 journal
- OpenAPI tag `EnterpriseTenants` + schema `TenantStoreWire`
