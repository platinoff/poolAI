# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (completion v2 · master backlog **251** · active **PH-S760…S769** · vision **rev 276** · rust_ratio **94.62%**)

| **← наступний** | **`абракадабра`** (drain band 11 PH-S760…S769) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **251** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S760…S769, band 11 Galaxy **§5.2–5.4** locality/hot-tier depth) |
| **Після band 11** | promote PH-S770…S779 (band 12 Galaxy §8.2 payout/settlement batch) |
| **Сесій drain** | **26** (`251÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S760…S769) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 11
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 11: **PH-S770…S779**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 251 → product-complete (PH-S760…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–10 | PH-S660…S759 | ✅ drained |
| 11 | PH-S760…S769 | **активна §5.12** — Galaxy **§5.2–5.4** locality/hot-tier depth |
| 12–14 | PH-S770…S799 | Galaxy depth (payout, fees, governance) |
| 15–19 | PH-S800…S849 | Admin wasm slim + stand smoke v2 + OpenAPI |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 11 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S760** | Locality-metrics HTTP wire depth | integration test |
| **PH-S761** | Hot-tier promote/evict metrics parity | JSON/Prom parity |
| **PH-S762** | Admin locality wasm glue | ui-core metrics strip |
| **PH-S763** | Stand smoke locality/prefetch band | runner extend |
| **PH-S764** | `locality_hot_tier_depth_stub` | unit test |
| **PH-S765** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S765 |
| **PH-S766** | Docs INDEX §7 canon sync | docs canon |
| **PH-S767** | `poolai-vision-sync --check` | drift gate green |
| **PH-S768** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S769** | `galaxy_horizon_s760_integration` | locality band close |

**Після band 11 promote:** PH-S770…S779 (band 12 — Galaxy §8.2 payout/settlement batch).

---

## Не повторювати

PH-S750…S759 ✅ · PH-S740…S749 ✅ · band 6–9 stand smoke parity baselines · signed capability gate (PH-S740) · prefetch-metrics API (PH-S750).
