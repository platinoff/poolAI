# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-17 (PH-S950…S959 ✅ band 30 · master backlog **51** · active **PH-S960…S969** · vision **rev 298** · rust_ratio **94.91%**)

| **← наступний** | **`абракадабра`** (drain band 31 PH-S960…S969) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **51** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S960…S969, band 31 DOCS_LEGACY audit close) |
| **Після band 31** | promote PH-S970…S979 (band 32 Galaxy concept markers) |
| **Сесій drain** | **6** (`51÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S960…S969) — acceptance у [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) band 31
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 31: **PH-S970…S979**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 51 → product-complete (PH-S970…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–30 | PH-S660…S959 | ✅ drained |
| 31 | PH-S960…S969 | **активна §5.12** — DOCS_LEGACY audit close |
| 32–33 | PH-S970…S989 | Galaxy concept + docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 31 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S960** | DOCS_LEGACY_AUDIT remaining rows triage | table update |
| **PH-S961** | stale banners on flat docs/*.md | pointer to INDEX/archive |
| **PH-S962** | concept root de-hype pass | poolAI_concept_root.txt zriz |
| **PH-S963** | ARCHITECT vs FM §5.1 alignment | NEXT_STEPS_ARCHITECT sync |
| **PH-S964** | docs archive pointer batch | DOCS_LEGACY §5.3 |
| **PH-S965** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S965 |
| **PH-S966** | INDEX step 12 FM pointer | docs |
| **PH-S967** | `poolai-vision-sync --check` | drift gate green |
| **PH-S968** | ratio advisory | hold |
| **PH-S969** | `galaxy_horizon_s960_integration` | DOCS_LEGACY close |

**Після band 31 promote:** PH-S970…S979 (band 32 — Galaxy concept implemented markers).

---

## Не повторювати

PH-S950…S959 ✅ · FUNCTIONALITY_DIGEST full sync · `digest_depth_stub` · grid/job/ui-wasm/bins tables · OpenAPI gap audit note · `galaxy_horizon_s950_integration`.
