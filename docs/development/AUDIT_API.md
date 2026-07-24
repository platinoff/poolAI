# Audit HTTP API Contracts — Enterprise Phase C (Band 73)

Canonical doc: [`AUDIT_API.md`](./AUDIT_API.md) (band 73, PH-S1376). Prev: [`AUDIT_STORE.md`](./AUDIT_STORE.md) (band 72 ✅). Next: [`AUDIT_ADMIN_OPS.md`](./AUDIT_ADMIN_OPS.md) (band 74 ✅).

## Overview

Band 73 verifies **HTTP** contracts for `/api/enterprise/audit/*`
(query lifecycle, store-wire read, event-field validation fixtures).
Complements band 72 [`AUDIT_STORE.md`](./AUDIT_STORE.md) (durable-path wire)
and band 71 [`AUDIT_DEPTH.md`](./AUDIT_DEPTH.md) (store env + field stub).

| Surface | Method | Auth | Notes |
|---------|--------|------|-------|
| `/audit/events` | GET | — | Query filters + `limit` pagination stub |
| `/audit/store` | GET | — | `AuditStoreWire` `{mode,durable_path,configured}` |
| `/audit/events/validate` | POST | — | Fixture: missing action/resource → 400 |

**Boundary:** band 73 = HTTP contract matrix + OpenAPI `AuditStoreWire`; durable
append/query CRUD and admin WASM remain later phase-C bands.
Master backlog template rows for band 73 are **overridden** here
(mirror SSO band 63 / [`SSO_API.md`](./SSO_API.md)).

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --audit-api
VERIFY_AUDIT_API=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-api
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_api_mode` | `true` when `--audit-api` (PH-S1375) |
| `audit_api_criteria_total` | Registry size (9) |
| `audit_api_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/audit_api_contracts_depth.rs)
- handlers: [`audit.rs`](../../src/network/enterprise_api/audit.rs)
- tests: `audit_api_contracts_integration.rs`, `galaxy_horizon_s1369_integration.rs`
- store wire (band 72): [`AUDIT_STORE.md`](./AUDIT_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.54 band 73 journal
- OpenAPI tag `EnterpriseAudit` + schema `AuditStoreWire`
