# Audit Horizon Close — Enterprise Phase C (Band 80)

Canonical doc: [`AUDIT_HORIZON.md`](./AUDIT_HORIZON.md) (band 80, PH-S1445).

## Overview

Band 80 closes **enterprise phase C (Audit)** by aggregating bands 71–79
under one **horizon** gate (`--audit-horizon` / `VERIFY_AUDIT_HORIZON`).
Phase C delivered depth scaffold, store wire, API contracts, admin/ops, stand smoke,
loc-audit, docs canon, vision-sync, and ratio-advisory.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `audit_horizon_depth.rs` | enum + criteria + `AUDIT_HORIZON_SLICES` |
| Slice aggregate | all phase-C `--audit*` + `AUDIT_RATIO_ADVISORY.md` | `--audit` · `--audit-store` · `--audit-api` · `--audit-admin-ops` · `--audit-stand-smoke` · `--audit-loc-audit` · `--audit-docs-canon` · `--audit-vision-sync` · `--audit-ratio-advisory` · `AUDIT_RATIO_ADVISORY.md` |
| Aggregate | `--audit-horizon` | `audit_horizon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_AUDIT_HORIZON` / `--audit-horizon` | phase-C horizon gate |
| Contracts | `audit_horizon_integration` | slices + criteria totals |

**Boundary:** band 80 = phase C horizon close (no new Audit durable SQLite CRUD / SIEM).
Prior ratio-advisory: [`AUDIT_RATIO_ADVISORY.md`](./AUDIT_RATIO_ADVISORY.md) (band 79).
Mirror: [`SSO_HORIZON.md`](./SSO_HORIZON.md). Next: band 81 **D Policies** depth scaffold (PH-S1449…).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --audit-horizon
cargo run --bin poolai-loc-audit -- --audit-horizon --advisory --min-ratio 0.95
VERIFY_AUDIT_HORIZON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --audit-horizon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `audit_horizon_mode` | `true` when `--audit-horizon` (PH-S1444) |
| `audit_horizon_criteria_total` | Registry size (10) |
| `audit_horizon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`audit_horizon_depth.rs`](../../crates/poolai-ui-core/src/audit_horizon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `audit_horizon_integration.rs`, `galaxy_horizon_s1439_integration.rs`
- stand-smoke export shape: `audit_horizon_band80_export_shape_ph_s1443`
- prior: [`AUDIT_RATIO_ADVISORY.md`](./AUDIT_RATIO_ADVISORY.md) · [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md) · [`AUDIT_STORE.md`](./AUDIT_STORE.md)
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.61 band 80 journal
- Slices: `--audit` · `--audit-store` · `--audit-api` · `--audit-admin-ops` · `--audit-stand-smoke` · `--audit-loc-audit` · `--audit-docs-canon` · `--audit-vision-sync` · `--audit-ratio-advisory` · `AUDIT_RATIO_ADVISORY.md`
- PH-S1439 · AUDIT_HORIZON_SLICES · audit_horizon_integration · VERIFY_AUDIT_HORIZON · PH-S1444 · --audit-horizon · PH-S1448
