# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S206 ✅ · vision **rev 148** · **3** відкритих (PH-S207…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S207** — Admin i18n slim next panel |
| **Відкритих** | **3** (PH-S207…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (3 відкритих: PH-S207…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S206 | Vision minimap selection ring | `#minimap-selection-ring` + viewport fill |
| PH-S205 | poolai-vision-sync manifest drift gate | `--check` vs FM §5.12; CI job |
| PH-S204 | Vision edge click neighbor select | edge click trace + endpoint select |

### Відкрито — vision + code band (PH-S207…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S207** | Admin i18n slim next panel |
| 2 | **PH-S208** | Stand smoke vision revision parity |
| 3 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S207 — scope

- Admin panel strings → `poolai-ui-core` i18n slim
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S207

```
PoolAI — спринт PH-S207 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S207 — Admin i18n slim next panel
Scope: poolai-ui-core i18n; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
