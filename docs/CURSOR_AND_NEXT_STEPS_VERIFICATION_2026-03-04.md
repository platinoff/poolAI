# Перевірка налаштувань Cursor та подальша розробка PoolAI

**Дата**: 2026-03-04  
**Мета**: Адаптувати інформацію щодо останніх налаштувань Cursor, перевірити модифіковані файли з плану документації та вказати наступні кроки розробки.

---

## 1. Налаштування Cursor — перевірка та адаптація

### 1.1 Правила проєкту (актуальні)

| Джерело | Шлях | Призначення |
|--------|------|-------------|
| **Стартовий контекст** | `.cursor/rules/chat-context.md` | При «налаштуйся для роботи з чатом» / старті сесії — читати першим |
| **Головні правила** | `.cursor/rules/.cursorrules` | Посилання на усі правила в `.cursor/rules/` |
| **AI-помічник** | `.cursor/rules/ai-assistant.md` | Ключові документи, sync концепт→статус→NEXT_STEPS |

**Термінал**: тільки **MSYS2 bash** — `C:\msys64\usr\bin\bash.exe` (UCRT64). Не PowerShell, не cmd.  
**Патчі**: `rust-toolchain.toml`, `.cursor`, `.vscode`, `scripts/`.

### 1.2 Відомі проблеми Cursor та дії

Згідно з `docs/troubleshooting/CURSOR_CHAT_EMPTY_RESPONSES.md`:

- **Симптоми**: порожні/обрізані відповіді, «критичне вікно заповнене», баг ~v2.2.44 (`Element already has context attribute: editor-instance`).
- **Дії по черзі**:
  1. **Reload Window**: `Ctrl+Shift+P` → Developer: Reload Window
  2. **Новий чат** (New Chat / New Agent)
  3. **Перезапуск Cursor**
  4. **Зменшити контекст**: менше відкритих вкладок, менше `@`-файлів у повідомленні; при обрізаних відповідях — новий чат і менше контексту
  5. **Перевірити консоль**: Help → Toggle Developer Tools → Console
- **Рекомендовані налаштування** (у `settings.json`):
  - `cursor.general.enableAgent`: true
  - `cursor.general.enableComposer`: true

### 1.3 Продуктивність Cursor

Згідно з `docs/development/CURSOR_PERFORMANCE_OPTIMIZATION_2026-01-19.md`:

- **File watcher**: виключення `**/Cargo.lock`, `**/.cargo/**`, `**/docs/**/*.md`, `**/.cursor/**`, `**/target/**`, `**/dist/**`, `**/build/**`, `**/tmp/**`
- **Rust Analyzer**: увімкнено checkOnSave, inlayHints, completion.autoimport, lens, hover, procMacro; вимкнено `cargo.allFeatures`
- **Editor**: largeFileOptimizations, semanticHighlighting, bracketPairColorization
- **Cursor Agent**: maxTokens/maxFileSize за потреби; enableIndexing з розумними exclusion

Якщо Cursor повільний: перезапуск Rust Analyzer (`rust-analyzer: Restart server`), перезапуск Agent (`Cursor: Restart Agent`), перевірка розміру `target/`.

### 1.4 Git та Cursor

- **Push**: тільки у **зовнішньому MSYS2 UCRT64** (не вбудований термінал Cursor — можливі CreateFileMapping, index.lock, обрізаний вивід). Перед push закрити Source Control.
- **Формат комітів**: Conventional Commits (`feat(scope): subject`).
- **Перед push**: `cargo fmt --all`; pre-push hook перевіряє формат.

---

## 2. Модифіковані файли з плану документації («шит»)

Джерело: `DOCUMENTATION_CLEANUP_PLAN_2026-01-22.md`, `DOCUMENTATION_ADAPTATION_SUMMARY_2026-01-22.md`.

### 2.1 Актуальні документи (залишити в робочій структурі)

**status/**  
- `PROJECT_STATUS_REPORT_2026-01-19.md` — основний статус  
- `STABLE_STATE_SUMMARY.md` — стабільний стан (v0.2.2)  
- `RUST_ARCHITECT_UPDATE_2026-01-22.md`  
- `MODULE_STATUS_2026.md`, `IMPROVEMENTS_2026.md` (якщо є)

**development/**  
- `NEXT_STEPS_2026-01-19.md` — актуальні наступні кроки  
- `NEXT_STEPS_ARCHITECT_2026-01-22.md` — план Architect  
- `FUTURE_DEVELOPMENT_ROADMAP.md`  
- `CONCEPT_IMPLEMENTATION_CHECKLIST.md` (якщо є)

**concept/**  
- `poolAI_concept_root.txt` — PRIMARY концепція  
- `CONCEPT_UPDATE_2026-01-19.md` / `poolAI_concept.txt` — за потреби

**Індекси**  
- `docs/README.md` — головний індекс (оновлений під v0.2.2)  
- `docs/status/README.md`, `docs/development/README.md` — індекси підкаталогів

### 2.2 Файли для архіву (перемістити в `docs/archive/`)

План передбачає **не видаляти**, а **перемістити** в `docs/archive/status/` та `docs/archive/development/`:

- **status/**: `CURRENT_STATUS.md`, `CURRENT_STATE_VERIFICATION_2026-01-19.md`, `DEVELOPMENT_STATUS_2025.md`, `PROGRESS_SUMMARY_2025.md`, `ARCHITECTURAL_ANALYSIS_2025.md`, `PROGRESS_REPORT.md`, `PERCENTAGE_PLAN.md`, `FINAL_VALIDATION_REPORT.md`, `RELEASE_READINESS_REPORT.md`, `ADMIN_PANEL_STATUS.md`, `BUTTON_FUNCTIONS_AUDIT_2026-01-19.md`, `UI_UX_STATUS_2026-01-19.md`, `UI_UX_VALIDATION_REPORT.md`
- **development/**: застарілі `NEXT_STEPS_*.md` (дати до 2026-01-19), старі `ARCHITECT_*.md`, `STATUS_UPDATE_*.md`, завершені `CURSOR_*.md`, `CLOUD_SDK_*.md`, `TLS_*.md` тощо — повний перелік у `DOCUMENTATION_CLEANUP_PLAN_2026-01-22.md`.

**Наступні кроки по документації** (ручне виконання в MSYS2 bash):

1. Створити каталоги: `docs/archive/status`, `docs/archive/development`
2. Перемістити застарілі файли згідно плану
3. Перевірити посилання в актуальних документах
4. За потреби: коміт змін у репозиторій (якщо `docs/` не в `.gitignore` у вашій робочій копії)

**Примітка**: У поточному `.gitignore` проєкту `docs/` та `.cursor/` виключені з git — тобто ці зміни лише локально. Якщо документація зберігається в репо — прибрати `docs/` з `.gitignore` і виконати кроки вище.

---

## 3. Наступні кроки розробки

Джерела: `docs/status/STABLE_STATE_SUMMARY.md`, `docs/development/NEXT_STEPS_2026-01-19.md`, `DEVELOPMENT_ROADMAP.md`, `chat-context.md`.

### 3.1 Поточний стан

- **Версія в репо**: v0.2.2 (Cargo.toml, src/version.rs).
- **Rust**: 1.92.0 (`rust-toolchain.toml`).
- **Статус**: стабільний; Cloud SDK 100%, HPA init ✅, Load Balancing ✅; Stage 4.4 AI/ML — stubs + на гілці **main** можуть бути вже закомічені ML.4 Model Versioning, ML.5 Experiment Tracking, ML.6 Pipeline Management, Context Memory, Runtime Instance library loading (перевірити `git log -5 --oneline`). Можливий стан **main ahead of origin** (локальні коміти не запушені).
- **Тести**: перевірка через `cargo test`; кількість залежить від features (орієнт 437+ і більше з ml/runtime).

### 3.2 Пріоритети подальшої розробки

| Пріоритет | Крок | Статус |
|-----------|------|--------|
| **P0** | Перевірити git (status, ahead of origin); при потребі push у MSYS2 bash | За потреби |
| **P1** | v0.2.2 release prep | ✅ Завершено |
| **P2** | v0.3.0: ML.4–ML.6, Context Memory, Runtime library на main → тести, CHANGELOG, release | Залежить від main |
| **P3** | ML.1 pruning; ML.2/ML.3 повна реалізація; опціонально performance, UI | Далі |

### 3.3 Конкретні наступні кроки

1. **Git**: `git status --short`, `git log origin/main..HEAD --oneline`. Якщо є коміти ahead — push у зовнішньому MSYS2 bash (PAT/SSH).
2. **Якщо на main є ML.4–ML.6 та Runtime library**: `cargo test`, оновити CHANGELOG (Unreleased → [0.3.0]), за потреби версію в Cargo.toml/version.rs.
3. **ML**: ML.1 pruning strategies; ML.2/ML.3 — pipeline та aggregation для AutoML і Federated Learning.
4. **Опціонально**: performance (connection pooling, API cache, SQLite), UI (Chart.js, таблиці, пошук), CI перевірка після push.

### 3.4 Базові команди (MSYS2 bash)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

git status --short
cargo fmt --all
cargo check --no-default-features --lib
cargo test --no-default-features --lib
# Повний набір тестів: cargo test
```

Git push — copy-paste блок з `.cursor/commands/git-push.md` у **зовнішньому** MSYS2 bash (не в терміналі Cursor).

---

## 4. Швидка перевірка перед роботою

- [ ] Відкрити `chat-context.md` при старті сесії або запиті «налаштуйся для роботи з чатом».
- [ ] Термінал: тільки MSYS2 bash; cargo/git — у зовнішньому bash для push.
- [ ] Якщо Cursor дає порожні/обрізані відповіді — Reload Window, новий чат, менше контексту.
- [ ] Концепт/статус/план: узгоджувати з `poolAI_concept_root.txt`, `STABLE_STATE_SUMMARY.md`, `NEXT_STEPS_2026-01-19.md`.

---

**Підсумок**: Стабільний стан і наступні кроки доадаптовано в `STABLE_STATE_SUMMARY.md` та `NEXT_STEPS_2026-01-19.md`. На main можуть бути ML.4–ML.6, Context Memory, Runtime Instance library — перевірити git; далі — push, v0.3.0 prep, ML.1 pruning, ML.2/ML.3.

**Файл оновлено**: 2026-03-04 (доадаптація docs).
