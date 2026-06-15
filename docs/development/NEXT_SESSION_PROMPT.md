# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S192 ✅ · vision **rev 130** · **8** відкритих (PH-S193…S200) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S193** — Dashboard shell formatters → wasm |
| **Відкритих** | **8** (PH-S193…S200) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (8 відкритих: PH-S193…S200)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S192 | Vision overview LOD + minimap | `map-overview` hub LOD; minimap inset; collapsed panel short titles |
| PH-S191 | Vision sprint queue panel | Rust parse FM §5.12 → `sprint_queue` panel |
| PH-S190 | Vision filter dropdowns + panel collapse | Layers/Types dropdown; grid auto-fill |

### Відкрито — vision + code band (PH-S193…S200)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S193** | Dashboard shell formatters → wasm |
| 2 | **PH-S194** | Galaxy fee split result counter stub |
| 3 | **PH-S195** | Galaxy seed_inventory GET stub |
| 4 | **PH-S196** | Stand smoke jobs lease renew |
| 5 | **PH-S197** | Admin updates-compat wasm wiring |
| 6 | **PH-S198** | Topology graph Rust labels slim |
| 7 | **PH-S199** | Vision feed.json RSS ticker |
| 8 | **PH-S200** | Cursor post-push PH-S* hook |

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

## PH-S193 — scope

- `crates/poolai-ui-core` + wasm — login/dashboard formatters; slim JS glue; `cargo test-ci`
- Acceptance: Playwright/admin smoke if UI touched; FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S193

```
PoolAI — спринт PH-S193 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S193 — Dashboard shell formatters → wasm
Scope: poolai-ui-core + wasm login/dashboard formatters; slim JS glue; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
