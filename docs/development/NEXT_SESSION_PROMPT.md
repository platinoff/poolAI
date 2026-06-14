# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S159 ✅ · vision **rev 93** · **10** відкритих (PH-S160…S169) · **stretch spirit 96%**

| **← наступний** | **PH-S160** — Admin theme normalize → Rust |
| **Відкритих** | **10** (PH-S160…S169) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (10 відкритих: maintain S160…S165 + replenish S166…S169)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S159 | Ratio **96%** stretch CI gate | CI `--warn-below 0.93`; stretch **96%**; replenish S166…S169 |
| PH-S158 | `poolai-e2e-stand` | Rust stand start/restart/stop; slim `e2e-playwright.sh` |
| PH-S157 | topology SVG Rust | `GET /topology/graph`; slim `topology_graph.js` |

### Відкрито — maintain (PH-S160…S165)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S160** | Admin theme → Rust |
| 2 | **PH-S161** | Admin modal a11y → wasm |
| 3 | **PH-S162** | Auth i18n subset Rust |
| 4 | **PH-S163** | Galaxy trust metrics wire |
| 5 | **PH-S164** | Verify sampling apply |
| 6 | **PH-S165** | **96%** hold gate |

### Відкрито — post-S159 replenish (PH-S166…S169)

| # | Sprint | Scope |
|---|--------|-------|
| 7 | **PH-S166** | Design tokens CSS → Rust |
| 8 | **PH-S167** | Galaxy prefetch metrics stub |
| 9 | **PH-S168** | Galaxy pricing cache age /metrics |
| 10 | **PH-S169** | Locality stale profile penalty stub |

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

## PH-S160 — scope

- `poolaiNormalizeTheme` + token map у `poolai-ui-core`; slim `admin_theme.js`
- Acceptance: `cargo test-ci`; Playwright admin smoke; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S160

```
PoolAI — спринт PH-S160 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S160 — Admin theme normalize → Rust
Scope: poolaiNormalizeTheme + token map у poolai-ui-core; slim admin_theme.js

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
