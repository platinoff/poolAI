# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S198 ✅ · vision **rev 136** · **2** відкритих (PH-S199…S200) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S199** — Vision feed.json RSS ticker |
| **Відкритих** | **2** (PH-S199…S200) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (2 відкритих: PH-S199…S200)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S198 | Topology hub labels Rust | `topology_graph.rs` label coords; slim `topology_graph.js` |
| PH-S197 | Admin updates-compat wasm | wasm labels; i18n → Rust |
| PH-S196 | Stand smoke lease renew | `poolai-http-stand-smoke --lease-renew` |

### Відкрито — vision + code band (PH-S199…S200)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S199** | Vision feed.json RSS ticker |
| 2 | **PH-S200** | Cursor post-push PH-S* hook |

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

## PH-S199 — scope

- `docs/vision/feed.json` RSS ticker panel; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S199

```
PoolAI — спринт PH-S199 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S199 — Vision feed.json RSS ticker
Scope: docs/vision/feed.json + index panel; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
