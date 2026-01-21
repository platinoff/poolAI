# Project Files Inventory
## Оновлено: 2026-01-19

**Директорія**: `S:\rust\poolAI`

---

## 📁 Структура Директорій

### Корінь проекту

**Конфігураційні файли:**
- `Cargo.toml` - основна конфігурація проекту
- `Cargo.lock` - lock файл залежностей
- `rust-toolchain.toml` - конфігурація Rust toolchain
- `.cursorrules` - правила Cursor IDE
- `.gitignore` - Git ignore rules
- `.gitattributes` - Git attributes
- `build.rs` - build script
- `LICENSE` - ліцензія

**Документація:**
- `README.md` - основний README
- `README.uk.md` - український README

**Конфігурація:**
- `config.toml` - конфігурація проекту
- `config.example.toml` - приклад конфігурації
- `config.https.example.toml` - приклад HTTPS конфігурації

**Cargo альтернативні конфігурації:**
- `Cargo.minimal.toml` - мінімальна конфігурація
- `Cargo.std.toml` - стандартна конфігурація

---

## 📂 Основні Директорії

### `src/` - Вихідний код
- Rust модулі проекту
- Основна логіка додатку

### `tests/` - Тести
- Integration тести
- Unit тести
- Test fixtures

**Піддиректорії:**
- `tests/integration/` - integration тести
  - `tests/integration/cloud/` - cloud provider тести

### `docs/` - Документація
- Вся документація проекту
- Статус звіти
- Плани розробки
- Концепції

**Піддиректорії:**
- `docs/status/` - статус проекту
- `docs/development/` - плани розробки
- `docs/concept/` - концепції проекту
- `docs/archive/` - архівні документи
- `docs/deployment/` - розгортання
- `docs/configuration/` - конфігурація
- `docs/monitoring/` - моніторинг
- `docs/security/` - безпека
- `docs/performance/` - продуктивність
- `docs/troubleshooting/` - troubleshooting
- `docs/migration/` - міграція
- `docs/vm/` - VM модуль
- `docs/cloud/` - Cloud модуль

### `scripts/` - Скрипти
- Build скрипти
- Setup скрипти
- Utility скрипти

**Файли:**
- `setup_msys2_path.ps1` - налаштування MSYS2 PATH
- `setup_msvc_environment.ps1` - налаштування MSVC environment (новий)
- `setup_rust_environment.ps1` - автоматичне налаштування Rust environment (новий)
- `build-with-https.sh` - компіляція з HTTPS features
- `fix_gcc.sh` - виправлення GCC
- `install_gcc.sh` - встановлення GCC
- Інші utility скрипти

### `.cursor/` - Cursor IDE Configuration
- `README.md` - опис конфігурації
- `CHANGELOG.md` - changelog
- `hooks.json` - hooks конфігурація
- `CURSOR_CONFIG_STATUS_2026-01-19.md` - статус конфігурації

**Піддиректорії:**
- `.cursor/rules/` - правила
  - `rust.md` - Rust правила
  - `project-structure.md` - правила структури проекту
- `.cursor/commands/` - команди
  - `check.md`, `test.md`, `review.md`, `fix-issue.md`, `pr.md`
- `.cursor/hooks/` - hooks скрипти
  - `check-tests.ps1` - перевірка тестів

### `.github/` - GitHub Configuration
- `CONTRIBUTING.md` - інструкції для контрибюторів
- `SECURITY.md` - політика безпеки
- `PULL_REQUEST_TEMPLATE.md` - шаблон PR
- `dependabot.yml` - конфігурація Dependabot

**Піддиректорії:**
- `.github/workflows/` - GitHub Actions
  - `ci.yml` - CI workflow
  - `docs.yml` - документація workflow
  - `release.yml` - release workflow
- `.github/ISSUE_TEMPLATE/` - шаблони issues
  - `bug_report.md`
  - `feature_request.md`

### `.vscode/` - VS Code Configuration
- `settings.json` - налаштування VS Code/Cursor

### `.cargo/` - Cargo Configuration
- `config.toml` - Cargo конфігурація

### `docker/` - Docker Files
- Dockerfile та docker-compose файли

### `certs/` - Сертифікати
- `cert.pem` - сертифікат
- `key.pem` - приватний ключ

### `data/` - Дані
- Дані проекту

### `target/` - Build Artifacts
- Результати компіляції (gitignored)

### `benches/` - Benchmarks
- Benchmark тести

---

## 📊 Статистика Файлів

### Rust файли (`.rs`)
- **Кількість**: 50+ файлів
- **Розташування**: `src/`, `tests/`, `benches/`

### Документація (`.md`)
- **Кількість**: 90+ файлів
- **Розташування**: `docs/`, корінь (README файли)

### Скрипти
- **PowerShell** (`.ps1`): `scripts/`, `.cursor/hooks/`
- **Bash** (`.sh`): `scripts/`

### Конфігурація
- **TOML** (`.toml`): корінь, `.cargo/`
- **JSON** (`.json`): `.vscode/`, `.cursor/`, `.github/`
- **YAML** (`.yml`): `.github/`

---

## 🔍 Детальний Перелік по Категоріям

### Тести (`tests/`)

**Cloud тести:**
- `cloud_integration.rs` (448 lines) - розширені integration тести
- `cloud_autoscaling.rs`
- `cloud_config_validation.rs`
- `cloud_kubernetes.rs`
- `cloud_loadbalancing.rs`
- `cloud_operator.rs`
- `cloud_providers.rs`

**Integration тести (`tests/integration/`):**
- `tests/integration/cloud/token_acquisition_tests.rs`
- `tests/integration/cloud/mock_servers.rs` (якщо є)
- `tests/integration/cloud/aws_tests.rs` (якщо є)
- `tests/integration/cloud/azure_tests.rs` (якщо є)
- `tests/integration/cloud/gcp_tests.rs` (якщо є)

**Інші тести:** 40+ файлів (runtime, vm, raid, network, enterprise, тощо)

---

### Документація (`docs/development/`)

**Нові файли (2026-01-19):**
- `NATIVECOMMANDERROR_ANALYSIS_2026-01-19.md` - аналіз помилки
- `TEST_CONFIGURATION_STATUS_2026-01-19.md` - статус тестів
- `TEST_TIMEOUT_RESEARCH_2026-01-19.md` - research про timeout
- `WINDOWS_ENVIRONMENT_STATUS_2026-01-19.md` - статус Windows середовища
- `PROJECT_FILES_INVENTORY_2026-01-19.md` - цей файл

**Інші файли:** 50+ документів про розробку

---

### Скрипти (`scripts/`)

**PowerShell скрипти:**
- `setup_msys2_path.ps1` - налаштування MSYS2 PATH
- `setup_msvc_environment.ps1` - налаштування MSVC environment (новий)
- `setup_rust_environment.ps1` - автоматичне налаштування Rust environment (новий)

**Bash скрипти:**
- `build-with-https.sh` - компіляція з HTTPS
- `fix_gcc.sh` - виправлення GCC
- `install_gcc.sh` - встановлення GCC
- `setup_rust_path.sh` - налаштування Rust PATH
- Інші utility скрипти

---

## 📝 Примітки

1. **Git Status**: Деякі файли можуть бути не додані до git
2. **Build Artifacts**: `target/` директорія gitignored
3. **Документація**: Більшість документації в `docs/` директорії
4. **Скрипти**: Всі скрипти в `scripts/` директорії

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-19
