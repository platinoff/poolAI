# GSV docs — Galaxy StarWalker Vision

Документація окремого проєкту **GSV (Galaxy StarWalker Vision)** — самостійної Rust-first vision-системи, мігрованої з `GSV/docs/vision/` у окремий проєкт `GSV/` репо PoolAI.

**Rust 95–100% · WebAssembly 0–5% (завжди) · без Python/Java.**

## Документи

| Файл | Призначення |
|------|-------------|
| [`GSV_ARCHITECTURE.md`](./GSV_ARCHITECTURE.md) | Архітектура сервера + боксів; Rust/wasm split; шари L0–L5 |
| [`GSV_SERVER.md`](./GSV_SERVER.md) | **exe/bin сервер** «Galaxy StarWalker Vision»: endpoints, update-повідомлення, offline-стійкість, metrics resync |
| [`GSV_BOXES.md`](./GSV_BOXES.md) | **Специфікація боксів**: Tracker · SLI console · Toolchain · IDE · Update · Box preview · SLI terminal · Rust tests/benchmarks hook |
| [`GSV_MIGRATION.md`](./GSV_MIGRATION.md) | Міграція з `GSV/docs/vision/` + `src/` у GSV: що переносимо, що лишається, як |
| [`GSV_TECH_ROADMAP.md`](./GSV_TECH_ROADMAP.md) | **TechPreroadMap**: логічний порядок реалізації → future sprints (bands 102 · 108) |
| [`../GSV/docs/GSV_ROLES.md`](../GSV_ROLES.md) | Ролі GSV VDT + канон сесії + Rust ratio gate (band 108) |

## Зв’язок з PoolAI docs

- Vision-UI-спадкоємець: **GSV** (`http://127.0.0.1:8891/`) — Rust-сервер з боксами; legacy [`GSV/docs/vision/index.html`](../vision/index.html) деактивований (band 117, 2026-08-07) — вказівник на GSV.
- Черга sprints: [`docs/catalog/FUNCTION_MANAGEMENT.md`](../../docs/catalog/FUNCTION_MANAGEMENT.md) §5.12 (band 102 ✅ · band 108 ✅ §5.89).
- Концепт: [`docs/concept/poolAI_concept_root.txt`](../../docs/concept/poolAI_concept_root.txt) · [`docs/concept/POOLAI_GALAXY_GRID.md`](../../docs/concept/POOLAI_GALAXY_GRID.md).
- Воркфлоу: [`docs/development/HANDOFF_NEW_SESSION.md`](../../docs/development/HANDOFF_NEW_SESSION.md) · [`docs/development/NEXT_SESSION_PROMPT.md`](../../docs/development/NEXT_SESSION_PROMPT.md).
- Пам'ять GSV: [`GSV/docs/MEMORY.md`](../MEMORY.md) · HANDOFF/NEXT: [`GSV/docs/`](../README.md).

## Правила (коротко)

1. **Rust-only** для runtime/API/tools; bins — лише `src/bin/`.
2. Python заборонено (0× `.py`). Java немає.
3. UI — vanilla HTML+CSS+JS; WASM — горизонт.
4. Бокси — панелі/можливості сервера GSV (детально в `GSV_BOXES.md`).
5. Кожен бінар — окремий Rust bin; тести — Rust (`tests/`), не нові Playwright API-специ.

## Статус

- **2026-08-05:** **band 108 roles/ratio canon ✅** (`PH-S1719…S1728` у FM §5.12 §5.89): `GSV/docs/GSV_ROLES.md` (roles + session canon + ratio gate), `gsv-loc-audit` bin + Ratio box (`GET /api/ratio` + UI card), `tests/gsv_ratio_contracts.rs` (7), memory/HANDOFF/NEXT, poolAI docs parity. Ratio **95.52%** (gate ≥95% ✅), **87 tests green** (46 unit + 18 contracts + 8 omni + 7 ratio + 8 update), clippy 0.
- **2026-08-05:** **OmniRouter box** (`GSV/src/boxes/omni/`) — Rust AI-проксі/роутер за шітом «AI providers by opencode» (Aug 2026): 17 провайдерів + 25 моделей (рекомендований список GPT 5.2 · GPT 5.2 Codex · Claude Opus 4.5 · Claude Sonnet 4.5 · Gemini 3 Pro · MiniMax M2.1), OpenAI-сумісний proxy + redacted конфіг `omni.toml`.
- **2026-08-02:** band 102 **реалізовано** (`PH-S1659…S1668` ✅ у FM §5.12 §5.83): `GSV/` окремий Rust-проєкт (gsv-server + бокси + single-page UI), **52 tests green**, clippy 0, live-smoke ок, timestamps RFC3339. Міграція docs/vision → GSV — ⏳ future.
- **2026-08-01:** архітектура + docs створено; GSV зареєстровано як band 102 (`PH-S1659…S1668`) у FM §5.12. Реалізація — future.
