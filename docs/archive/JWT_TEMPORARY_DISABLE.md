# ⚠️ JWT Тимчасово вимкнено

## Проблема

`jsonwebtoken` crate використовує `ring` для криптографії, який потребує C компілятор (gcc).

---

## ✅ Рішення 1: Встановити GCC (РЕКОМЕНДОВАНО)

**Автоматичне встановлення:**

```bash
cd /s/rust/poolAI
bash install_gcc.sh
source ~/.bashrc
```

**Ручне встановлення:**

```bash
# В MSYS2 UCRT64 терміналі
pacman -Syu
pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain

# Додати до PATH
echo 'export PATH="/c/msys64/ucrt64/bin:$PATH"' >> ~/.bashrc
echo 'export CC="gcc"' >> ~/.bashrc
echo 'export CC_x86_64_pc_windows_gnu="gcc"' >> ~/.bashrc
source ~/.bashrc

# Перевірити
gcc --version
```

**Після встановлення GCC:**

1. Розкоментувати в `Cargo.toml`:
```toml
jsonwebtoken = "9.3"
```

2. Розкоментувати в `src/network/auth.rs`:
```rust
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
```

3. Відновити оригінальні функції `generate_token` та `validate_token`

---

## 🔧 Рішення 2: Тимчасово вимкнено (поточний стан)

**Що зроблено:**
- ✅ Видалено `jsonwebtoken` з `Cargo.toml`
- ✅ Додано просту заглушку з `base64` для розробки
- ✅ Функції `generate_token` та `validate_token` працюють з простими токенами

**⚠️ УВАГА:** Токени НЕ безпечні для продакшну! Це тільки для розробки.

---

## 📊 Поточний статус

- ✅ **Компіляція**: Працює без gcc
- ⚠️ **JWT**: Тимчасово вимкнено (проста заглушка)
- ✅ **Аутентифікація**: Працює з простими токенами (dev only)

---

## 🚀 Після встановлення GCC

```bash
# 1. Встановити GCC
bash install_gcc.sh

# 2. Оновити Cargo.toml та auth.rs (розкоментувати JWT)

# 3. Перевірити компіляцію
cargo check
cargo build
```

---

**Для продакшну обов'язково встановіть GCC та увімкніть справжній JWT!** ⚠️

