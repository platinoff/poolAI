# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S414…S423 ✅ · vision **rev 228** · **0** відкритих · rust_ratio **94.35%**)

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

## Закрито (смуга PH-S414…S423)

PH-S414 ✅ — `galaxy_verification_sampling_evaluations_total` on grid result path.
PH-S415 ✅ — `galaxy_replay_evaluations_total` on grid result path.
PH-S416 ✅ — dashboard `updateDashboardRefreshedAt` + `formatLocaleTimeHms` wasm-first.
PH-S417 ✅ — stand smoke verify + replay evaluation `/metrics` shape.
PH-S418 ✅ — dashboard refreshed-at wasm glue tests.
PH-S419…S423 ✅ — loc-audit **94.35%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.35%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
