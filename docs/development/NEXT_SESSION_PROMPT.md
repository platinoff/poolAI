# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S155 ✅ · vision **rev 88** · **4** відкритих (PH-S156…S159) · **stretch spirit 96%**

| **← наступний** | **PH-S156** — jobs_raid → Rust stand smoke |
| **Відкритих** | **4** (PH-S156…S159) |
| **Ratio spirit** | формально **90–95%** · орієнтир **96%** — більше Rust, краще ([`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md)) |
| **VDT** | [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · один PH-S* = 1 commit |

---

## Зріз §5.12 (ratio stretch band PH-S156…S159)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S155 | ML charts → wasm | `poolai-ui-core/ml` + wasm exports; `admin_charts.js` canvas glue |
| PH-S154 | Admin i18n subset Rust | `poolai-ui-core/i18n.rs`; admin_layout patch; `i18n_core.js` **−103 LOC** |
| PH-S153 | admin_common slim | `poolai-ui-core/table` + wasm; admin_common **−426 LOC** |

### Відкрито (черга — 4)

| # | Sprint | Scope | Acceptance | Ratio |
|---|--------|-------|------------|-------|
| 1 | **PH-S156** | jobs_raid → Rust smoke | `poolai-http-stand-smoke --raid-restart`; no `jobs_raid` in `test:ci` | −TS |
| 2 | **PH-S157** | topology SVG Rust | data from `topology.rs`; slim `topology_graph.js` | −JS |
| 3 | **PH-S158** | `poolai-e2e-stand` bin | Rust stand start/restart; slim `e2e-playwright.sh` | −shell |
| 4 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish §5.12 | **96%** |

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

## PH-S156 — scope підказка

| Файл | Дія |
|------|-----|
| `src/bin/poolai_http_stand_smoke.rs` | `--raid-restart` smoke (POST job → restart → GET) |
| `e2e/package.json` | прибрати `jobs_raid` з `test:ci` |
| FM §5.12 / HANDOFF | PH-S156 ✅; NEXT → PH-S157 |

---

## Закриття спринту (канон)

1. `cargo fmt --all`
2. `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci`
3. `cargo run --bin poolai-loc-audit -- --min-ratio 0.91`
4. FM §5.12 PH-S* ✅ · HANDOFF · цей файл → PH-S156 · vision rev++
5. `git push origin main` (MSYS2; `bin/git-commit-tree-msg.sh`)

---

## Copy-paste — наступна сесія (PH-S156)

```
PoolAI — спринт PH-S156 (odin PH-S*, VDT).

Scope: jobs_raid e2e → Rust stand smoke — poolai-http-stand-smoke --raid-restart; remove jobs_raid from e2e test:ci.

Acceptance: cargo test-ci; loc-audit --min-ratio 0.91; FM/HANDOFF/NEXT/vision; git push main.
```
