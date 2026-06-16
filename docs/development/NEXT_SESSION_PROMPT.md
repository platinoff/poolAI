# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S204 ✅ · vision **rev 146** · **5** відкритих (PH-S205…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S205** — poolai-vision-sync manifest drift gate |
| **Відкритих** | **5** (PH-S205…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (5 відкритих: PH-S205…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S204 | Vision edge click neighbor select | edge click → `edgeTraceNodeId` + endpoint select + trace |
| PH-S203 | Vision keyboard nav linked nodes | Arrow keys cycle 1-hop neighbors |
| PH-S202 | Vision sprint-queue → map focus | queue card → `focusMapNode` |

### Відкрито — vision + code band (PH-S205…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S205** | poolai-vision-sync manifest drift gate |
| 2 | **PH-S206** | Vision minimap selection ring |
| 3 | **PH-S207** | Admin i18n slim next panel |
| 4 | **PH-S208** | Stand smoke vision revision parity |
| 5 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S205 — scope

- `poolai-vision-sync` / CI — manifest revision drift gate vs FM; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S205

```
PoolAI — спринт PH-S205 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S205 — poolai-vision-sync manifest drift gate
Scope: src/bin/poolai_vision_sync.rs drift check; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
