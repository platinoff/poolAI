# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S184 ✅ · vision **rev 121** · **7** відкритих (PH-S185…S191) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S185** — Galaxy cross region egress mb metrics stub |
| **Відкритих** | **7** (PH-S185…S191) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (7 відкритих: PH-S185…S191)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S184 | Galaxy prefetch bytes total metrics stub | `plan_prefetch` → `galaxy_prefetch_bytes_total` |
| PH-S183 | Galaxy shard local hit ratio metrics stub | rank path → `galaxy_shard_local_hit_ratio` |
| PH-S182 | Galaxy trust score metrics stub | grid result → `galaxy_trust_score` gauge |

### Відкрито — metrics band (PH-S185…S187)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S185** | Galaxy cross region egress mb metrics stub |
| 2 | **PH-S186** | Galaxy verification sample scheduled /metrics export |
| 3 | **PH-S187** | Galaxy settlement cleared total metrics stub |

### Відкрито — vision UX band (PH-S188…S191)

| # | Sprint | Scope |
|---|--------|-------|
| 4 | **PH-S188** | Vision map filters UX — independent toggles; LAYERS/TYPES select-all |
| 5 | **PH-S189** | Vision Eco/FX/**Ms** hover trace — 1-hop edge highlight on mouse |
| 6 | **PH-S190** | Vision overview LOD + minimap — readable map without zoom |
| 7 | **PH-S191** | Vision sprint queue panel + `feed.json` RSS + Cursor post-push hook |

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

## PH-S185 — scope

- `galaxy_cross_region_egress_mb` gauge stub on rank/prefetch path; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S185

```
PoolAI — спринт PH-S185 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S185 — Galaxy cross region egress mb metrics stub
Scope: galaxy_cross_region_egress_mb gauge on rank/prefetch path; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
