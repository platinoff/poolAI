# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S393 ✅ · vision **rev 224** · **0** відкритих · rust_ratio **94.33%**)

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

## Закрито (смуга PH-S384…S393)

PH-S384 ✅ — `galaxy_trust_gate_default_score` gauge on `/metrics`.
PH-S385 ✅ — dashboard `formatUptime` wasm-first.
PH-S386 ✅ — `buildDashboardMetricsWindowUrl` wasm glue.
PH-S387 ✅ — stand smoke trust gate default score `/metrics` shape.
PH-S388 ✅ — dashboard uptime + metrics window wasm glue tests.
PH-S389…S393 ✅ — loc-audit **94.33%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.33%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
