# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S182 ✅ · vision **rev 119** · **9** відкритих (PH-S183…S191) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S183** — Galaxy shard local hit ratio metrics stub |
| **Відкритих** | **9** (PH-S183…S191) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (9 відкритих: PH-S183…S191)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S182 | Galaxy trust score metrics stub | grid result → `galaxy_trust_score` gauge |
| PH-S181 | Galaxy pricing market min usd_micro metrics stub | `try_quote` → `galaxy_pricing_market_min_usd_micro` |
| PH-S180 | Galaxy verification match metrics stub | `verification_verdict: match` → `galaxy_verification_match_total` |

### Відкрито — metrics band (PH-S183…S187)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S183** | Galaxy shard local hit ratio metrics stub |
| 2 | **PH-S184** | Galaxy prefetch bytes total metrics stub |
| 3 | **PH-S185** | Galaxy cross region egress mb metrics stub |
| 4 | **PH-S186** | Galaxy verification sample scheduled /metrics export |
| 5 | **PH-S187** | Galaxy settlement cleared total metrics stub |

### Відкрито — vision UX band (PH-S188…S191)

| # | Sprint | Scope |
|---|--------|-------|
| 6 | **PH-S188** | Vision map filters UX — independent toggles; LAYERS/TYPES select-all |
| 7 | **PH-S189** | Vision Eco/FX/**Ms** hover trace — 1-hop edge highlight on mouse |
| 8 | **PH-S190** | Vision overview LOD + minimap — readable map without zoom |
| 9 | **PH-S191** | Vision sprint queue panel + `feed.json` RSS + Cursor post-push hook |

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

## PH-S183 — scope

- `galaxy_shard_local_hit_ratio` gauge stub on locality rank path; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S183

```
PoolAI — спринт PH-S183 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S183 — Galaxy shard local hit ratio metrics stub
Scope: galaxy_shard_local_hit_ratio gauge on locality rank path; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```

---

## Copy-paste — PH-S188 (vision, після S187)

```
PoolAI — спринт PH-S188 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md
Vision: docs/vision/manifest.json · docs/vision/README.md

Спринт PH-S188 — Vision map filters UX
Scope: independent layer/type chips; LAYERS/TYPES header select-all/none; decouple 3D stack from map filters (vision.js)
Research: docs/vision/vision.js (toggleLayerChip, renderMapFilterDock, setMapLayerFocus)

Acceptance: manual UX in open-docs-vision.ps1; docs/vision/README.md; manifest rev++; FM/HANDOFF/NEXT; git push main
```
