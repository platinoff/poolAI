# ⚡ НЕГАЙНЕ ВИПРАВЛЕННЯ: Cargo не працює в MSYS2

## 🔧 Виконайте в MSYS2 UCRT64 терміналі:

### Крок 1: Додати Rust до PATH (для поточної сесії)
```bash
export PATH="/c/Users/$USER/.cargo/bin:$PATH"
```

### Крок 2: Перевірити
```bash
cargo --version
```

### Крок 3: Якщо працює - зробити постійним
```bash
# Додати до ~/.bashrc
echo 'export PATH="/c/Users/$USER/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Крок 4: Перевірити toolchain
```bash
rustup show
# Має показувати: stable-x86_64-pc-windows-gnu
# Якщо показує MSVC - виправити:
rustup default stable-x86_64-pc-windows-gnu
```

### Крок 5: Тест компіляції
```bash
cd /s/rust/poolAI
cargo check
```

---

## ✅ Альтернатива: Використати готовий скрипт

```bash
cd /s/rust/poolAI
bash fix_cargo_now.sh
source ~/.bashrc
cargo --version
```

---

**Після виконання цих команд cargo має працювати!** ✅

