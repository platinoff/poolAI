# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S153 ✅ · vision **rev 86** · **6** відкритих (PH-S154…S159) · **stretch spirit 96%**

| **← наступний** | **PH-S154** — Admin i18n subset Rust |
| **Відкритих** | **6** (PH-S154…S159) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — більше Rust, краще ([`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md)) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (ratio stretch band PH-S154…S159)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S153 | admin_common slim | `poolai-ui-core/table` + wasm; admin_common **−426 LOC**; theme/modal split |
| PH-S152 | wasm jobs lease display | shared `POOLAI_UI_WASM_MODULE`; jobs `leaseStateLabel` |
| PH-S151 | wasm grid-pricing wiring | `/ui/wasm/*` + formatters via wasm |

### Відкрито (черга — 6)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S154** | Admin i18n subset Rust | grid-pricing + jobs keys у Rust; slim `i18n_core.js` | −JS |
| 2 | **PH-S155** | ML charts → wasm | metrics parse wasm; canvas glue only | −JS |
| 3 | **PH-S156** | jobs_raid → Rust smoke | `poolai-http-stand-smoke --raid-restart`; no `jobs_raid` in `test:ci` | −TS |
| 4 | **PH-S157** | topology SVG Rust | data from `topology.rs`; slim `topology_graph.js` | −JS |
| 5 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 6 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish §5.12 | **96%** |

---

## S0 (на початку сесії)

```bash
git fetch origin
# HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
# FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md
df -h /s
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export CARGO_TARGET_DIR=/s/rust/poolAI/target
export K8S_OPENAPI_ENABLED_VERSION=1.28
```

---

## PH-S154 — scope підказка

| Файл | Дія |
|------|-----|
| `src/ui/i18n_core.js` | slim admin block; keys for grid-pricing + jobs lease |
| `src/ui/admin/*.rs` | embed EN/UK subset in Rust templates where feasible |
| FM §5.12 / HANDOFF | PH-S154 ✅; NEXT → PH-S155 |

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91`
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл → PH-S154 · vision rev++
5. `git push origin main` (MSYS2; `bin/git-commit-tree-msg.sh`)

---

## Copy-paste — наступна сесія (PH-S154)

```
PoolAI — спринт PH-S154 (odin PH-S*, VDT).

Scope: Admin i18n subset in Rust — grid-pricing + jobs lease EN/UK keys; slim i18n_core.js admin block.

Acceptance: cargo test-ci; loc-audit --min-ratio 0.91; FM/HANDOFF/NEXT/vision; git push main.
```
