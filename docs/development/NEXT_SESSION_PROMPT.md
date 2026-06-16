# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S203 ✅ · vision **rev 144** · **6** відкритих (PH-S204…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S204** — Vision edge click neighbor select |
| **Відкритих** | **6** (PH-S204…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (6 відкритих: PH-S204…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S203 | Vision keyboard nav linked nodes | Arrow keys cycle 1-hop manifest neighbors on map |
| PH-S202 | Vision sprint-queue → map focus | queue card click → `focusMapNode` |
| PH-S201 | Cursor post-push PH-S* hook | `post-push-ph-s-notify.sh` |

### Відкрито — vision + code band (PH-S204…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S204** | Vision edge click neighbor select |
| 2 | **PH-S205** | poolai-vision-sync manifest drift gate |
| 3 | **PH-S206** | Vision minimap selection ring |
| 4 | **PH-S207** | Admin i18n slim next panel |
| 5 | **PH-S208** | Stand smoke vision revision parity |
| 6 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S204 — scope

- `docs/vision/vision.js` — edge click → trace + select endpoint; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S204

```
PoolAI — спринт PH-S204 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S204 — Vision edge click neighbor select
Scope: docs/vision/vision.js edge click handler; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
