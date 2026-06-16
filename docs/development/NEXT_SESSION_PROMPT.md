# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S208 ✅ · vision **rev 150** · **1** відкритий (PH-S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S209** — Vision map a11y focus ring |
| **Відкритих** | **1** (PH-S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (1 відкритий: PH-S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S208 | Stand smoke vision revision parity | `X-PoolAI-Vision-Revision` + FM/manifest |
| PH-S207 | Admin i18n slim monitoring panel | `admin.mon.*` → `poolai-ui-core` |
| PH-S206 | Vision minimap selection ring | `#minimap-selection-ring` |

### Відкрито

| Sprint | Scope |
|--------|-------|
| **PH-S209** | Vision map a11y focus ring |

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

## PH-S209 — scope

- `docs/vision/vision.js` / `vision.css` — keyboard focus-visible on map controls + nodes
- Acceptance: FM/HANDOFF/NEXT; `cargo test-ci`; push

---

## Copy-paste — PH-S209

```
PoolAI — спринт PH-S209 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S209 — Vision map a11y focus ring
Scope: docs/vision focus-visible ring; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
