# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S158 ✅ · vision **rev 92** · **9** відкритих (PH-S159…S165) · **stretch spirit 96%**

| **← наступний** | **PH-S159** — Ratio **96%** stretch CI gate |
| **Відкритих** | **9** (PH-S159…S165) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (9 відкритих: stretch S159 + maintain S160…S165)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S158 | `poolai-e2e-stand` | Rust stand start/restart/stop; slim `e2e-playwright.sh` |
| PH-S157 | topology SVG Rust | `GET /topology/graph`; slim `topology_graph.js` |
| PH-S156 | jobs_raid → Rust smoke | `--raid-restart` |

### Відкрито — stretch (PH-S159)

| # | Sprint | Scope | Acceptance |
|---|--------|-------|------------|
| 1 | **PH-S159** | **96%** stretch gate | warn **93%**; stretch **96%**; replenish post-S159 |

### Відкрито — maintain (PH-S160…S165)

| # | Sprint | Scope |
|---|--------|-------|
| 2 | **PH-S160** | Admin theme → Rust |
| 3 | **PH-S161** | Admin modal a11y → wasm |
| 4 | **PH-S162** | Auth i18n subset Rust |
| 5 | **PH-S163** | Galaxy trust metrics wire |
| 6 | **PH-S164** | Verify sampling apply |
| 7 | **PH-S165** | **96%** hold gate |

---

## S0

```bash
git fetch origin
df -h /s
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export CARGO_TARGET_DIR=/s/rust/poolAI/target
export K8S_OPENAPI_ENABLED_VERSION=1.28
```

---

## PH-S159 — scope

- `poolai-loc-audit` warn **93%**, stretch **96%** у CI/docs
- FM replenish post-S159 band
- Acceptance: `cargo test-ci`; loc-audit; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S159

```
PoolAI — спринт PH-S159 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S159 — 96% stretch CI gate
Scope: poolai-loc-audit warn 93% / stretch 96%; CI + docs; replenish §5.12

Acceptance: cargo fmt; cargo test-ci; loc-audit; FM/HANDOFF/NEXT/vision; git push main
```
