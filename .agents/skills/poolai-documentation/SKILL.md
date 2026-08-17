---
name: poolai-documentation
description: >-
  PoolAI documentation map (steps 1–12), functionality digest, function management, and where to
  update docs after code or API changes. Use when editing docs/, README,
  planning features, functionality backlog, or answering "what does PoolAI do / where is X documented".
metadata:
  audience: poolai-vdt
  clients: cursor-opencode
---

# PoolAI — документація та витяг функціоналу

Canon: `.agents/skills/poolai-documentation/`. Client copies under `.cursor/skills/` and
`.opencode/skills/` must stay identical (Cursor + OpenCode).

## Runtime stack (перед будь-якою імплементацією)

- Читай **`.cursor/rules/runtime-stack-policy.mdc`** — Rust primary; **no Python runtime**; **target 90–95% Rust** — [`RUST_RATIO_STRATEGY_2026-06-13.md`](../../../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md).
- API tests — **Rust** `tests/` ([`poolai-testing-policy.mdc`](../../../.cursor/rules/poolai-testing-policy.mdc)); Playwright — browser only.
- ML/TurboQuant — `src/ml/` (Rust). Архівні docs з Python — не план імплементації.
- **Secrets:** never stage `.env`, `*.pem`/`*.key`, `certs/*.pem`, `data/audit/*` — [`SECRETS_MANAGEMENT.md`](../../../docs/security/SECRETS_MANAGEMENT.md) §1 · [`certs/README.md`](../../../certs/README.md).

## Код репозиторію (не плутати папки)

- **`docs/development/REPOSITORY_LAYOUT.md`** — `src/` vs `src/bin/` vs `bin/` vs `scripts/` vs `tests/` vs `crates/`.
- Нові dev launchers → **`bin/`**; toolchain → **`scripts/`**; Rust CLI → **`src/bin/`**.

## Канонічний порядок (завжди той самий)

Узгодь посилання з кореневим `README.md` → **кроки 1–12**:

1. Кореневий `README.md`
2. `docs/INDEX_2026-03-17.md` (для структури папок і спадщини — **`docs/STRUCTURE.md`**)
3. `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` (таблиця P1–P7 і підрозділ **«Операційний порядок»** — дзеркало **§5.1** у `FUNCTION_MANAGEMENT.md`)
4. `docs/development/HANDOFF_NEW_SESSION.md`
5. Концепція (`docs/concept/poolAI_concept_root.txt`, Grid/Memory/Job, `GRID_PROTOCOL_CONCEPT_2026-04-06.md`, `SOLANA_ADAPTER_CONCEPT_2026-04-06.md`)
6. `docs/ARCHITECTURE_REVIEW.md`, `docs/ARCHITECTURE_BEST_PRACTICES.md`
7. `docs/performance/BENCHMARKS.md`, `PROFILING.md` (Criterion + **`poolai_health_load`** / **`--json`** для `GET /api/v1/health`)
8. `.github/workflows/ci.yml`
9. `file_list.csv` (оновлюй також після змін у `docs/catalog/`, `.agents/skills/`, `.cursor/skills/`, `.opencode/skills/`)
10. `.cursor/commands/git-push.md`
11. **`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`** — витяг функціоналу
12. **`docs/catalog/FUNCTION_MANAGEMENT.md`** — керування функціоналом, тікети `FM-*`, **§5.1** (наступні кроки); правило **`.cursor/rules/functionality-management.mdc`**

## Витяг функціоналу (крок 11)

- Файл: `docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`
- **Оновлюй** після суттєвих змін: модулів `src/`, публічних маршрутів, feature-прапорців у `Cargo.toml`, етапів Stage 4.x у README.
- OpenAPI (`docs/openapi.yaml`) може бути **неповним**; для точних шляхів див. `src/network/`.

## Керування функціоналом (крок 12)

- Файл: `docs/catalog/FUNCTION_MANAGEMENT.md` — звірка з **STABLE_STATE**, таблиця **FM-***, **§5.1** — єдиний пріоритезований список наступних кроків (узгоджено з Architect-планом); шаблон тікета для розробки.
- Після зміни **NEXT_STEPS** або великого релізу — онови дату, **§5.1** та релевантні рядки `FM-*`.

## Авторозробка та оркестратор

- Найновіший **`docs/development/AUTO_RUN_SESSION_*.md`** — черга спринтів і copy-paste промпт.
- **`docs/development/AUTO_DEV_PATTERNS.md`** — реєстр конкретних патернів (оновлювати після P0/S6).
- Правила: **`.cursor/rules/autonomous-orchestrator.mdc`** (субагенти `explore` / `shell`), **`.cursor/rules/functionality-management.mdc`** (охоплення docs за `STRUCTURE.md`). OpenCode always-on: **`AGENTS.md`**.

## Docs vision (карта зв’язків)

- **`docs/vision/`** — [`vision.svg`](../../../docs/vision/vision.svg), [`manifest.json`](../../../docs/vision/manifest.json), [`extensions.json`](../../../docs/vision/extensions.json), [`index.html`](../../../docs/vision/index.html) (4 панелі в браузері / Cursor Simple Browser).
- Після закриття **PH-S*** + оновлення HANDOFF / FM §5.11 — онови manifest + extensions; правило **`.cursor/rules/docs-vision.mdc`**.

## Правила для агента

- Нові **плани / статус / концепт** — лише під `docs/` у відповідній підпапці (див. `.cursor/rules/documentation.mdc`).
- Не дублюй довгі чеклисти в кореневий `README` — посилайся на `docs/development/` та витяг (крок 11).
- Після додавання головного документа: онови `docs/README.md` або `INDEX`, за потреби `file_list.csv`.
- Запити на **беклог, прогалини, індекс функцій, охоплення docs** — `.cursor/rules/functionality-management.mdc`.
