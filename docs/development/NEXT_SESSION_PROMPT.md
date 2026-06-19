# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-19 (PH-S554 ✅ · vision **rev 245** · **0** відкритих · rust_ratio **94.66%**)

| **← наступний** | **`абракадабра`** (project scan + drain) **або** один PH-S* |
| **Відкритих** | **0** |

---

## Тригер «абракадабра» (drain **всього проєкту**)

Одне слово в **новій сесії** → project scan → drain → push (без підтвердження кожного PH-S*):

```
абракадабра
```

| Крок | Дія |
|------|-----|
| S0 | `git fetch`; HANDOFF; FM **§1–§5.1**; STABLE_STATE; **`poolai-vision-sync --check` ok**; `df -h /s` |
| Project scan | Якщо §5.12 **< 10** → scan **всього репо** → **10 PH-S*** у §5.12 |
| Drain | Усі відкриті PH-S* (код + scope-тести + docs **без** Vision rev footer) |
| Vision close | FM §5.12 ✅ + HANDOFF + NEXT → **один** `poolai-vision-sync` → rev з `manifest.json` → `--check` |
| Test | `cargo fmt` → **один** `cargo test-ci` (після vision close) |
| Git | **один** commit: код + `docs/vision/*` + FM/HANDOFF/NEXT → `git-commit-tree-msg.sh` + push + **самарі** |

**Vision:** не sync mid-drain; не другий commit «sync rev» — див. `.cursor/rules/poolai-session-iteration.mdc` § Vision close band.

**Наступна сесія:** знову **`абракадабра`** → S0 → project scan → drain → push.

---

## Закрито (PH-S554)

PH-S554 ✅ — Horizon close band S545–S553: quorum gate, prefetch timeout, capacity preemption, locality tie-break, update notify, on-chain settlement toggle, trust store, payout-batch admin, integration + loc-audit + vision-sync.

## Закрито (смуга PH-S545…S553)

PH-S545 ✅ — replication quorum digest gate before Cleared settlement.
PH-S546 ✅ — strict_locality prefetch deadline → `prefetch-timeout`.
PH-S547 ✅ — `capacity-preemption` lease failover reason.
PH-S548 ✅ — locality rank queue_depth + pricing tie-break.
PH-S549 ✅ — `POOLAI_UPDATE_POLICY` notify tick + verify-release hook.
PH-S550 ✅ — `POOLAI_SETTLEMENT_ON_CHAIN` payout-batch mode stub.
PH-S551 ✅ — Telegram cold-mining MVP docs (Galaxy §8.2 TBD #2).
PH-S552 ✅ — peer `trust_score` JSON store + register-remote hydrate.
PH-S553 ✅ — `/ui/admin/payout-batch` read-only panel.

**rust_ratio:** **94.66%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
