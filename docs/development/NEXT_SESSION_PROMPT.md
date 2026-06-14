# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S157 ✅ · vision **rev 91** · **10** відкритих (PH-S158…S165) · **stretch spirit 96%**

| **← наступний** | **PH-S158** — `poolai-e2e-stand` bin |
| **Відкритих** | **10** (PH-S158…S165) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (10 відкритих: stretch S158…S159 + maintain S160…S165)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S157 | topology SVG Rust | `topology_graph.rs` layout; `GET /topology/graph`; slim JS |
| PH-S156 | jobs_raid → Rust smoke | `--raid-restart` stand smoke; `test:ci` browser-only |
| PH-S155 | ML charts → wasm | `poolai-ui-core/ml` + wasm |

### Відкрито — stretch band (PH-S158…S159)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 2 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish post-S159 | **96%** |

### Відкрито — maintain band (PH-S160…S165)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 3 | **PH-S160** | Admin theme → Rust | `poolaiNormalizeTheme` у ui-core; slim `admin_theme.js` | −JS |
| 4 | **PH-S161** | Admin modal a11y → wasm | focus-trap helpers wasm; slim `admin_modal_a11y.js` | −JS |
| 5 | **PH-S162** | Auth i18n subset Rust | auth/dashboard keys у `i18n.rs`; slim auth block | −JS |
| 6 | **PH-S163** | Galaxy trust metrics wire | Prometheus на grid result; unit tests | +Rust |
| 7 | **PH-S164** | Verify sampling apply | `galaxy_verify_sampling` HTTP/grid stub; tests | +Rust |
| 8 | **PH-S165** | **96%** hold gate | CI `--min-ratio 0.95`; replenish next band | **96%** |

---

## S0 (на початку сесії)

```bash
git fetch origin
df -h /s
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export CARGO_TARGET_DIR=/s/rust/poolAI/target
export K8S_OPENAPI_ENABLED_VERSION=1.28
```

---

## PH-S158 — scope підказка

| Файл | Дія |
|------|-----|
| `src/bin/poolai_e2e_stand.rs` (новий) | Rust stand start/restart |
| `bin/e2e-playwright.sh` | slim — delegate stand lifecycle |
| FM §5.12 / HANDOFF | PH-S158 ✅; NEXT → PH-S159 |

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91`
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл · vision rev++
5. `git push origin main` (MSYS2)

---

## Copy-paste — наступна сесія (PH-S158)

```
PoolAI — спринт PH-S158 (один PH-S*, VDT ітераційно).

HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S158 — poolai-e2e-stand bin
Scope:
  - src/bin/poolai_e2e_stand.rs — Rust stand start/restart
  - bin/e2e-playwright.sh — slim stand lifecycle
  - Rust integration; без Python; без нового Playwright API spec

Acceptance:
  - cargo fmt --all; cargo test-ci; poolai-loc-audit --min-ratio 0.91
  - FM §5.12 PH-S158 ✅; HANDOFF; NEXT → PH-S159; vision rev++
  - git push origin main (MSYS2)
```
