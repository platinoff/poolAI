# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S164 ✅ · vision **rev 99** · **5** відкритих (PH-S165…S169) · **stretch spirit 96%**

| **← наступний** | **PH-S165** — Ratio 96% hold gate |
| **Відкритих** | **5** (PH-S165…S169) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: maintain S165 + replenish S166…S169)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S164 | Verify sampling env apply | middleware header + dispatch stub; integration test |
| PH-S163 | Galaxy trust metrics wire | grid result → Prometheus gauges |
| PH-S162 | Auth i18n subset Rust | `i18n.rs` auth+dash shell |

### Відкрито — maintain (PH-S165)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S165** | **96%** hold gate |

### Відкрито — replenish (PH-S166…S169)

| # | Sprint | Scope |
|---|--------|-------|
| 2 | **PH-S166** | Design tokens CSS → Rust |
| 3 | **PH-S167** | Galaxy prefetch metrics stub |
| 4 | **PH-S168** | Galaxy pricing cache age /metrics |
| 5 | **PH-S169** | Locality stale profile penalty stub |

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

## PH-S165 — scope

- CI `--min-ratio 0.95`; maintain spirit **96%**
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S165

```
PoolAI — спринт PH-S165 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S165 — Ratio 96% hold band gate
Scope: CI --min-ratio 0.95; maintain stretch spirit 96%

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
