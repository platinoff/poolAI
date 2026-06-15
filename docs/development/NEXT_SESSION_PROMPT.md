# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-15 · PH-S196 ✅ · vision **rev 134** · **4** відкритих (PH-S197…S200) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S197** — Admin updates-compat wasm wiring |
| **Відкритих** | **4** (PH-S197…S200) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (4 відкритих: PH-S197…S200)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S196 | Stand smoke lease renew | `poolai-http-stand-smoke --lease-renew`; e2e stand gate |
| PH-S195 | Galaxy seed_inventory GET | `GET /api/v1/grid/seed-inventory` coordinator stub |
| PH-S194 | Galaxy fee split result counter | `galaxy_fee_split_applied_total` on grid result wire |

### Відкрито — vision + code band (PH-S197…S200)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S197** | Admin updates-compat wasm wiring |
| 2 | **PH-S198** | Topology graph Rust labels slim |
| 3 | **PH-S199** | Vision feed.json RSS ticker |
| 4 | **PH-S200** | Cursor post-push PH-S* hook |

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

## PH-S197 — scope

- `/ui/admin/updates-compat` → wasm labels; Playwright smoke; `cargo test-ci`
- Acceptance: FM/HANDOFF/NEXT; push

---

## Copy-paste — PH-S197

```
PoolAI — спринт PH-S197 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S197 — Admin updates-compat wasm wiring
Scope: /ui/admin/updates-compat wasm labels; cargo test-ci

Acceptance: cargo test-ci; FM/HANDOFF/NEXT; git push main
```
