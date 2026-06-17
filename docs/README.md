# PoolAI documentation

**Last updated:** 2026-06-17 (PH-S243 ✅ · §5.12 **9** PH-S244…S252 · rust_ratio **92.78%**)

## Canonical reading order

Узгоджено з кореневим [`README.md`](../README.md) (розділ *Documentation map*): **кроки 1–12**.

1. **Кореневий [`README.md`](../README.md)** — швидкий старт, збірка, CI.
2. **[INDEX_2026-03-17.md](./INDEX_2026-03-17.md)** — карта всього `docs/` (концепція, статус, ML, cloud, troubleshooting).
3. **[development/NEXT_STEPS_ARCHITECT_2026-03-17.md](./development/NEXT_STEPS_ARCHITECT_2026-03-17.md)** — план Rust Architect (P1–P6, TurboQuant, звірка з CI).
4. **[development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md)** — старт **нової сесії** (`main`, порядок доків, git-push, зріз зробленого, next steps).
5. **Концепція** — [concept/poolAI_concept_root.txt](./concept/poolAI_concept_root.txt); Grid / Memory / Job / tokenization: [concept/POOLAI_GRID_NODE.md](./concept/POOLAI_GRID_NODE.md), **[concept/POOLAI_GALAXY_GRID.md](./concept/POOLAI_GALAXY_GRID.md)** (федеративна мережа srvN), [concept/POOLAI_MEMORY_LAYER.md](./concept/POOLAI_MEMORY_LAYER.md), [development/JOB_LAYER_CONCEPT_2026-03-17.md](./development/JOB_LAYER_CONCEPT_2026-03-17.md), [development/GRID_PROTOCOL_CONCEPT_2026-04-06.md](./development/GRID_PROTOCOL_CONCEPT_2026-04-06.md), [development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md](./development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md).
6. **Архітектура** — [ARCHITECTURE_REVIEW.md](./ARCHITECTURE_REVIEW.md), [ARCHITECTURE_BEST_PRACTICES.md](./ARCHITECTURE_BEST_PRACTICES.md).
7. **Продуктивність** — [performance/BENCHMARKS.md](./performance/BENCHMARKS.md), [performance/PROFILING.md](./performance/PROFILING.md); опційний workflow Criterion: [`.github/workflows/benchmarks.yml`](../.github/workflows/benchmarks.yml); HTTP health — **`poolai_health_load`** (опційно **`--json`**, див. `BENCHMARKS.md`).
8. **CI** — обов’язкові перевірки: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) (у т.ч. три кроки **`cargo clippy … -D warnings`** за feature-матрицями; див. [`STABLE_STATE_SUMMARY.md`](./status/STABLE_STATE_SUMMARY.md)).
9. **Інвентар** — кореневий [file_list.csv](../file_list.csv) (оновлюй також після змін у `src/services/`, `src/network/`, `.github/workflows/`, `.cursor/`, `docs/catalog/`); повний список: `git ls-files`.
10. **Git push (Windows)** — [`.cursor/commands/git-push.md`](../.cursor/commands/git-push.md).
11. **Витяг функціоналу** — [catalog/FUNCTIONALITY_DIGEST_2026-04-06.md](./catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) (зведення за доками та кодом).
12. **Керування функціоналом** — [catalog/FUNCTION_MANAGEMENT.md](./catalog/FUNCTION_MANAGEMENT.md) (звірка зі сталевим станом, прогалини, тікети `FM-*`, **§5.12** — черга PH-S*, **§5.13** — Rust ratio); правило Cursor — [`.cursor/rules/functionality-management.mdc`](../.cursor/rules/functionality-management.mdc).

**Rust ratio 90–95%:** [development/RUST_RATIO_STRATEGY_2026-06-13.md](./development/RUST_RATIO_STRATEGY_2026-06-13.md) · testing [`.cursor/rules/poolai-testing-policy.mdc`](../.cursor/rules/poolai-testing-policy.mdc).

**Сталевий стан (декларація CI / збірки / модулів)** — [status/STABLE_STATE_SUMMARY.md](./status/STABLE_STATE_SUMMARY.md). Звіряти разом із кроком 12 (**§5.1**); операційний зріз — [development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md).

## Таксономія та правила

- **Де що лежить у `docs/`** — [STRUCTURE.md](./STRUCTURE.md) (каталоги, спадщина плоских `.md`, інвентар, тести vs doctests).
- **Правила для AI/агента (Cursor)** — [`.cursor/rules/documentation.md`](../.cursor/rules/documentation.md); skill — [`.cursor/skills/poolai-documentation/SKILL.md`](../.cursor/skills/poolai-documentation/SKILL.md).
- **Galaxy docs vision** — `.\bin\open-docs-vision.ps1` → [vision/index.html](http://127.0.0.1:8765/docs/vision/index.html) (L0–L5, cluster collapse, pan/zoom, **◎ Sprint** / **⊟ Folders**). Док: [vision/README.md](./vision/README.md) · [`.cursor/rules/docs-vision.mdc`](../.cursor/rules/docs-vision.mdc).

## Short pointers

- **Galaxy docs vision (рекомендовано)** — [vision/index.html](./vision/index.html) + `manifest.json` (rev **114**, git HEAD pill); `../bin/open-docs-vision.ps1`.
- **Galaxy Grid (концепт)** — [concept/POOLAI_GALAXY_GRID.md](./concept/POOLAI_GALAXY_GRID.md) (§4.2 pricing + `/metrics`, §4.3 lease + OTel spans, §5.2 locality_score, §6 verification metrics); код: `src/grid/galaxy_*`, `src/job/lease_*`.
- **Solana adapter** — [development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md](./development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md) · `crates/poolai-solana-adapter/` (FM-033; без `solana-sdk` у main).
- **Commit-msg чернетки** — `comitmsg/README.md` (не комітити `comitmsg/*.txt`).
- **Каталог / витяг функціоналу** — [catalog/FUNCTIONALITY_DIGEST_2026-04-06.md](./catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) (крок 11; оновлюй при змінах модулів або публічного API).
- **Беклог і тікети** — [catalog/FUNCTION_MANAGEMENT.md](./catalog/FUNCTION_MANAGEMENT.md) (крок 12; **§5.1** — порядок наступних кроків за FM-*).
- **Сталевий стан** — [status/STABLE_STATE_SUMMARY.md](./status/STABLE_STATE_SUMMARY.md).
- **Status / plans** — `status/`, `development/` (індекс планів: [development/README.md](./development/README.md)).
- **Авторозробка** — найновіший [development/AUTO_RUN_SESSION_2026-07-01.md](./development/AUTO_RUN_SESSION_2026-07-01.md); оркестратор — [`.cursor/rules/autonomous-orchestrator.mdc`](../.cursor/rules/autonomous-orchestrator.mdc).
- **Virtual nodes (FM-016)** — `poolai-worker`, `poolai-telegram-bot`, `src/network/api/virtual_nodes.rs`, `src/services/virtual_node_*`; dev stand — `bin/verify-dev-stand.*` (див. [HANDOFF](./development/HANDOFF_NEW_SESSION.md) §2a–2b).
- **Середовище та Cursor-оновлення** — [development/ENVIRONMENT_AND_CURSOR_UPDATES_2026-05-05.md](./development/ENVIRONMENT_AND_CURSOR_UPDATES_2026-05-05.md).
- **REST API** — [openapi.yaml](./openapi.yaml) (OpenAPI 3).
- **UI / admin ↔ API** — [development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md](./development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md).
- **Playwright E2E (S23+)** — [development/E2E_PLAYWRIGHT.md](./development/E2E_PLAYWRIGHT.md); `e2e/tests/smoke.spec.ts`, `admin.spec.ts`, `a11y.spec.ts`, `visual.spec.ts`.
- **Raft / VM (PH-S03…S06)** — `tests/vm_api_contracts.rs`, `tests/raft_wire_integration.rs`, `tests/raft_multi_node_harness.rs`; `cargo test-raft-ci`; [`ADMIN_UI_JSON_CONTRACTS.md`](./development/ADMIN_UI_JSON_CONTRACTS.md) §RAID cluster status.
- **OpenTelemetry (FM-038)** — [development/OPENTELEMETRY_TRACING.md](./development/OPENTELEMETRY_TRACING.md); feature `otel`, `src/observability/`.
- **Unified API errors (P3 / FM-005)** ✅ — `src/network/json_errors.rs` (**`HttpAppError`**, **`RestError`**), **`auth.rs`** (включно **`login`/`refresh`** ланцюжок), **`ws.rs`**, **`rate_limit.rs`**, **`enterprise_api/`**, **`check_permission`**. Див. **FM-005** у [catalog/FUNCTION_MANAGEMENT.md](./catalog/FUNCTION_MANAGEMENT.md); [development/HANDOFF_NEW_SESSION.md](./development/HANDOFF_NEW_SESSION.md).
- **Benchmarks (P4)** — [performance/BENCHMARKS.md](./performance/BENCHMARKS.md); baseline (зараз є **dev-sample** — замінити на референс-хост); HTTP **`/api/v1/health`** — **`poolai_health_load`** (**`--json`** для збереження звіту) або **`wrk`**; опційно — [benchmarks.yml](../.github/workflows/benchmarks.yml).
- **Архів одноразових нотаток** — [archive/](./archive/) (у т.ч. колишні кореневі `PUSH_*.md`).
