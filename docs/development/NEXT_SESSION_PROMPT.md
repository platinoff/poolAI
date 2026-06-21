# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **211** · active **PH-S800…S809** · vision **rev 280** · rust_ratio **94.69%**)

| **← наступний** | **`абракадабра`** (drain band 15 PH-S800…S809) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **211** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S800…S809, band 15 admin wasm slim) |
| **Після band 15** | promote PH-S810…S819 (band 16 security/topology wasm) |
| **Сесій drain** | **22** (`211÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S800…S809) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 15
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 15: **PH-S810…S819**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 211 → product-complete (PH-S800…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–14 | PH-S660…S799 | ✅ drained |
| 15 | PH-S800…S809 | **активна §5.12** — admin wasm slim monitoring/payout |
| 16–19 | PH-S810…S849 | admin wasm slim + stand smoke v2 |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 15 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S800** | wasm slim monitoring ML panel | `poolaiRenderMlPipelineMetricsPanel` wasm-only |
| **PH-S801** | wasm slim payout-batch panel | ui-core → wasm export |
| **PH-S802** | admin/mod.rs regression PH-S800/S801 | `parsePrometheusGauge` tests |
| **PH-S803** | stand smoke monitoring/payout APIs | runner shape tests |
| **PH-S804** | admin wasm slim depth stub extend | unit test |
| **PH-S805** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S805 |
| **PH-S806** | Docs canon sync | HANDOFF/NEXT/STABLE |
| **PH-S807** | `poolai-vision-sync --check` | drift gate green |
| **PH-S808** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S809** | `galaxy_horizon_s800_integration` | wasm monitoring band close |

**Після band 15 promote:** PH-S810…S819 (band 16 — admin wasm slim security/topology).

---

## Не повторювати

PH-S790…S799 ✅ (Galaxy governance band). PH-S780…S789 ✅ (fee split production band). BLOCKED: PH-S02/S16/S35 LAN. Deferred: PH-S01/S15/S36 Cloud SDK.
