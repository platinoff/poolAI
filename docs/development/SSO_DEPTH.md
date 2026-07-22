# SSO Depth — Enterprise Phase B (Band 61+)

Canonical doc: [`SSO_DEPTH.md`](./SSO_DEPTH.md) (band 61, PH-S1255).

## Overview

Band 61 scaffolds the SSO **production path** for FM-horizon v2 (enterprise §5.17 criterion 2).

Today `SecurityManager` keeps OAuth2/SAML providers **in-memory**. Horizon path:

| Mode | Env | Status |
|------|-----|--------|
| `memory` | `POOLAI_SSO_STORE=memory` (default) | Current — restart-unsafe |
| `sqlite` | `POOLAI_SSO_STORE=sqlite` | Band 62+ store wire — see [`SSO_STORE.md`](./SSO_STORE.md) |

Production verify stub (PH-S1250): SAML assertion checks **Audience** + **NotOnOrAfter** under `cargo test-ci` (no live IdP). Full XML signature verify remains later in phase B.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --sso
cargo run --bin poolai-loc-audit -- --sso --advisory --min-ratio 0.95
VERIFY_SSO=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_mode` | `true` when `--sso` (PH-S1254) |
| `sso_criteria_total` | Registry size (8) |
| `sso_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_depth.rs`](../../crates/poolai-ui-core/src/sso_depth.rs)
- domain: [`security.rs`](../../src/enterprise/security.rs) — `POOLAI_SSO_STORE`, `sso_store_mode()`, SAML audience/time stub
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.42 band 61 journal
- Phase A tenants closed at band 60 — see [`TENANT_HORIZON.md`](./TENANT_HORIZON.md)
