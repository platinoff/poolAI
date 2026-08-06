# GSV Boxes — панелі/можливості «Galaxy StarWalker Vision»

Специфікація боксів сервера GSV. Кожен бокс — панель UI + Rust-модуль.

## 1. Tracker (технічні параметри workflow)

**Роль:** показує технічні параметри виконаного воркфлоу (що реально виконувалось).

Дані: спринти (PH-S*), команди, часові мітки, статуси, кількість файлів/LOC, wall-clock.

| Поле | Джерело |
|------|---------|
| Sprint id / band | FM §5.12 |
| Виконані команди | shell history / logs |
| Тривалість кроків | timestamps |
| LOC / files | `poolai-loc-audit` |
| Статус / ✅ | FM §5.12 |

Rust модуль: `tracker/` → `gsv_tracker.json`.

## 2. SLI console (команди + SLI-функції)

**Роль:** бачити, які команди використовуються, та **всі SLI-функції, які можна створити з наявних скриптів** (+ нові).

- Парсинг `bin/` + `scripts/` + `src/bin/` → каталог SLI-команд (назва, опис, входи).
- Виводить фактично використані команди (з Tracker/history).
- Пропонує **незадіяні скрипти** → потенційні нові SLI-функції.
- Відкритий реєстр для нових функцій.

Rust модуль: `sli/` → `gsv_sli.json`.

## 3. Toolchain (які тули використовуються)

**Роль:** інвентар тулів проєкту.

| Тул | Версія | Джерело |
|-----|--------|---------|
| rustc / cargo | 1.92.0 | `rust-toolchain.toml` |
| clippy / rustfmt | — | toolchain |
| MSYS2 bash | — | AGENTS.md |
| Node / Playwright | — | `e2e/` |
| Cursor / opencode | 3.13.21 | service |

Rust модуль: `toolchain/` → `gsv_toolchain.json`.

## 4. IDE (opencode + cursor чати; вибір, з чим працювати)

**Роль:** портувати opencode + cursor чати; можливість обирати, з чим працювати.

- Читання сесій/чатів opencode (`~/.local/share/opencode/`) та cursor (`.cursor/`).
- Список сесій у UI; вибір активної → стрічка повідомлень.
- Вибір робочого фолдера/спринту.

Rust модуль: `ide/` (read-only).

## 5. Update (оновлення бінарника; offline-стійкість)

**Роль:** якщо оновлюємо/дебажимо vision Rust-кодбазу і запущена bin-версія — сервер приймає **повідомлення про апдейт**; вебсторінка не падає при офлайн.

Поведінка:
1. Перекомпіляція → новий бінарник.
2. UI: **«Update»** замість reload.
3. Сторінка не падає — просто «offline».
4. Після реконекту — **всі метрики синхронізуються** (resync).

Деталі: [`GSV_SERVER.md`](./GSV_SERVER.md) (endpoints `/api/update`, `/events`, offline-кешування).

## 6. Box preview (Rust-кольори відповідно до синтаксису)

**Роль:** превʼю файлів, де **Rust-кольори відповідають синтаксису** (висвітлення синтаксису Rust).

- `GET /api/preview?file=…` → HTML з токен-висвітленням (Rust-палітра).
- Підтримка `.rs`, `.toml`, `.md`, `.js`, `.css`.

## 7. SLI terminal (AI → команди)

**Роль:** щоб AI (ШІ) міг посилати команди на сервер.

- `POST /api/terminal {command}` — виконати SLI-команду.
- Аудит у Tracker; результат — JSON/стdout.
- Обмеження: whitelist SLI-каталогу, sandbox (без довільних команд поза реєстром).

## 8. Rust tests / benchmarks hook (без перекомпіляції)

**Роль:** запуск тестів/бенчмарків **без перекомпіляції** (read-only hook).

- `GET /api/hooks/tests` → статус + результати з `target/` (deps, `test-*` bins) без `cargo build`.
- `GET /api/hooks/bench` → Criterion medians (read `target/criterion/`).
- Дані не перебудовують проєкт — лише зчитують наявні артефакти.

## 9. OmniRouter (Rust AI-проксі/роутер)

**Роль:** Rust-роутер по AI-провайдерах із шіта «AI providers by opencode» (Aug 2026) — рекомендований список (GPT 5.2, GPT 5.1 Codex, Claude Opus 4.5, Claude Sonnet 4.5, MiniMax M2.1, Gemini 3 Pro) + китайські (DeepSeek V4-Pro/Flash, Kimi K3/K2.7 Code, GLM-4.6, Qwen3 Coder 480B) + free/швидкі хости (OpenRouter `:free`, Groq, Cerebras, NVIDIA, Hugging Face, GitHub Copilot Free).

**Дані:**

| Поле | Джерело |
|------|---------|
| Каталог провайдерів | `catalog.rs` (17 providers) |
| Каталог моделей (ctx / max output) | шіт «AI providers», `catalog.rs` (25 models) |
| Конфіг / тюнінг | `GSV/data/omni.toml` + env `OMNI_<PROVIDER>_API_KEY` / `_BASE_URL` |
| Рекомендований список | шіт (6 моделей) |

**Endpoints:**

- `GET /api/omni` — overview wire (providers, models, recommended, routing).
- `GET /api/omni/config` — конфіг **redacted** (лише `key_set`, без ключів).
- `POST /api/omni/config` — тюнінг (base_url / api_key / enabled / priority / routing).
- `GET /api/omni/v1/models` — OpenAI-сумісний список моделей.
- `POST /api/omni/v1/chat/completions` — OpenAI-сумісний proxy (SSE passthrough для `stream:true`; dry-run через `X-Omni-Dry-Run: 1`).
- `POST /api/omni/test {provider}` — connectivity check (`GET {base}/models`).

**Роутинг** (`proxy.rs::select_provider`): `X-Omni-Provider` header / `provider` у тілі → власник моделі з каталогу → `routing.default_provider` → `routing.fallback_order` → найвищий пріоритет серед enabled провайдерів з base_url. `base_url` може вказувати на OmniRoute (`http://127.0.0.1:20128/v1`) — тоді Rust-роутер проксірує запити через OmniRoute.

Rust модуль: `omni/` (catalog.rs, config.rs, proxy.rs) → `GSV/data/omni.toml`.

## Зведена таблиця

| Box | Rust module | Endpoint | Джерело даних |
|-----|-------------|----------|---------------|
| Tracker | `tracker/` | `/api/tracker` | FM §5.12, logs, loc-audit |
| SLI console | `sli/` | `/api/sli` | `bin/`, `scripts/`, `src/bin/` |
| Toolchain | `toolchain/` | `/api/toolchain` | toolchain, env |
| IDE | `ide/` | `/api/ide/…` | opencode/cursor сесії |
| Update | `update/` | `/api/update` · `/events` | бінарник/версія |
| Box preview | `preview/` | `/api/preview` | файли |
| SLI terminal | `terminal/` | `/api/terminal` | SLI-каталог |
| Tests/bench hooks | `hooks/` | `/api/hooks/…` | `target/` артефакти |
| OmniRouter | `omni/` | `/api/omni/…` | шіт «AI providers», `omni.toml`, proxy |
| Vision | `vision/` (`boxes/vision.rs`) | `/api/vision*` · `/assets/vision.svg` | `docs/vision/{manifest,feed,extensions}.json` → `GSV/data/gsv_*.json` |
