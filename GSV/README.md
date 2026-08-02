# GSV — Galaxy StarWalker Vision (окремий проєкт)

**Канон міграції vision у окремий проєкт.** GSV — самостійний Rust-first проєкт у фолдері `GSV/` репо PoolAI. Працює **95–100% на Rust**, **0–5% WebAssembly** (завжди), UI — тонкий JS/DOM glue у `src/ui/` (без Python).

**Версія:** v0.1.0 (planning) · **Стан:** архітектура + docs (`docs/gsv/`) · **Реалізація:** future sprints band 102 (FM §5.12 `PH-S1659…S1668`).

## Суть

Окремий **bin/exe сервер** «Galaxy StarWalker Vision» (`gsv-server`) на Rust, який:

1. Віддає vision UI (доки ↔ код ↔ спринти) — спадкоємець `docs/vision/index.html`.
2. Виконує **бокси GSV** (панелі/можливості):
   - **Tracker** — технічні параметри виконаного workflow (спринти, команди, часи).
   - **SLI console** — які команди використовуються + усі SLI-функції, які можна створити з наявних скриптів (+ нові).
   - **Toolchain** — які тулси використовуються (rustc/cargo/clippy/MSYS2/…).
   - **IDE** — портовані opencode + cursor чати; вибір, з чим працювати.
   - **Update** — якщо запущено bin-версію, сервер приймає **повідомлення про апдейт** (перекомпіляція → у UI «Update» замість reload); вебсторінка **не падає при офлайн**; після реконекту всі метрики синхронізуються.
   - **Box preview** — Rust-кольори відповідно до синтаксису.
   - **SLI terminal** — щоб AI міг посилати команди.
   - **Rust tests / benchmarks hook** — запуск без перекомпіляції.
3. Дотримується правила: **Rust-only** для runtime/API/ML/tools; bins — лише Rust (`src/bin/`), жодного Python/Java.

## Структура (план)

```
GSV/
├── README.md            ← цей файл (архітектура / entry)
├── Cargo.toml           ← gsv workspace / package (future)
├── src/
│   ├── bin/
│   │   └── gsv_server.rs    ← exe/bin «Galaxy StarWalker Vision» (future)
│   └── (boxes, sli, toolchain, tracker, ide, update — future PH-S*)
└── docs/                ← внутрішні docs проєкту (future)
```

Канонічна документація проєкту — `docs/gsv/` (див. нижче).

## Правила (канон з AGENTS.md)

- **Rust-only** runtime/API/ML/RAID/VM/tools. Python заборонено (0× `.py`). Java немає.
- Бinaries — лише `src/bin/` (`cargo run --bin …`).
- UI — vanilla HTML+CSS+JS у `src/ui/`; WASM — лише горизонт (0–5%).
- Термінал — MSYS2 bash (не PowerShell) для `cargo`/`git`.
- Rust стиль: `AppError`, `?`, без `unwrap()`/`expect()` у продукті, `Arc<RwLock<T>>`, `tokio`, `tracing`, модулі через `mod.rs`.

## Docs (канон)

| Файл | Призначення |
|------|-------------|
| [`docs/gsv/README.md`](../docs/gsv/README.md) | Індекс docs проєкту GSV |
| [`docs/gsv/GSV_ARCHITECTURE.md`](../docs/gsv/GSV_ARCHITECTURE.md) | Архітектура сервера + боксів (Rust / wasm split) |
| [`docs/gsv/GSV_SERVER.md`](../docs/gsv/GSV_SERVER.md) | exe/bin сервер «Galaxy StarWalker Vision» (endpoints, update, offline) |
| [`docs/gsv/GSV_BOXES.md`](../docs/gsv/GSV_BOXES.md) | Специфікація боксів (Tracker, SLI console, Toolchain, IDE, Update, Preview, SLI terminal, Tests/bench hooks) |
| [`docs/gsv/GSV_MIGRATION.md`](../docs/gsv/GSV_MIGRATION.md) | Що мігруємо з `docs/vision/` / `src/` у GSV і як |
| [`docs/gsv/GSV_TECH_ROADMAP.md`](../docs/gsv/GSV_TECH_ROADMAP.md) | **TechPreroadMap** — логічний порядок → future sprints (band 102) |

## Статус

| Етап | Статус |
|------|--------|
| Архітектура (цей README + `docs/gsv/`) | **✅ (ця сесія)** |
| Реєстрація sprints (FM §5.12 band 102 `PH-S1659…S1668`) | **✅ (ця сесія)** |
| gsv-server bin (Cargo/`gsv_server.rs`) | ⏳ future |
| Бокси (Tracker, SLI console, Toolchain, IDE, Update, Preview, SLI terminal, Tests/bench) | ⏳ future |
| Vision docs sync / migration | ⏳ future |
