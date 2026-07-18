# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-07-18 (PH-S1000…S1009 ✅ band 35 · master backlog **1** · active **PH-S1010** · vision **rev 305** · rust_ratio **94.95%**)

| **← наступний** | **`абракадабра`** (drain PH-S1010 product-complete) |
| **Completion plan** | [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · FM **§5.14–§5.15** |
| **Master backlog** | **1** pending — [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) |
| **Активних §5.12** | **1** (PH-S1010, band 36 product-complete closure) |
| **Після band 36** | maintenance mode (FM §5.15) |
| **Сесій drain** | **1** (tail PH-S1010) |

---

## Тригер «абракадабра»

Скопіюй у **новий чат**:

```
абракадабра
```

---

## S0 + drain (канон)

1. `git fetch`; HANDOFF; FM §5.12 + **§5.14**; `poolai-vision-sync --check`; `df -h /s`
2. **Drain** PH-S1010 — acceptance у [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) band 36
3. Vision close → `poolai-vision-sync` → `--check`
4. `cargo fmt --all` → `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
5. Commit + `git push origin main` + самарі
6. **Maintenance mode** — HANDOFF/NEXT без нових PH-S* до scan власника

```bash
export PATH="/c/Users/$USER/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_TARGET_DIR=/s/rust/poolAI/target
cd /s/rust/poolAI
```

---

## Master backlog 1 → product-complete (PH-S1010)

План фаз: [`PH_S_COMPLETION_ROADMAP_2026-06-20.md`](./PH_S_COMPLETION_ROADMAP_2026-06-20.md) · реєстр: [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) · FM [`§5.14`](../catalog/FUNCTION_MANAGEMENT.md#514-master-backlog-ph-s720s1010-291-pending--product-complete-2026-06-20) · closure [`§5.15`](../catalog/FUNCTION_MANAGEMENT.md#515-product-complete-closure-ph-s1010).

| Band | Sprints | Theme |
|------|---------|-------|
| 1–35 | PH-S660…S1009 | ✅ drained |
| 36 | PH-S1010 | Product-complete (FM §5.15) |

**Не в backlog:** FM-003 LAN · FM-041 Cloud SDK · ZK/TEE (BLOCKED/Deferred/roadmap).

**Regen:** `bash scripts/generate-ph-s-master-backlog-351.sh`.

---

## Активна смуга (band 36 — drain зараз)

| Sprint | Focus | Acceptance |
|--------|--------|------------|
| **PH-S1010** | FM §5.15 product-complete declaration | STABLE «development complete»; HANDOFF maintenance; vision-sync; BLOCKED/Deferred documented; **no** new PH-S until owner scan |

---

## Не повторювати

Band 35 (PH-S1000…S1009) ✅ — `multi_module_wire_smoke.rs`, `multi_module_admin_wasm_regression.rs`, `multi_module_stand_smoke_audit.rs`, `galaxy_horizon_s1000_integration.rs`.
