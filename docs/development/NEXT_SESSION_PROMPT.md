# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 (PH-S343…S352 ✅ · vision **rev 219** · **0** відкритих · rust_ratio **94.35%**)

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

## Закрито (смуга PH-S343…S352)

PH-S343 ✅ — `galaxy_verification_sample_completed_total` on verdict path.
PH-S344 ✅ — `buildMonitoringAlertsUrl` wasm; `poolaiFetchMonitoringAlerts` wasm-first.
PH-S345 ✅ — `galaxy_verification_sample_skipped_total` on edge NotSelected stub.
PH-S346 ✅ — stand smoke verification completed + skipped metrics.
PH-S347 ✅ — `buildAlertRulesUrl` wasm; `poolaiFetchAlertRules` wasm-first.
PH-S348…S352 ✅ — loc-audit **94.35%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.35%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
