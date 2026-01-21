# NativeCommandError Analysis & Comparison with Settings
## Оновлено: 2026-01-19

**Помилка**: `NativeCommandError` з `RemoteException` під час компіляції `proc-macro2` в PowerShell

---

## 🔍 Web Research Results

### Що таке NativeCommandError?

**Визначення:**
- `NativeCommandError` - це PowerShell обгортка для помилок виконання зовнішніх (native) команд
- `RemoteException` - тип винятку, який виникає коли команда не може виконатися
- Це не помилка Rust, а PowerShell обгортка навколо реальної помилки

**Типові причини:**
1. MSVC Build Tools не встановлені або не налаштовані
2. Неправильний або неініціалізований environment (PATH, LIB, INCLUDE)
3. Проблеми з permissions / Execution Policy
4. Конфлікт між MSVC та GNU toolchains
5. Проблеми з PATH (дублікати, неправильний порядок)

---

## 📊 Порівняння з Вашими Налаштуваннями

### ✅ Що Працює Правильно:

1. **Rust Toolchains Встановлені:**
   - ✅ `stable-x86_64-pc-windows-msvc` (1.92.0) - active
   - ✅ `stable-x86_64-pc-windows-gnu` (1.92.0)
   - ✅ Коли PATH налаштовано правильно: `cargo 1.92.0`, `rustc 1.92.0`

2. **MSYS2 Встановлено:**
   - ✅ UCRT64: `C:\msys64\ucrt64\bin\gcc.exe` (GCC 15.1.0)
   - ✅ MINGW64: `C:\msys64\mingw64\bin\gcc.exe`
   - ✅ Tools: `dlltool.exe` доступний

3. **Microsoft Visual Studio:**
   - ✅ Встановлено в `C:\Program Files\Microsoft Visual Studio`

---

### ⚠️ Виявлені Проблеми:

#### 1. **Toolchain Override Конфлікт** 🔴

**Проблема:**
```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
targets = ["x86_64-pc-windows-gnu"]  # ← Вказує на GNU
default = true
```

**Але:**
```bash
rustup show
# active toolchain: stable-x86_64-pc-windows-msvc  # ← Override на MSVC
```

**Вплив:**
- Конфлікт між GNU та MSVC toolchains
- Cargo може шукати неправильні компоненти
- `proc-macro2` може компілюватися з неправильним linker

**Рішення:**
- Узгодити `rust-toolchain.toml` з активним toolchain
- Або видалити override і використовувати GNU

---

#### 2. **PATH Дублювання** 🟡

**Проблема:**
- PATH містить дублікати MSYS2 шляхів (з 78 до 40 записів після очищення)
- MSYS2 шляхи можуть конфліктувати з MSVC

**Вплив:**
- Cargo може знайти неправильний `gcc.exe` або `link.exe`
- Конфлікт між MSYS2 GCC та MSVC linker

**Рішення:**
- Очистити PATH від дублікатів
- Переконатися, що MSVC tools в PATH перед MSYS2 (для MSVC toolchain)

---

#### 3. **MSVC Environment Variables** 🟡

**Проблема:**
- MSVC tools потребують правильних environment variables:
  - `PATH` - має містити Visual Studio Build Tools
  - `LIB` - має вказувати на Windows SDK libraries
  - `INCLUDE` - має вказувати на Windows SDK headers

**Вплив:**
- `link.exe` може не знайти необхідні `.lib` файли
- `proc-macro2` build script може не знайти MSVC compiler

**Рішення:**
- Використовувати Developer PowerShell для Visual Studio
- Або налаштувати environment variables вручну

---

#### 4. **proc-macro2 Compilation Context** 🟡

**Проблема:**
- `proc-macro2` потребує правильного Rust toolchain
- Build script може викликати MSVC compiler
- Якщо toolchain невідповідний, компіляція падає

**Вплив:**
- `NativeCommandError` може виникати коли:
  - Build script намагається викликати `cl.exe` (MSVC compiler)
  - Але PATH не містить Visual Studio Build Tools
  - Або використовується GNU toolchain замість MSVC

---

## 🎯 Рекомендації на Основі Research

### 1. Узгодити Toolchain Configuration

**Варіант A: Використовувати MSVC (рекомендовано для Windows)**
```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
targets = ["x86_64-pc-windows-msvc"]  # ← Змінити на MSVC
default = true
```

**Варіант B: Використовувати GNU (якщо потрібен gcc.exe)**
```toml
# rust-toolchain.toml (залишити як є)
[toolchain]
channel = "stable"
targets = ["x86_64-pc-windows-gnu"]
default = true
```

**Але потрібно:**
- Видалити override: `rustup override unset`
- Переконатися, що MSYS2 PATH налаштовано правильно

---

### 2. Налаштувати PATH Правильно

**Для MSVC Toolchain:**
```powershell
# Додати Visual Studio Build Tools до PATH
$vsPath = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\*\bin\Hostx64\x64"
$env:PATH = "$vsPath;$env:PATH"

# Додати Windows SDK
$sdkPath = "C:\Program Files (x86)\Windows Kits\10\bin\*\x64"
$env:PATH = "$sdkPath;$env:PATH"

# Cargo має бути перед MSYS2
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
```

**Для GNU Toolchain:**
```powershell
# MSYS2 має бути перед іншими
$env:PATH = "C:\msys64\ucrt64\bin;C:\msys64\usr\bin;$env:USERPROFILE\.cargo\bin;$env:PATH"
```

---

### 3. Використовувати Developer PowerShell

**Рекомендація:**
- Відкрити "Developer PowerShell for VS" замість звичайного PowerShell
- Або налаштувати VS Code/Cursor terminal на використання Developer PowerShell

**Налаштування `.vscode/settings.json`:**
```json
{
  "terminal.integrated.profiles.windows": {
    "Developer PowerShell": {
      "path": "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\Common7\\Tools\\LaunchDevCmd.bat",
      "args": ["-NoExit", "-Command", "& {Import-Module 'C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\Common7\\Tools\\Microsoft.VisualStudio.DevShell.dll'; Enter-VsDevShell -VsInstallPath 'C:\\Program Files\\Microsoft Visual Studio\\2022\\Community' -SkipAutomaticLocation}"
    }
  }
}
```

---

### 4. Перевірити Permissions

**Можливі проблеми:**
- Antivirus блокує виконання `.exe` файлів
- Execution Policy блокує скрипти
- Permissions на `target/` директорії

**Рішення:**
```powershell
# Перевірити Execution Policy
Get-ExecutionPolicy

# Якщо потрібно, змінити (для поточного користувача)
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser

# Перевірити permissions на target/
icacls "target" /grant "$env:USERNAME:(OI)(CI)F"
```

---

### 5. Clean Build

**Якщо проблема персистує:**
```powershell
# Очистити build artifacts
cargo clean

# Перевірити toolchain
rustup show

# Спробувати з verbose output
cargo build --verbose 2>&1 | Tee-Object build_log.txt
```

---

## 📋 Checklist для Вирішення

- [ ] Узгодити `rust-toolchain.toml` з активним toolchain
- [ ] Очистити PATH від дублікатів
- [ ] Налаштувати правильний порядок PATH (Cargo → MSVC/MSYS2)
- [ ] Використовувати Developer PowerShell або налаштувати environment variables
- [ ] Перевірити permissions на `target/` директорії
- [ ] Виконати `cargo clean` та спробувати знову
- [ ] Перевірити з `cargo build --verbose` для деталей помилки

---

## 🔬 Діагностика

**Команди для діагностики:**

```powershell
# 1. Перевірити toolchain
rustup show

# 2. Перевірити PATH
$env:PATH -split ';' | Where-Object { $_ -like '*cargo*' -or $_ -like '*msys*' -or $_ -like '*Visual Studio*' }

# 3. Перевірити MSVC tools
where.exe link.exe
where.exe cl.exe

# 4. Перевірити Rust components
rustup component list --installed

# 5. Спробувати мінімальний build
cargo new test_project
cd test_project
cargo build
```

---

## ✅ Очікуваний Результат

Після виправлення:
- ✅ `cargo build` працює без `NativeCommandError`
- ✅ `proc-macro2` компілюється успішно
- ✅ Правильний toolchain використовується
- ✅ PATH налаштовано правильно
- ✅ Environment variables встановлені

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-19  
**Джерела**: PowerShell documentation, Rust toolchain issues, MSVC setup guides
