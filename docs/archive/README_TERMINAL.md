# Налаштування терміналу MSYS2 UCRT64

## Автоматичне використання MSYS2 UCRT64

Файл `.vscode/settings.json` налаштований для автоматичного використання MSYS2 UCRT64 оболонки в терміналі Cursor/VS Code.

## ⚠️ Важливо: Налаштування Rust PATH

Якщо бачите помилку `cargo: command not found` в MSYS2 UCRT64 терміналі:

### Швидке рішення (для поточного сеансу):

```bash
# Додати Rust до PATH (тимчасово)
export PATH="/c/Users/$USER/.cargo/bin:$PATH"

# Перевірити
cargo --version
```

### Постійне рішення:

Додайте до `~/.bashrc` або `~/.bash_profile`:

```bash
# Rust/Cargo PATH для MSYS2 UCRT64
if [ -d "/c/Users/$USER/.cargo/bin" ]; then
    export PATH="/c/Users/$USER/.cargo/bin:$PATH"
fi

# Rust toolchain для MSYS2 UCRT64
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
```

Потім перезавантажте термінал або виконайте:
```bash
source ~/.bashrc
```

## Переключення Rust toolchain на GNU

Для компіляції через MSYS2 потрібно використовувати GNU toolchain:

```bash
rustup default stable-x86_64-pc-windows-gnu
```

Або для конкретного проекту:

```bash
rustup override set stable-x86_64-pc-windows-gnu
```

## Перевірка toolchain

```bash
rustup show
```

## Компіляція проекту

Після переключення на GNU toolchain, компіляція буде використовувати MSYS2:

```bash
cargo build
cargo run
cargo check
```

## Відкриття нового терміналу

Після зміни налаштувань:
1. Закрийте всі відкриті термінали
2. Відкрийте новий термінал (Ctrl+Shift+`)
3. Термінал автоматично запуститься в MSYS2 UCRT64

## Ручне переключення оболонки

Якщо потрібно переключитися вручну:
- Виберіть профіль терміналу зі списку (стрілка вниз біля значка "+")
- Оберіть "MSYS2 UCRT64"

## Детальні інструкції

Дивіться `MSYS2_RUST_SETUP.md` для повної інструкції з налаштування Rust в MSYS2 UCRT64.
