# SSO Admin/Ops Glue — Enterprise Phase B (Band 64)

Canonical doc: [`SSO_ADMIN_OPS.md`](./SSO_ADMIN_OPS.md) (band 64, PH-S1286).

## Overview

Band 64 wires **admin UI + ops hooks** for SSO store/provider surfaces
already verified as HTTP contracts in band 63 ([`SSO_API.md`](./SSO_API.md)).

| Surface | Where | Notes |
|---------|-------|-------|
| Store-wire strip | `/ui/admin/security` `#sso-store-badge` | `GET /api/enterprise/security/sso/store` |
| OAuth2 refresh | tab action **Refresh** (`refreshOAuth2Providers`) | `GET /api/enterprise/security/oauth2/providers` |
| SAML refresh | tab action **Refresh** (`refreshSamlProviders`) | `GET /api/enterprise/security/saml/providers` |
| Verify / quick | `VERIFY_SSO_ADMIN_OPS` / `--sso-admin-ops` | loc-audit gate |

**Boundary:** band 64 = admin/ops glue + verify/loc-audit; live IdP callback flows
remain fixture-only (band 63). Live stand smoke: [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md) (band 65).

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --sso-admin-ops
cargo run --bin poolai-loc-audit -- --sso-admin-ops --advisory --min-ratio 0.95
VERIFY_SSO_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-admin-ops
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_admin_ops_mode` | `true` when `--sso-admin-ops` (PH-S1285) |
| `sso_admin_ops_criteria_total` | Registry size (10) |
| `sso_admin_ops_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/sso_admin_ops_depth.rs)
- admin UI: [`security.rs`](../../src/ui/admin/security.rs)
- tests: `sso_admin_ops_integration.rs`, `galaxy_horizon_s1279_integration.rs`
- API contracts (band 63): [`SSO_API.md`](./SSO_API.md)
- store wire (band 62): [`SSO_STORE.md`](./SSO_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)
- completion: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.45 band 64 journal
- OpenAPI tag Enterprise Security + schema `SsoStoreWire`
