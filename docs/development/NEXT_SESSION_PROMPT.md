# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S168 ✅ · vision **rev 104** · **5** відкритих (PH-S169…S173) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S169** — Locality stale profile penalty stub |
| **Відкритих** | **5** (PH-S169…S173) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S169…S173)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S168 | Galaxy pricing cache age /metrics | `galaxy_pricing_cache_age_seconds` gauge on L1 hit |
| PH-S167 | Galaxy prefetch metrics stub | `plan_prefetch` → Prometheus counters |
| PH-S166 | Design tokens CSS → Rust | `design_tokens.rs` + slim CSS files |

### Відкрито — replenish (PH-S169…S173)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S169** | Locality stale profile penalty stub |
| 2 | **PH-S170** | Galaxy settlement pending_verification stub |
| 3 | **PH-S171** | Galaxy replication strict tier stub |
| 4 | **PH-S172** | Galaxy pricing provider catalog metrics stub |
| 5 | **PH-S173** | Galaxy pricing provider errors metrics stub |

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

## PH-S169 — scope

- `stale_network_profile_penalty` у `galaxy_locality.rs`; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S169

```
PoolAI — спринт PH-S169 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S169 — Locality stale profile penalty stub
Scope: stale_network_profile_penalty у galaxy_locality.rs; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
