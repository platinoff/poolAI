# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S171 ✅ · vision **rev 107** · **5** відкритих (PH-S172…S176) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S172** — Galaxy pricing provider catalog metrics stub |
| **Відкритих** | **5** (PH-S172…S176) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S172…S176)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S171 | Galaxy replication strict tier stub | `replication_strict` 3-of-3 tier on grid job ingest |
| PH-S170 | Galaxy settlement pending_verification stub | `SettlementStatus::PendingVerification` on grid result |
| PH-S169 | Locality stale profile penalty stub | `stale_network_profile_penalty` у `locality_score` |

### Відкрито — replenish (PH-S172…S176)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S172** | Galaxy pricing provider catalog metrics stub |
| 2 | **PH-S173** | Galaxy pricing provider errors metrics stub |
| 3 | **PH-S174** | Galaxy pricing quote usd_micro metrics stub |
| 4 | **PH-S175** | Galaxy verification mismatch metrics stub |
| 5 | **PH-S176** | Galaxy replay pending metrics stub |

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

## PH-S172 — scope

- Prometheus counters on provider allow-list hits; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S172

```
PoolAI — спринт PH-S172 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S172 — Galaxy pricing provider catalog metrics stub
Scope: Prometheus counters on provider allow-list hits; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
