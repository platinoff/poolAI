# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-19 (PH-S534…S543 ✅ · vision **rev 243** · **0** відкритих · rust_ratio **94.63%**)

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

## Закрито (смуга PH-S534…S543)

PH-S534 ✅ — shadow verification checker JobStore submit (`local_srv`).
PH-S535 ✅ — replay verification job enqueue on mismatch.
PH-S536 ✅ — strict-tier replication M parallel executor jobs.
PH-S537 ✅ — peer HTTP seed-pull prefetch (`POOLAI_GALAXY_PREFETCH_PEER_HTTP_URL`).
PH-S538 ✅ — payout-batch `payout_pubkey` from telegram wallet bind.
PH-S539 ✅ — Cleared settlement → Solana NDJSON `JobCompleted` stub.
PH-S540 ✅ — telegram_edge GPU admission gate (`raid_artifact_probe` history).
PH-S541 ✅ — cold-mining limits on Galaxy worker DTO.
PH-S542 ✅ — checker_timeout → retry / verification_inconclusive policy.
PH-S543 ✅ — horizon S534 integration + stand smoke + loc-audit + vision-sync.

**rust_ratio:** **94.63%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
