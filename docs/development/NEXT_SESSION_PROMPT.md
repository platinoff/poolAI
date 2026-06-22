# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-22 (completion v2 · master backlog **71** · active **PH-S940…S949** · vision **rev 294** · rust_ratio **94.88%**)

| **← наступний** | **`абракадабра`** (drain band 29 PH-S940…S949) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **71** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S940…S949, band 29 Ratio 96% stretch e2e scope audit) |
| **Після band 29** | promote PH-S950…S959 (band 30 docs product-complete) |
| **Сесій drain** | **8** (`71÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S940…S949) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 29
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 29: **PH-S950…S959**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 71 → product-complete (PH-S950…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–28 | PH-S660…S939 | ✅ drained |
| 29 | PH-S940…S949 | **активна §5.12** — Ratio 96% stretch e2e scope audit |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 29 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S940** | e2e scope audit — API-only removed | no duplicate Rust tests |
| **PH-S941** | e2e TS loc reduction plan executed | shrink legacy API specs |
| **PH-S942** | ratio 96% stretch spirit check | loc-audit stretch flag |
| **PH-S943** | ops shell audit — no product logic | bin/ vs scripts/ canon |
| **PH-S944** | stretch depth stub | unit test |
| **PH-S945** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S945 |
| **PH-S946** | RUST_RATIO 96% spirit docs | docs |
| **PH-S947** | `poolai-vision-sync --check` | drift gate green |
| **PH-S948** | ratio advisory | stretch note |
| **PH-S949** | `galaxy_horizon_s940_integration` | ratio stretch close |

**Після band 29 promote:** PH-S950…S959 (band 30 — docs product-complete).

---

## Не повторювати

PH-S930…S939 ✅ · admin_common wasm-only table/empty · `mergeRustI18nPatch` · `ratio_95_formal_gate_met` · `ui_js_loc_reduction` · `galaxy_horizon_s930_integration`.
