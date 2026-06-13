# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S151 ✅ · vision **rev 83** · **rust_ratio 92.00%** · **8** відкритих (PH-S152…S159) · **stretch spirit 96%**

| **← наступний** | **PH-S152** — wasm jobs lease display |
| **Відкритих** | **8** (PH-S152…S159) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — більше Rust, краще ([`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md)) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (ratio stretch band PH-S152…S159)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S151 | wasm grid-pricing wiring | `/ui/wasm/*` + module import; grid-pricing formatters via wasm |
| PH-S150 | Ratio CI advisory | CI `rust-ratio-audit`; **92.00%** |
| PH-S147 | `poolai-ui-wasm` | wasm32 POC |
| PH-S148 | Slim `e2e/` browser-only | **91.99%** |

### Відкрито (черга — 8)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S152** | wasm jobs lease display | jobs panel wasm lease labels; slim JS | −JS |
| 2 | **PH-S153** | admin_common slim | api_error/format/table → Rust/wasm; −≥400 LOC JS | −JS |
| 3 | **PH-S154** | Admin i18n subset Rust | grid-pricing + jobs keys у Rust; slim `i18n_core.js` | −JS |
| 4 | **PH-S155** | ML charts → wasm | metrics parse wasm; canvas glue only | −JS |
| 5 | **PH-S156** | jobs_raid → Rust smoke | `poolai-http-stand-smoke --raid-restart`; no `jobs_raid` in `test:ci` | −TS |
| 6 | **PH-S157** | topology SVG Rust | data from `topology.rs`; slim `topology_graph.js` | −JS |
| 7 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 8 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish §5.12 | **96%** |

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

## PH-S152 — scope підказка

| Файл | Дія |
|------|-----|
| `src/ui/admin/jobs.rs` | wasm `leaseStateLabel`; slim inline JS |
| `crates/poolai-ui-wasm/` | reuse module bootstrap pattern from PH-S151 |
| `e2e/tests/admin.spec.ts` | jobs lease wasm smoke |
| FM §5.12 / HANDOFF | PH-S152 ✅; NEXT → PH-S153 |

**Ratio spirit:** кожен PH-S153…S158 — зменшити non-Rust LOC. **96% — stretch spirit.**

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91`
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл → PH-S152 · vision rev++
5. `git push origin main` (MSYS2)

---

## Copy-paste — наступна сесія (PH-S152)

```
PoolAI — спринт PH-S152 (odin PH-S*, VDT).

S0: git fetch; HANDOFF; FM §5.12; df -h; MSYS2 PATH.

PH-S152: wasm jobs lease display
- jobs admin panel → poolai-ui-wasm leaseStateLabel; slim JS
- Playwright admin smoke; cargo test-ci
- poolai-loc-audit --min-ratio 0.91
- FM/HANDOFF/NEXT_SESSION/vision revision++

Ratio spirit: 96% stretch — більше Rust краще.
Канон: RUST_RATIO_STRATEGY_2026-06-13.md · FM §5.12.
Черга: PH-S152…S159 (8 відкритих).
```
