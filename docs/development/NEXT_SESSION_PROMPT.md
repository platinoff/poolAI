# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-20 (completion v2 · master backlog **261** · active **PH-S750…S759** · vision **rev 275** · rust_ratio **94.59%**)

| **← наступний** | **`абракадабра`** (drain band 10 PH-S750…S759) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **261** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S750…S759, band 10 Galaxy **§5.5** prefetch live pull depth) |
| **Після band 10** | promote PH-S760…S769 (band 11 Galaxy locality/hot-tier) |
| **Сесій drain** | **27** (`261÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S750…S759) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 10
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 10: **PH-S760…S769**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 261 → product-complete (PH-S750…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–9 | PH-S660…S749 | ✅ drained |
| 10 | PH-S750…S759 | **активна §5.12** — Galaxy **§5.5** prefetch live pull depth |
| 11–14 | PH-S760…S799 | Galaxy depth (locality, payout, fees, governance) |
| 15–19 | PH-S800…S849 | Admin wasm slim + stand smoke v2 + OpenAPI |
| 20–26 | PH-S850…S919 | Job/Memory/Solana + production gates |
| 27–29 | PH-S920…S949 | Ratio **95–96%** |
| 30–33 | PH-S950…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 10 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S750** | Prefetch live bytes pull metric wire | `galaxy_prefetch_pull_bytes_total` parity JSON/Prom |
| **PH-S751** | Prefetch backpressure from profile bandwidth | unit + integration test |
| **PH-S752** | Admin prefetch metrics wasm strip | ui-core helper / metrics fetch glue |
| **PH-S753** | Stand smoke prefetch-metrics API shape | runner + unit test |
| **PH-S754** | `prefetch_depth_stub` band theme | unit test |
| **PH-S755** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S755 |
| **PH-S756** | GALAXY §5.5 implemented table refresh | docs canon |
| **PH-S757** | `poolai-vision-sync --check` | drift gate green |
| **PH-S758** | ratio advisory | `--min-ratio 0.95 --advisory` |
| **PH-S759** | `galaxy_horizon_s750_integration` | prefetch band close |

**Після band 10 promote:** PH-S760…S769 (band 11 — Galaxy locality/hot-tier).

---

## Не повторювати (закрито)

PH-S740…S749 ✅ · band 9 signed capability admission · `galaxy_horizon_s740_integration` · rust_ratio **94.59%** advisory hold.

Band 8 PH-S730…S739 ✅ network_profile persist · band 7 PH-S720…S729 ✅ routing/re-migrate · bands 1–6 ✅.

BLOCKED: PH-S02/S16/S35 LAN · Deferred: PH-S01/S15/S36 Cloud SDK.
