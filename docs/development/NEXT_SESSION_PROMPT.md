# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S163 ✅ · vision **rev 98** · **6** відкритих (PH-S164…S169) · **stretch spirit 96%**

| **← наступний** | **PH-S164** — Verify sampling env apply |
| **Відкритих** | **6** (PH-S164…S169) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (6 відкритих: maintain S164…S165 + replenish S166…S169)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S163 | Galaxy trust metrics wire | grid result → Prometheus gauges; integration test |
| PH-S162 | Auth i18n subset Rust | `i18n.rs` auth+dash shell; slim `i18n_core.js` |
| PH-S161 | Admin modal a11y → wasm | `modal.rs` + wasm `trapTabAction` |

### Відкрито — maintain (PH-S164…S165)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S164** | Verify sampling env apply |
| 2 | **PH-S165** | **96%** hold gate |

### Відкрито — replenish (PH-S166…S169)

| # | Sprint | Scope |
|---|--------|-------|
| 3 | **PH-S166** | Design tokens CSS → Rust |
| 4 | **PH-S167** | Galaxy prefetch metrics stub |
| 5 | **PH-S168** | Galaxy pricing cache age /metrics |
| 6 | **PH-S169** | Locality stale profile penalty stub |

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

## PH-S164 — scope

- `galaxy_verify_sampling` у HTTP/grid middleware stub; tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S164

```
PoolAI — спринт PH-S164 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S164 — Verify sampling env apply
Scope: galaxy_verify_sampling у HTTP/grid middleware stub; tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
