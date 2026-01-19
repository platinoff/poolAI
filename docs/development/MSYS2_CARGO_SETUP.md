# Налаштування Cargo в MSYS2

## Швидке налаштування

Якщо ви використовуєте MSYS2 bash (`C:\msys64\usr\bin\bash.exe -l`), виконайте:

```bash
# 1. Додати cargo до PATH для поточної сесії
export PATH="/c/Users/$USER/.cargo/bin:$PATH"

# 2. Перевірити
cargo --version

# 3. Зробити постійним (додати до ~/.bashrc)
echo 'export PATH="/c/Users/$USER/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Автоматичне налаштування

Використайте скрипт з проекту:

```bash
cd /s/rust/poolAI
bash scripts/setup_rust_path.sh
source ~/.bashrc
```

## Перевірка

Після налаштування перевірте:

```bash
cargo --version
rustc --version
cd /s/rust/poolAI
cargo check --features cloud,cloud-sdk
```

## Примітки

- В MSYS2 використовується Unix-формат шляхів: `/c/Users/...` замість `C:\Users\...`
- Профіль "bash (MSYS2)" в `.vscode/settings.json` налаштований правильно
- Для компіляції з `cloud-sdk` потрібен MSYS2 (gcc.exe)
