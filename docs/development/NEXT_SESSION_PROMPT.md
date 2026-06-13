# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S154 ✅ · vision **rev 87** · **5** відкритих (PH-S155…S159) · **stretch spirit 96%**

| **← наступний** | **PH-S155** — ML charts → wasm |
| **Відкритих** | **5** (PH-S155…S159) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — більше Rust, краще ([`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md)) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (ratio stretch band PH-S155…S159)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S154 | Admin i18n subset Rust | `poolai-ui-core/i18n.rs`; admin_layout patch; `i18n_core.js` **−223 LOC** |
| PH-S153 | admin_common slim | `poolai-ui-core/table` + wasm; admin_common **−426 LOC**; theme/modal split |
| PH-S152 | wasm jobs lease display | shared `POOLAI_UI_WASM_MODULE`; jobs `leaseStateLabel` |

### Відкрито (черга — 5)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S155** | ML charts → wasm | metrics parse wasm; canvas glue only | −JS |
| 2 | **PH-S156** | jobs_raid → Rust smoke | `poolai-http-stand-smoke --raid-restart`; no `jobs_raid` in `test:ci` | −TS |
| 3 | **PH-S157** | topology SVG Rust | data from `topology.rs`; slim `topology_graph.js` | −JS |
| 4 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 5 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish §5.12 | **96%** |

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

## PH-S155 — scope підказка

| Файл | Дія |
|------|-----|
| `crates/poolai-ui-core/src/ml.rs` | metrics parse helpers → wasm export |
| `src/ui/admin_charts.js` | slim — canvas glue only |
| FM §5.12 / HANDOFF | PH-S155 ✅; NEXT → PH-S156 |

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91`
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл → PH-S155 · vision rev++
5. `git push origin main` (MSYS2; `bin/git-commit-tree-msg.sh`)

---

## Copy-paste — наступна сесія (PH-S155)

```
PoolAI — спринт PH-S155 (odin PH-S*, VDT).

Scope: ML charts data → Rust/wasm — metrics parse in poolai-ui-core/ml + wasm; admin_charts.js canvas glue only.

Acceptance: cargo test-ci; loc-audit --min-ratio 0.91; FM/HANDOFF/NEXT/vision; git push main.
```
