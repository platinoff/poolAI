# SSO Horizon Close — Enterprise Phase B (Band 70)

Canonical doc: [`SSO_HORIZON.md`](./SSO_HORIZON.md) (band 70, PH-S1345).

## Overview

Band 70 closes **enterprise phase B (SSO)** by aggregating bands 61–69
under one **horizon** gate (`--sso-horizon` / `VERIFY_SSO_HORIZON`).
Phase B delivered depth scaffold, store wire, API contracts, admin/ops, stand smoke,
loc-audit, docs canon, vision-sync, and ratio-advisory.

| Surface | Where | Notes |
|---------|-------|-------|
| Depth module | `sso_horizon_depth.rs` | enum + criteria + `SSO_HORIZON_SLICES` |
| Slice aggregate | all phase-B `--sso*` + `SSO_RATIO_ADVISORY.md` | `--sso` · `--sso-store` · `--sso-api` · `--sso-admin-ops` · `--sso-stand-smoke` · `--sso-loc-audit` · `--sso-docs-canon` · `--sso-vision-sync` · `--sso-ratio-advisory` · `SSO_RATIO_ADVISORY.md` |
| Aggregate | `--sso-horizon` | `sso_horizon_*` fields in `rust_ratio.json` |
| Verify / quick | `VERIFY_SSO_HORIZON` / `--sso-horizon` | phase-B horizon gate |
| Contracts | `sso_horizon_integration` | slices + criteria totals |

**Boundary:** band 70 = phase B horizon close (no new SSO domain API).
Prior ratio-advisory: [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md) (band 69).
Mirror: [`TENANT_HORIZON.md`](./TENANT_HORIZON.md). Next: band 71 **C Audit** depth scaffold (PH-S1349…).

## Loc-audit / verify

```bash
cargo run --bin poolai-loc-audit -- --sso-horizon
cargo run --bin poolai-loc-audit -- --sso-horizon --advisory --min-ratio 0.95
VERIFY_SSO_HORIZON=1 bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh quick --sso-horizon
```

| Field (`rust_ratio.json`) | Meaning |
|---------------------------|---------|
| `sso_horizon_mode` | `true` when `--sso-horizon` (PH-S1344) |
| `sso_horizon_criteria_total` | Registry size (10) |
| `sso_horizon_criteria_met_count` | Markers found in canonical paths |

## Module

- ui-core: [`sso_horizon_depth.rs`](../../crates/poolai-ui-core/src/sso_horizon_depth.rs)
- loc-audit: [`poolai_loc_audit.rs`](../../src/bin/poolai_loc_audit.rs)
- tests: `sso_horizon_integration.rs`, `galaxy_horizon_s1339_integration.rs`
- stand-smoke export shape: `sso_horizon_band70_export_shape_ph_s1343`
- prior: [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md) · [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md) · [`SSO_STORE.md`](./SSO_STORE.md)
- roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md)

## Related

- FM §5.17 enterprise 100% · §5.51 band 70 journal
- Slices: `--sso` · `--sso-store` · `--sso-api` · `--sso-admin-ops` · `--sso-stand-smoke` · `--sso-loc-audit` · `--sso-docs-canon` · `--sso-vision-sync` · `--sso-ratio-advisory` · `SSO_RATIO_ADVISORY.md`
- PH-S1339 · SSO_HORIZON_SLICES · sso_horizon_integration · VERIFY_SSO_HORIZON · PH-S1344 · --sso-horizon · PH-S1348
