# Контекст чату — подальша робота з AI

**При старті сесії або при "налаштуйся для роботи з чатом"** — використовуй цей файл як стартовий контекст.

---

## Проект

- **Назва**: PoolAI  
- **Версія**: v0.2.2 (Rust 1.92.0)  
- **Корінь**: `S:\rust\poolAI`  
- **Роль**: Rust Architect — концепт, статус, плани, код.
- **Стек (канон):** Rust (`src/`, `tests/`, `crates/`); UI — JS у `src/ui/`; **без Python runtime** (див. `runtime-stack-policy.mdc`).

---

## Ключові документи (читати першими)

| Призначення | Файл |
|-------------|------|
| **Старт сесії / handoff** | `docs/development/HANDOFF_NEW_SESSION.md` (кроки 1–12) |
| **Авторозробка (оркестратор)** | `docs/development/AUTO_RUN_SESSION_2026_POST_HORIZON.md` |
| **Стек / no Python** | `.cursor/rules/runtime-stack-policy.mdc` |
| **Патерни для автопрогону** | `docs/development/AUTO_DEV_PATTERNS.md` |
| **FM / беклог (крок 12)** | `docs/catalog/FUNCTION_MANAGEMENT.md` §5.1 |
| **Концепт (PRIMARY)** | `docs/concept/poolAI_concept_root.txt` |
| **Статус** | `docs/status/STABLE_STATE_SUMMARY.md` |
| **Architect-план** | `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` |
| **Таксономія docs** | `docs/STRUCTURE.md` |
| **Git push** | `.cursor/commands/git-push.md`, `docs/troubleshooting/GIT_PUSH_FAILED.md` |
| **Помічник по файлах** | `file_list.csv` |

**Правила Cursor:** `.cursor/rules/autonomous-orchestrator.mdc`, `.cursor/rules/functionality-management.mdc`, skill `.cursor/skills/poolai-documentation/SKILL.md`

---

## Термінал і середовище

- **Усі локальні команди** (`cargo`, `git`, скрипти): **MSYS2 bash** — `C:\msys64\usr\bin\bash.exe` (UCRT64). Для push — зовнішнє вікно MSYS2, не інтегрований термінал Cursor (див. `git-push.md`).  
- **CI** на GitHub — окреме середовище; локально PowerShell/cmd для цього репо **не** використовуємо.  
- **Патчі**: `rust-toolchain.toml`, `.cursor`, `.vscode`, `scripts/`.  
- **PATH у bash** (для `cargo`/`rustc` з rustup і MSYS2 gcc):  
  `export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"`  
- **Шлях у MSYS2**: `/s/rust/poolAI` = `S:\rust\poolAI`.

---

## Git

- **Без .sh**: git — copy-paste блок з `.cursor/commands/git-push.md`.  
- **Push**: **тільки зовнішній MSYS2 UCRT64** (ніколи термінал Cursor — CreateFileMapping, index.lock, обрізаний вивід). Закрити Source Control.  
- **Формат комітів**: Conventional Commits (`feat(scope): subject`) + **тіло з Summary** (кілька `-m` або абзаци): що змінено, які тести, нотатки (див. `git-push.md` п.3a).  
- **Після push**: короткий самарі для чату — `git-push.md` п.3b.  
- **Перед push**: `cargo fmt --all`; pre-push hook перевіряє формат.

---

## Поточний стан і далі

- **Post-Horizon:** FM-020…025 ✅; наступна — **FM-026** (`NEXT_SESSION_PROMPT.md`).  
- **Черга:** `AUTO_RUN_SESSION_2026_POST_HORIZON.md` §4.  
- **Тести (канон):** `K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci` (не повний `cargo test` з doctests на Windows без потреби).  
- **Не робити:** Python / `.py` у репо (0 файлів); OpenAPI audit — `cargo run --bin poolai-openapi-gap-audit`.  
- **Пріоритети:** `FUNCTION_MANAGEMENT.md` §5.1 → Architect → HANDOFF.

---

## Правила для AI

1. Старт: `HANDOFF_NEW_SESSION.md` + `FUNCTION_MANAGEMENT.md` §5.1; авторозробка — `autonomous-orchestrator.mdc` + найновіший `AUTO_RUN_SESSION_*.md`.  
2. Менеджер функціоналу: `functionality-management.mdc` (охоплення docs за `STRUCTURE.md`, не весь репо).  
3. Оновлювати концепт/статус/FM/DIGEST узгоджено після змін API.  
4. **Git і cargo** — зовнішній MSYS2 bash; `cargo test-ci` як у CI.  
5. Push — `git-push.md`; помилки — `GIT_PUSH_FAILED.md`.
