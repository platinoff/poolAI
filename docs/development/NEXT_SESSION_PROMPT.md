# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (master backlog **291** · theme-aware slots · active **PH-S720…S729** · vision **rev 272** · rust_ratio **94.76%**)

| **← наступний** | **`абракадабра`** (drain band 7 PH-S720…S729) |
| **Master backlog** | **291** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM **§5.14** |
| **Активних §5.12** | **10** (PH-S720…S729, band 7 concept wire) |
| **Після band 7** | promote PH-S730…S739 (band 8 ops loc-audit) |
| **Сесій drain** | **30** (`291÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (зараз PH-S720…S729) — деталі в master backlog band 7
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 7: **PH-S730…S739**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 291 (PH-S720…S1010)

Повний реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s660s1010-351-pending-2026-06-20).

| Band | Sprints | Статус |
|------|---------|--------|
| 1 | PH-S660…S669 | ✅ drained |
| 2 | PH-S670…S679 | ✅ drained |
| 3 | PH-S680…S689 | ✅ drained |
| 4 | PH-S690…S699 | ✅ drained |
| 5 | PH-S700…S709 | ✅ drained |
| 6 | PH-S710…S719 | ✅ drained |
| 7 | PH-S720…S729 | **активна §5.12** `[ ]` |
| 8–35 | PH-S730…S1009 | queued |
| 36 | PH-S1010 | tail / replenish |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK (BLOCKED/Deferred).

**Band slot map:** slots 1–2 = theme (Concept wire stub / …); regen `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 7 — drain зараз)

| Sprint | Фокус | Acceptance |
|--------|--------|------------|
| **PH-S720** | Galaxy concept helper stub #1 | scope test green |
| **PH-S721** | Galaxy concept helper stub #2 | scope test green |
| **PH-S722** | Admin panel wasm glue | poolai-ui-core + admin JSON/metrics fetch |
| **PH-S723** | Stand smoke /metrics export | poolai-http-stand-smoke JSON metric API |
| **PH-S724** | Galaxy concept helper stub | unit test |
| **PH-S725** | loc-audit → `rust_ratio.json` | sprint zriz PH-S725 |
| **PH-S726** | INDEX / HANDOFF / NEXT / STABLE / GALAXY sync | docs canon |
| **PH-S727** | `poolai-vision-sync --check` | drift gate green |
| **PH-S728** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S729** | `galaxy_horizon_s720_integration` | horizon close + docs |

**Після band 7 promote:** PH-S730…S739 (band 8 — **Ops loc-audit + ratio advisory**).

---

## Не повторювати

PH-S710…S719 ✅ band 6 drain (2026-06-20): `stand_smoke_metrics_parity` JSON↔Prometheus; ui-core `stand_smoke_metrics` + verification admin wasm strip; stand smoke band6 parity runner; `galaxy_horizon_s710_integration`; rust_ratio **94.76%** hold advisory.

PH-S700…S709 ✅ band 5 drain (2026-06-20): ui-core `render_grid_replication_pricing_panel_html` + wasm; admin_charts wasm-only canvas glue; `admin_wasm_slim_depth_stub`; stand smoke export shape; `galaxy_horizon_s700_integration`; rust_ratio **94.75%** hold advisory.

PH-S690…S699 ✅ band 4 drain (2026-06-20): replication/pricing metrics HTTP; `replication_pricing_depth_stub`; wasm `parsePrometheusGauge` on grid-replication-pricing; stand smoke replication/pricing API; `galaxy_horizon_s690_integration`; rust_ratio **94.67%** hold advisory.
