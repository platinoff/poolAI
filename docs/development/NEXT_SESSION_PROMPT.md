# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S173 ✅ · vision **rev 109** · **5** відкритих (PH-S174…S178) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S174** — Galaxy pricing quote usd_micro metrics stub |
| **Відкритих** | **5** (PH-S174…S178) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S174…S178)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S173 | Galaxy pricing provider errors metrics stub | provider fetch fail → `/metrics` |
| PH-S172 | Galaxy pricing provider catalog metrics stub | catalog allow-list hits on `/metrics` |
| PH-S171 | Galaxy replication strict tier stub | `replication_strict` 3-of-3 tier |

### Відкрито — replenish (PH-S174…S178)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S174** | Galaxy pricing quote usd_micro metrics stub |
| 2 | **PH-S175** | Galaxy verification mismatch metrics stub |
| 3 | **PH-S176** | Galaxy replay pending metrics stub |
| 4 | **PH-S177** | Galaxy verification sample total metrics stub |
| 5 | **PH-S178** | Galaxy settlement pending_verification metrics stub |

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

## PH-S174 — scope

- `galaxy_pricing_quote_usd_micro` gauge on last quote; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S174

```
PoolAI — спринт PH-S174 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S174 — Galaxy pricing quote usd_micro metrics stub
Scope: galaxy_pricing_quote_usd_micro gauge on last quote; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
