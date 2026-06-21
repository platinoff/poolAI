# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **141** · active **PH-S870…S879** · vision **rev 287** · rust_ratio **94.70%**)

| **← наступний** | **`абракадабра`** (drain band 22 PH-S870…S879) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **141** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S870…S879, band 22 Solana on-chain cleared) |
| **Після band 22** | promote PH-S880…S889 (band 23 Verification checker lifecycle) |
| **Сесій drain** | **15** (`141÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S870…S879) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 22
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 22: **PH-S880…S889**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 141 → product-complete (PH-S880…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–21 | PH-S660…S869 | ✅ drained |
| 22 | PH-S870…S879 | **активна §5.12** — Solana on-chain cleared depth |
| 23–26 | PH-S880…S919 | Verification/replication/pricing/trust gates |
| 27–29 | PH-S920…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 22 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S870** | on-chain cleared mock RPC depth | POOLAI_SETTLEMENT_ON_CHAIN test |
| **PH-S871** | solana-adapter event schema v1 | crate test |
| **PH-S872** | job onchain events NDJSON persist | domain_events test |
| **PH-S873** | stand smoke on-chain metrics if exposed | runner case |
| **PH-S874** | solana depth stub | unit test |
| **PH-S875** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S875 |
| **PH-S876** | SOLANA_ADAPTER_CONCEPT sync | docs |
| **PH-S877** | `poolai-vision-sync --check` | drift gate green |
| **PH-S878** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S879** | `galaxy_horizon_s870_integration` | Solana band close |

**Після band 22 promote:** PH-S880…S889 (band 23 — Verification checker lifecycle).

---

## Не повторювати

Band 21 PH-S860…S869 ✅ — memory persist, seed-inventory depth, wasm meta strip, `galaxy_horizon_s860_integration`.
