# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-18 (PH-S990…S999 ✅ band 34 · master backlog **11** · active **PH-S1000…S1009** · vision **rev 304** · rust_ratio **94.94%**)

| **← наступний** | **`абракадабра`** (drain band 35 PH-S1000…S1009) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **11** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S1000…S1009, band 35 final multi-module horizon) |
| **Після band 35** | promote PH-S1010 (band 36 product-complete) |
| **Сесій drain** | **2** (`11÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S1000…S1009) — acceptance у [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) band 35
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** PH-S1010 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 35)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 11 → product-complete (PH-S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–34 | PH-S660…S999 | ✅ drained |
| 35 | PH-S1000…S1009 | **активна §5.12** — final multi-module horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 35 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S1000** | multi-module wire smoke harness | top 5 grid APIs one test |
| **PH-S1001** | multi-module admin wasm regression | ui-core full test gate |
| **PH-S1002** | multi-module stand smoke full suite | bin --json all green |
| **PH-S1003** | cargo test-ci scope note final | HANDOFF |
| **PH-S1004** | openapi-gap + test-ci dual gate doc | FM |
| **PH-S1005** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S1005 |
| **PH-S1006** | vision manifest final sprint_queue | poolai-vision-sync |
| **PH-S1007** | `poolai-vision-sync --check` | drift gate green |
| **PH-S1008** | ratio advisory | final pre-S1010 hold |
| **PH-S1009** | `galaxy_horizon_s1000_integration` | final code band close |

**Після band 35 promote:** PH-S1010 (band 36 — product-complete closure).

---

## Не повторювати

Band 34 (PH-S990…S999) ✅ — `integration_gap_audit.rs`, `telegram_wallet_integration.rs`, archived API-smoke canon.
