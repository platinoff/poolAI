# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S152 ✅ · vision **rev 85** · **rust_ratio 91.99%** · **7** відкритих (PH-S153…S159) · **stretch spirit 96%**

| **← наступний** | **PH-S153** — admin_common slim |
| **Відкритих** | **7** (PH-S153…S159) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — більше Rust, краще ([`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md)) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (ratio stretch band PH-S153…S159)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S152 | wasm jobs lease display | shared `POOLAI_UI_WASM_MODULE`; jobs `leaseStateLabel`; grid-pricing migrated |
| PH-S151 | wasm grid-pricing wiring | `/ui/wasm/*` + module import; grid-pricing formatters via wasm |
| PH-S150 | Ratio CI advisory | CI `rust-ratio-audit`; **92.00%** |

### Відкрито (черга — 7)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S153** | admin_common slim | api_error/format/table → Rust/wasm; −≥400 LOC JS | −JS |
| 2 | **PH-S154** | Admin i18n subset Rust | grid-pricing + jobs keys у Rust; slim `i18n_core.js` | −JS |
| 3 | **PH-S155** | ML charts → wasm | metrics parse wasm; canvas glue only | −JS |
| 4 | **PH-S156** | jobs_raid → Rust smoke | `poolai-http-stand-smoke --raid-restart`; no `jobs_raid` in `test:ci` | −TS |
| 5 | **PH-S157** | topology SVG Rust | data from `topology.rs`; slim `topology_graph.js` | −JS |
| 6 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 7 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish §5.12 | **96%** |

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

## PH-S153 — scope підказка

| Файл | Дія |
|------|-----|
| `src/ui/admin_common.js` | slim −≥400 LOC via Rust/wasm |
| `crates/poolai-ui-core/` | api_error, format, table helpers |
| `crates/poolai-ui-wasm/` | export wasm wrappers where needed |
| FM §5.12 / HANDOFF | PH-S153 ✅; NEXT → PH-S154 |

**Ratio spirit:** кожен PH-S153…S158 — зменшити non-Rust LOC. **96% — stretch spirit.**

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91`
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл → PH-S153 · vision rev++
5. `git push origin main` (MSYS2)

---

## Copy-paste — наступна сесія (PH-S153)

```
PoolAI — спринт PH-S153 (odin PH-S*, VDT).

S0: git fetch; HANDOFF; FM §5.12; df -h /s; MSYS2 PATH + CARGO_TARGET_DIR.

Scope: admin_common slim — api_error/format/table helpers → poolai-ui-core/wasm; −≥400 LOC JS.

Acceptance:
- cargo fmt --all; cargo test-ci; poolai-loc-audit --min-ratio 0.91
- FM §5.12 PH-S153 ✅; HANDOFF; NEXT_SESSION → PH-S154; vision rev++
- git push origin main (MSYS2; bin/git-commit-tree-msg.sh if hook breaks subject)

Ratio spirit: 96% stretch; formal 90–95%; baseline 91.99%.
```
