# Audit Admin/Ops Glue — Enterprise Phase C (Band 74)

Canonical doc: [`AUDIT_ADMIN_OPS.md`](./AUDIT_ADMIN_OPS.md) (band 74, PH-S1386).
Prev: [`AUDIT_API.md`](./AUDIT_API.md) (band 73 ✅). Mirror: [`SSO_ADMIN_OPS.md`](./SSO_ADMIN_OPS.md).

## Overview

Band 74 wires **admin UI + ops hooks** for audit store/query surfaces
already verified as HTTP contracts in band 73 ([`AUDIT_API.md`](./AUDIT_API.md)).

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire strip | `/ui/admin/audit` `#audit-store-badge` | `GET /api/enterprise/audit/store` |
| Query refresh | header action **Refresh** (`refreshAuditEvents`) | `GET /api/enterprise/audit/events` filters |
| Verify / quick | `VERIFY_AUDIT_ADMIN_OPS` / `--audit-admin-ops` | loc-audit gate |

**Boundary:** band 74 = admin/ops glue + verify/loc-audit; live stand smoke for
audit is band 75. Master backlog template rows for band 74 are **overridden** here
(mirror SSO band 64).

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --audit-admin-ops
cargo run --bin poolai-loc-audit -- --audit-admin-ops --advisory --min-ratio 0.95
VERIFY_AUDIT_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-admin-ops
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_admin_ops_mode` | `true` when `--audit-admin-ops` (PH-S1385) |
| `audit_admin_ops_criteria_total` | Registry size (10) |
| `audit_admin_ops_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/audit_admin_ops_depth.rs)
- admin UI: [`audit.rs`](../../src/ui/admin/audit.rs)
- tests: `audit_admin_ops_integration.rs`, `galaxy_horizon_s1379_integration.rs`
- stand smoke (band 75): [`AUDIT_STAND_SMOKE.md`](./AUDIT_STAND_SMOKE.md)
- API contracts (band 73): [`AUDIT_API.md`](./AUDIT_API.md)
- store wire (band 72): [`AUDIT_STORE.md`](./AUDIT_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)
- completion: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.55 band 74 journal
- OpenAPI tag `EnterpriseAudit` + schema `AuditStoreWire`
