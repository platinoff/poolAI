# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-16 · PH-S200 ✅ · vision **rev 139** · **9** відкритих (PH-S201…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S201** — Cursor post-push PH-S* hook |
| **Відкритих** | **9** (PH-S201…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (9 відкритих: PH-S201…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S200 | Vision feed.json RSS ticker | `poolai-vision-sync` → `feed.json`; header ticker panel; click → sprint queue |
| PH-S199 | Vision map Ms hit-test + focus nav | planes pass-through; edge trace; click focus ~14px; zoom back; sidebar scroll |
| PH-S198 | Topology hub labels Rust | `topology_graph.rs` label coords; slim `topology_graph.js` |

### Відкрито — vision + code band (PH-S201…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S201** | Cursor post-push PH-S* hook |
| 2 | **PH-S202** | Vision sprint-queue chip → map focus |
| 3 | **PH-S203** | Vision keyboard nav linked nodes |
| 4 | **PH-S204** | Vision edge click neighbor select |
| 5 | **PH-S205** | poolai-vision-sync manifest drift gate |
| 6 | **PH-S206** | Vision minimap selection ring |
| 7 | **PH-S207** | Admin i18n slim next panel |
| 8 | **PH-S208** | Stand smoke vision revision parity |
| 9 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S201 — scope

- `.cursor/hooks` post-push notify after PH-S* close; docs sync pointer; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S201

```
PoolAI — спринт PH-S201 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S201 — Cursor post-push PH-S* hook
Scope: .cursor/hooks + docs sync pointer; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
