# SSO Stand Smoke — Enterprise Phase B (Band 65)

Canonical doc: [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md) (band 65, PH-S1296).

## Overview

Band 65 adds **live HTTP stand smoke** for SSO store / OAuth2+SAML CRUD / callback
fixtures against a running coordinator (`POOLAI_BASE_URL`), plus loc-audit / verify hooks.
In-process CI canon remains `tests/sso_stand_smoke_integration.rs` (no stand).

| Surface | Where | Notes |
|---------|-------|-------|
| Store wire | `GET /api/enterprise/security/sso/store` | `{mode,durable_path,configured}` |
| CRUD | OAuth2 + SAML list → create → get → delete | admin Bearer via `/api/v1/login` |
| Callbacks | OAuth missing code / SAML missing provider | fixture-only, **no live IdP** |
| CLI | `--sso-stand-smoke` / `POOLAI_STAND_SMOKE_SSO=1` | live suite |
| Verify / quick | `VERIFY_SSO_STAND_SMOKE` / `--sso-stand-smoke` | live + loc-audit |

**Boundary:** band 65 = live stand smoke + ops gates. Prior: [`SSO_ADMIN_OPS.md`](./SSO_ADMIN_OPS.md)
(band 64), [`SSO_API.md`](./SSO_API.md) (band 63), [`SSO_STORE.md`](./SSO_STORE.md) (band 62).
Mirror: [`TENANT_STAND_SMOKE.md`](./TENANT_STAND_SMOKE.md) (band 55).

## Live smoke / loc-audit / verify

```bash
export POOLAI_BASE_URL=http://127.0.0.1:8080
cargo run --bin poolai-http-stand-smoke -- --sso-stand-smoke
# or: POOLAI_STAND_SMOKE_SSO=1

cargo run --bin poolai-loc-audit -- --sso-stand-smoke
cargo run --bin poolai-loc-audit -- --sso-stand-smoke --advisory --min-ratio 0.95
VERIFY_SSO_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-stand-smoke
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_stand_smoke_mode` | `true` when `--sso-stand-smoke` (PH-S1294) |
| `sso_stand_smoke_criteria_total` | Registry size (10) |
| `sso_stand_smoke_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/sso_stand_smoke_depth.rs)
- live runners: [`poolai_http_stand_smoke.rs`](../../src/bin/poolai_http_stand_smoke.rs) (`smoke_sso_*`)
- tests: `sso_stand_smoke_integration.rs`, `galaxy_horizon_s1289_integration.rs`
- admin/ops (band 64): [`SSO_ADMIN_OPS.md`](./SSO_ADMIN_OPS.md)
- API contracts (band 63): [`SSO_API.md`](./SSO_API.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.46 band 65 journal
- OpenAPI enterprise SSO store / providers tags
