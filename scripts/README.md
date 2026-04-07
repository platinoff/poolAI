# 🔧 PoolAI Scripts

Цей каталог містить всі shell скрипти для проекту PoolAI.

## 📋 Список скриптів

### 🛠️ Setup & Configuration

- **`setup_rust_path.sh`** - Налаштування Rust/Cargo PATH для MSYS2 UCRT64
  - Використання: `bash scripts/setup_rust_path.sh`
  - Опис: Автоматично налаштовує PATH для Rust та Cargo в MSYS2 UCRT64

- **`setup_msys2_path.ps1`** - Налаштування MSYS2 PATH для Windows (PowerShell)
  - Використання: `.\scripts\setup_msys2_path.ps1`
  - Опис: Додає `C:\msys64\usr\bin` до PATH для доступу до `dlltool.exe` та інших MinGW інструментів
  - Призначення: Необхідно для компіляції Rust крейтів з нативними Windows залежностями (windows-sys, chrono)
  - Коли використовувати: Перед `cargo build` або `cargo test` на Windows, якщо виникає помилка "dlltool.exe: program not found"

- **`QUICK_FIX_RUST_PATH.sh`** - Швидке виправлення Rust PATH
  - Використання: `bash scripts/QUICK_FIX_RUST_PATH.sh`
  - Опис: Швидке виправлення PATH для Rust

### 🔧 Fix Scripts

- **`fix_cargo_now.sh`** - Негайне виправлення cargo в MSYS2 UCRT64
  - Використання: `bash scripts/fix_cargo_now.sh`
  - Опис: Виправляє PATH для cargo в MSYS2 UCRT64

- **`fix_gcc.sh`** - Виправлення GCC
  - Використання: `bash scripts/fix_gcc.sh`
  - Опис: Виправляє проблеми з GCC

### 📦 Installation

- **`install_gcc.sh`** - Встановлення GCC
  - Використання: `bash scripts/install_gcc.sh`
  - Опис: Встановлює GCC для MSYS2

### ✅ Verification

- **`check_target_disk.sh`** — вільне місце на томі репозиторію та розмір `target/` (запобігання переповненню диска / падінню тестів)
  - Використання: `bash scripts/check_target_disk.sh` (попередження) або `bash scripts/check_target_disk.sh --enforce` (exit 1 при порушенні порогів)
  - Змінні: `POOLAI_MIN_FREE_DISK_GB` (default 12), `POOLAI_MAX_TARGET_DIR_GB` (default 48), `CARGO_TARGET_DIR`, `POOLAI_ENFORCE_DISK_LIMIT=1`
  - Див. `.cursor/rules/rust-architect.md` — підрозділ **target/ і ліміт дискового простору**

- **`verify_build.sh`** - Перевірка збірки
  - Використання: `bash scripts/verify_build.sh`
  - Опис: Перевіряє чи проект успішно збирається

### 🚀 Git (MSYS2 bash, **без .sh** — copy-paste блок)

- **Git push**: команди в MSYS2 bash, без скриптів. Див. **`.cursor/commands/git-push.md`**, `docs/troubleshooting/GIT_PUSH_FAILED.md`.
- Опційно: `git-push-poolai.sh`, `git-push-only.sh` — якщо зручніше скрипти.

- **`PUSH_COMMANDS.sh`** - Команди для Git push (гілка fix/unsafe-global-...)
  - Використання: `bash scripts/PUSH_COMMANDS.sh`
  - Опис: Допоміжні команди для Git push

## 📝 Правила створення нових скриптів

1. **Створюйте в `scripts/`**:
   ```bash
   # ✅ ПРАВИЛЬНО
   scripts/my_script.sh
   
   # ❌ НЕПРАВИЛЬНО
   my_script.sh (в корені)
   ```

2. **Додавайте shebang**:
   ```bash
   #!/bin/bash
   # Або
   #!/usr/bin/env bash
   ```

3. **Додавайте опис**:
   ```bash
   #!/bin/bash
   # Description: What this script does
   # Usage: bash scripts/script_name.sh
   ```

4. **Оновлюйте цей README**:
   - Додайте опис нового скрипта
   - Додайте приклад використання

5. **Робіть виконуваним**:
   ```bash
   chmod +x scripts/script_name.sh
   ```

## 🔍 Категорії скриптів

- **Setup** - Налаштування середовища
- **Fix** - Виправлення проблем
- **Install** - Встановлення залежностей
- **Verify** - Перевірка стану
- **Git** - Git допоміжні скрипти
- **Build** - Скрипти збірки
- **CI/CD** - Continuous Integration/Deployment

## 📚 Посилання

- [MSYS2 Documentation](https://www.msys2.org/)
- [Rust Documentation](https://www.rust-lang.org/)
- [Bash Scripting Guide](https://www.gnu.org/software/bash/manual/)

---

**Примітка**: Всі скрипти повинні бути в `scripts/` каталозі для чистої структури проекту.

