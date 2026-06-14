# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S156 ✅ · vision **rev 90** · **10** відкритих (PH-S157…S165) · **stretch spirit 96%**

| **← наступний** | **PH-S157** — topology SVG Rust |
| **Відкритих** | **10** (PH-S157…S165) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (10 відкритих: stretch S157…S159 + maintain S160…S165)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S156 | jobs_raid → Rust smoke | `--raid-restart` stand smoke; `test:ci` browser-only |
| PH-S155 | ML charts → wasm | `poolai-ui-core/ml` + wasm; `admin_charts.js` canvas glue |
| PH-S154 | Admin i18n subset Rust | `i18n.rs` patch; `i18n_core.js` **−103 LOC** |

### Відкрито — stretch band (PH-S157…S159)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S157** | topology SVG Rust | data from `topology.rs`; slim `topology_graph.js` | −JS |
| 2 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 3 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish post-S159 | **96%** |

### Відкрито — maintain band (PH-S160…S165)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 4 | **PH-S160** | Admin theme → Rust | `poolaiNormalizeTheme` у ui-core; slim `admin_theme.js` | −JS |
| 5 | **PH-S161** | Admin modal a11y → wasm | focus-trap helpers wasm; slim `admin_modal_a11y.js` | −JS |
| 6 | **PH-S162** | Auth i18n subset Rust | auth/dashboard keys у `i18n.rs`; slim auth block | −JS |
| 7 | **PH-S163** | Galaxy trust metrics wire | Prometheus на grid result; unit tests | +Rust |
| 8 | **PH-S164** | Verify sampling apply | `galaxy_verify_sampling` HTTP/grid stub; tests | +Rust |
| 9 | **PH-S165** | **96%** hold gate | CI `--min-ratio 0.95`; replenish next band | **96%** |

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

## PH-S157 — scope підказка

| Файл | Дія |
|------|-----|
| `src/network/api/topology.rs` (або аналог) | masked topology data з Rust |
| `src/ui/admin/topology_graph.js` | slim — consume Rust JSON |
| FM §5.12 / HANDOFF | PH-S157 ✅; NEXT → PH-S158 |

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91`
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл · vision rev++
5. `git push origin main` (MSYS2; `bin/git-commit-tree-msg.sh`)

---

## Copy-paste — наступна сесія (PH-S157)

```
PoolAI — спринт PH-S157 (один PH-S*, VDT ітераційно).

Правила: .cursor/rules/virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md
Ratio: docs/development/RUST_RATIO_STRATEGY_2026-06-13.md · baseline 92.19% · formal 90–95% · spirit 96%

S0:
  git fetch origin
  df -h /s
  export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
  export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
  export CARGO_TARGET_DIR=/s/rust/poolAI/target
  export K8S_OPENAPI_ENABLED_VERSION=1.28

Спринт PH-S157 — topology SVG from Rust
Scope:
  - topology data з Rust (`topology.rs`); slim `topology_graph.js`
  - Rust integration; без Python; без нового Playwright API spec

Acceptance:
  - cargo fmt --all
  - K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
  - cargo run --bin poolai-loc-audit -- --min-ratio 0.91
  - FM §5.12 PH-S157 ✅; HANDOFF; NEXT_SESSION → PH-S158; vision rev++
  - git push origin main (MSYS2)

Commit: один PH-S157; не stage data/audit/, bin/commit-*.sh, comitmsg/*.txt
```
