# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **301** · theme-aware slots · active **PH-S710…S719** · vision **rev 271** · rust_ratio **94.75%**)

| **← наступний** | **`абракадабра`** (drain band 6 PH-S710…S719) |
| **Master backlog** | **301** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM **§5.14** |
| **Активних §5.12** | **10** (PH-S710…S719, band 6 stand smoke) |
| **Після band 6** | promote PH-S720…S729 (band 7 concept wire) |
| **Сесій drain** | **31** (`301÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (зараз PH-S710…S719) — деталі в master backlog band 6
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 6: **PH-S720…S729**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 301 (PH-S710…S1010)

Повний реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s660s1010-351-pending-2026-06-20).

| Band | Sprints | Статус |
|------|---------|--------|
| 1 | PH-S660…S669 | ✅ drained |
| 2 | PH-S670…S679 | ✅ drained |
| 3 | PH-S680…S689 | ✅ drained |
| 4 | PH-S690…S699 | ✅ drained |
| 5 | PH-S700…S709 | ✅ drained |
| 6 | PH-S710…S719 | **активна §5.12** `[ ]` |
| 7–35 | PH-S720…S1009 | queued |
| 36 | PH-S1010 | tail / replenish |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK (BLOCKED/Deferred).

**Band slot map:** slots 1–2 = theme (Stand smoke / wasm slim / …); regen `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 6 — drain зараз)

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-S710** | Stand smoke JSON metric API #1 | export shape scope test |
| **PH-S711** | Stand smoke JSON metric API #2 | export shape scope test |
| **PH-S712** | Admin panel wasm glue | poolai-ui-core + admin JSON/metrics fetch |
| **PH-S713** | Stand smoke runner extend | poolai-http-stand-smoke band APIs |
| **PH-S714** | Galaxy concept helper stub | unit test |
| **PH-S715** | loc-audit → `rust_ratio.json` | sprint zriz PH-S715 |
| **PH-S716** | INDEX / HANDOFF / NEXT / STABLE / GALAXY sync | docs canon |
| **PH-S717** | `poolai-vision-sync --check` | drift gate green |
| **PH-S718** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S719** | `galaxy_horizon_s710_integration` | horizon close + docs |

**Після band 6 promote:** PH-S720…S729 (band 7 — **Concept wire stub**).

---

## Не повторювати

PH-S700…S709 ✅ band 5 drain (2026-06-20): ui-core `render_grid_replication_pricing_panel_html` + wasm; admin_charts wasm-only canvas glue; `admin_wasm_slim_depth_stub`; stand smoke export shape; `galaxy_horizon_s700_integration`; rust_ratio **94.75%** hold advisory.

PH-S690…S699 ✅ band 4 drain (2026-06-20): replication/pricing metrics HTTP; `replication_pricing_depth_stub`; wasm `parsePrometheusGauge` on grid-replication-pricing; stand smoke replication/pricing API; `galaxy_horizon_s690_integration`; rust_ratio **94.67%** hold advisory.
