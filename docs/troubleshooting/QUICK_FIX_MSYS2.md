# 🔧 Швидке виправлення dlltool в MSYS2 терміналі

## Виконайте ці команди в MSYS2 терміналі:

```bash
# 1. Додати MSYS2 до PATH
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"

# 2. Встановити CC та AR змінні
export CC="gcc"
export CC_x86_64_pc_windows_gnu="gcc"
export AR="ar"
export AR_x86_64_pc_windows_gnu="ar"

# 3. Перевірити (використайте command -v замість which)
command -v gcc
command -v dlltool
command -v ar

# 4. Спробувати компіляцію
cargo build
```

## Якщо все ще не працює:

```bash
# Перевірити, чи існують файли
ls /c/msys64/ucrt64/bin/gcc.exe
ls /c/msys64/usr/bin/dlltool.exe

# Якщо файли не знайдені - встановити MSYS2 toolchain:
# В MSYS2 UCRT64 terminal:
pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain
```
