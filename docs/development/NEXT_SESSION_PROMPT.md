# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **171** · active **PH-S840…S849** · vision **rev 284** · rust_ratio **94.70%**)

| **← наступний** | **`абракадабра`** (drain band 19 PH-S840…S849) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **171** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S840…S849, band 19 OpenAPI gap) |
| **Після band 19** | promote PH-S850…S859 (band 20 Job store RAID) |
| **Сесій drain** | **18** (`171÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S840…S849) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 19
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 19: **PH-S850…S859**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 171 → product-complete (PH-S840…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–18 | PH-S660…S839 | ✅ drained |
| 19 | PH-S840…S849 | **активна §5.12** — OpenAPI gap 0 + contract band |
| 20–22 | PH-S850…S879 | Job/Memory/Solana + production gates |
| 23–26 | PH-S880…S929 | Verification/replication/pricing/trust gates |
| 27–29 | PH-S930…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 19 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S840** | openapi.yaml sync band APIs | routes match grid.rs |
| **PH-S841** | poolai-openapi-gap-audit 0 | CI gate green |
| **PH-S842** | contract test band top routes | tests/*_contracts.rs extend |
| **PH-S843** | stand smoke OpenAPI path smoke | key paths 200 shape |
| **PH-S844** | OpenAPI examples for grid metrics | yaml examples |
| **PH-S845** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S845 |
| **PH-S846** | OPENAPI_GAP_AUDIT doc sync | docs |
| **PH-S847** | `poolai-vision-sync --check` | drift gate green |
| **PH-S848** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S849** | `galaxy_horizon_s840_integration` | OpenAPI band close |

**Після band 19 promote:** PH-S850…S859 (band 20 — Job store RAID production path).

---

## Не повторювати

PH-S830…S839 ✅ (stand smoke v2 full grid parity) · PH-S820…S829 wasm vm/workers/libs · bands 1–17 — див. FM §5.12 archive + HANDOFF.
