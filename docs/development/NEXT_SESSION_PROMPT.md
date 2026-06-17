# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S253 ✅ · vision **rev 202** · **9** відкритих у §5.12 · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S254** — Galaxy fee_split_applied stand smoke |
| **Відкритих** | **9** (PH-S254…S262) |

---

## Copy-paste — ітераційна сесія (VDT)

```
S0: git fetch; HANDOFF; FM §5.12 (9 відкритих PH-S254…S262); df -h /s

Один PH-S* за сесію з §5.12 (наступний: PH-S254):
- scope лише файли спринту
- cargo fmt --all
- targeted tests (stand smoke / poolai-ui-core / --features enterprise для admin)
- FM §5.12 ✅ + HANDOFF + NEXT + vision (poolai-vision-sync --check)
- MSYS2 commit+push

Після закриття S262 — replenish ≤10 з §5.13 / rg "TODO|FIXME" src/ / Galaxy §8 horizon.
```

---

## Черга §5.12 (replenish 2026-06-17)

| # | Sprint | Scope | Тип |
|---|--------|-------|-----|
| 1 | **PH-S254** | `galaxy_fee_split_applied_total` stand smoke | tests |
| 2 | **PH-S255** | `galaxy_cross_region_egress_mb` stand smoke | tests |
| 3 | **PH-S256** | `galaxy_replay_pending` stand smoke | tests |
| 4 | **PH-S257** | `workers.*` → `workers_patch` (`i18n_core.js` slim) | code/ui |
| 5 | **PH-S258** | `home.*` → `home_patch` | code/ui |
| 6 | **PH-S259** | `form.*` + residual `err.*` slim patch | code/ui |
| 7 | **PH-S260** | shared `ui.*` toolbar glue slim patch | code/ui |
| 8 | **PH-S261** | Docs canon sync (INDEX, STABLE_STATE, docs/README, GALAXY_ROADMAP) | docs |
| 9 | **PH-S262** | `poolai-loc-audit` → `rust_ratio.json` + hold gate note | ops |

**Джерела:** FM §5.3 «не зроблено» · [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md) (gauges без stand smoke) · [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) §4.2/§5.3/§6 · [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md).

**BLOCKED / Deferred (не в черзі):** FM-003 LAN (2 хости) · FM-041 Cloud SDK live.

---

## Acceptance по типах

| Тип | Канон |
|-----|-------|
| **stand smoke** | `poolai-http-stand-smoke` live `/metrics` + unit `ph_sNNN`; pattern PH-S247 |
| **i18n slim** | `poolai-ui-core` patch + inject admin/dashboard; keys out of `i18n_core.js`; audit tests |
| **docs** | FM/HANDOFF/NEXT/INDEX/README/STABLE_STATE sync; vision `--check` |
| **ops** | `cargo run --bin poolai-loc-audit`; оновити `rust_ratio.json` + FM footer |

---

## Закрито (попередня смуга)

PH-S128…S253 ✅ (2026-06-17) — Galaxy stand smoke through S253 + admin i18n slim through S252 + docs S251.
