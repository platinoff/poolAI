# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S524…S533 ✅ · vision **rev 241** · **0** відкритих · rust_ratio **94.61%**)

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

## Закрито (смуга PH-S524…S533)

PH-S524 ✅ — worker-unhealthy lease failover (`fail_reason=worker-unhealthy`).
PH-S525 ✅ — scheduler/grid bind skips unhealthy peers.
PH-S526 ✅ — `POOLAI_JOB_MAX_TOTAL_RUNTIME_SECS` wall-clock cap.
PH-S527 ✅ — signed capability `expires_at` enforcement (telegram_edge).
PH-S528 ✅ — governance Prometheus gauges (`poolai_release_verify_*`, `poolai_update_notify_pending`).
PH-S529 ✅ — discovery startup hydrate persisted `network_profile`.
PH-S530 ✅ — queue starvation failover (`POOLAI_JOB_QUEUE_STARVATION_SECS`, `leased_at`).
PH-S531 ✅ — payout-batch `settlement_mode: offline_batch` wire.
PH-S532 ✅ — `admin_charts.js` wasm-first slim (line/sparkline fallbacks removed).
PH-S533 ✅ — horizon S524 integration + stand smoke + loc-audit + vision-sync.

**rust_ratio:** **94.61%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
