# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S191 ✅ · vision **rev 129** · **9** відкритих (PH-S192…S200) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S192** — Vision overview LOD + minimap |
| **Відкритих** | **9** (PH-S192…S200) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (9 відкритих: PH-S192…S200)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S191 | Vision sprint queue panel | Rust parse FM §5.12 → `sprint_queue` panel; collapse/grid UX polish |
| PH-S190 | Vision filter dropdowns + panel collapse | Layers/Types dropdown; − strip collapse; grid auto-fill |
| PH-S189 | Vision Eco/FX/Ms hover trace | tri-mode; 1-hop hover highlight |

### Відкрито — vision + code band (PH-S192…S200)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S192** | Vision overview LOD + minimap |
| 2 | **PH-S193** | Dashboard shell formatters → wasm |
| 3 | **PH-S194** | Galaxy fee split result counter stub |
| 4 | **PH-S195** | Galaxy seed_inventory GET stub |
| 5 | **PH-S196** | Stand smoke jobs lease renew |
| 6 | **PH-S197** | Admin updates-compat wasm wiring |
| 7 | **PH-S198** | Topology graph Rust labels slim |
| 8 | **PH-S199** | Vision feed.json RSS ticker |
| 9 | **PH-S200** | Cursor post-push PH-S* hook |

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

## PH-S192 — scope

- `docs/vision/vision.js` — `map-overview` при low zoom; hub-only labels; viewport inset minimap; rev++
- Acceptance: manual vision check; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S192

```
PoolAI — спринт PH-S192 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S192 — Vision overview LOD + minimap
Scope: map-overview low zoom; hub labels; minimap inset in docs/vision; rev++

Acceptance: vision manual check; FM/HANDOFF/NEXT/vision; git push main
```
