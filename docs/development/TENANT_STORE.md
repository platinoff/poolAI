# Tenant Store Wire — Enterprise Phase A (Band 52)

Canonical doc: [`TENANT_STORE.md`](./TENANT_STORE.md) (band 52, PH-S1165).

## Overview

Band 52 wires the durable-path / production-verify stub for multi-tenancy
(FM-horizon v2). **Restart-safe SQLite CRUD** landed in band 59
([`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md)).

| Mode | Env | Status |
|------|-----|--------|
| `memory` | `POOLAI_TENANT_STORE=memory` (default) | In-memory CRUD |
| `sqlite` unconfigured | `POOLAI_TENANT_STORE=sqlite` without data dir | Wire label `sqlite_unconfigured` |
| `sqlite` configured | `POOLAI_TENANT_STORE=sqlite` + `POOLAI_TENANT_DATA_DIR=…` | Durable path → `…/tenants.sqlite` + restart-safe CRUD |

**Boundary:** band 52 resolves the wire (`tenant_store_wire()`);
band 59 persists create/get/update/delete via `persist_tenant_to_sqlite`.

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --tenant-store
VERIFY_TENANT_STORE=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-store
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_store_mode` | `true` when `--tenant-store` (PH-S1164) |
| `tenant_store_criteria_total` | Registry size (7) |
| `tenant_store_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_depth.rs`](../../crates/poolai-ui-core/src/tenant_depth.rs)
- domain: [`multi_tenancy.rs`](../../src/enterprise/multi_tenancy.rs) — `tenant_store_wire()`
- scaffold (band 51): [`TENANT_PERSIST.md`](./TENANT_PERSIST.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.33 band 52 journal
- `POOLAI_TENANT_DATA_DIR` — durable directory for future sqlite file
