# Виправлення помилки: dlltool.exe not found

## Проблема

При компіляції з GNU toolchain (`x86_64-pc-windows-gnu`) виникає помилка:
```
error: Error calling dlltool 'dlltool.exe': program not found
```

## Причина

Rust GNU toolchain потребує `dlltool.exe` з MSYS2/MinGW, але він не знайдений в PATH.

## Рішення

### Варіант 1: Додати MSYS2 до PATH (Рекомендовано)

**Для поточної сесії PowerShell:**
```powershell
$env:PATH = "C:\msys64\ucrt64\bin;$env:PATH"
```

**Для постійного налаштування (Windows):**
1. Відкрийте "Системні змінні середовища"
2. Додайте до PATH: `C:\msys64\ucrt64\bin`
3. Перезапустіть термінал

**Для MSYS2 UCRT64 shell:**
MSYS2 автоматично додає свої бінарники до PATH, тому якщо ви працюєте в MSYS2 терміналі, все має працювати.

### Варіант 2: Використати MSVC toolchain (Альтернатива)

Якщо не хочете використовувати MSYS2:

```bash
rustup override set stable-x86_64-pc-windows-msvc
```

**Примітка:** MSVC toolchain не потребує dlltool, але може мати інші залежності (Visual Studio Build Tools).

### Варіант 3: Перевірка наявності MSYS2

Перевірте, чи встановлений MSYS2:
```powershell
Test-Path "C:\msys64\ucrt64\bin\dlltool.exe"
```

Якщо `False`, встановіть MSYS2:
1. Завантажте з https://www.msys2.org/
2. Встановіть в `C:\msys64`
3. Запустіть MSYS2 UCRT64 terminal
4. Оновіть пакети: `pacman -Syu`

## Перевірка

Після налаштування перевірте:

```bash
# Перевірка dlltool
where.exe dlltool
# Має показати: C:\msys64\ucrt64\bin\dlltool.exe

# Перевірка компіляції
cargo check --features enterprise
# Має компілюватися без помилок
```

## Поточне налаштування проекту

Проект використовує `rust-toolchain.toml` для автоматичного вибору GNU toolchain:
- Toolchain: `stable-x86_64-pc-windows-gnu`
- Потребує: MSYS2 UCRT64 з dlltool в PATH

## Додаткова інформація

- MSYS2 UCRT64: https://www.msys2.org/
- Rust toolchains: https://rust-lang.github.io/rustup/concepts/toolchains.html
