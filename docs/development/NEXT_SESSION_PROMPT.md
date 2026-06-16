# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S199 ✅ · vision **rev 137** · **10** відкритих (PH-S200…S209) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S200** — Vision feed.json RSS ticker |
| **Відкритих** | **10** (PH-S200…S209) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (10 відкритих: PH-S200…S209)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S199 | Vision map Ms hit-test + focus nav | planes pass-through; edge trace; click focus ~14px; zoom back; sidebar scroll |
| PH-S198 | Topology hub labels Rust | `topology_graph.rs` label coords; slim `topology_graph.js` |
| PH-S197 | Admin updates-compat wasm | wasm labels; i18n → Rust |

### Відкрито — vision + code band (PH-S200…S209)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S200** | Vision feed.json RSS ticker |
| 2 | **PH-S201** | Cursor post-push PH-S* hook |
| 3 | **PH-S202** | Vision sprint-queue chip → map focus |
| 4 | **PH-S203** | Vision keyboard nav linked nodes |
| 5 | **PH-S204** | Vision edge click neighbor select |
| 6 | **PH-S205** | poolai-vision-sync manifest drift gate |
| 7 | **PH-S206** | Vision minimap selection ring |
| 8 | **PH-S207** | Admin i18n slim next panel |
| 9 | **PH-S208** | Stand smoke vision revision parity |
| 10 | **PH-S209** | Vision map a11y focus ring |

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

## PH-S200 — scope

- `docs/vision/feed.json` RSS ticker panel; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S200

```
PoolAI — спринт PH-S200 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S200 — Vision feed.json RSS ticker
Scope: docs/vision/feed.json + index panel; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
