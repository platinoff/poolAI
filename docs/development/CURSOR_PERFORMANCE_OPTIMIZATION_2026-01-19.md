# Cursor Performance Optimization
## Дата: 2026-01-19

**Проблема**: Cursor працює дуже повільно під час розробки  
**Рішення**: Оптимізація налаштувань для підвищення продуктивності

---

## 🔧 Оптимізації

### 1. File Watcher Exclusions

Додано додаткові виключення для file watcher:
- `**/Cargo.lock` - великий файл, часто змінюється
- `**/.cargo/**` - кеш Cargo
- `**/docs/**/*.md` - документація (не потребує індексації в реальному часі)
- `**/.cursor/**` - конфігурація Cursor
- `**/dist/**`, `**/build/**`, `**/tmp/**`, `**/temp/**` - тимчасові файли

### 2. Search Exclusions

Оптимізовано пошук:
- Виключено `target/`, `Cargo.lock`, `.git/`, `node_modules/`
- Документація залишена для пошуку (корисна для розробки)

### 3. Rust Analyzer Optimizations

**Увімкнено**:
- ✅ `checkOnSave.enable` - перевірка при збереженні
- ✅ `inlayHints` - підказки типів
- ✅ `completion.autoimport` - автоматичний імпорт
- ✅ `lens` - показ референсів та реалізацій
- ✅ `hover.actions` - дії при наведенні
- ✅ `procMacro.enable` - підтримка procedural macros

**Вимкнено**:
- ❌ `cargo.allFeatures` - не завантажувати всі features одночасно (покращує швидкість)

### 4. Editor Performance

- `maxMemoryForLargeFilesMB: 4096` - збільшено пам'ять для великих файлів
- `largeFileOptimizations: true` - оптимізації для великих файлів
- `semanticHighlighting.enabled` - семантичне підсвічування
- `bracketPairColorization` - кольорове підсвічування дужок

### 5. Cursor Agent Optimizations

- `maxTokens: 4000` - обмеження токенів для швидшої роботи
- `maxFileSize: 1000000` - обмеження розміру файлів (1MB)
- `enableIndexing: true` - увімкнено індексацію
- `indexingExclude` - виключення з індексації для швидшої роботи

---

## 📊 Очікувані покращення

1. **Швидкість відгуку**: Зменшення навантаження на file watcher
2. **Швидкість пошуку**: Виключення непотрібних директорій
3. **Швидкість Rust Analyzer**: Оптимізовані налаштування перевірки
4. **Швидкість Cursor Agent**: Обмеження розміру файлів та токенів

---

## 🔍 Моніторинг

Якщо продуктивність все ще низька:

1. Перевірте розмір `target/` директорії:
   ```powershell
   Get-ChildItem -Path target -Recurse | Measure-Object -Property Length -Sum
   ```

2. Перевірте активні процеси Rust Analyzer:
   ```powershell
   Get-Process | Where-Object {$_.ProcessName -like "*rust*"}
   ```

3. Перезапустіть Rust Analyzer:
   - Command Palette (`Ctrl+Shift+P`)
   - `rust-analyzer: Restart server`

4. Перезапустіть Cursor Agent:
   - Command Palette (`Ctrl+Shift+P`)
   - `Cursor: Restart Agent`

---

## 📚 Посилання

- [`CURSOR_SETTINGS_ANALYSIS.md`](./CURSOR_SETTINGS_ANALYSIS.md) - Аналіз налаштувань Cursor
- [`../status/CURRENT_STATUS.md`](../status/CURRENT_STATUS.md) - Поточний стан проекту

---

**Статус**: ✅ **Оптимізовано**  
**Дата**: 2026-01-19
