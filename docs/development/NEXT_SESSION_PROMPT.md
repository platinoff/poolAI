# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-17 (PH-S960…S969 ✅ band 31 · master backlog **41** · active **PH-S970…S979** · vision **rev 300** · rust_ratio **94.92%**)

| **← наступний** | **`абракадабра`** (drain band 32 PH-S970…S979) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **41** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S970…S979, band 32 Galaxy concept implemented markers) |
| **Після band 32** | promote PH-S980…S989 (band 33 STABLE + INDEX product-complete) |
| **Сесій drain** | **5** (`41÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S970…S979) — acceptance у [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) band 32
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 32: **PH-S980…S989**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 41 → product-complete (PH-S980…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–31 | PH-S660…S969 | ✅ drained |
| 32 | PH-S970…S979 | **активна §5.12** — Galaxy concept implemented markers |
| 33–34 | PH-S980…S999 | STABLE + INDEX product-complete + integration gap |
| 35 | PH-S1000…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 32 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S970** | Galaxy §1–3 implemented markers | POOLAI_GALAXY_GRID.md |
| **PH-S971** | Galaxy §4–6 implemented markers | same |
| **PH-S972** | Galaxy §7–9 implemented markers | same |
| **PH-S973** | §8 TBD closed or BLOCKED noted | §8.2 payout ✅; LAN blocked |
| **PH-S974** | GALAXY_GRID_ROADMAP horizon table final | all rows ✅ or BLOCKED |
| **PH-S975** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S975 |
| **PH-S976** | concept cross-links INDEX | docs |
| **PH-S977** | `poolai-vision-sync --check` | drift gate green |
| **PH-S978** | ratio advisory | hold |
| **PH-S979** | `galaxy_horizon_s970_integration` | concept markers close |

**Після band 32 promote:** PH-S980…S989 (band 33 — STABLE + INDEX product-complete).

---

## Не повторювати

PH-S960…S969 (band 31 DOCS_LEGACY) · PH-S950…S959 (band 30 DIGEST) · BLOCKED LAN · Deferred Cloud SDK.
