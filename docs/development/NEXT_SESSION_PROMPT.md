# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-19 · **Копіюй блок нижче** (оркестратор + менеджер функціоналу).

---

## Промпт

```
PoolAI — автономна ітеративна сесія (оркестратор + FM).

## S0 — зріз

1. git fetch && git status -sb (main).
2. HANDOFF_NEW_SESSION.md
3. FUNCTION_MANAGEMENT.md §5.1, §5.3, §5.5
4. DOCS_LEGACY_AUDIT_2026-05-19.md (legacy docs — не читати архівні [ ])
5. AUTO_RUN_SESSION_2026-07-01.md §1–2, §4
6. .cursor/rules/autonomous-orchestrator.mdc

Останні коміти: S30 FM legacy audit; S29 Playwright; S28 OpenAPI. Прогрес ~93%.

## Не повторювати

S21–S30 (OpenAPI v1 gap, pa11y CI, UI_QUALITY P1, Playwright admin×4, FM stale audit).
Архівні плани січень 2026 — лише DOCS_LEGACY_AUDIT, не їхні чеклисти.

## Мета (обери одну)

| # | Фокус | Критерій |
|---|--------|----------|
| 1 | OpenAPI `/raid/distributed/*` (7 POST) | yaml + OPENAPI_GAP_AUDIT |
| 2 | ML ops | PIPELINE_MANAGEMENT.md + DIGEST §ML |
| 3 | Playwright raid/topology | specs + E2E_PLAYWRIGHT.md |

## Завершення

Зміни src/ → cargo fmt → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28).
Зміни e2e/ → bash bin/e2e-playwright.sh --start.
Docs-only FM → commit без test-ci (опційно).
commit + push MSYS2 з Summary; HANDOFF, FM §5.1, CHANGELOG.

Поза обсягом: FM-003 §4 (BLOCKED, 2 хости), FM-004/006/009/010.
Не стаджити data/audit/*.log.gz.
```

---

Прогрес: [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md) · Legacy: [`DOCS_LEGACY_AUDIT_2026-05-19.md`](./DOCS_LEGACY_AUDIT_2026-05-19.md).
