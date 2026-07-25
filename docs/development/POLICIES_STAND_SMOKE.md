# Policies Stand Smoke — Enterprise Phase D (Band 85)

Canonical doc: [`POLICIES_STAND_SMOKE.md`](./POLICIES_STAND_SMOKE.md) (band 85, PH-S1496).

## Overview

Band 85 adds **live HTTP stand smoke** for the policy store wire, security policies
query and policy-field validation fixtures against a running coordinator
(`POOLAI_BASE_URL`), plus loc-audit / verify hooks. In-process CI canon remains
`tests/policy_stand_smoke_integration.rs` (no stand).

| Surface | Where | Notes |
|---------|-------|-------|
| Store wire | `GET /api/enterprise/policy/store` | `{mode,durable_path,configured}` |
| Policies query | `GET /api/enterprise/security/policies?limit=5` | optional `name=` filter; array body |
| Field fixtures | `POST /api/enterprise/security/policies/validate` | empty name / `session_timeout=0` → 400 `POLICY_MISSING_NAME` / `POLICY_INVALID_TIMEOUT` |
| CLI | `--policy-stand-smoke` / `POOLAI_STAND_SMOKE_POLICY=1` | live suite |
| Verify / quick | `VERIFY_POLICY_STAND_SMOKE` / `--policy-stand-smoke` | live + loc-audit |

**Boundary:** band 85 = live stand smoke + ops gates. Prior: [`POLICIES_ADMIN_OPS.md`](./POLICIES_ADMIN_OPS.md)
(band 84), [`POLICIES_API.md`](./POLICIES_API.md) (band 83), [`POLICIES_STORE.md`](./POLICIES_STORE.md) (band 82).
Next: Policies loc-audit aggregate (band 86). Mirror: [`AUDIT_STAND_SMOKE.md`](./AUDIT_STAND_SMOKE.md) (band 75).

## Live smoke / loc-audit / verify

```bash
export POOLAI_BASE_URL=http://127.0.0.1:8080
cargo run --bin poolai-http-stand-smoke -- --policy-stand-smoke
# or: POOLAI_STAND_SMOKE_POLICY=1

cargo run --bin poolai-loc-audit -- --policy-stand-smoke
cargo run --bin poolai-loc-audit -- --policy-stand-smoke --advisory --min-ratio 0.95
VERIFY_POLICY_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --policy-stand-smoke
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `policy_stand_smoke_mode` | `true` when `--policy-stand-smoke` (PH-S1494) |
| `policy_stand_smoke_criteria_total` | Registry size (10) |
| `policy_stand_smoke_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`policy_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/policy_stand_smoke_depth.rs)
- live runners: [`poolai_http_stand_smoke.rs`](../../src/bin/poolai_http_stand_smoke.rs) (`smoke_policy_*`)
- tests: `policy_stand_smoke_integration.rs`, `galaxy_horizon_s1489_integration.rs`
- admin/ops (band 84): [`POLICIES_ADMIN_OPS.md`](./POLICIES_ADMIN_OPS.md)
- API contracts (band 83): [`POLICIES_API.md`](./POLICIES_API.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.66 band 85 journal
- OpenAPI tag `EnterpriseSecurity` + schema `PolicyStoreWire`
