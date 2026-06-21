# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **131** · active **PH-S880…S889** · vision **rev 288** · rust_ratio **94.71%**)

| **← наступний** | **`абракадабра`** (drain band 23 PH-S880…S889) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **131** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S880…S889, band 23 Verification checker lifecycle) |
| **Після band 23** | promote PH-S890…S899 (band 24 Replication quorum production gates) |
| **Сесій drain** | **14** (`131÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S880…S889) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 23
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 23: **PH-S890…S899**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 131 → product-complete (PH-S890…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–22 | PH-S660…S879 | ✅ drained |
| 23 | PH-S880…S889 | **активна §5.12** — Verification checker lifecycle |
| 24–26 | PH-S890…S919 | Replication/pricing/trust gates |
| 27–29 | PH-S920…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 23 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S880** | checker task drain lifecycle | PH-S495 extend integration |
| **PH-S881** | checker shadow job submit depth | integration test |
| **PH-S882** | admin grid-verification wasm complete | metrics+tasks strip |
| **PH-S883** | stand smoke verification-checker/tasks | runner |
| **PH-S884** | verification lifecycle depth stub | unit test |
| **PH-S885** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S885 |
| **PH-S886** | Galaxy §6.2 implemented table | docs |
| **PH-S887** | `poolai-vision-sync --check` | drift gate green |
| **PH-S888** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S889** | `galaxy_horizon_s880_integration` | verification band close |

**Після band 23 promote:** PH-S890…S899 (band 24 — Replication quorum production gates).

---

## Не повторювати

Band 22 PH-S870…S879 ✅ — on-chain cleared mock RPC depth, solana-adapter schema v1, NDJSON persist, `galaxy_horizon_s870_integration`.
