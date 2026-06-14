# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S161 ✅ · vision **rev 95** · **8** відкритих (PH-S162…S169) · **stretch spirit 96%**

| **← наступний** | **PH-S162** — Auth i18n subset Rust |
| **Відкритих** | **8** (PH-S162…S169) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (8 відкритих: maintain S162…S165 + replenish S166…S169)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S161 | Admin modal a11y → wasm | `modal.rs` + wasm `trapTabAction`; slim `admin_modal_a11y.js` |
| PH-S160 | Admin theme → Rust | `theme.rs` + wasm `normalizeTheme` |
| PH-S159 | Ratio **96%** stretch CI gate | CI `--warn-below 0.93` |

### Відкрито — maintain (PH-S162…S165)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S162** | Auth i18n subset Rust |
| 2 | **PH-S163** | Galaxy trust metrics wire |
| 3 | **PH-S164** | Verify sampling apply |
| 4 | **PH-S165** | **96%** hold gate |

### Відкрито — replenish (PH-S166…S169)

| # | Sprint | Scope |
|---|--------|-------|
| 5 | **PH-S166** | Design tokens CSS → Rust |
| 6 | **PH-S167** | Galaxy prefetch metrics stub |
| 7 | **PH-S168** | Galaxy pricing cache age /metrics |
| 8 | **PH-S169** | Locality stale profile penalty stub |

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

## PH-S162 — scope

- login/dashboard shell keys у `i18n.rs`; slim `i18n_core.js` auth block
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S162

```
PoolAI — спринт PH-S162 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S162 — Auth i18n subset Rust
Scope: login/dashboard shell keys у poolai-ui-core/i18n.rs; slim i18n_core.js auth block

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
