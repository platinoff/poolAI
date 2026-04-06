---
name: poolai-documentation
description: >-
  PoolAI documentation map (steps 1–11), functionality digest, and where to
  update docs after code or API changes. Use when editing docs/, README,
  planning features, or answering "what does PoolAI do / where is X documented".
---

# PoolAI — документація та витяг функціоналу

## Канонічний порядок (завжди той самий)

Узгодь посилання з кореневим `README.md` → **кроки 1–11**:

1. Кореневий `README.md`
2. `docs/INDEX_2026-03-17.md`
3. `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`
4. `docs/development/HANDOFF_NEW_SESSION.md`
5. Концепція (`docs/concept/poolAI_concept_root.txt`, Grid/Memory/Job)
6. `docs/ARCHITECTURE_REVIEW.md`, `docs/ARCHITECTURE_BEST_PRACTICES.md`
7. `docs/performance/BENCHMARKS.md`, `PROFILING.md`
8. `.github/workflows/ci.yml`
9. `file_list.csv` (оновлюй також після змін у `docs/catalog/`, `.cursor/skills/`)
10. `.cursor/commands/git-push.md`
11. **`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`** — витяг функціоналу

## Витяг функціоналу (крок 11)

- Файл: `docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`
- **Оновлюй** після суттєвих змін: модулів `src/`, публічних маршрутів, feature-прапорців у `Cargo.toml`, етапів Stage 4.x у README.
- OpenAPI (`docs/openapi.yaml`) може бути **неповним**; для точних шляхів див. `src/network/`.

## Правила для агента

- Нові **плани / статус / концепт** — лише під `docs/` у відповідній підпапці (див. `.cursor/rules/documentation.md`).
- Не дублюй довгі чеклисти в кореневий `README` — посилайся на `docs/development/` та витяг (крок 11).
- Після додавання головного документа: онови `docs/README.md` або `INDEX`, за потреби `file_list.csv`.
