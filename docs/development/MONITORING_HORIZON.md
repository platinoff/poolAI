# Monitoring Horizon Close — Enterprise Phase E (Band 100)

Canonical doc: [`MONITORING_HORIZON.md`](./MONITORING_HORIZON.md) (band 100, PH-S1645).

## Overview

Band 100 closes **enterprise phase E (Monitoring)** by aggregating bands 91–99
under one **horizon** gate (`--monitoring-horizon` / `VERIFY_MONITORING_HORIZON`).
Phase E delivered depth scaffold, store wire, API contracts, admin/ops, stand smoke,
loc-audit, docs canon, vision-sync, and ratio-advisory.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `monitoring_horizon_depth.rs` | enum + criteria + `MONITORING_HORIZON_SLICES` |
| Slice aggregate | all phase-E `--monitoring*` + `MONITORING_RATIO_ADVISORY.md` | `--monitoring` · `--monitoring-store` · `--monitoring-api` · `--monitoring-admin-ops` · `--monitoring-stand-smoke` · `--monitoring-loc-audit` · `--monitoring-docs-canon` · `--monitoring-vision-sync` · `--monitoring-ratio-advisory` · `MONITORING_RATIO_ADVISORY.md` |
| Aggregate | `--monitoring-horizon` | `monitoring_horizon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_MONITORING_HORIZON` / `--monitoring-horizon` | phase-E horizon gate |
| Contracts | `monitoring_horizon_integration` | slices + criteria totals |

**Boundary:** band 100 = phase E horizon close.
Prior ratio-advisory: [`MONITORING_RATIO_ADVISORY.md`](./MONITORING_RATIO_ADVISORY.md) (band 99).
Next: monitoring band close integration suite (`PH-S1648`).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --monitoring-horizon
cargo run --bin poolai-loc-audit -- --monitoring-horizon --advisory --min-ratio 0.95
VERIFY_MONITORING_HORIZON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-horizon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_horizon_mode` | `true` when `--monitoring-horizon` (PH-S1644) |
| `monitoring_horizon_criteria_total` | Registry size (10) |
| `monitoring_horizon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_horizon_depth.rs`](../../crates/poolai-ui-core/src/monitoring_horizon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `monitoring_horizon_integration.rs`, `galaxy_horizon_s1639_integration.rs`
- stand-smoke export shape: `monitoring_horizon_band100_export_shape_ph_s1643`
- prior: [`MONITORING_RATIO_ADVISORY.md`](./MONITORING_RATIO_ADVISORY.md) · `MONITORING_VISION_SYNC.md` · `MONITORING_STORE.md`
- roadmap: [`PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md`](./PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md)

## Related

- FM §5.17 enterprise 100% · §5.81 band 100 journal
- Slices: `--monitoring` · `--monitoring-store` · `--monitoring-api` · `--monitoring-admin-ops` · `--monitoring-stand-smoke` · `--monitoring-loc-audit` · `--monitoring-docs-canon` · `--monitoring-vision-sync` · `--monitoring-ratio-advisory` · `MONITORING_RATIO_ADVISORY.md`
- PH-S1639 · `MONITORING_HORIZON_SLICES` · `monitoring_horizon_integration` · `VERIFY_MONITORING_HORIZON` · PH-S1644 · `--monitoring-horizon` · PH-S1648
