# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S172 ✅ · vision **rev 108** · **5** відкритих (PH-S173…S177) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S173** — Galaxy pricing provider errors metrics stub |
| **Відкритих** | **5** (PH-S173…S177) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S173…S177)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S172 | Galaxy pricing provider catalog metrics stub | catalog allow-list hits on `/metrics` |
| PH-S171 | Galaxy replication strict tier stub | `replication_strict` 3-of-3 tier |
| PH-S170 | Galaxy settlement pending_verification stub | `SettlementStatus::PendingVerification` |

### Відкрито — replenish (PH-S173…S177)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S173** | Galaxy pricing provider errors metrics stub |
| 2 | **PH-S174** | Galaxy pricing quote usd_micro metrics stub |
| 3 | **PH-S175** | Galaxy verification mismatch metrics stub |
| 4 | **PH-S176** | Galaxy replay pending metrics stub |
| 5 | **PH-S177** | Galaxy verification sample total metrics stub |

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

## PH-S173 — scope

- `galaxy_pricing_provider_errors_total` counter on provider fetch fail; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S173

```
PoolAI — спринт PH-S173 (одin PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S173 — Galaxy pricing provider errors metrics stub
Scope: galaxy_pricing_provider_errors_total counter on provider fetch fail; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
