# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S165 ✅ · vision **rev 101** · **5** відкритих (PH-S166…S170) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S166** — Design tokens CSS → Rust |
| **Відкритих** | **5** (PH-S166…S170) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S166…S170)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S165 | Ratio 96% hold gate | CI `--min-ratio 0.95` advisory; target **95%**; spirit **96%** |
| PH-S164 | Verify sampling env apply | middleware header + dispatch stub; integration test |
| PH-S163 | Galaxy trust metrics wire | grid result → Prometheus gauges |

### Відкрито — replenish (PH-S166…S170)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S166** | Design tokens CSS → Rust |
| 2 | **PH-S167** | Galaxy prefetch metrics stub |
| 3 | **PH-S168** | Galaxy pricing cache age /metrics |
| 4 | **PH-S169** | Locality stale profile penalty stub |
| 5 | **PH-S170** | Galaxy settlement pending_verification stub |

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

## PH-S166 — scope

- CSS var map у `poolai-ui-core`; slim `design_tokens.css` / `admin_styles.css`
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S166

```
PoolAI — спринт PH-S166 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S166 — Design tokens CSS → Rust
Scope: CSS var map у poolai-ui-core; slim design_tokens.css / admin_styles.css

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
