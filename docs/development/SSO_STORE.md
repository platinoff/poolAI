# SSO Store Wire — Enterprise Phase B (Band 62)

Canonical doc: [`SSO_STORE.md`](./SSO_STORE.md) (band 62, PH-S1265).

## Overview

Band 62 wires the durable-path stub for SSO provider store (FM-horizon v2,
enterprise §5.17 criterion 2). **Restart-safe SQLite CRUD** remains a later
phase-B band (API contracts / persist), mirroring tenants (wire band 52 →
CRUD band 59).

| Mode | Env | Status |
|------|-----|--------|
| `memory` | `POOLAI_SSO_STORE=memory` (default) | In-memory providers (band 61+) |
| `sqlite` unconfigured | `POOLAI_SSO_STORE=sqlite` without data dir | Wire label `sqlite_unconfigured` |
| `sqlite` configured | `POOLAI_SSO_STORE=sqlite` + `POOLAI_SSO_DATA_DIR=…` | Durable path → `…/sso.sqlite` |

**Boundary:** band 62 resolves the wire (`sso_store_wire()`);
later bands persist OAuth2/SAML provider CRUD via sqlite.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --sso-store
cargo run --bin poolai-loc-audit -- --sso-store --advisory --min-ratio 0.95
VERIFY_SSO_STORE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-store
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_store_mode` | `true` when `--sso-store` (PH-S1264) |
| `sso_store_criteria_total` | Registry size (7) |
| `sso_store_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_store_depth.rs`](../../crates/poolai-ui-core/src/sso_store_depth.rs)
- domain: [`security.rs`](../../src/enterprise/security.rs) — `sso_store_wire()`
- scaffold (band 61): [`SSO_DEPTH.md`](./SSO_DEPTH.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.43 band 62 journal
- `POOLAI_SSO_DATA_DIR` — durable directory for future sqlite file
