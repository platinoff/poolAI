# Cursor Agent Configuration Changelog

## 2026-07-22 — Cursor 3.12.30 service band (PH-SVC21…SVC30)

- **`cursor-environment-baseline.mdc`**: local `cursor` **3.12.30**; public changelog latest numbered IDE = **3.11**; High Contrast flicker ops note; pointer to `CURSOR_UPDATE_RESEARCH_2026-07-22.md`.
- **`poolai-agent-roles.mdc`**: research pointer → Jul 22; High Contrast flicker note.
- **Vision:** `poolai-vision-sync` мержить FM enterprise `queue — band` секції → Sprint queue + Feed показують останні закриті PH-S* (не лише журнал §5.12 до PH-S1018).
- **Docs:** `CURSOR_UPDATE_RESEARCH_2026-07-22.md`; HANDOFF/NEXT/README/INDEX/ENV zriz; FM §5.16 PH-SVC21…SVC30.
- **Product queue unchanged:** §5.12 **0** · next session **`абракадабра`** → band 62.

## 2026-07-21 — Cursor 3.12.29 service band (PH-SVC11…SVC20)

- **`cursor-environment-baseline.mdc`**: local `cursor` **3.12.29**; note that public changelog latest numbered IDE = **3.11**; Jul 17 = Slack-only; pointer to `CURSOR_UPDATE_RESEARCH_2026-07-21.md`.
- **`poolai-agent-roles.mdc`**: side chats **local-only**; mode picker (Shift+Tab); `/multitask`; cloud hooks list includes `afterAgentResponse` / `stop`.
- **Docs:** `CURSOR_UPDATE_RESEARCH_2026-07-21.md`; HANDOFF/NEXT/README/INDEX/ENV zriz; FM §5.16 PH-SVC11…SVC20.
- **Product queue unchanged:** §5.12 **0** · next session **`абракадабра`** → band 59.

## 2026-07-17 — Cursor 3.12.17 service band (PH-SVC01…SVC10)

- **`cursor-environment-baseline.mdc`**: `cursor` **3.12.17**; `git` **2.50.0**; pointer to `CURSOR_UPDATE_RESEARCH_2026-07-17.md`; notes for 3.11 side chats + cloud hooks.
- **`poolai-agent-roles.mdc`**: side chats for research tangents; transcript search; FM **§5.16** service band.
- **Docs:** `CURSOR_UPDATE_RESEARCH_2026-07-17.md`; HANDOFF/NEXT/README/INDEX zriz; FM §5.16 journal.
- **Vision:** `poolai-vision-sync` → rev **297**; `--check` green.

## 2026-06-18 — Cursor 3.7.42 + абракадабра vision close band

- **`cursor-environment-baseline.mdc`**: `cursor` **3.7.42**; Multitask Mode, SwitchMode, нові subagent types.
- **`poolai-agent-roles.mdc`**: `bugbot`, `security-review`, `best-of-n-runner` (лише за явним запитом).
- **`poolai-session-iteration.mdc`**: канон drain → **vision close** → **один** `test-ci` → **один** commit (код + `docs/vision/*`); § Vision close band.
- **`virtual-development-team.mdc`**: one-liner `абракадабра` узгоджено з NEXT_SESSION.

## 2026-06-16 — PH-S201 post-push VDT hook

- **`hooks.json`**: `postToolUse` → `.cursor/hooks/post-push-ph-s-notify.sh` (Shell matcher).
- Після успішного `git push`, якщо subject містить `PH-S*` — `additional_context` з чеклістом FM/HANDOFF/NEXT/vision.
- Додано [`.cursor/hooks/README.md`](hooks/README.md); self-test: `bash .cursor/hooks/post-push-ph-s-notify.sh --self-test`.

## 2026-06-15 — Cursor 3.7.36 baseline

- Оновлено **`.cursor/rules/cursor-environment-baseline.mdc`**: `cursor` **3.7.36** (було 3.2.21); нотатка про subagent/Task tool і user skills path.
- Перевірено: `hooks.json` — `hooks: {}` (канон без stop-hook); MSYS2 bash для `cargo`/`git` без змін.

## 2026-05-20 — Runtime stack policy (block Python)

- Додано **`.cursor/rules/runtime-stack-policy.mdc`** (`alwaysApply: true`) — Rust primary; заборона Python runtime; dev-only `bin/openapi-gap-audit.py`.
- Оновлено: `project-structure.md`, `rust.md`, `rust-architect.md`, `ai-assistant.md`, `chat-context.md`, `autonomous-orchestrator.mdc`, `.cursorrules`, `poolai-documentation/SKILL.md`.
- Доки: `docs/STRUCTURE.md` §7, `ARCHITECTURE_BEST_PRACTICES.md` § Technology stack, `README.md`, `HANDOFF`, `NEXT_SESSION_PROMPT`.

## 2026-04-06 — Без PowerShell і без `.ps1` у `.cursor/`

- Видалено `.cursor/hooks/check-tests.ps1`; у `hooks.json` поле `hooks` порожнє (немає stop-hook на PowerShell).
- Усі правила знову канонічно вимагають **MSYS2 bash** для локальних `cargo`/`git` (без PowerShell/cmd для цього репо); оновлено `msys2-windows.md`, `rust-architect.md`, `chat-context.md`, `.cursorrules`, `commands/test.md`, `commands/git-push.md`, `.cursor/README.md`.
- У `git-push.md`: `export K8S_OPENAPI_ENABLED_VERSION=1.28`, закоментований рядок CI-тестів, `git add -f` для `hooks.json` / `CHANGELOG` / `README` у `.cursor/`.

## 2026-04-05 — Windows 11 / MSYS2 нотатки (без PowerShell-винятків)

- Нотатки для **Windows 11 (24H2+, збірки 26100+)** — Defender і `target/`, long paths, **host MSVC** vs **target GNU** у `rust-toolchain.toml`.
- **Місце на диску**: `cargo clean` або `CARGO_TARGET_DIR` при великому `target/`.
- (Запис про дозвіл PowerShell для агента **скасовано** 2026-04-06.)

## 2026-04-04 — Welcome TurboQuant (план)

- Додано `docs/ml/TURBOQUANT_INTEGRATION.md` (ресерч Google TurboQuant / PolarQuant / QJL, придатність для PoolAI, фази інтеграції).
- Оновлено `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` (**Priority 2b**), `docs/concept/poolAI_concept_root.txt`, `docs/concept/CONCEPT_UPDATE_2026-01-19.md`, `docs/INDEX_2026-03-17.md`, `docs/ml/PIPELINE_MANAGEMENT.md`, `README.md`, `.cursor/rules/rust-architect.md`.

## 2026-04-04 (пізніше) — TurboQuant: лише Rust

- Політика: **без Python**; імплементація в `src/ml/…`. Оновлено `TURBOQUANT_INTEGRATION.md`, Priority 2b, концепт, INDEX, PIPELINE_MANAGEMENT, README; додано таблицю **наступних кроків за пріоритетом** у `NEXT_STEPS_ARCHITECT_2026-03-17.md`.

## 2026-04-03 — Узгодження з CI та toolchain

- Оновлено `.cursor/rules/rust-architect.md`: MSYS2 для git push; Dependabot; рекомендований набір тестів `cargo test --lib --tests --features ml,enterprise,cloud` замість обовʼязкового `cargo test --all-features` на Windows MSVC; виправлено опис `file_list.csv`. (Рядок про PowerShell для агента більше не актуальний — див. 2026-04-06.)
- Перевірено: `cargo fmt`, `cargo clippy --all-targets --all-features` (exit 0, з попередженнями), CI-еквівалент тестів після правок у `tests/ml_pruning_integration.rs` та `tests/saml_auth_flow_integration.rs`.

## 2026-01-19 - Оптимізація налаштувань

### Виправлення проблем з втратою зв'язку з агентом

**Проблема**: Агент втрачав зв'язок під час роботи.

**Виправлення**:
1. ✅ Додано налаштування для Cursor agent в `.vscode/settings.json`
2. ✅ Додано file watcher exclusions для покращення продуктивності
3. ✅ Додано опціональні Cursor hooks (раніше — перевірка тестів; з 2026-04-06 hooks порожні, без `.ps1`)
4. ✅ Оптимізовано налаштування терміналу

### Зміни

#### `.vscode/settings.json`
- Додано `cursor.chat.model`, `cursor.general.enableAgent`, `cursor.general.enableComposer`
- Додано `files.watcherExclude` для виключення `target/`, `.git/`, `node_modules/`
- Вимкнено `editor.formatOnSave` для уникнення конфліктів з агентом

#### `.cursor/hooks.json` (новий)
- Створено `hooks.json` для опціонального використання; з **2026-04-06** об’єкт `hooks` порожній (без stop-hook і без `.ps1`).

#### `.cursor/hooks/check-tests.ps1` (видалено 2026-04-06)
- Раніше: PowerShell-скрипт для stop-hook; замінено політикою «лише MSYS2 bash» і порожнім `hooks` у `hooks.json`.

### Рекомендації

1. **Перезапустіть Cursor** після змін
2. **Перевірте роботу агента** - спробуйте `/check` або `/test`
3. **Використовуйте Plan Mode** (Shift+Tab) для складних задач
4. **Починайте нові розмови** при переході до інших задач

### Посилання

- [Cursor Agent Best Practices](https://cursor.com/blog/agent-best-practices)
- [Детальний аналіз](../../docs/development/CURSOR_SETTINGS_ANALYSIS.md)
