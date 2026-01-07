# 🔧 Виправлення: cargo не працює в MSYS2 UCRT64

## ⚡ Швидке виправлення (30 секунд)

**В MSYS2 UCRT64 терміналі виконайте:**

```bash
# 1. Додати Rust до PATH
export PATH="/c/Users/$USER/.cargo/bin:$PATH"

# 2. Перевірити
cargo --version

# 3. Якщо працює - зробити постійним
echo 'export PATH="/c/Users/$USER/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

## 📋 Детальне виправлення

### Варіант 1: Автоматичний скрипт

```bash
cd /s/rust/poolAI
bash setup_rust_path.sh
source ~/.bashrc
```

### Варіант 2: Миттєве виправлення (тільки для поточної сесії)

```bash
cd /s/rust/poolAI
bash fix_cargo_now.sh
```

### Варіант 3: Ручне налаштування

1. Відкрити `~/.bashrc`:
```bash
nano ~/.bashrc
```

2. Додати в кінець файлу:
```bash
# Rust/Cargo PATH for MSYS2 UCRT64
export PATH="/c/Users/$USER/.cargo/bin:$PATH"
if [ -d "$HOME/.cargo/bin" ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
fi
```

3. Зберегти (Ctrl+O, Enter, Ctrl+X) та перезавантажити:
```bash
source ~/.bashrc
```

---

## ✅ Перевірка

Після виправлення перевірте:

```bash
# 1. Cargo версія
cargo --version
# Очікуваний результат: cargo 1.87.0 (99624be96 2025-05-06)

# 2. Rustc версія
rustc --version
# Очікуваний результат: rustc 1.87.0 (17067e9ac 2025-05-09)

# 3. Toolchain
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
```

### Перевірка PATH

```bash
# Показати поточний PATH
echo $PATH | tr ':' '\n' | grep -i cargo
```

### Встановлення Rust (якщо не встановлений)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
export PATH="$HOME/.cargo/bin:$PATH"
```

### Встановлення GNU toolchain

```bash
rustup default stable-x86_64-pc-windows-gnu
rustup override set stable-x86_64-pc-windows-gnu
```

---

## 💡 Важливо

- Після додавання до `~/.bashrc` потрібно виконати `source ~/.bashrc` або перезапустити термінал
- VS Code/Cursor автоматично додає Rust PATH через `.vscode/settings.json`
- Для нових терміналів PATH буде застосовано автоматично

---

**Після виконання цих кроків cargo має працювати!** ✅

