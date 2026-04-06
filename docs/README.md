# PoolAI documentation

**Last updated:** 2026-04-06

## Canonical reading order

Узгоджено з кореневим [`README.md`](../README.md) (розділ *Documentation map*).

1. **[INDEX_2026-03-17.md](./INDEX_2026-03-17.md)** — карта всього `docs/` (концепція, статус, ML, cloud, troubleshooting).
2. **[development/NEXT_STEPS_ARCHITECT_2026-03-17.md](./development/NEXT_STEPS_ARCHITECT_2026-03-17.md)** — план Rust Architect (P1–P6, TurboQuant, звірка з CI).
3. **[development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md)** — старт **нової сесії** (`main`, порядок доків, git-push, зріз зробленого, next steps).
4. **Концепція** — [concept/poolAI_concept_root.txt](./concept/poolAI_concept_root.txt); Grid / Memory / Job: `concept/POOLAI_GRID_NODE.md`, `concept/POOLAI_MEMORY_LAYER.md`, [development/JOB_LAYER_CONCEPT_2026-03-17.md](./development/JOB_LAYER_CONCEPT_2026-03-17.md).
5. **Архітектура** — [ARCHITECTURE_REVIEW.md](./ARCHITECTURE_REVIEW.md), [ARCHITECTURE_BEST_PRACTICES.md](./ARCHITECTURE_BEST_PRACTICES.md).
6. **Продуктивність** — [performance/BENCHMARKS.md](./performance/BENCHMARKS.md), [performance/PROFILING.md](./performance/PROFILING.md); опційний workflow Criterion: [`.github/workflows/benchmarks.yml`](../.github/workflows/benchmarks.yml).
7. **CI** — обов’язкові перевірки: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml).
8. **Інвентар** — кореневий [file_list.csv](../file_list.csv) (оновлюй також після змін у `src/network/`, `.github/workflows/`); повний список: `git ls-files`.
9. **Git push (Windows)** — [`.cursor/commands/git-push.md`](../.cursor/commands/git-push.md).

## Short pointers

- **Status / plans** — `status/`, `development/` (індекс планів: [development/README.md](./development/README.md)).
- **Unified API errors (P3)** — `src/network/json_errors.rs`, REST + **`enterprise_api.rs`**, **`raid.rs`**, **`auth.rs`**, **`ws.rs`**, **`rate_limit.rs`**. Деталі — [development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md).
- **Benchmarks (P4)** — [performance/BENCHMARKS.md](./performance/BENCHMARKS.md); baseline (зараз є **dev-sample** — замінити на референс-хост); HTTP — `wrk` вручну; опційно — запуск [benchmarks.yml](../.github/workflows/benchmarks.yml) у Actions.
- **Архів одноразових нотаток** — [archive/](./archive/) (у т.ч. колишні кореневі `PUSH_*.md`).
