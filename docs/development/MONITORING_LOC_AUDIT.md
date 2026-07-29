# Monitoring Loc-Audit Aggregate — Enterprise Phase E (Band 96)

Canonical doc: [`MONITORING_LOC_AUDIT.md`](./MONITORING_LOC_AUDIT.md) (band 96, PH-S1605).

## Overview

Band 96 consolidates band 91–95 **`--monitoring*` loc-audit slices** under one aggregate
gate (`--monitoring-loc-audit` / `VERIFY_MONITORING_LOC_AUDIT`). Slice flags remain available
individually; the aggregate registry proves all five exist plus verify/docs/band-close.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `monitoring_loc_audit_depth.rs` | enum + criteria + `MONITORING_LOC_AUDIT_SLICES` |
| Slice flags | `poolai-loc-audit` | `--monitoring` … `--monitoring-stand-smoke` |
| Aggregate | `--monitoring-loc-audit` | `monitoring_loc_audit_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_MONITORING_LOC_AUDIT` / `--monitoring-loc-audit` | loc-audit gate only |
| Contracts | `monitoring_loc_audit_integration` | slice presence + criteria totals |

**Boundary:** band 96 = aggregate loc-audit ops gate; prior live smoke remains
[`MONITORING_STAND_SMOKE.md`](./MONITORING_STAND_SMOKE.md) (band 95). Next: Monitoring docs canon (band 97).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --monitoring-loc-audit
cargo run --bin poolai-loc-audit -- --monitoring-loc-audit --migration-advisory --advisory --min-ratio 0.95
VERIFY_MONITORING_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --monitoring-loc-audit
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `monitoring_loc_audit_mode` | `true` when `--monitoring-loc-audit` (PH-S1604) |
| `monitoring_loc_audit_criteria_total` | Registry size (10) |
| `monitoring_loc_audit_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`monitoring_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/monitoring_loc_audit_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- verify: `VERIFY_MONITORING_LOC_AUDIT` in [`verify-dev-stand.sh`](../../bin/verify-dev-stand.sh)
- tests: `monitoring_loc_audit_integration.rs`, `galaxy_horizon_s1599_integration.rs`
- FM: §5.77 PH-S1599…S1608
