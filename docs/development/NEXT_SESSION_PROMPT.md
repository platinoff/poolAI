# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-14 · PH-S174 ✅ · vision **rev 110** · **5** відкритих (PH-S175…S179) · **hold 95%** · **stretch spirit 96%**

| **← наступний** | **PH-S175** — Galaxy verification mismatch metrics stub |
| **Відкритих** | **5** (PH-S175…S179) |
| **VDT** | один PH-S* = 1 commit |

---

## Зріз §5.12 (5 відкритих: PH-S175…S179)

### Закрито недавно ✅

| Sprint | Scope | Зріз |
|--------|-------|------|
| PH-S174 | Galaxy pricing quote usd_micro metrics stub | last quote gauge on `/metrics` |
| PH-S173 | Galaxy pricing provider errors metrics stub | provider fetch fail → `/metrics` |
| PH-S172 | Galaxy pricing provider catalog metrics stub | catalog allow-list hits on `/metrics` |

### Відкрито — replenish (PH-S175…S179)

| # | Sprint | Scope |
|---|--------|-------|
| 1 | **PH-S175** | Galaxy verification mismatch metrics stub |
| 2 | **PH-S176** | Galaxy replay pending metrics stub |
| 3 | **PH-S177** | Galaxy verification sample total metrics stub |
| 4 | **PH-S178** | Galaxy settlement pending_verification metrics stub |
| 5 | **PH-S179** | Galaxy replication strict tier metrics stub |

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

## PH-S175 — scope

- `galaxy_verification_mismatch_total` counter stub; unit tests
- Acceptance: `cargo test-ci`; FM/HANDOFF/NEXT/vision; push

---

## Copy-paste — PH-S175

```
PoolAI — спринт PH-S175 (один PH-S*, VDT ітераційно).
HANDOFF: docs/development/HANDOFF_NEW_SESSION.md
FM §5.12: docs/catalog/FUNCTION_MANAGEMENT.md

Спринт PH-S175 — Galaxy verification mismatch metrics stub
Scope: galaxy_verification_mismatch_total counter stub; unit tests

Acceptance: cargo fmt; cargo test-ci; FM/HANDOFF/NEXT/vision; git push main
```
