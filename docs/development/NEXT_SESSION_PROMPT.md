# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S193 ✅ · vision **rev 131** · **7** відкритих (PH-S194…S200) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S194** — Galaxy fee split result counter stub |
| **Відкритих** | **7** (PH-S194…S200) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (7 відкритих: PH-S194…S200)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S193 | Dashboard shell formatters → wasm | `formatIsoDatetime`/`formatLocaleTimeHms`/`escapeRegex` wasm; slim JS glue in `src/ui/mod.rs` |
| PH-S192 | Vision overview LOD + minimap | `map-overview` hub LOD; minimap inset; collapsed panel short titles |
| PH-S191 | Vision sprint queue panel | Rust parse FM §5.12 → `sprint_queue` panel |

### Відкрито — vision + code band (PH-S194…S200)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S194** | Galaxy fee split result counter stub |
| 2 | **PH-S195** | Galaxy seed_inventory GET stub |
| 3 | **PH-S196** | Stand smoke jobs lease renew |
| 4 | **PH-S197** | Admin updates-compat wasm wiring |
| 5 | **PH-S198** | Topology graph Rust labels slim |
| 6 | **PH-S199** | Vision feed.json RSS ticker |
| 7 | **PH-S200** | Cursor post-push PH-S* hook |

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

## PH-S194 — scope

- `galaxy_fee_split_applied_total` counter on grid result path; unit tests; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S194

```
PoolAI — спринт PH-S194 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S194 — Galaxy fee split result counter stub
Scope: galaxy_fee_split_applied_total on grid result path; unit tests; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
