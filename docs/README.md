# PoolAI documentation

**Last updated:** 2026-04-10

## Canonical reading order

Узгоджено з кореневим [`README.md`](../README.md) (розділ *Documentation map*): **кроки 1–12**.

1. **Кореневий [`README.md`](../README.md)** — швидкий старт, збірка, CI.
2. **[INDEX_2026-03-17.md](./INDEX_2026-03-17.md)** — карта всього `docs/` (концепція, статус, ML, cloud, troubleshooting).
3. **[development/NEXT_STEPS_ARCHITECT_2026-03-17.md](./development/NEXT_STEPS_ARCHITECT_2026-03-17.md)** — план Rust Architect (P1–P6, TurboQuant, звірка з CI).
4. **[development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md)** — старт **нової сесії** (`main`, порядок доків, git-push, зріз зробленого, next steps).
5. **Концепція** — [concept/poolAI_concept_root.txt](./concept/poolAI_concept_root.txt); Grid / Memory / Job / tokenization: `concept/POOLAI_GRID_NODE.md`, `concept/POOLAI_MEMORY_LAYER.md`, [development/JOB_LAYER_CONCEPT_2026-03-17.md](./development/JOB_LAYER_CONCEPT_2026-03-17.md), [development/GRID_PROTOCOL_CONCEPT_2026-04-06.md](./development/GRID_PROTOCOL_CONCEPT_2026-04-06.md), [development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md](./development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md).
6. **Архітектура** — [ARCHITECTURE_REVIEW.md](./ARCHITECTURE_REVIEW.md), [ARCHITECTURE_BEST_PRACTICES.md](./ARCHITECTURE_BEST_PRACTICES.md).
7. **Продуктивність** — [performance/BENCHMARKS.md](./performance/BENCHMARKS.md), [performance/PROFILING.md](./performance/PROFILING.md); опційний workflow Criterion: [`.github/workflows/benchmarks.yml`](../.github/workflows/benchmarks.yml); HTTP health — **`poolai_health_load`** (опційно **`--json`**, див. `BENCHMARKS.md`).
8. **CI** — обов’язкові перевірки: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) (у т.ч. три кроки **`cargo clippy … -D warnings`** за feature-матрицями; див. [`STABLE_STATE_SUMMARY.md`](./status/STABLE_STATE_SUMMARY.md)).
9. **Інвентар** — кореневий [file_list.csv](../file_list.csv) (оновлюй також після змін у `src/services/`, `src/network/`, `.github/workflows/`, `.cursor/`, `docs/catalog/`); повний список: `git ls-files`.
10. **Git push (Windows)** — [`.cursor/commands/git-push.md`](../.cursor/commands/git-push.md).
11. **Витяг функціоналу** — [catalog/FUNCTIONALITY_DIGEST_2026-04-06.md](./catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) (зведення за доками та кодом).
12. **Керування функціоналом** — [catalog/FUNCTION_MANAGEMENT.md](./catalog/FUNCTION_MANAGEMENT.md) (звірка зі сталевим станом, прогалини, тікети `FM-*`, **§5.1 — пріоритезовані наступні кроки**); правило Cursor — [`.cursor/rules/functionality-management.mdc`](../.cursor/rules/functionality-management.mdc).

**Сталевий стан (декларація CI / збірки / модулів)** — [status/STABLE_STATE_SUMMARY.md](./status/STABLE_STATE_SUMMARY.md). Звіряти разом із кроком 12 (**§5.1**); операційний зріз — [development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md).

## Таксономія та правила

- **Де що лежить у `docs/`** — [STRUCTURE.md](./STRUCTURE.md) (каталоги, спадщина плоских `.md`, інвентар, тести vs doctests).
- **Правила для AI/агента (Cursor)** — [`.cursor/rules/documentation.md`](../.cursor/rules/documentation.md); skill — [`.cursor/skills/poolai-documentation/SKILL.md`](../.cursor/skills/poolai-documentation/SKILL.md).

## Short pointers

- **Каталог / витяг функціоналу** — [catalog/FUNCTIONALITY_DIGEST_2026-04-06.md](./catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) (крок 11; оновлюй при змінах модулів або публічного API).
- **Беклог і тікети** — [catalog/FUNCTION_MANAGEMENT.md](./catalog/FUNCTION_MANAGEMENT.md) (крок 12; **§5.1** — порядок наступних кроків за FM-*).
- **Сталевий стан** — [status/STABLE_STATE_SUMMARY.md](./status/STABLE_STATE_SUMMARY.md).
- **Status / plans** — `status/`, `development/` (індекс планів: [development/README.md](./development/README.md)).
- **REST API** — [openapi.yaml](./openapi.yaml) (OpenAPI 3).
- **UI / admin ↔ API** — [development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md](./development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md).
- **Unified API errors (P3)** — `src/network/json_errors.rs`, REST + **`network/enterprise_api/`**, **`raid.rs`**, **`auth.rs`**, **`ws.rs`**, **`rate_limit.rs`**. Деталі — [development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md).
- **Benchmarks (P4)** — [performance/BENCHMARKS.md](./performance/BENCHMARKS.md); baseline (зараз є **dev-sample** — замінити на референс-хост); HTTP **`/api/v1/health`** — **`poolai_health_load`** (**`--json`** для збереження звіту) або **`wrk`**; опційно — [benchmarks.yml](../.github/workflows/benchmarks.yml).
- **Архів одноразових нотаток** — [archive/](./archive/) (у т.ч. колишні кореневі `PUSH_*.md`).
