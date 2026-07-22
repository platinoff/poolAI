# SSO HTTP API Contracts — Enterprise Phase B (Band 63)

Canonical doc: [`SSO_API.md`](./SSO_API.md) (band 63, PH-S1277).

## Overview

Band 63 verifies **HTTP** contracts for `/api/enterprise/security/*`
(OAuth2/SAML provider CRUD lifecycle, store-wire read, callback fixtures without live IdP).
Complements band 62 [`SSO_STORE.md`](./SSO_STORE.md) (durable-path wire)
and band 61 [`SSO_DEPTH.md`](./SSO_DEPTH.md) (store env + audience/time stub).

| Surface | Method | Auth | Notes |
|---------|--------|------|-------|
| `/security/oauth2/providers` | GET | — | List |
| `/security/oauth2/providers` | POST | JWT + `admin:all` | Register |
| `/security/oauth2/providers/{name}` | GET | — | Get |
| `/security/oauth2/providers/{name}` | PUT | JWT + `admin:all` | Update |
| `/security/oauth2/providers/{name}` | DELETE | JWT + `admin:all` | Delete |
| `/security/saml/providers` | GET/POST | same pattern | SAML CRUD |
| `/security/saml/providers/{name}` | GET/PUT/DELETE | same pattern | SAML CRUD |
| `/security/sso/store` | GET | — | `SsoStoreWire` `{mode,durable_path,configured}` |
| `/auth/github/callback` | GET | — | Fixture: missing code → `OAUTH2_MISSING_CODE` |
| `/auth/saml/{provider}/callback` | POST | — | Fixture: missing/invalid assertion → 400 |

**Boundary:** band 63 = HTTP contract matrix + OpenAPI `SsoStoreWire`; restart-safe
SQLite provider CRUD and production signature verify remain later phase-B bands.
Master backlog template rows for band 63 (`sso_depth scaffold`) are **overridden** here
(mirror tenant band 53 / [`TENANT_API.md`](./TENANT_API.md)).

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --sso-api
VERIFY_SSO_API=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-api
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_api_mode` | `true` when `--sso-api` (PH-S1276) |
| `sso_api_criteria_total` | Registry size (10) |
| `sso_api_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/sso_api_contracts_depth.rs)
- handlers: [`security.rs`](../../src/network/enterprise_api/security.rs)
- tests: `sso_api_contracts_integration.rs`, `galaxy_horizon_s1269_integration.rs`
- store wire (band 62): [`SSO_STORE.md`](./SSO_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.44 band 63 journal
- OpenAPI tag `EnterpriseSecurity` + schema `SsoStoreWire`
