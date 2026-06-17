# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 (PH-S363 ✅ · vision **rev 221** · **0** відкритих · rust_ratio **94.33%**)

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

## Закрито (смуга PH-S354…S363)

PH-S354 ✅ — `galaxy_settlement_not_applicable_total` on grid result path.
PH-S355 ✅ — `buildMonitoringActiveAlertsUrl` wasm; monitoring page `acknowledged: false`.
PH-S356 ✅ — `galaxy_verification_sample_not_applicable_total` on local origin stub.
PH-S357 ✅ — stand smoke settlement + verify not-applicable `/metrics` shape.
PH-S358 ✅ — `admin_charts_*_wasm_first_ph_s353/355` glue tests.
PH-S359…S363 ✅ — loc-audit **94.33%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.33%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
