# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S178 ✅ · vision **rev 115** · **10** відкритих (PH-S179…S187) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S179** — Galaxy replication strict tier metrics stub |
| **Відкритих** | **10** (PH-S179…S187) |
| **VDT** | один PH-S* = 1 commit (code) + docs sync |

---

## Зріз §5.12 (10 відкритих: PH-S179…S187)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S178 | Galaxy settlement pending_verification metrics stub | `PendingVerification` → `galaxy_settlement_pending_verification_total` |
| PH-S177 | Galaxy verification sample total metrics stub | edge sample → `galaxy_verification_sample_total` |
| PH-S176 | Galaxy replay pending metrics stub | mismatch → `galaxy_replay_pending` gauge |

### Відкрито — replenish (PH-S179…S187)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S179** | Galaxy replication strict tier metrics stub |
| 2 | **PH-S180** | Galaxy verification match metrics stub |
| 3 | **PH-S181** | Galaxy pricing market min usd_micro metrics stub |
| 4 | **PH-S182** | Galaxy trust score metrics stub |
| 5 | **PH-S183** | Galaxy shard local hit ratio metrics stub |
| 6 | **PH-S184** | Galaxy prefetch bytes total metrics stub |
| 7 | **PH-S185** | Galaxy cross region egress mb metrics stub |
| 8 | **PH-S186** | Galaxy verification sample scheduled /metrics export |
| 9 | **PH-S187** | Galaxy settlement cleared total metrics stub |

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

## PH-S179 — scope

- `galaxy_replication_strict_total` counter stub on grid job ingest path; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S179

```
PoolAI — спринт PH-S179 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S179 — Galaxy replication strict tier metrics stub
Scope: galaxy_replication_strict_total counter on grid job ingest path; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
