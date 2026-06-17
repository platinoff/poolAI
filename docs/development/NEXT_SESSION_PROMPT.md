# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S313…S322 ✅ · vision **rev 214** · **0** відкритих · rust_ratio **94.39%**

| **← наступний** | replenish §5.13 **або** **`абракадабра`** (drain) |
| **Відкритих** | **0** |

---

## Тригер «абракадабра» (drain черги)

Одне слово в новій сесії → повний цикл без підтвердження кожного PH-S*:

```
абракадабра
```

Агент: S0 → якщо §5.12 **< 10** → replenish до **10** → drain усіх → vision-sync `--check` → git push + самарі.

Правила: [`.cursor/rules/poolai-session-iteration.mdc`](../.cursor/rules/poolai-session-iteration.mdc) § «Тригер абракадабра».

---

## Copy-paste — одна PH-S* (звичайна ітерація)

```
S0: git fetch; HANDOFF; FM §5.12; df -h /s
Один PH-S* з §5.12 → fmt → test-ci → FM/HANDOFF/NEXT → vision-sync --check → push
```

---

## Закрито (смуга PH-S313…S322)

PH-S313 ✅ — `galaxy_prefetch_ingest_total` on prefetch ingest stub.
PH-S314 ✅ — `buildMetricHistoryUrl` wasm; metric history fetch wasm-first.
PH-S315 ✅ — `galaxy_locality_rank_empty_workers_total` on empty worker inventory.
PH-S316 ✅ — stand smoke export includes ingest + empty workers metrics.
PH-S317 ✅ — `buildMetricsWindowUrl` wasm; metrics window fetch wasm-first.
PH-S308…S322 ✅ — loc-audit **94.39%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.39%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
