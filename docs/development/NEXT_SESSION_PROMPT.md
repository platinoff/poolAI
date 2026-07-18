# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-18 (PH-S980…S989 ✅ band 33 · master backlog **21** · active **PH-S990…S999** · vision **rev 303** · rust_ratio **94.92%**)

| **← наступний** | **`абракадабра`** (drain band 34 PH-S990…S999) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **21** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S990…S999, band 34 integration gap fill) |
| **Після band 34** | promote PH-S1000…S1009 (band 35 final multi-module horizon) |
| **Сесій drain** | **3** (`21÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S990…S999) — acceptance у [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) band 34
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 34: **PH-S1000…S1009**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 21 → product-complete (PH-S1000…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–33 | PH-S660…S989 | ✅ drained |
| 34 | PH-S990…S999 | **активна §5.12** — integration gap fill |
| 35 | PH-S1000…S1009 | Final multi-module horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 34 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S990** | integration gap: telegram wallet | tests/* if missing |
| **PH-S991** | integration gap: grid job lease | extend if gap |
| **PH-S992** | integration gap: protocol middleware | extend if gap |
| **PH-S993** | integration gap: jobs raid restart | extend if gap |
| **PH-S994** | integration gap: vm write lifecycle | extend if gap |
| **PH-S995** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S995 |
| **PH-S996** | `poolai-testing-policy` gap note | docs |
| **PH-S997** | `poolai-vision-sync --check` | drift gate green |
| **PH-S998** | ratio advisory | hold |
| **PH-S999** | `galaxy_horizon_s990_integration` | integration gap close |

**Після band 34 promote:** PH-S1000…S1009 (band 35 — final multi-module horizon).

---

## Не повторювати

PH-S980…S989 (band 33 STABLE product-complete) · PH-S970…S979 (band 32 Galaxy concept markers) · BLOCKED LAN · Deferred Cloud SDK.
