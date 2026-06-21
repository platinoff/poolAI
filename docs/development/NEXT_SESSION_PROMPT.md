# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **231** · active **PH-S780…S789** · vision **rev 278** · rust_ratio **94.65%**)

| **← наступний** | **`абракадабра`** (drain band 13 PH-S780…S789) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **231** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S780…S789, band 13 Galaxy **§1.2** fee split production) |
| **Після band 13** | promote PH-S790…S799 (band 14 Galaxy governance) |
| **Сесій drain** | **24** (`231÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S780…S789) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 13
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 13: **PH-S790…S799**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 231 → product-complete (PH-S780…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–12 | PH-S660…S779 | ✅ drained |
| 13 | PH-S780…S789 | **активна §5.12** — Galaxy **§1.2** fee split production |
| 14–19 | PH-S790…S849 | Galaxy governance + admin wasm slim + stand smoke v2 |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 13 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S780** | Fee split applied metric production parity | JSON/Prom parity |
| **PH-S781** | Primary/secondary fee hint admin read-only strip | ui-core or grid-pricing extend |
| **PH-S782** | Stand smoke fee-split metrics export | unit test |
| **PH-S783** | `galaxy_fee_split_depth_stub` | unit test |
| **PH-S784** | Bench gate fee-split in BENCHMARKS | docs pointer |
| **PH-S785** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S785 |
| **PH-S786** | Docs concept §1.2 implemented | docs canon |
| **PH-S787** | `poolai-vision-sync --check` | drift gate green |
| **PH-S788** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S789** | `galaxy_horizon_s780_integration` | fee band close |

**Після band 13 promote:** PH-S790…S799 (band 14 — Galaxy governance ops).

---

## Не повторювати

PH-S770…S779 ✅ (payout-batch queue wire, history wasm panel, stand smoke parity, depth stub, horizon close). PH-S760…S769 ✅ locality band. BLOCKED: FM-003 LAN. Deferred: FM-041 Cloud SDK.
