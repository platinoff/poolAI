# Контекст чату — подальша робота з AI

**При старті сесії або при "налаштуйся для роботи з чатом"** — використовуй цей файл як стартовий контекст.

---

## Проект

- **Назва**: PoolAI  
- **Версія**: v0.2.2 (Rust 1.92.0)  
- **Корінь**: `S:\rust\poolAI`  
- **Роль**: Rust Architect — концепт, статус, плани, код.

---

## Ключові документи (читати першими)

| Призначення | Файл |
|-------------|------|
| **Концепт (PRIMARY)** | `docs/concept/poolAI_concept_root.txt` |
| **Статус** | `docs/status/STABLE_STATE_SUMMARY.md`, `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` |
| **Наступні кроки** | `docs/development/NEXT_STEPS_2026-01-19.md`, `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` |
| **Перевірка Cursor і кроки** | `docs/CURSOR_AND_NEXT_STEPS_VERIFICATION_2026-03-04.md` |
| **Знімок контексту сесії** | `docs/CONTEXT_SNAPSHOT_2026-03-04.md` (локально); корінь репо: `CONTEXT_SNAPSHOT_2026-03-04.md` |
| **Git push** | `.cursor/commands/git-push.md`, `docs/troubleshooting/GIT_PUSH_FAILED.md` |
| **Помічник по файлах** | `file_list.csv` |

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

- **Стабільний**: Cloud SDK 100%, HPA init ✅, Stage 4.4 AI/ML у коді з `feature ml`.  
- **ML.6 pipeline**: кроки Profiling, Tuning, Quantization, Pruning, AutoML, **FederatedAggregation**, Evaluation, Deployment; AutoML за замовчуванням пише в **ML.4/ML.5** (реєстр + експеримент), `automl_skip_registry=true` щоб вимкнути.  
- **HTTP**: `GET /api/enterprise/ai-ml/pipeline/demo` (enterprise+ml) — демо одного кроку без спільного стану.  
- **Док-плани**: тримати узгодженими `NEXT_STEPS_*.md` з фактичним `src/ml/`.

---

## Правила для AI

1. Спочатку звірятися з `docs/concept/poolAI_concept_root.txt` і `NEXT_STEPS_2026-01-19.md`.  
2. Оновлювати концепт/статус/плани узгоджено (концепт → статус → NEXT_STEPS).  
3. **Git і cargo** — у зовнішньому MSYS2 bash (блок з `git-push.md`), зокрема тести як у CI: `export K8S_OPENAPI_ENABLED_VERSION=1.28` і `cargo test --lib --tests --features ml,enterprise,cloud`.  
4. При git push — давати блок з `git-push.md`; якщо не виходить — посилати на `GIT_PUSH_FAILED.md`.
