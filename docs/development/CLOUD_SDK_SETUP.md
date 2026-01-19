# 🔧 Cloud SDK Setup Guide - Windows

## Проблеми та рішення

### Проблема 1: Duplicate key `chrono` в Cargo.toml

**Помилка**: `error: duplicate key 'chrono' in table 'dependencies'`

**Рішення**: ✅ Виправлено - `chrono` вже є в dependencies (рядок 55), не потрібно додавати в cloud-sdk feature.

---

### Проблема 2: `gcc.exe` not found

**Помилка**: `failed to find tool "gcc.exe": program not found`

**Причина**: Деякі залежності (наприклад, `azure_core`, `azure_identity`) потребують native toolchain для компіляції C кодів.

**Рішення**:

#### Крок 1: Перевірити наявність MSYS2

```powershell
Test-Path "C:\msys64\ucrt64\bin\gcc.exe"
```

Якщо `False`, встановіть MSYS2:
1. Завантажте з https://www.msys2.org/
2. Встановіть в `C:\msys64`
3. Запустіть MSYS2 UCRT64 terminal
4. Встановіть toolchain:
   ```bash
   pacman -Syu
   pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain
   ```

#### Крок 2: Додати MSYS2 до PATH

**Для поточної сесії PowerShell:**
```powershell
.\scripts\setup_msys2_path.ps1
```

**Або вручну:**
```powershell
$env:PATH = "C:\msys64\ucrt64\bin;C:\msys64\usr\bin;$env:PATH"
```

**Для постійного налаштування:**
1. Відкрийте "Системні змінні середовища"
2. Додайте до PATH: `C:\msys64\ucrt64\bin` та `C:\msys64\usr\bin`
3. Перезапустіть термінал

#### Крок 3: Перевірити встановлення

```powershell
gcc --version
dlltool --version
```

---

## Залежності для Cloud SDK

### Обов'язкові (для cloud-sdk feature):
- ✅ `aws-sign-v4` - AWS Signature Version 4 signing (Rust 1.70+ compatible)
- ✅ `http` - HTTP types для request building
- ✅ `azure_core`, `azure_identity`, `azure_mgmt_compute` - Azure SDK
- ✅ `k8s-openapi` - Kubernetes API client

### Вже встановлені (використовуються cloud-sdk):
- ✅ `chrono` - Timestamps (вже в dependencies)
- ✅ `reqwest` - HTTP client (вже в dependencies)
- ✅ `serde_json` - JSON serialization (вже в dependencies)

### Не потрібні (закоментовано):
- ❌ `aws-sigv4` - потребує Rust 1.88+
- ❌ `aws-credential-types` - потребує Rust 1.88+
- ❌ AWS SDK crates - потребують Rust 1.88+

---

## Компіляція з Cloud SDK

### Без cloud-sdk feature (без native toolchain):
```bash
cargo check
cargo build
```

### З cloud-sdk feature (потребує MSYS2/gcc):
```bash
# Спочатку налаштуйте PATH
.\scripts\setup_msys2_path.ps1

# Потім компілюйте
cargo check --features cloud,cloud-sdk
cargo build --features cloud,cloud-sdk
```

---

## Troubleshooting

### Якщо все ще помилка з gcc:

1. **Перевірте PATH**:
   ```powershell
   $env:PATH -split ';' | Select-String "msys64"
   ```

2. **Перевірте встановлення MSYS2**:
   ```powershell
   Test-Path "C:\msys64\ucrt64\bin\gcc.exe"
   Test-Path "C:\msys64\usr\bin\dlltool.exe"
   ```

3. **Встановіть binutils якщо потрібно**:
   ```bash
   # В MSYS2 UCRT64 terminal:
   pacman -S mingw-w64-ucrt-x86_64-binutils
   ```

4. **Альтернатива - використати MSVC toolchain**:
   ```bash
   rustup override set stable-x86_64-pc-windows-msvc
   ```
   **Примітка**: MSVC не потребує gcc, але потребує Visual Studio Build Tools.

---

## Статус реалізації

### AWS SDK Implementation
- ✅ `aws-sign-v4` додано до Cargo.toml
- ✅ AWS SigV4 signing реалізовано для EC2
- ✅ AWS SigV4 signing реалізовано для ECS
- ⚠️ Потребує MSYS2/gcc для компіляції

### Azure SDK Implementation
- ✅ REST API підхід реалізовано
- ⚠️ Потребує MSYS2/gcc для компіляції azure_core

### GCP SDK Implementation
- ✅ REST API підхід реалізовано
- ✅ Не потребує native toolchain

---

**Останнє оновлення**: 2026-01-19
