# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **181** · active **PH-S830…S839** · vision **rev 283** · rust_ratio **94.68%**)

| **← наступний** | **`абракадабра`** (drain band 18 PH-S830…S839) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **181** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S830…S839, band 18 stand smoke v2) |
| **Після band 18** | promote PH-S840…S849 (band 19 OpenAPI gap) |
| **Сесій drain** | **19** (`181÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S830…S839) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 18
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 18: **PH-S840…S849**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 181 → product-complete (PH-S830…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–17 | PH-S660…S829 | ✅ drained |
| 18 | PH-S830…S839 | **активна §5.12** — stand smoke v2 full grid parity |
| 19–20 | PH-S840…S869 | OpenAPI gap + contract band |
| 21–26 | PH-S870…S929 | Job/Memory/Solana + production gates |
| 27–29 | PH-S930…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 18 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S830** | stand_smoke_metrics_parity all 6 APIs | validate_band6 extend v2 |
| **PH-S831** | stand smoke prefetch/locality parity | JSON↔Prom unit tests |
| **PH-S832** | stand smoke governance/fee parity | unit tests |
| **PH-S833** | live runner grid_metrics_json_prometheus_parity | stand smoke case green |
| **PH-S834** | stand smoke export shape regression suite | bin unit tests |
| **PH-S835** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S835 |
| **PH-S836** | PROMETHEUS_METRICS.md stand smoke sync | docs |
| **PH-S837** | `poolai-vision-sync --check` | drift gate green |
| **PH-S838** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S839** | `galaxy_horizon_s830_integration` | stand smoke v2 band close |

**Після band 18 promote:** PH-S840…S849 (band 19 — OpenAPI gap 0 + contract band).

---

## Не повторювати

PH-S820…S829 ✅ (wasm slim vm/workers/libs) · PH-S810…S819 security/topology · bands 1–16 — див. FM §5.12 archive + HANDOFF.
