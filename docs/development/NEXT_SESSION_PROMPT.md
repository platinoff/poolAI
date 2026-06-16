# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S205 ✅ · vision **rev 147** · **4** відкритих (PH-S206…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S206** — Vision minimap selection ring |
| **Відкритих** | **4** (PH-S206…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (4 відкритих: PH-S206…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S205 | poolai-vision-sync manifest drift gate | `--check` vs FM §5.12; CI job |
| PH-S204 | Vision edge click neighbor select | edge click → `edgeTraceNodeId` + endpoint select + trace |
| PH-S203 | Vision keyboard nav linked nodes | Arrow keys cycle 1-hop neighbors |

### Відкрито — vision + code band (PH-S206…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S206** | Vision minimap selection ring |
| 2 | **PH-S207** | Admin i18n slim next panel |
| 3 | **PH-S208** | Stand smoke vision revision parity |
| 4 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S206 — scope

- `docs/vision/vision.js` / `vision.css` — minimap viewport + selected node ring
- Acceptance: FM/HANDOFF/NEXT; `cargo test-ci`; push

---

## Copy-paste — PH-S206

```
PoolAI — спринт PH-S206 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S206 — Vision minimap selection ring
Scope: docs/vision/vision.js minimap ring; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
