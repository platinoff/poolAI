# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **201** · active **PH-S810…S819** · vision **rev 281** · rust_ratio **94.68%**)

| **← наступний** | **`абракадабра`** (drain band 16 PH-S810…S819) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **201** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S810…S819, band 16 admin wasm slim security/topology) |
| **Після band 16** | promote PH-S820…S829 (band 17 vm/workers/libs) |
| **Сесій drain** | **21** (`201÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S810…S819) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 16
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 16: **PH-S820…S829**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 201 → product-complete (PH-S810…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–15 | PH-S660…S809 | ✅ drained |
| 16 | PH-S810…S819 | **активна §5.12** — admin wasm slim security/topology |
| 17–19 | PH-S820…S859 | admin wasm slim + stand smoke v2 |
| 20–26 | PH-S860…S929 | Job/Memory/Solana + production gates |
| 27–29 | PH-S930…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 16 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S810** | wasm slim security panel glue | secret rotation strip wasm |
| **PH-S811** | wasm slim topology panel glue | topology timestamp wasm |
| **PH-S812** | admin/mod.rs regression PH-S810/S811 | wasm glue tests |
| **PH-S813** | stand smoke security/topology APIs | export shape if applicable |
| **PH-S814** | concept stub security/topology | unit test |
| **PH-S815** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S815 |
| **PH-S816** | Docs canon sync | HANDOFF/NEXT/STABLE |
| **PH-S817** | `poolai-vision-sync --check` | drift gate green |
| **PH-S818** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S819** | `galaxy_horizon_s810_integration` | security/topology wasm band close |

**Після band 16 promote:** PH-S820…S829 (band 17 — admin wasm slim vm/workers/libs).

---

## Не повторювати

PH-S800…S809 ✅ (wasm slim monitoring/payout) · PH-S790…S799 governance ops · bands 1–14 — див. FM §5.12 archive + HANDOFF.
