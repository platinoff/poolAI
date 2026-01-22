# 🔧 Виправлення: gcc.exe та dlltool.exe не знайдено

## Проблема

При компіляції на Windows з GNU toolchain виникають помилки:
```
error: failed to find tool "gcc.exe": program not found
error: error calling dlltool 'dlltool.exe': program not found
```

## Причина

MSYS2/MinGW tools (`gcc.exe`, `dlltool.exe`) не додані до PATH.

## Рішення

### Швидке рішення (PowerShell)

```powershell
# Додати MSYS2 до PATH для поточної сесії
.\scripts\setup_msys2_path.ps1

# Після цього запустити cargo build знову
cargo build
```

### Ручне додавання до PATH (PowerShell)

```powershell
# Додати MSYS2 UCRT64 до PATH
$env:PATH += ";C:\msys64\ucrt64\bin;C:\msys64\usr\bin"

# Перевірити, що gcc доступний
gcc --version
dlltool --version

# Тепер можна компілювати
cargo build
```

### Ручне додавання до PATH (CMD)

```cmd
set PATH=%PATH%;C:\msys64\ucrt64\bin;C:\msys64\usr\bin

gcc --version
dlltool --version

cargo build
```

### Постійне додавання до системного PATH

1. Відкрити **System Properties** → **Environment Variables**
2. Знайти `Path` в **System variables**
3. Додати:
   - `C:\msys64\ucrt64\bin`
   - `C:\msys64\usr\bin`
4. Перезапустити термінал

### Альтернатива: Встановити CC environment variable

```powershell
# Встановити CC для поточної сесії
$env:CC = "C:\msys64\ucrt64\bin\gcc.exe"
$env:AR = "C:\msys64\ucrt64\bin\ar.exe"

cargo build
```

### Перевірка MSYS2 встановлення

```powershell
# Перевірити, чи MSYS2 встановлено
Test-Path "C:\msys64\ucrt64\bin\gcc.exe"

# Якщо false - потрібно встановити MSYS2
# Завантажити з: https://www.msys2.org/
```

## Детальні кроки

### 1. Перевірити наявність MSYS2

```powershell
# Перевірити gcc
Get-Command gcc -ErrorAction SilentlyContinue

# Перевірити dlltool
Get-Command dlltool -ErrorAction SilentlyContinue
```

### 2. Якщо MSYS2 не встановлено

1. Завантажити MSYS2: https://www.msys2.org/
2. Встановити в `C:\msys64\`
3. Запустити MSYS2 UCRT64 terminal
4. Оновити пакети:
   ```bash
   pacman -Syu
   ```
5. Встановити toolchain:
   ```bash
   pacman -S mingw-w64-ucrt-x86_64-toolchain
   ```

### 3. Додати до PATH

**PowerShell (поточна сесія):**
```powershell
$env:PATH = "C:\msys64\ucrt64\bin;C:\msys64\usr\bin;" + $env:PATH
```

**PowerShell (постійно):**
```powershell
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "Machine") + ";C:\msys64\ucrt64\bin;C:\msys64\usr\bin",
    "Machine"
)
```

**CMD (поточна сесія):**
```cmd
set PATH=C:\msys64\ucrt64\bin;C:\msys64\usr\bin;%PATH%
```

### 4. Перевірити встановлення

```powershell
# Перевірити версії
gcc --version
dlltool --version
ar --version

# Перевірити PATH
$env:PATH -split ';' | Select-String "msys64"
```

### 5. Спробувати компіляцію знову

```powershell
# Очистити попередні build artifacts
cargo clean

# Спробувати компіляцію
cargo build
```

## Альтернативні рішення

### Використання bundled SQLite

Якщо не хочете встановлювати MSYS2, можна використати bundled SQLite:

```toml
# Cargo.toml
[dependencies]
rusqlite = { version = "0.32", features = ["bundled"] }
```

### Використання MSVC toolchain

Замість GNU toolchain можна використати MSVC:

```bash
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
```

**Примітка:** MSVC toolchain не потребує MSYS2, але потребує Visual Studio Build Tools.

## Troubleshooting

### Проблема: PATH додано, але все ще не працює

**Рішення:**
1. Перезапустити термінал
2. Перевірити, що PATH містить правильні шляхи:
   ```powershell
   $env:PATH -split ';' | Select-String "msys64"
   ```
3. Перевірити, що файли існують:
   ```powershell
   Test-Path "C:\msys64\ucrt64\bin\gcc.exe"
   ```

### Проблема: gcc знайдено, але dlltool ні

**Рішення:**
```powershell
# dlltool знаходиться в usr\bin
$env:PATH += ";C:\msys64\usr\bin"
```

### Проблема: Різні версії MSYS2

**Рішення:**
Використовуйте UCRT64 версію (рекомендовано для Rust):
```powershell
$env:PATH = "C:\msys64\ucrt64\bin;C:\msys64\usr\bin;" + $env:PATH
```

## Перевірка після виправлення

```powershell
# Перевірити всі необхідні tools
gcc --version
dlltool --version
ar --version

# Спробувати компіляцію
cargo build --no-default-features
```

## Додаткові ресурси

- [MSYS2 Installation Guide](https://www.msys2.org/)
- [Rust Windows Setup](https://forge.rust-lang.org/infra/channel-layout.html)
- [MSYS2_CARGO_SETUP.md](../development/MSYS2_CARGO_SETUP.md)

---

**Останнє оновлення**: 2026-01-21
