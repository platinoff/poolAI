# 🔧 Rust Version Issue - AWS SDK Compatibility

## Проблема

При спробі запустити `cargo test --features cloud --test cloud_providers` виникає помилка:

```
error: rustc 1.87.0 is not supported by the following packages:
  aws-config@1.8.12 requires rustc 1.88
  aws-sdk-ec2@1.200.0 requires rustc 1.88.0
  ...
```

## Причина

AWS SDK вимагає Rust 1.88+, але поточна версія Rust в системі - 1.87.0.

## Рішення

### 1. Перевірити поточну версію Rust

```bash
rustc --version
rustup show
```

### 2. Оновити Rust toolchain до 1.92.0

Проект налаштований на Rust 1.92.0 (див. `rust-toolchain.toml`). Оновіть toolchain:

```bash
# В MSYS2 bash
rustup update stable
rustup override set 1.92.0-x86_64-pc-windows-gnu
# або
rustup override set 1.92.0

# Перевірити
rustc --version  # має показати 1.92.0
```

### 3. Якщо використовується GNU toolchain

```bash
rustup toolchain install 1.92.0-x86_64-pc-windows-gnu
rustup override set 1.92.0-x86_64-pc-windows-gnu
```

### 4. Перевірити, що все працює

```bash
cd /s/rust/poolAI
cargo check --features cloud
cargo test --features cloud --test cloud_providers
```

## Альтернативне рішення (якщо не можна оновити Rust)

Якщо неможливо оновити Rust до 1.88+, можна використати старіші версії AWS SDK:

```toml
# В Cargo.toml
aws-config = "1.7"  # підтримує Rust 1.87
aws-sdk-ec2 = "1.180"  # підтримує Rust 1.87
aws-sdk-ecs = "1.100"  # підтримує Rust 1.87
aws-sdk-s3 = "1.110"  # підтримує Rust 1.87
```

**Увага**: Це не рекомендується, оскільки проект налаштований на Rust 1.92.0.

## Перевірка конфігурації

Файл `rust-toolchain.toml` має містити:

```toml
[toolchain]
channel = "1.92.0"
targets = ["x86_64-pc-windows-gnu"]
components = ["rustfmt", "clippy"]
```

Якщо файл правильний, але версія все одно 1.87.0, виконайте:

```bash
rustup override unset
rustup override set 1.92.0-x86_64-pc-windows-gnu
```

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-22
