# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 (PH-S373 ✅ · vision **rev 222** · **0** відкритих · rust_ratio **94.32%**)

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

## Закрито (смуга PH-S364…S373)

PH-S364 ✅ — `galaxy_trust_payout_not_applicable_total` on local-origin trust gate.
PH-S365 ✅ — dashboard `buildMonitoringActiveAlertsUrl` wasm-first.
PH-S366 ✅ — `buildMonitoringMetricLatestUrl` wasm glue.
PH-S367 ✅ — stand smoke trust not-applicable `/metrics` shape.
PH-S368 ✅ — dashboard + metric latest wasm glue tests.
PH-S369…S373 ✅ — loc-audit **94.32%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.32%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
