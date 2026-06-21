# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **151** · active **PH-S860…S869** · vision **rev 286** · rust_ratio **94.71%**)

| **← наступний** | **`абракадабра`** (drain band 21 PH-S860…S869) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **151** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S860…S869, band 21 Memory shard persist) |
| **Після band 21** | promote PH-S870…S879 (band 22 Solana on-chain cleared) |
| **Сесій drain** | **16** (`151÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S860…S869) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 21
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 21: **PH-S870…S879**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 151 → product-complete (PH-S870…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–20 | PH-S660…S859 | ✅ drained |
| 21 | PH-S860…S869 | **активна §5.12** — Memory shard persist + seed inventory |
| 22–23 | PH-S870…S889 | Solana on-chain cleared + production gates |
| 24–26 | PH-S890…S929 | Verification/replication/pricing/trust gates |
| 27–29 | PH-S930…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 21 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S860** | memory shard persist stub | MemoryShardStore persist + test |
| **PH-S861** | seed-inventory HTTP depth | GET /grid/seed-inventory extend |
| **PH-S862** | admin memory/seed wasm glue | ui-core helper |
| **PH-S863** | stand smoke seed-inventory API | runner case |
| **PH-S864** | memory layer depth stub | unit test |
| **PH-S865** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S865 |
| **PH-S866** | POOLAI_MEMORY_LAYER.md sync | docs |
| **PH-S867** | `poolai-vision-sync --check` | drift gate green |
| **PH-S868** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S869** | `galaxy_horizon_s860_integration` | Memory band close |

**Після band 21 promote:** PH-S870…S879 (band 22 — Solana on-chain cleared depth).

---

## Не повторювати

PH-S850…S859 ✅ (Job store RAID band) · PH-S840…S849 ✅ (OpenAPI gap 0 + contract band) · PH-S830…S839 ✅ (stand smoke v2) · bands 1–19 — див. FM §5.12 archive + HANDOFF.
