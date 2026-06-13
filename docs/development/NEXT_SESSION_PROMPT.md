# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S150 ✅ · vision **rev 82** · **rust_ratio 92.00%** · **9** відкритих (PH-S151…S159) · **stretch spirit 96%**

| **← наступний** | **PH-S151** — wasm grid-pricing wiring |
| **Відкритих** | **9** (PH-S151…S159) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — більше Rust, краще ([`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md)) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (ratio stretch band PH-S151…S159)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S150 | Ratio CI advisory | CI `rust-ratio-audit`; `--warn-below 0.88` `--target 0.93` `--stretch 0.96` `--advisory`; **92.00%** |
| PH-S143 | `poolai-loc-audit` | baseline **91.48%** |
| PH-S144 | Playwright API → Rust | 7 specs → archive |
| PH-S145 | `poolai-http-stand-smoke` | Rust HTTP stand |
| PH-S146 | `poolai-ui-core` | validators + 16 tests |
| PH-S147 | `poolai-ui-wasm` | wasm32 POC |
| PH-S148 | Slim `e2e/` browser-only | `test:ci` smoke/admin/a11y/visual/jobs_raid; **91.99%** |

### Відкрито (черга — 9)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S151** | wasm grid-pricing wiring | `/ui/admin/grid-pricing` → wasm; Playwright smoke | +wasm |
| 2 | **PH-S152** | wasm jobs lease display | jobs panel wasm lease labels; slim JS | −JS |
| 3 | **PH-S153** | admin_common slim | api_error/format/table → Rust/wasm; −≥400 LOC JS | −JS |
| 4 | **PH-S154** | Admin i18n subset Rust | grid-pricing + jobs keys у Rust; slim `i18n_core.js` | −JS |
| 5 | **PH-S155** | ML charts → wasm | metrics parse wasm; canvas glue only | −JS |
| 6 | **PH-S156** | jobs_raid → Rust smoke | `poolai-http-stand-smoke --raid-restart`; no `jobs_raid` in `test:ci` | −TS |
| 7 | **PH-S157** | topology SVG Rust | data from `topology.rs`; slim `topology_graph.js` | −JS |
| 8 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 9 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish §5.12 | **96%** |

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

## PH-S151 — scope підказка

| Файл | Дія |
|------|-----|
| `src/ui/admin/grid_pricing.rs` | wire `poolai-ui-wasm` formatters (USD micro, unit labels) |
| `crates/poolai-ui-wasm/` | export helpers used by grid-pricing panel |
| `e2e/tests/admin.spec.ts` | smoke grid-pricing with wasm loaded |
| FM §5.12 / HANDOFF | PH-S151 ✅; NEXT → PH-S152 |

**Ratio spirit:** кожен PH-S152…S158 — зменшити non-Rust LOC (JS → wasm/Rust, TS → Rust smoke, shell → Rust bin). **96% — stretch, не компроміс якості.**

**Не в scope:** повернення archived Playwright API specs; Python runtime.

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91` — підтвердити ratio не падає
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл → наступний PH-S* · vision rev++
5. `git push origin main` (MSYS2)

---

## Copy-paste — наступна сесія (PH-S151)

```
PoolAI — спринт PH-S151 (odin PH-S*, VDT).

S0: git fetch; HANDOFF; FM §5.12; df -h; MSYS2 PATH + RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu.

PH-S151: wasm grid-pricing wiring
- `/ui/admin/grid-pricing` → poolai-ui-wasm formatters; slim inline JS
- Playwright admin smoke; cargo test-ci
- poolai-loc-audit --min-ratio 0.91 (не регрес)
- FM/HANDOFF/NEXT_SESSION/vision revision++

Ratio spirit: тягнутись до 96% Rust product code (більше краще). Канон: docs/development/RUST_RATIO_STRATEGY_2026-06-13.md
Черга: PH-S151…S159 (9 відкритих).
```

---

## Copy-paste — шаблон (будь-який PH-S151…S159)

```
PoolAI — спринт PH-SNNN (odin PH-S*, VDT).

S0: git fetch; HANDOFF; FM §5.12; df -h; MSYS2 PATH.

PH-SNNN: <scope з FM §5.12>
- cargo test-ci; poolai-loc-audit --min-ratio 0.91 — ratio не нижче baseline
- FM/HANDOFF/NEXT_SESSION/vision revision++

Ratio spirit: 96% stretch — перенос JS/TS/shell → Rust/wasm; більше Rust краще.
Канон: RUST_RATIO_STRATEGY_2026-06-13.md · FM §5.12.
```
