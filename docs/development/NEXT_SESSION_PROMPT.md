# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-19 · **Копіюй блок нижче в новий чат** (оркестратор, `.cursor/rules/autonomous-orchestrator.mdc`).

---

## Промпт (наступна сесія)

```
PoolAI — автономна ітеративна сесія (оркестратор).

## S0 — зріз

1. `git fetch && git status -sb` (main).
2. HANDOFF_NEW_SESSION.md, AUTO_RUN_SESSION_2026-07-01.md (§1–2, §4),
   DEVELOPMENT_PROGRESS_2026-05-19.md, FUNCTION_MANAGEMENT.md §5.1/§5.3/§5.5,
   .cursor/rules/autonomous-orchestrator.mdc
3. Останній коміт S29 (Playwright security + audit); прогрес ~93%.

## Не повторювати

S21–S29 (OpenAPI, pa11y CI, Playwright smoke + admin tenants/monitoring/security/audit, UI_QUALITY P1).

## Мета (обери одну)

| Пріоритет | Фокус | Критерій |
|-----------|--------|----------|
| A | OpenAPI — `/raid/distributed/*` (7 POST) | yaml + OPENAPI_GAP_AUDIT |
| B | ML ops | PIPELINE_MANAGEMENT.md hardening |
| C | Playwright — raid/topology/vm admin | 1–2 specs + E2E_PLAYWRIGHT.md |

## Завершення

cargo fmt → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28) якщо зміни в src/;
bash bin/e2e-playwright.sh --start після змін e2e/ → commit + push MSYS2 з Summary.

Поза обсягом: FM-003 §4 (BLOCKED), FM-004/006/009/010. Не стаджити data/audit/*.log.gz.
```

---

Прогрес: [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md).
