# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S176 ✅ · vision **rev 112** · **5** відкритих (PH-S177…S181) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S177** — Galaxy verification sample total metrics stub |
| **Відкритих** | **5** (PH-S177…S181) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S177…S181)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S176 | Galaxy replay pending metrics stub | mismatch → `galaxy_replay_pending` gauge |
| PH-S175 | Galaxy verification mismatch metrics stub | `verification_verdict: mismatch` → `/metrics` |
| PH-S174 | Galaxy pricing quote usd_micro metrics stub | last quote gauge on `/metrics` |

### Відкрито — replenish (PH-S177…S181)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S177** | Galaxy verification sample total metrics stub |
| 2 | **PH-S178** | Galaxy settlement pending_verification metrics stub |
| 3 | **PH-S179** | Galaxy replication strict tier metrics stub |
| 4 | **PH-S180** | Galaxy verification match metrics stub |
| 5 | **PH-S181** | Galaxy pricing market min usd_micro metrics stub |

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

## PH-S177 — scope

- `galaxy_verification_sample_total` counter stub; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S177

```
PoolAI — спринт PH-S177 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S177 — Galaxy verification sample total metrics stub
Scope: galaxy_verification_sample_total counter stub; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
