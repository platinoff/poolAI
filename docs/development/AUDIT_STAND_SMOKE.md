# Audit Stand Smoke — Enterprise Phase C (Band 75)

Canonical doc: [`AUDIT_STAND_SMOKE.md`](./AUDIT_STAND_SMOKE.md) (band 75, PH-S1396).

## Overview

Band 75 adds **live HTTP stand smoke** for audit store / events query / event-field
validation fixtures against a running coordinator (`POOLAI_BASE_URL`), plus loc-audit / verify hooks.
In-process CI canon remains `tests/audit_stand_smoke_integration.rs` (no stand).

| Surface | Where | Notes |
|---------|-------|-------|
| Store wire | `GET /api/enterprise/audit/store` | `{mode,durable_path,configured}` |
| Events query | `GET /api/enterprise/audit/events?limit=5` | optional `action=` filter; array body |
| Field fixtures | `POST /api/enterprise/audit/events/validate` | empty action / blank `resource_type` → 400 `AUDIT_MISSING_*` |
| CLI | `--audit-stand-smoke` / `POOLAI_STAND_SMOKE_AUDIT=1` | live suite |
| Verify / quick | `VERIFY_AUDIT_STAND_SMOKE` / `--audit-stand-smoke` | live + loc-audit |

**Boundary:** band 75 = live stand smoke + ops gates. Prior: [`AUDIT_ADMIN_OPS.md`](./AUDIT_ADMIN_OPS.md)
(band 74), [`AUDIT_API.md`](./AUDIT_API.md) (band 73), [`AUDIT_STORE.md`](./AUDIT_STORE.md) (band 72).
Mirror: [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md) (band 65).

## Live smoke / loc-audit / verify

```bash
export POOLAI_BASE_URL=http://127.0.0.1:8080
cargo run --bin poolai-http-stand-smoke -- --audit-stand-smoke
# or: POOLAI_STAND_SMOKE_AUDIT=1

cargo run --bin poolai-loc-audit -- --audit-stand-smoke
cargo run --bin poolai-loc-audit -- --audit-stand-smoke --advisory --min-ratio 0.95
VERIFY_AUDIT_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-stand-smoke
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_stand_smoke_mode` | `true` when `--audit-stand-smoke` (PH-S1394) |
| `audit_stand_smoke_criteria_total` | Registry size (10) |
| `audit_stand_smoke_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/audit_stand_smoke_depth.rs)
- live runners: [`poolai_http_stand_smoke.rs`](../../src/bin/poolai_http_stand_smoke.rs) (`smoke_audit_*`)
- tests: `audit_stand_smoke_integration.rs`, `galaxy_horizon_s1389_integration.rs`
- admin/ops (band 74): [`AUDIT_ADMIN_OPS.md`](./AUDIT_ADMIN_OPS.md)
- API contracts (band 73): [`AUDIT_API.md`](./AUDIT_API.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.56 band 75 journal
- OpenAPI enterprise audit store / events tags
