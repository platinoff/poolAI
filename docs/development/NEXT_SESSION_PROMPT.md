# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **241** · active **PH-S770…S779** · vision **rev 277** · rust_ratio **94.63%**)

| **← наступний** | **`абракадабра`** (drain band 12 PH-S770…S779) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **241** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S770…S779, band 12 Galaxy **§8.2** payout/settlement batch) |
| **Після band 12** | promote PH-S780…S789 (band 13 Galaxy depth) |
| **Сесій drain** | **25** (`241÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S770…S779) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 12
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 12: **PH-S780…S789**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 241 → product-complete (PH-S770…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–11 | PH-S660…S769 | ✅ drained |
| 12 | PH-S770…S779 | **активна §5.12** — Galaxy **§8.2** payout/settlement batch |
| 13–14 | PH-S780…S799 | Galaxy depth (payout, fees, governance) |
| 15–19 | PH-S800…S849 | Admin wasm slim + stand smoke v2 + OpenAPI |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 12 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S770** | Offline payout batch settlement wire depth | cleared → batch queue stub + metric |
| **PH-S771** | Payout batch history admin wasm panel | ui-core render + fetch |
| **PH-S772** | Stand smoke payout-batch/history API | runner green |
| **PH-S773** | `settlement_payout_depth_stub` | Galaxy §8.2 unit test |
| **PH-S774** | On-chain vs offline mode gate doc stub | `galaxy_settlement_mode` test extend |
| **PH-S775** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S775 |
| **PH-S776** | Docs Galaxy §8.2 payout row | docs canon |
| **PH-S777** | `poolai-vision-sync --check` | drift gate green |
| **PH-S778** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S779** | `galaxy_horizon_s770_integration` | payout band close |

**Після band 12 promote:** PH-S780…S789 (band 13 — Galaxy depth continuation).

---

## Не повторювати

PH-S760…S769 ✅ (locality-metrics HTTP, hot-tier parity, admin wasm strip, stand smoke, depth stub, horizon close). PH-S750…S759 ✅ prefetch band. BLOCKED: FM-003 LAN. Deferred: FM-041 Cloud SDK.
