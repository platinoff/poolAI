# Policies HTTP API Contracts — Enterprise Phase D (Band 83)

Canonical doc: [`POLICIES_API.md`](./POLICIES_API.md) (band 83, PH-S1476). Prev: [`POLICIES_STORE.md`](./POLICIES_STORE.md) (band 82 ✅). Next: [`POLICIES_ADMIN_OPS.md`](./POLICIES_ADMIN_OPS.md) (band 84 ✅).

## Overview

Band 83 verifies **HTTP** contracts for `/api/enterprise/security/policies*`
and `/api/enterprise/policy/store`
(query lifecycle, store-wire read, policy-field validation fixtures).
Complements band 82 [`POLICIES_STORE.md`](./POLICIES_STORE.md) (durable-path wire)
and band 81 [`POLICIES_DEPTH.md`](./POLICIES_DEPTH.md) (store env + field stub).

| Surface | Method | Auth | Notes |
|---------|--------|------|-------|
| `/security/policies` | GET | — | Query filters (`name`, `require_mfa`) + `limit` pagination stub |
| `/policy/store` | GET | — | `PolicyStoreWire` `{mode,durable_path,configured}` |
| `/security/policies/validate` | POST | — | Fixture: missing name / invalid timeout → 400 |

**Boundary:** band 83 = HTTP contract matrix + OpenAPI `PolicyStoreWire`; durable
sqlite CRUD and admin WASM remain later phase-D bands.
Master backlog template rows for band 83 are **overridden** here
(mirror Audit band 73 / [`AUDIT_API.md`](./AUDIT_API.md)).

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --policy-api
VERIFY_POLICY_API=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-api
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_api_mode` | `true` when `--policy-api` (PH-S1475) |
| `policy_api_criteria_total` | Registry size (9) |
| `policy_api_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/policy_api_contracts_depth.rs)
- handlers: [`security.rs`](../../src/network/enterprise_api/security.rs)
- tests: `policy_api_contracts_integration.rs`, `galaxy_horizon_s1469_integration.rs`
- store wire (band 82): [`POLICIES_STORE.md`](./POLICIES_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.64 band 83 journal
- OpenAPI tag `EnterpriseSecurity` + schema `PolicyStoreWire`
- PH-S1469 · policy_api_contracts_depth · PH-S1471 · GET /policy/store · PH-S1474 · VERIFY_POLICY_API · PH-S1475 · --policy-api · PH-S1478
