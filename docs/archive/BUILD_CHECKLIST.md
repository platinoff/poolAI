# ✅ Чеклист перевірки збірки

## 🔍 Перед commit - перевірка збірки

### Крок 1: Перевірка Rust інструментів

```bash
# В MSYS2 UCRT64 терміналі
export PATH="/c/Users/$USER/.cargo/bin:$PATH"

# Перевірити версії
cargo --version
rustc --version
rustup show
```

**Очікуваний результат:**
- ✅ Cargo: cargo 1.87.0 (99624be96 2025-05-06)
- ✅ Rustc: rustc 1.87.0 (17067e9ac 2025-05-09)
- ✅ Toolchain: stable-x86_64-pc-windows-gnu (active, default)

---

### Крок 2: Перевірка форматування коду

```bash
cd /s/rust/poolAI
cargo fmt -- --check
```

**Якщо є помилки:**
```bash
cargo fmt
```

---

### Крок 3: Перевірка лінтером (Clippy)

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Якщо є попередження:**
- Перевірити та виправити попередження
- Або додати `#[allow(clippy::warning_name)]` якщо навмисне

---

### Крок 4: Перевірка компіляції (без збірки)

```bash
cargo check --all-features
```

**Очікуваний результат:**
- ✅ `Finished dev [unoptimized + debuginfo] target(s) in X.XXs`

---

### Крок 5: Повна збірка

```bash
cargo build --all-features
```

**Очікуваний результат:**
- ✅ `Finished dev [unoptimized + debuginfo] target(s) in X.XXs`
- ✅ Файл `target/debug/poolai.exe` створено

---

### Крок 6: Перевірка попереджень

```bash
cargo build --all-features 2>&1 | grep -i warning
```

**Очікуваний результат:**
- ✅ Немає попереджень (або мінімальні, не критичні)

---

## 🚀 Швидка перевірка (автоматичний скрипт)

```bash
cd /s/rust/poolAI
bash verify_build.sh
```

**Або з очищенням:**
```bash
bash verify_build.sh --clean
```

---

## ✅ Чеклист перед commit

- [ ] Rust інструменти працюють (`cargo --version`)
- [ ] GNU toolchain встановлено (`rustup show`)
- [ ] Код відформатовано (`cargo fmt --check`)
- [ ] Clippy перевірка пройдена (`cargo clippy`)
- [ ] Компіляція успішна (`cargo check`)
- [ ] Збірка успішна (`cargo build`)
- [ ] Немає критичних попереджень
- [ ] Тести проходять (якщо є: `cargo test`)

---

## 🐛 Якщо щось не працює

### Помилка: `cargo: command not found`
```bash
export PATH="/c/Users/$USER/.cargo/bin:$PATH"
source ~/.bashrc
```

### Помилка: `linker not found`
```bash
rustup default stable-x86_64-pc-windows-gnu
rustup override set stable-x86_64-pc-windows-gnu
```

### Помилка компіляції
1. Перевірити помилки в виводі
2. Перевірити `Cargo.toml` на наявність залежностей
3. Спробувати `cargo clean && cargo build`

### Помилка: `ring` crate не компілюється
- Це нормально, HTTPS тимчасово вимкнено
- Перевірити що в `Cargo.toml` немає `rustls` або `ring`

---

## 📊 Після успішної перевірки

Якщо всі перевірки пройшли успішно:

```bash
# Перевірити статус git
git status

# Переглянути зміни
git diff --cached --stat

# Створити commit
git commit -F COMMIT_MESSAGE.md

# Push бранча
git push -u origin fix/unsafe-global-state
```

---

**Успішної збірки!** 🚀

