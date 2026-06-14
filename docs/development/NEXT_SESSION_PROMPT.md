# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S166 ✅ · vision **rev 102** · **5** відкритих (PH-S167…S171) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S167** — Galaxy prefetch metrics stub |
| **Відкритих** | **5** (PH-S167…S171) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S167…S171)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S166 | Design tokens CSS → Rust | `design_tokens.rs` + slim CSS files |
| PH-S165 | Ratio 96% hold gate | CI `--min-ratio 0.95` advisory; target **95%** |
| PH-S164 | Verify sampling env apply | middleware header + dispatch stub |

### Відкрито — replenish (PH-S167…S171)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S167** | Galaxy prefetch metrics stub |
| 2 | **PH-S168** | Galaxy pricing cache age /metrics |
| 3 | **PH-S169** | Locality stale profile penalty stub |
| 4 | **PH-S170** | Galaxy settlement pending_verification stub |
| 5 | **PH-S171** | Galaxy replication strict tier stub |

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

## PH-S167 — scope

- Prometheus counters on `plan_prefetch`; unit tests; no live enqueue
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S167

```
PoolAI — спринт PH-S167 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S167 — Galaxy prefetch metrics stub
Scope: Prometheus counters on plan_prefetch; unit tests; no live enqueue

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
