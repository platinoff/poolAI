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
