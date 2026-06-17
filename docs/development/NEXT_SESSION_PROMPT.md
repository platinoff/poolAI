# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 (PH-S333…S342 ✅ · vision **rev 218** · **0** відкритих · rust_ratio **94.37%**)

| **← наступний** | replenish §5.13 **або** **`абракадабра`** (drain) |
| **Відкритих** | **0** |

---

## Тригер «абракадабра» (drain черги)

Одне слово в новій сесії → повний цикл без підтвердження кожного PH-S*:

```
абракадабра
```

Агент: S0 → якщо §5.12 **< 10** → replenish до **10** → drain усіх → vision-sync → FM rev → `--check` → git push + самарі.

Правила: [`.cursor/rules/poolai-session-iteration.mdc`](../.cursor/rules/poolai-session-iteration.mdc) § «Тригер абракадабра» (shell MSYS2, один test-ci, amend-head-msg).

---

## Copy-paste — одна PH-S* (звичайна ітерація)

```
S0: git fetch; HANDOFF; FM §5.12; df -h /s
Один PH-S* з §5.12 → fmt → test-ci → FM/HANDOFF/NEXT → vision-sync --check → push
```

---

## Закрито (смуга PH-S333…S342)

PH-S333 ✅ — `galaxy_replay_pending_scheduled_total` on replay schedule.
PH-S334 ✅ — `buildMetricHistoryUrlWithHours` wasm; metric history fetch wasm-first.
PH-S335 ✅ — `galaxy_replay_pending_resolved_total` on replay verdict.
PH-S336 ✅ — stand smoke replay scheduled + resolved metrics.
PH-S337 ✅ — `buildMetricsWindowUrlWithHours` wasm; metrics window fetch wasm-first.
PH-S338…S342 ✅ — loc-audit **94.37%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.37%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
