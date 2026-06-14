# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S186 ✅ · vision **rev 123** · **5** відкритих (PH-S187…S191) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S187** — Galaxy settlement cleared total metrics stub |
| **Відкритих** | **5** (PH-S187…S191) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (5 відкритих: PH-S187…S191)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S186 | Galaxy verification sample scheduled /metrics export | PH-S164 counter → `GET /metrics` |
| PH-S185 | Galaxy cross region egress mb metrics stub | rank/prefetch → `galaxy_cross_region_egress_mb` |
| PH-S184 | Galaxy prefetch bytes total metrics stub | `plan_prefetch` → `galaxy_prefetch_bytes_total` |

### Відкрито — metrics band (PH-S187)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S187** | Galaxy settlement cleared total metrics stub |

### Відкрито — vision UX band (PH-S188…S191)

| # | Sprint | Scope |
|---|--------|-------|
| 2 | **PH-S188** | Vision map filters UX — independent toggles; LAYERS/TYPES select-all |
| 3 | **PH-S189** | Vision Eco/FX/**Ms** hover trace — 1-hop edge highlight on mouse |
| 4 | **PH-S190** | Vision overview LOD + minimap — readable map without zoom |
| 5 | **PH-S191** | Vision sprint queue panel + `feed.json` RSS + Cursor post-push hook |

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

## PH-S187 — scope

- `galaxy_settlement_cleared_total` counter on grid result Cleared path; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S187

```
PoolAI — спринт PH-S187 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S187 — Galaxy settlement cleared total metrics stub
Scope: galaxy_settlement_cleared_total counter on grid result Cleared path; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
