# Автономний прогін (PoolAI) — 2026-06-08

**Попередній:** [`AUTO_RUN_SESSION_2026-06-07.md`](./AUTO_RUN_SESSION_2026-06-07.md) (менеджер функціоналу: §5.3 audit + підготовка сесії).

**Ціль:** звірка FM ↔ docs після FM-019; **P4** `poolai_health_load` на ref-host (coordinator `:8080`).

**Критерії:**
- [x] `FUNCTION_MANAGEMENT.md` §5.3 оновлено (2026-06-07 audit)
- [x] HANDOFF, STABLE_STATE, README Next Focus синхронізовано
- [x] **P4 (ops)** — `poolai_health_load --json` → `BENCHMARKS.md` (**2026-05-18**)
- [x] docs sync (менеджер функціоналу)

**BLOCKED:** FM-003 §4 LAN (2 хости).

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

## Стартовий промпт (оркестратор)

```
S0: git fetch && git status -sb; HANDOFF + FUNCTION_MANAGEMENT §5.1–5.3 + AUTO_RUN_SESSION_2026-06-08.
Пріоритет: P4 poolai_health_load (якщо сервер піднято) АБО FM-003 runbook refresh (BLOCKED).
Не робити: FM-004/006/009/010 без явного запиту. Після коду: cargo fmt, cargo test-ci, push MSYS2 (-c commit.template=).
```

## S1 — виконання (2026-05-18)

**S0:** `main...origin/main`; health `GET :8080/api/v1/health` → **200**.

**P4:** MSYS2 UCRT64 release:

```bash
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo run --release --bin poolai_health_load -- \
  --json http://127.0.0.1:8080/api/v1/health 5 50
```

| Поле | Значення |
|------|----------|
| `wall_seconds` | 5.016 |
| `ok_requests` | 18221 |
| `rps_ok_only` | 3632.46 |
| `latency_p50_ms` | 12.136 |
| `latency_p95_ms` | 24.848 |
| `latency_p99_ms` | 34.903 |

Рядок додано в [`BENCHMARKS.md`](../performance/BENCHMARKS.md). Baseline **2026-04-10** залишається в таблиці для порівняння (інший профіль навантаження/середовища).

**Код:** без змін у `src/` — `cargo test-ci` не запускався.

**Push:** зовнішній MSYS2 — [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md).
