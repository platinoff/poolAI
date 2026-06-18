# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-18 (PH-S404…S413 ✅ · vision **rev 226** · **0** відкритих · rust_ratio **94.34%**)

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

## Закрито (смуга PH-S404…S413)

PH-S404 ✅ — `galaxy_settlement_resolved_total` on grid result path.
PH-S405 ✅ — `galaxy_trust_explicit_score_total` when trust_score provided.
PH-S406 ✅ — dashboard active alerts `alertSeverityBadgeClass` wasm-first.
PH-S407 ✅ — stand smoke settlement resolved + explicit score `/metrics` shape.
PH-S408 ✅ — dashboard alert severity wasm glue tests.
PH-S409…S413 ✅ — loc-audit **94.34%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.34%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
