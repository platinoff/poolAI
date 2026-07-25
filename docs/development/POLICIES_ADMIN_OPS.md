# Policies Admin/Ops Glue — Enterprise Phase D (Band 84)

Canonical doc: [`POLICIES_ADMIN_OPS.md`](./POLICIES_ADMIN_OPS.md) (band 84, PH-S1486).
Prev: [`POLICIES_API.md`](./POLICIES_API.md) (band 83 ✅). Next: Policies stand smoke (band 85). Mirror: [`AUDIT_ADMIN_OPS.md`](./AUDIT_ADMIN_OPS.md).

## Overview

Band 84 wires **admin UI + ops hooks** for policy store/query surfaces already
verified as HTTP contracts in band 83 ([`POLICIES_API.md`](./POLICIES_API.md)).

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire strip | `/ui/admin/security` `#policy-store-badge` | `GET /api/enterprise/policy/store` |
| Query refresh | Security Policies header **Refresh** (`refreshSecurityPolicies`) | policy list reload |
| Verify / quick | `VERIFY_POLICY_ADMIN_OPS` / `--policy-admin-ops` | loc-audit gate |

**Boundary:** band 84 = admin/ops glue + verify/loc-audit; live stand smoke is a
later phase-D band. Master backlog template rows for band 84 are overridden here
(mirror Audit band 74).

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --policy-admin-ops
cargo run --bin poolai-loc-audit -- --policy-admin-ops --advisory --min-ratio 0.95
VERIFY_POLICY_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-admin-ops
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_admin_ops_mode` | `true` when `--policy-admin-ops` (PH-S1485) |
| `policy_admin_ops_criteria_total` | Registry size (10) |
| `policy_admin_ops_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/policy_admin_ops_depth.rs)
- admin UI: [`security.rs`](../../src/ui/admin/security.rs)
- tests: `policy_admin_ops_integration.rs`, `galaxy_horizon_s1479_integration.rs`
- API contracts: [`POLICIES_API.md`](./POLICIES_API.md)
- store wire: [`POLICIES_STORE.md`](./POLICIES_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.65 band 84 journal
- OpenAPI tag `EnterpriseSecurity` + schema `PolicyStoreWire`
