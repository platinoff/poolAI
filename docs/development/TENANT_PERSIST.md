# Tenant Persistence — Enterprise Phase A (Band 51+)

Canonical doc: [`TENANT_PERSIST.md`](./TENANT_PERSIST.md) (band 51, PH-S1157).

## Overview

Band 51 scaffolds durable multi-tenancy for FM-horizon v2 (enterprise 100%).

Today `TenantManager` defaults to **in-memory** storage. Horizon path:

| Mode | Env | Status |
|------|-----|--------|
| `memory` | `POOLAI_TENANT_STORE=memory` (default) | Current |
| `sqlite` | `POOLAI_TENANT_STORE=sqlite` | Band 52+ store wire — see [`TENANT_STORE.md`](./TENANT_STORE.md) |

## Loc-audit / verify hooks

```bash
cargo run --bin poolai-loc-audit -- --tenant-persist
VERIFY_TENANT_PERSIST=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --tenant-persist
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `tenant_persist_mode` | `true` when `--tenant-persist` (PH-S1150) |
| `tenant_persist_criteria_total` | Registry size (7) |
| `tenant_persist_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`tenant_persistence_depth.rs`](../../crates/poolai-ui-core/src/tenant_persistence_depth.rs)
- domain: [`multi_tenancy.rs`](../../src/enterprise/multi_tenancy.rs) — `POOLAI_TENANT_STORE` hint
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.32 band 51 journal
- `docs/development/CI_CANON.md` — local dual-gate (orthogonal)
