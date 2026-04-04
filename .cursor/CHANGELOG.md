# Cursor Agent Configuration Changelog

## 2026-04-04 — Welcome TurboQuant (план)

- Додано `docs/ml/TURBOQUANT_INTEGRATION.md` (ресерч Google TurboQuant / PolarQuant / QJL, придатність для PoolAI, фази інтеграції).
- Оновлено `docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md` (**Priority 2b**), `docs/concept/poolAI_concept_root.txt`, `docs/concept/CONCEPT_UPDATE_2026-01-19.md`, `docs/INDEX_2026-03-17.md`, `docs/ml/PIPELINE_MANAGEMENT.md`, `README.md`, `.cursor/rules/rust-architect.md`.

## 2026-04-03 — Узгодження з CI та toolchain

- Оновлено `.cursor/rules/rust-architect.md`: MSYS2 лишається канонічним для ручного git push; дозволено контекст PowerShell для агента/CI як у GitHub Actions; Dependabot; рекомендований набір тестів `cargo test --lib --tests --features ml,enterprise,cloud` замість обовʼязкового `cargo test --all-features` на Windows MSVC; виправлено опис `file_list.csv`.
- Перевірено: `cargo fmt`, `cargo clippy --all-targets --all-features` (exit 0, з попередженнями), CI-еквівалент тестів після правок у `tests/ml_pruning_integration.rs` та `tests/saml_auth_flow_integration.rs`.

## 2026-01-19 - Оптимізація налаштувань

### Виправлення проблем з втратою зв'язку з агентом

**Проблема**: Агент втрачав зв'язок під час роботи.

**Виправлення**:
1. ✅ Додано налаштування для Cursor agent в `.vscode/settings.json`
2. ✅ Додано file watcher exclusions для покращення продуктивності
3. ✅ Створено hooks для автоматичної перевірки тестів
4. ✅ Оптимізовано налаштування терміналу

### Зміни

#### `.vscode/settings.json`
- Додано `cursor.chat.model`, `cursor.general.enableAgent`, `cursor.general.enableComposer`
- Додано `files.watcherExclude` для виключення `target/`, `.git/`, `node_modules/`
- Вимкнено `editor.formatOnSave` для уникнення конфліктів з агентом

#### `.cursor/hooks.json` (новий)
- Створено hooks.json для опціонального використання
- Додано hook для перевірки тестів перед зупинкою агента

#### `.cursor/hooks/check-tests.ps1` (новий)
- Створено PowerShell скрипт для автоматичної перевірки тестів

### Рекомендації

1. **Перезапустіть Cursor** після змін
2. **Перевірте роботу агента** - спробуйте `/check` або `/test`
3. **Використовуйте Plan Mode** (Shift+Tab) для складних задач
4. **Починайте нові розмови** при переході до інших задач

### Посилання

- [Cursor Agent Best Practices](https://cursor.com/blog/agent-best-practices)
- [Детальний аналіз](../../docs/development/CURSOR_SETTINGS_ANALYSIS.md)
