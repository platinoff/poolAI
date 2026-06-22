# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-22 (completion v2 · master backlog **61** · active **PH-S950…S959** · vision **rev 295** · rust_ratio **94.91%**)

| **← наступний** | **`абракадабра`** (drain band 30 PH-S950…S959) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **61** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **10** (PH-S950…S959, band 30 FUNCTIONALITY_DIGEST full sync) |
| **Після band 30** | promote PH-S960…S969 (band 31 DOCS_LEGACY audit close) |
| **Сесій drain** | **7** (`61÷10` + tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** активних 10 з §5.12 (PH-S950…S959) — acceptance у [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) band 30
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Promote** наступні 10 з [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) → §5.12 `[ ]` (після push band 30: **PH-S960…S969**)

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 61 → product-complete (PH-S960…S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–29 | PH-S660…S949 | ✅ drained |
| 30 | PH-S950…S959 | **активна §5.12** — FUNCTIONALITY_DIGEST full sync |
| 31–33 | PH-S960…S989 | Docs product-complete |
| 34–35 | PH-S990…S1009 | Final integration horizon |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 30 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S950** | FUNCTIONALITY_DIGEST grid section sync | all src/grid modules listed |
| **PH-S951** | FUNCTIONALITY_DIGEST job/lease sync | src/job rows |
| **PH-S952** | FUNCTIONALITY_DIGEST ui/wasm sync | crates rows |
| **PH-S953** | FUNCTIONALITY_DIGEST bins table | src/bin/ all listed |
| **PH-S954** | DIGEST OpenAPI pointer refresh | gap audit note |
| **PH-S955** | `poolai-loc-audit` → `rust_ratio.json` | sprint zriz PH-S955 |
| **PH-S956** | file_list.csv catalog sync | key paths |
| **PH-S957** | `poolai-vision-sync --check` | drift gate green |
| **PH-S958** | ratio advisory | hold |
| **PH-S959** | `galaxy_horizon_s950_integration` | digest band close |

**Після band 30 promote:** PH-S960…S969 (band 31 — DOCS_LEGACY audit close).

---

## Не повторювати

PH-S940…S949 ✅ · e2e scope audit · `jobs_raid` archive · `stretch_spirit_gate_met` · `ops_shell_canon_met` · `stretch_depth_stub` · `e2e_ts_loc_reduction` · `galaxy_horizon_s940_integration`.
