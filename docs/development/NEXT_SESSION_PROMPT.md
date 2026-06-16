# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S201 ✅ · vision **rev 140** · **8** відкритих (PH-S202…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S202** — Vision sprint-queue chip → map focus |
| **Відкритих** | **8** (PH-S202…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (8 відкритих: PH-S202…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S201 | Cursor post-push PH-S* hook | `postToolUse` → `post-push-ph-s-notify.sh`; VDT docs-sync checklist |
| PH-S200 | Vision feed.json RSS ticker | `poolai-vision-sync` → `feed.json`; header ticker panel |
| PH-S199 | Vision map Ms hit-test + focus nav | planes pass-through; edge trace; click focus |

### Відкрито — vision + code band (PH-S202…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S202** | Vision sprint-queue chip → map focus |
| 2 | **PH-S203** | Vision keyboard nav linked nodes |
| 3 | **PH-S204** | Vision edge click neighbor select |
| 4 | **PH-S205** | poolai-vision-sync manifest drift gate |
| 5 | **PH-S206** | Vision minimap selection ring |
| 6 | **PH-S207** | Admin i18n slim next panel |
| 7 | **PH-S208** | Stand smoke vision revision parity |
| 8 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S202 — scope

- `docs/vision/` — sprint queue card click centers map node; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S202

```
PoolAI — спринт PH-S202 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S202 — Vision sprint-queue chip → map focus
Scope: docs/vision/vision.js + sprint queue click; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
