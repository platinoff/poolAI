# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S169 ✅ · vision **rev 105** · **5** відкритих (PH-S170…S174) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S170** — Galaxy settlement pending_verification stub |
| **Відкритих** | **5** (PH-S170…S174) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S170…S174)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S169 | Locality stale profile penalty stub | `stale_network_profile_penalty` у `locality_score` |
| PH-S168 | Galaxy pricing cache age /metrics | `galaxy_pricing_cache_age_seconds` gauge on L1 hit |
| PH-S167 | Galaxy prefetch metrics stub | `plan_prefetch` → Prometheus counters |

### Відкрито — replenish (PH-S170…S174)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S170** | Galaxy settlement pending_verification stub |
| 2 | **PH-S171** | Galaxy replication strict tier stub |
| 3 | **PH-S172** | Galaxy pricing provider catalog metrics stub |
| 4 | **PH-S173** | Galaxy pricing provider errors metrics stub |
| 5 | **PH-S174** | Galaxy pricing quote usd_micro metrics stub |

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

## PH-S170 — scope

- `pending_verification` verdict stub on grid result path; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S170

```
PoolAI — спринт PH-S170 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S170 — Galaxy settlement pending_verification stub
Scope: pending_verification verdict stub on grid result path; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
