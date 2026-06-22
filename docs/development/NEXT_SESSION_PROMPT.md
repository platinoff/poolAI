# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-22 (completion v2 · master backlog **81** · active **PH-S930…S939** · vision **rev 293** · rust_ratio **94.80%**)

| **← наступний** | **`абракадабра`** (drain band 28 PH-S930…S939) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **81** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S930…S939, band 28 Ratio 95% gate admin_common slim) |
| **Після band 28** | promote PH-S940…S949 (band 29 Ratio 96% stretch e2e scope audit) |
| **Сесій drain** | **9** (`81÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S930…S939) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 28
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 28: **PH-S940…S949**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 81 → product-complete (PH-S940…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–27 | PH-S660…S929 | ✅ drained |
| 28 | PH-S930…S939 | **активна §5.12** — Ratio 95% gate admin_common slim |
| 29–30 | PH-S940…S979 | Ratio **96%** stretch + docs |
| 31–33 | PH-S980…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 28 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S930** | admin_common.js table init slim | delegate to ui-core where possible |
| **PH-S931** | admin_common.js empty state slim | wasm/html from ui-core |
| **PH-S932** | i18n_core.js audit — no duplicate logic | rg audit + fix |
| **PH-S933** | ratio 95% gate test | rust_ratio ≥ 0.95 or advisory documented |
| **PH-S934** | ui JS loc reduction stub metric | loc-audit by_category ui_js down |
| **PH-S935** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S935 |
| **PH-S936** | RUST_RATIO §5.13 band 28 note | docs |
| **PH-S937** | `poolai-vision-sync --check` | drift gate green |
| **PH-S938** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S939** | `galaxy_horizon_s930_integration` | ratio 95% band close |

**Після band 28 promote:** PH-S940…S949 (band 29 — Ratio 96% stretch e2e scope audit).

---

## Не повторювати

PH-S920…S929 ✅ (admin_charts wasm-only sparkline/line, charts_depth_stub, horizon close). PH-S910…S919 ✅ trust persist band. Див. FM §5.12 журнал.
