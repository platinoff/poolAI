# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S160 ✅ · vision **rev 94** · **9** відкритих (PH-S161…S169) · **stretch spirit 96%**

| **← наступний** | **PH-S161** — Admin modal a11y → wasm |
| **Відкритих** | **9** (PH-S161…S169) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (9 відкритих: maintain S161…S165 + replenish S166…S169)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S160 | Admin theme → Rust | `theme.rs` + `__poolaiAdminThemesRust`; wasm `normalizeTheme`; slim `admin_theme.js` |
| PH-S159 | Ratio **96%** stretch CI gate | CI `--warn-below 0.93`; stretch **96%** |
| PH-S158 | `poolai-e2e-stand` | Rust stand lifecycle; slim `e2e-playwright.sh` |

### Відкрито — maintain (PH-S161…S165)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S161** | Admin modal a11y → wasm |
| 2 | **PH-S162** | Auth i18n subset Rust |
| 3 | **PH-S163** | Galaxy trust metrics wire |
| 4 | **PH-S164** | Verify sampling apply |
| 5 | **PH-S165** | **96%** hold gate |

### Відкрито — replenish (PH-S166…S169)

| # | Sprint | Scope |
|---|--------|-------|
| 6 | **PH-S166** | Design tokens CSS → Rust |
| 7 | **PH-S167** | Galaxy prefetch metrics stub |
| 8 | **PH-S168** | Galaxy pricing cache age /metrics |
| 9 | **PH-S169** | Locality stale profile penalty stub |

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

## PH-S161 — scope

- focus-trap / modal helpers у `poolai-ui-core`/wasm; slim `admin_modal_a11y.js`
- Acceptance: `cargo test-ci`; Playwright admin smoke; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S161

```
PoolAI — спринт PH-S161 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S161 — Admin modal a11y → wasm
Scope: focus-trap / modal helpers у ui-core/wasm; slim admin_modal_a11y.js

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
