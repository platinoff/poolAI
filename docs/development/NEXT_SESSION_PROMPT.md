# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S207 ✅ · vision **rev 149** · **2** відкритих (PH-S208…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S208** — Stand smoke vision revision parity |
| **Відкритих** | **2** (PH-S208…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (2 відкритих: PH-S208…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S207 | Admin i18n slim monitoring panel | `admin.mon.*` → `poolai-ui-core` |
| PH-S206 | Vision minimap selection ring | `#minimap-selection-ring` |
| PH-S205 | poolai-vision-sync manifest drift gate | `--check` vs FM §5.12 |

### Відкрито

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S208** | Stand smoke vision revision parity |
| 2 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S208 — scope

- `poolai-http-stand-smoke` checks vision rev header vs manifest
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S208

```
PoolAI — спринт PH-S208 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S208 — Stand smoke vision revision parity
Scope: poolai-http-stand-smoke vision rev check; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
