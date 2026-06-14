# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S162 ✅ · vision **rev 97** · **7** відкритих (PH-S163…S169) · **stretch spirit 96%**

| **← наступний** | **PH-S163** — Galaxy trust metrics wire |
| **Відкритих** | **7** (PH-S163…S169) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (7 відкритих: maintain S163…S165 + replenish S166…S169)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S162 | Auth i18n subset Rust | `i18n.rs` auth+dash shell; `__poolaiAuthDashI18nRust`; slim `i18n_core.js` |
| PH-S161 | Admin modal a11y → wasm | `modal.rs` + wasm `trapTabAction`; slim `admin_modal_a11y.js` |
| PH-S160 | Admin theme → Rust | `theme.rs` + wasm `normalizeTheme` |

### Відкрито — maintain (PH-S163…S165)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S163** | Galaxy trust metrics wire |
| 2 | **PH-S164** | Verify sampling apply |
| 3 | **PH-S165** | **96%** hold gate |

### Відкрито — replenish (PH-S166…S169)

| # | Sprint | Scope |
|---|--------|-------|
| 4 | **PH-S166** | Design tokens CSS → Rust |
| 5 | **PH-S167** | Galaxy prefetch metrics stub |
| 6 | **PH-S168** | Galaxy pricing cache age /metrics |
| 7 | **PH-S169** | Locality stale profile penalty stub |

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

## PH-S163 — scope

- trust gate Prometheus на grid result path (Galaxy §6.5)
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S163

```
PoolAI — спринт PH-S163 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S163 — Galaxy trust metrics wire
Scope: trust gate Prometheus на grid result path; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
