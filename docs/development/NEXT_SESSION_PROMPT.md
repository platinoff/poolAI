# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **191** · active **PH-S820…S829** · vision **rev 282** · rust_ratio **94.67%**)

| **← наступний** | **`абракадабра`** (drain band 17 PH-S820…S829) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **191** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S820…S829, band 17 admin wasm slim vm/workers/libs) |
| **Після band 17** | promote PH-S830…S839 (band 18 stand smoke v2) |
| **Сесій drain** | **20** (`191÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S820…S829) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 17
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 17: **PH-S830…S839**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 191 → product-complete (PH-S820…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–16 | PH-S660…S819 | ✅ drained |
| 17 | PH-S820…S829 | **активна §5.12** — admin wasm slim vm/workers/libs |
| 18–19 | PH-S830…S859 | stand smoke v2 + OpenAPI gap |
| 20–26 | PH-S860…S929 | Job/Memory/Solana + production gates |
| 27–29 | PH-S930…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 17 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S820** | wasm slim vm panel glue | vm admin wasm render |
| **PH-S821** | wasm slim workers/libs panels | ui-core → wasm |
| **PH-S822** | admin/mod.rs regression PH-S820/S821 | wasm glue tests |
| **PH-S823** | stand smoke vm/workers API shape | runner tests |
| **PH-S824** | concept stub vm/workers DTO | unit test |
| **PH-S825** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S825 |
| **PH-S826** | Docs canon sync | HANDOFF/NEXT/STABLE |
| **PH-S827** | `poolai-vision-sync --check` | drift gate green |
| **PH-S828** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S829** | `galaxy_horizon_s820_integration` | vm/workers wasm band close |

**Після band 17 promote:** PH-S830…S839 (band 18 — stand smoke v2 full grid parity).

---

## Не повторювати

PH-S810…S819 ✅ (wasm slim security/topology) · PH-S800…S809 monitoring/payout · bands 1–15 — див. FM §5.12 archive + HANDOFF.
