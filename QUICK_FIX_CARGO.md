# Quick Fix: Cargo не працює в MSYS2 UCRT64

## 🔧 Швидке виправлення

### Варіант 1: Автоматичний скрипт (Рекомендовано)

В MSYS2 UCRT64 терміналі виконайте:

```bash
cd /s/rust/poolAI
bash setup_rust_path.sh
source ~/.bashrc
cargo --version
```

### Варіант 2: Ручне додавання до PATH

В MSYS2 UCRT64 терміналі:

```bash
# Додати Rust до PATH для поточної сесії
export PATH="/c/Users/$USER/.cargo/bin:$PATH"

# Перевірити
cargo --version
rustc --version
```

### Варіант 3: Постійне налаштування

Додайте до `~/.bashrc`:

```bash
# Відкрити файл
nano ~/.bashrc

# Додати ці рядки:
export PATH="/c/Users/$USER/.cargo/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"

# Зберегти та перезавантажити
source ~/.bashrc
```

---

## ✅ Перевірка

Після налаштування перевірте:

```bash
# 1. Перевірка cargo
cargo --version
# Очікуваний результат: cargo 1.xx.x (stable-x86_64-pc-windows-gnu)

# 2. Перевірка rustc
rustc --version
# Очікуваний результат: rustc 1.xx.x (stable-x86_64-pc-windows-gnu)

# 3. Перевірка toolchain
rustup show
# Має показувати: stable-x86_64-pc-windows-gnu (active, default)

# 4. Тест компіляції
cd /s/rust/poolAI
cargo check
```

---

## 🐛 Якщо все ще не працює

### Перевірка встановлення Rust

```bash
# Перевірити чи існує Rust
ls /c/Users/$USER/.cargo/bin/cargo.exe
ls ~/.cargo/bin/cargo 2>/dev/null

# Якщо не знайдено, встановіть Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Перевірка PATH

```bash
# Показати поточний PATH
echo $PATH | tr ':' '\n' | grep -i cargo
echo $PATH | tr ':' '\n' | grep -i rust
```

### Встановлення GNU toolchain

```bash
# Якщо rustup доступний
rustup default stable-x86_64-pc-windows-gnu
rustup override set stable-x86_64-pc-windows-gnu
```

---

## 📝 Оновлення налаштувань VS Code/Cursor

Файл `.vscode/settings.json` вже оновлено для автоматичного додавання Rust до PATH.

Після оновлення:
1. Перезавантажте Cursor/VS Code
2. Відкрийте новий термінал (Ctrl+Shift+`)
3. Rust має бути доступний автоматично

---

**Після виконання цих кроків cargo має працювати в MSYS2 UCRT64!** ✅

