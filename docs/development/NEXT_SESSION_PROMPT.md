# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S185 ✅ · vision **rev 122** · **6** відкритих (PH-S186…S191) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S186** — Galaxy verification sample scheduled /metrics export |
| **Відкритих** | **6** (PH-S186…S191) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (6 відкритих: PH-S186…S191)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S185 | Galaxy cross region egress mb metrics stub | rank/prefetch → `galaxy_cross_region_egress_mb` |
| PH-S184 | Galaxy prefetch bytes total metrics stub | `plan_prefetch` → `galaxy_prefetch_bytes_total` |
| PH-S183 | Galaxy shard local hit ratio metrics stub | rank path → `galaxy_shard_local_hit_ratio` |

### Відкрито — metrics band (PH-S186…S187)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S186** | Galaxy verification sample scheduled /metrics export |
| 2 | **PH-S187** | Galaxy settlement cleared total metrics stub |

### Відкрито — vision UX band (PH-S188…S191)

| # | Sprint | Scope |
|---|--------|-------|
| 3 | **PH-S188** | Vision map filters UX — independent toggles; LAYERS/TYPES select-all |
| 4 | **PH-S189** | Vision Eco/FX/**Ms** hover trace — 1-hop edge highlight on mouse |
| 5 | **PH-S190** | Vision overview LOD + minimap — readable map without zoom |
| 6 | **PH-S191** | Vision sprint queue panel + `feed.json` RSS + Cursor post-push hook |

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

## PH-S186 — scope

- `galaxy_verification_sample_scheduled_total` on `GET /metrics`; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S186

```
PoolAI — спринт PH-S186 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S186 — Galaxy verification sample scheduled /metrics export
Scope: galaxy_verification_sample_scheduled_total on GET /metrics; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
