# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S190 ✅ · vision **rev 127** · **10** відкритих (PH-S191…S200) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S191** — Vision sprint queue panel |
| **Відкритих** | **10** (PH-S191…S200) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (10 відкритих: PH-S191…S200)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S190 | Vision filter dropdowns + panel collapse | Layers/Types dropdown; − strip collapse; grid auto-fill |
| PH-S189 | Vision Eco/FX/Ms hover trace | tri-mode; 1-hop hover highlight |
| PH-S188 | Vision map filters UX | independent toggles; All/None |

### Відкрито — vision + code band (PH-S191…S200)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S191** | Vision sprint queue panel — Rust parse FM §5.12 |
| 2 | **PH-S192** | Vision overview LOD + minimap |
| 3 | **PH-S193** | Dashboard shell formatters → wasm |
| 4 | **PH-S194** | Galaxy fee split result counter stub |
| 5 | **PH-S195** | Galaxy seed_inventory GET stub |
| 6 | **PH-S196** | Stand smoke jobs lease renew |
| 7 | **PH-S197** | Admin updates-compat wasm wiring |
| 8 | **PH-S198** | Topology graph Rust labels slim |
| 9 | **PH-S199** | Vision feed.json RSS ticker |
| 10 | **PH-S200** | Cursor post-push PH-S* hook |

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

## PH-S191 — scope

- `poolai-vision-sync` або vision panel: parse FM §5.12 → `sprint_queue` UI; rev++
- Acceptance: manual vision check; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S191

```
PoolAI — спринт PH-S191 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S191 — Vision sprint queue panel
Scope: Rust parse FM §5.12 → sprint_queue panel in docs/vision; rev++

Acceptance: vision manual check; FM/HANDOFF/NEXT/vision; git push main
```
