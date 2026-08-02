# GSV docs — Galaxy StarWalker Vision

Документація окремого проєкту **GSV (Galaxy StarWalker Vision)** — самостійної Rust-first vision-системи, мігрованої з `docs/vision/` у окремий проєкт `GSV/` репо PoolAI.

**Rust 95–100% · WebAssembly 0–5% (завжди) · без Python/Java.**

## Документи

| Файл | Призначення |
|------|-------------|
| [`GSV_ARCHITECTURE.md`](./GSV_ARCHITECTURE.md) | Архітектура сервера + боксів; Rust/wasm split; шари L0–L5 |
| [`GSV_SERVER.md`](./GSV_SERVER.md) | **exe/bin сервер** «Galaxy StarWalker Vision»: endpoints, update-повідомлення, offline-стійкість, metrics resync |
| [`GSV_BOXES.md`](./GSV_BOXES.md) | **Специфікація боксів**: Tracker · SLI console · Toolchain · IDE · Update · Box preview · SLI terminal · Rust tests/benchmarks hook |
| [`GSV_MIGRATION.md`](./GSV_MIGRATION.md) | Міграція з `docs/vision/` + `src/` у GSV: що переносимо, що лишається, як |
| [`GSV_TECH_ROADMAP.md`](./GSV_TECH_ROADMAP.md) | **TechPreroadMap**: логічний порядок реалізації → future sprints (band 102 `PH-S1659…S1668`) |

## Зв’язок з PoolAI docs

- Vision-UI-спадкоємець: [`docs/vision/index.html`](../vision/index.html) — статичний Galaxy UI; GSV переносить логіку в Rust-сервер з боксами.
- Черга sprints: [`docs/catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (band 102).
- Концепт: [`docs/concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt) · [`docs/concept/POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md).
- Воркфлоу: [`docs/development/HANDOFF_NEW_SESSION.md`](../development/HANDOFF_NEW_SESSION.md) · [`docs/development/NEXT_SESSION_PROMPT.md`](../development/NEXT_SESSION_PROMPT.md).

## Правила (коротко)

1. **Rust-only** для runtime/API/tools; bins — лише `src/bin/`.
2. Python заборонено (0× `.py`). Java немає.
3. UI — vanilla HTML+CSS+JS; WASM — горизонт.
4. Бокси — панелі/можливості сервера GSV (детально в `GSV_BOXES.md`).
5. Кожен бінар — окремий Rust bin; тести — Rust (`tests/`), не нові Playwright API-специ.

## Статус

- **2026-08-01:** архітектура + docs створено; GSV зареєстровано як band 102 (`PH-S1659…S1668`) у FM §5.12. Реалізація — future.
