# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-19 · **Шар A + B (autoprogon):** **100%** — S33/S34.

---

## Промпт (підтримка / horizon)

```
PoolAI — сесія після 100% autoprogon (оркестратор + FM).

## S0 — зріз

1. git fetch && git status -sb (main).
2. HANDOFF_NEW_SESSION.md
3. FUNCTION_MANAGEMENT.md §5.1, §5.3, §5.5
4. DEVELOPMENT_PROGRESS_2026-05-19.md
5. RUN_LOCAL.md — запуск: bash bin/run-poolai.sh single
6. .cursor/rules/autonomous-orchestrator.mdc

Останні: S34 docs harmonization + libs E2E; S33 layer A 100%.

## Не повторювати

S21–S34 (OpenAPI, pa11y, axe, Playwright admin повний surface, run-poolai, FM 100%).

## Мета (лише за явним запитом)

| # | Фокус | Умова |
|---|--------|--------|
| 1 | FM-003 §4 LAN sign-off | 2 фізичні хости |
| 2 | FM-004 SIMD / FM-006 cloud-sdk | Deferred |
| 3 | FM-009/010 Grid/Solana | Concept-only |
| 4 | Horizon Layer C | P6, on-chain, deep cloud |

## Завершення (якщо були зміни)

src/ → cargo fmt → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28).
e2e/ → bash bin/e2e-playwright.sh --start.
Docs → commit + push MSYS2 з Summary.

Не стаджити: data/audit/*.log.gz, data/dev/, data/lan-stand/.
```

---

Прогрес: [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md) · Запуск: [`RUN_LOCAL.md`](./RUN_LOCAL.md).
