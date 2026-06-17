# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-17 · PH-S323…S332 ✅ · vision **rev 217** · **0** відкритих · rust_ratio **94.37%**

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

## Закрито (смуга PH-S323…S332)

PH-S323 ✅ — `galaxy_prefetch_skip_ingest_total` on empty shard list.
PH-S324 ✅ — `buildMlPipelinesUrl` wasm; ML pipelines fetch wasm-first.
PH-S325 ✅ — `galaxy_locality_rank_skip_total` on empty shard list.
PH-S326 ✅ — stand smoke export includes skip ingest + rank skip metrics.
PH-S327 ✅ — `buildMlPipelineDemoUrl` wasm; ML demo fetch wasm-first.
PH-S328…S332 ✅ — loc-audit **94.37%**, docs canon, vision `--check`, INDEX maintain.

**rust_ratio:** **94.37%** (formal 90–95% ✅; hold 95% advisory).

**BLOCKED / Deferred:** FM-003 LAN · FM-041 Cloud SDK live.
