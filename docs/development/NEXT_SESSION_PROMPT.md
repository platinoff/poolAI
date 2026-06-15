# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S188 ✅ · vision **rev 125** · **3** відкритих (PH-S189…S191) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S189** — Vision Eco/FX/Ms hover trace |
| **Відкритих** | **3** (PH-S189…S191) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (3 відкритих: PH-S189…S191)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S188 | Vision map filters UX | independent toggles; All/None; decouple 3D stack ↔ map chips |
| PH-S187 | Galaxy settlement cleared total metrics stub | Cleared path → `galaxy_settlement_cleared_total` |
| PH-S186 | Galaxy verification sample scheduled /metrics export | PH-S164 counter → `GET /metrics` |

### Відкрито — vision UX band (PH-S189…S191)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S189** | Vision Eco/FX/**Ms** hover trace — 1-hop edge highlight on mouse |
| 2 | **PH-S190** | Vision overview LOD + minimap — readable map without zoom |
| 3 | **PH-S191** | Vision sprint queue panel + `feed.json` RSS + Cursor post-push hook |

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

## PH-S189 — scope

- Tri-mode **Eco→FX→Ms**; hover 1-hop edge/node highlight; `localStorage`; rev++
- Acceptance: manual vision check; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S189

```
PoolAI — спринт PH-S189 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S189 — Vision Eco/FX/Ms hover trace
Scope: tri-mode Eco→FX→Ms; hover 1-hop edge/node highlight; localStorage; rev++

Acceptance: vision manual check; FM/HANDOFF/NEXT/vision; git push main
```
