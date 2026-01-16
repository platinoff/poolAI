# 🛡️ Avast False Positive для Rust бінарників

## Проблема

Avast антивірус може циклічно сканувати Rust бінарники (`rust_out.exe`, `poolai.exe`, тощо) в папці `target/`, що:
- Затримує компіляцію
- Викликає циклічні сканування (по 60 секунд)
- Блокує doc-tests
- Перешкоджає нормальній роботі Rust

## ✅ Рішення: Додати виключення в Avast

### Варіант 1: Виключити всю папку `target/` (рекомендовано)

1. Відкрийте **Avast**
2. Перейдіть до **Settings** (Налаштування)
3. Виберіть **General** → **Exceptions** (Загальні → Виключення)
4. Натисніть **Add Exception** (Додати виключення)
5. Виберіть **Folder** (Папка)
6. Додайте повний шлях до `target/`:
   ```
   S:\rust\poolAI\target
   ```
7. Натисніть **OK** і перезапустіть Avast

### Варіант 2: Виключити конкретний файл

Якщо потрібно виключити тільки `rust_out.exe`:

1. Відкрийте **Avast** → **Settings** → **General** → **Exceptions**
2. Натисніть **Add Exception**
3. Виберіть **File** (Файл)
4. Додайте:
   ```
   S:\rust\poolAI\target\**\rust_out.exe
   ```

### Варіант 3: Виключити всі `.exe` в `target/`

1. Відкрийте **Avast** → **Settings** → **General** → **Exceptions**
2. Натисніть **Add Exception**
3. Виберіть **File** (Файл)
4. Додайте pattern:
   ```
   S:\rust\poolAI\target\**\*.exe
   ```

## 🔍 Перевірка

Після додавання виключення:

1. Запустіть `cargo build`
2. Перевірте, що Avast не сканує файли в `target/`
3. Запустіть `cargo test --doc` - не повинно бути зависань

## 📝 Додаткові нотатки

- **`.gitignore`** вже містить `target/` - це нормально
- Rust компілює бінарники в `target/debug/` та `target/release/`
- `rust_out.exe` - це тестовий бінарник Rust для doc-tests
- False positive відбувається, бо Rust генерує бінарники з мінімальними метаданими

## 🚨 Якщо проблема залишається

1. **Оновіть Avast** до останньої версії
2. **Додайте виключення в Windows Defender** також:
   - Windows Security → Virus & threat protection → Manage settings → Exclusions
   - Додайте `S:\rust\poolAI\target`
3. **Перевірте інші антивіруси** (якщо встановлені)
4. **Вимкніть Avast на час компіляції** (небезпечно, але якщо потрібно)

## 📚 Посилання

- [Rust Issue: Antivirus false positives](https://github.com/rust-lang/rust/issues/70890)
- [Avast Exclusions Guide](https://support.avast.com/en-us/article/Add-Remove-Exceptions-antivirus/)

---

**Статус**: ✅ Рекомендовано додати `target/` до виключень Avast
