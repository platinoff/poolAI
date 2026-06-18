# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S474…S483 ✅ · vision **rev 236** · **0** відкритих · rust_ratio **94.39%**)

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

## Закрито (смуга PH-S474…S483)

PH-S474 ✅ — prefetch `lan_only` egress guardrail + `galaxy_prefetch_egress_blocked_total`.
PH-S475 ✅ — `POOLAI_TELEGRAM_SEAT_LIMIT`; register-remote **409** `seat_exhausted`.
PH-S476 ✅ — `POOLAI_GALAXY_CAPABILITY_VERIFY_PK_HEX` env override for capability verify.
PH-S477…S478 ✅ — payout-batch + verification-replay **history** read APIs.
PH-S479 ✅ — peer seed inventory prefetch fetch stub + metrics.
PH-S480 ✅ — workers admin panel wasm (`renderWorkersPanel`).
PH-S481…S482 ✅ — horizon S474 integration + stand smoke band.
PH-S483 ✅ — loc-audit **94.39%**, vision-sync + `--check`.

**rust_ratio:** **94.39%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
