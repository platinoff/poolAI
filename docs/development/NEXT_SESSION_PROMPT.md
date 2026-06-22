# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-22 (completion v2 · master backlog **91** · active **PH-S920…S929** · vision **rev 292** · rust_ratio **94.78%**)

| **← наступний** | **`абракадабра`** (drain band 27 PH-S920…S929) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **91** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S920…S929, band 27 wasm admin_charts migration) |
| **Після band 27** | promote PH-S930…S939 (band 28 Ratio 95% gate admin_common slim) |
| **Сесій drain** | **10** (`91÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S920…S929) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 27
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 27: **PH-S930…S939**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 91 → product-complete (PH-S930…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–26 | PH-S660…S919 | ✅ drained |
| 27 | PH-S920…S929 | **активна §5.12** — wasm admin_charts migration |
| 28–29 | PH-S930…S969 | Ratio **95–96%** |
| 30–33 | PH-S970…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 27 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S920** | admin_charts ML sparkline → wasm | render_sparkline_html wasm-only |
| **PH-S921** | admin_charts line chart → wasm | render_line_chart_html wasm-only |
| **PH-S922** | admin_charts regression tests | mod.rs PH-S920/S921 |
| **PH-S923** | build-ui-wasm.sh gate in drain doc | bin verify |
| **PH-S924** | charts depth stub | unit test |
| **PH-S925** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S925 |
| **PH-S926** | RUST_RATIO §5.13 charts row | docs |
| **PH-S927** | `poolai-vision-sync --check` | drift gate green |
| **PH-S928** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S929** | `galaxy_horizon_s920_integration` | charts wasm band close |

**Після band 27 promote:** PH-S930…S939 (band 28 — Ratio 95% gate admin_common slim).

---

## Не повторювати

Закриті PH-S910…S919 (trust SQLite persist band 26) — не re-open без нового FM-*.
