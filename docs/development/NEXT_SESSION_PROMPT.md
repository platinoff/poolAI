# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S202 ✅ · vision **rev 142** · **7** відкритих (PH-S203…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S203** — Vision keyboard nav linked nodes |
| **Відкритих** | **7** (PH-S203…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (7 відкритих: PH-S203…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S202 | Vision sprint-queue → map focus | queue card click → `focusMapNode` + `selectNode`; `map-linked` chip |
| PH-S201 | Cursor post-push PH-S* hook | `postToolUse` → `post-push-ph-s-notify.sh` |
| PH-S200 | Vision feed.json RSS ticker | `feed.json` header marquee |

### Відкрито — vision + code band (PH-S203…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S203** | Vision keyboard nav linked nodes |
| 2 | **PH-S204** | Vision edge click neighbor select |
| 3 | **PH-S205** | poolai-vision-sync manifest drift gate |
| 4 | **PH-S206** | Vision minimap selection ring |
| 5 | **PH-S207** | Admin i18n slim next panel |
| 6 | **PH-S208** | Stand smoke vision revision parity |
| 7 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S203 — scope

- `docs/vision/vision.js` — Arrow keys cycle 1-hop neighbors on map; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S203

```
PoolAI — спринт PH-S203 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S203 — Vision keyboard nav linked nodes
Scope: docs/vision/vision.js map keyboard nav; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
