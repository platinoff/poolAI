# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-21 (completion v2 · master backlog **161** · active **PH-S850…S859** · vision **rev 285** · rust_ratio **94.71%**)

| **← наступний** | **`абракадабра`** (drain band 20 PH-S850…S859) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **161** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S850…S859, band 20 Job store RAID) |
| **Після band 20** | promote PH-S860…S869 (band 21 Memory shard persist) |
| **Сесій drain** | **17** (`161÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S850…S859) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 20
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 20: **PH-S860…S869**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 161 → product-complete (PH-S860…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–19 | PH-S660…S849 | ✅ drained |
| 20 | PH-S850…S859 | **активна §5.12** — Job store RAID production path |
| 21–22 | PH-S860…S879 | Memory/Solana + production gates |
| 23–26 | PH-S880…S929 | Verification/replication/pricing/trust gates |
| 27–29 | PH-S930…S959 | Ratio **95–96%** |
| 30–33 | PH-S960…S999 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 20 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S850** | job store RAID restart persistence | integration test like PH-S52 |
| **PH-S851** | verify-dev-stand RAID jobs path | bin script green |
| **PH-S852** | admin jobs store_backend badge wire | UI wasm glue |
| **PH-S853** | stand smoke jobs store_backend | runner case |
| **PH-S854** | job store depth stub | unit test |
| **PH-S855** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S855 |
| **PH-S856** | RUN_LOCAL.md RAID jobs preset | docs |
| **PH-S857** | `poolai-vision-sync --check` | drift gate green |
| **PH-S858** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S859** | `galaxy_horizon_s850_integration` | Job RAID band close |

**Після band 20 promote:** PH-S860…S869 (band 21 — Memory shard persist + seed inventory).

---

## Не повторювати

PH-S840…S849 ✅ (OpenAPI gap 0 + contract band) · PH-S830…S839 ✅ (stand smoke v2) · bands 1–18 — див. FM §5.12 archive + HANDOFF.
