# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-18 (PH-S970…S979 ✅ band 32 · master backlog **31** · active **PH-S980…S989** · vision **rev 302** · rust_ratio **94.92%**)

| **← наступний** | **`абракадабра`** (drain band 33 PH-S980…S989) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **31** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S980…S989, band 33 STABLE + INDEX product-complete) |
| **Після band 33** | promote PH-S990…S999 (band 34 integration gap fill) |
| **Сесій drain** | **4** (`31÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S980…S989) — acceptance у [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) band 33
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 33: **PH-S990…S999**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 31 → product-complete (PH-S990…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–32 | PH-S660…S979 | ✅ drained |
| 33 | PH-S980…S989 | **активна §5.12** — STABLE + INDEX product-complete |
| 34–35 | PH-S990…S1009 | Integration gap + final multi-module horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 33 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S980** | STABLE_STATE product-complete draft | development complete section |
| **PH-S981** | INDEX product-complete zriz | step 1–12 final |
| **PH-S982** | README Next Focus → maintenance | root README |
| **PH-S983** | HANDOFF maintenance mode template | post-S1010 prep |
| **PH-S984** | DEVELOPMENT_PROGRESS 100% code scope | honest scope note |
| **PH-S985** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S985 |
| **PH-S986** | FM §5.15 draft product-complete | FM catalog |
| **PH-S987** | `poolai-vision-sync --check` | drift gate green |
| **PH-S988** | ratio advisory | hold |
| **PH-S989** | `galaxy_horizon_s980_integration` | STABLE band close |

**Після band 33 promote:** PH-S990…S999 (band 34 — integration gap fill).

---

## Не повторювати

PH-S970…S979 (band 32 Galaxy concept markers) · PH-S960…S969 (band 31 DOCS_LEGACY) · BLOCKED LAN · Deferred Cloud SDK.
