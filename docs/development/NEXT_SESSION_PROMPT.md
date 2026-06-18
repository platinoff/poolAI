# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S394…S403 ✅ · vision **rev 225** · **0** відкритих · rust_ratio **94.34%**)

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

## Закрито (смуга PH-S394…S403)

PH-S394 ✅ — `galaxy_trust_gate_evaluations_total` gauge on `/metrics`.
PH-S395 ✅ — `galaxy_trust_default_score_applied_total` on grid result path.
PH-S396 ✅ — dashboard audit timestamps `formatIsoDatetime` wasm-first.
PH-S397 ✅ — stand smoke trust gate evaluation counters `/metrics` shape.
PH-S398 ✅ — dashboard audit + wasm glue tests.
PH-S399…S403 ✅ — loc-audit **94.34%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.34%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
