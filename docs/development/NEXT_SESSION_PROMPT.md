# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S194 ✅ · vision **rev 132** · **6** відкритих (PH-S195…S200) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S195** — Galaxy seed_inventory GET stub |
| **Відкритих** | **6** (PH-S195…S200) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (6 відкритих: PH-S195…S200)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S194 | Galaxy fee split result counter | `galaxy_fee_split_applied_total` on grid result `gross_lamports` wire |
| PH-S193 | Dashboard shell formatters → wasm | wasm formatters; slim JS glue in `src/ui/mod.rs` |
| PH-S192 | Vision overview LOD + minimap | hub LOD; minimap inset; panel dock bar UX (rev 132) |

### Відкрито — vision + code band (PH-S195…S200)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S195** | Galaxy seed_inventory GET stub |
| 2 | **PH-S196** | Stand smoke jobs lease renew |
| 3 | **PH-S197** | Admin updates-compat wasm wiring |
| 4 | **PH-S198** | Topology graph Rust labels slim |
| 5 | **PH-S199** | Vision feed.json RSS ticker |
| 6 | **PH-S200** | Cursor post-push PH-S* hook |

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

## PH-S195 — scope

- read-only `GET /api/v1/grid/seed-inventory`; OpenAPI; integration test; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S195

```
PoolAI — спринт PH-S195 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S195 — Galaxy seed_inventory GET stub
Scope: GET /api/v1/grid/seed-inventory; OpenAPI; integration test; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
