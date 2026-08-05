# GSV — Galaxy StarWalker Vision (окремий проєкт)

**Канон міграції vision у окремий проєкт.** GSV — самостійний Rust-first проєкт у фолдері `GSV/` репо PoolAI. Працює **95–100% на Rust**, **0–5% WebAssembly** (завжди), UI — тонкий JS/DOM glue у `src/ui/` (без Python).

**Версія:** v0.1.0 · **Стан:** band 102 реалізовано (FM §5.12 §5.83 **✅** `PH-S1659…S1668`).

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
   - **OmniRouter** — Rust AI-проксі/роутер за каталогом «AI providers» (Aug 2026): рекомендований список GPT 5.2 · GPT 5.2 Codex · Claude Opus 4.5 · Claude Sonnet 4.5 · Gemini 3 Pro · MiniMax M2.1 + китайські (DeepSeek V4, Kimi K3, GLM-4.6, Qwen3 Coder) та free-хости (OpenRouter, Groq, Cerebras, NVIDIA, Hugging Face). OpenAI-сумісний proxy (`/api/omni/v1/chat/completions`), каталог моделей з токен-вікнами зі шіта, конфіг `GSV/data/omni.toml` (redacted у UI).
3. Дотримується правила: **Rust-only** для runtime/API/ML/tools; bins — лише Rust (`src/bin/`), жодного Python/Java.

## Структура

```
GSV/
├── README.md            ← цей файл (архітектура / entry)
├── Cargo.toml           ← gsv package (workspace members=["."]; bin `gsv-server`)
├── .cargo/config.toml   ← [build] target-dir="target"
├── src/
│   ├── bin/
│   │   └── gsv_server.rs    ← exe/bin «Galaxy StarWalker Vision» (CLI --host/--port/--repo-root/--data-dir)
│   ├── lib.rs           ← модулі (app_error, boxes, server, state, tracker, vision)
│   ├── app_error.rs     ← AppError (Display + From + IntoResponse JSON)
│   ├── state.rs         ← AppState (tracker, ide_selection, update_flag, events broadcast)
│   ├── tracker.rs       ← TrackerStore + FM §5.12 sprint snapshot parse
│   ├── vision.rs        ← RFC3339 timestamps, git_head, vision JSON read
│   ├── server/mod.rs    ← router (/, /api/*, /events SSE)
│   └── boxes/
│       ├── mod.rs
│       ├── sli.rs       ← SLI каталог з bin/ + scripts/ + src/bin/
│       ├── toolchain.rs ← інвентар тулсів
│       ├── ide.rs       ← opencode + cursor сесії
│       ├── update.rs    ← pending rebuild detection
│       ├── preview.rs   ← Rust syntax highlight + traversal guard
│       ├── terminal.rs  ← whitelist + injection guard
│       ├── hooks.rs     ← tests/bench (read-only target/ + rust_diagnostics)
│       └── omni/        ← OmniRouter: catalog.rs (providers/models зі шіта),
│                          config.rs (omni.toml + env overrides),
│                          proxy.rs (OpenAI-сумісний chat completions / models)
├── ui/index.html        ← single-page UI (SSE, offline/update/resync)
├── tests/
│   ├── gsv_server_contracts.rs  ← 18 integration tests
│   ├── gsv_omni_contracts.rs    ← 8 OmniRouter integration tests
│   └── gsv_update_flow.rs       ← 8 update/SSE tests
└── data/                ← gsv_tracker.json, omni.toml (durable stores, gitignored)
```

Канонічна документація проєкту — `docs/gsv/` (див. нижче).

## Правила (канон з AGENTS.md)

- **Rust-only** runtime/API/ML/RAID/VM/tools. Python заборонено (0× `.py`). Java немає.
- Бinaries — лише `src/bin/` (`cargo run --bin …`).
- UI — vanilla HTML+CSS+JS у `src/ui/`; WASM — лише горизонт (0–5%).
- Термінал — MSYS2 bash (не PowerShell) для `cargo`/`git`.
- Rust стиль: `AppError`, `?`, без `unwrap()`/`expect()` у продукті, `Arc<RwLock<T>>`, `tokio`, `tracing`, модулі через `mod.rs`.

## Збірка / тести

```
# з кореня репо (unset CARGO_TARGET_DIR — GSV має свій target/):
export PATH="/c/Users/${USER}/.cargo/bin:$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN="stable-x86_64-pc-windows-gnu"
cd GSV
cargo build --all-targets
cargo test          # 76 tests (42 unit + 18 contracts + 8 omni + 8 update-flow)
cargo clippy --all-targets   # 0 warnings/errors
cargo run --bin gsv-server -- --port 8890   # live smoke
```

## Запуск

```
cargo run --bin gsv-server -- --host 127.0.0.1 --port 8765 --repo-root S:/rust/poolAI --data-dir GSV/data
```

Endpoints: `GET /` (UI), `/api/health`, `/api/tracker`, `/api/sli`, `/api/toolchain`, `/api/ide/sessions`, `POST /api/ide/select`, `/api/update`, `POST /api/update/notify`, `/api/preview?file=…`, `POST /api/terminal`, `/api/hooks/tests`, `/api/hooks/bench`, `/api/omni`, `/api/omni/config` (GET/POST), `/api/omni/v1/models`, `POST /api/omni/v1/chat/completions`, `POST /api/omni/test`, `GET /events` (SSE).

## Docs (канон)

| Файл | Призначення |
|------|-------------|
| [`docs/gsv/README.md`](../docs/gsv/README.md) | Індекс docs проєкту GSV |
| [`docs/gsv/GSV_ARCHITECTURE.md`](../docs/gsv/GSV_ARCHITECTURE.md) | Архітектура сервера + боксів (Rust / wasm split) |
| [`docs/gsv/GSV_SERVER.md`](../docs/gsv/GSV_SERVER.md) | exe/bin сервер «Galaxy StarWalker Vision» (endpoints, update, offline) |
| [`docs/gsv/GSV_BOXES.md`](../docs/gsv/GSV_BOXES.md) | Специфікація боксів (Tracker, SLI console, Toolchain, IDE, Update, Preview, SLI terminal, Tests/bench hooks) |
| [`docs/gsv/GSV_MIGRATION.md`](../docs/gsv/GSV_MIGRATION.md) | Що мігруємо з `docs/vision/` / `src/` у GSV і як |
| [`docs/gsv/GSV_TECH_ROADMAP.md`](../docs/gsv/GSV_TECH_ROADMAP.md) | **TechPreroadMap** — логічний порядок → future sprints |

## Статус

| Етап | Статус |
|------|--------|
| Архітектура (цей README + `docs/gsv/`) | **✅** |
| Реєстрація sprints (FM §5.12 band 102 `PH-S1659…S1668`) | **✅** |
| gsv-server bin (Cargo/`gsv_server.rs`) | **✅** |
| Бокси (Tracker, SLI console, Toolchain, IDE, Update, Preview, SLI terminal, Tests/bench) | **✅** |
| Тести (52: unit + contracts + update-flow) | **✅** |
| Vision docs sync / migration | **⏳ future** |
