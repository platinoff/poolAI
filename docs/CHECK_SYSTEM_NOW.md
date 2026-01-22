# 🔍 Перевірка Системи
## Дата: 2026-01-22

Команди для перевірки конфігурації системи для git push.

---

## ⚡ Швидка Перевірка

Виконай в MSYS2 bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
bash scripts/check_system.sh
```

---

## 📋 Ручна Перевірка

### 1. SSH Keys

```bash
# Перевірити чи є SSH ключі
ls -la ~/.ssh/*.pub

# Якщо є, показати публічний ключ
cat ~/.ssh/id_ed25519.pub 2>/dev/null || cat ~/.ssh/id_rsa.pub 2>/dev/null
```

**Очікуваний результат**: 
- ✅ Якщо є ключі - покаже файли та публічний ключ
- ❌ Якщо немає - `No such file or directory`

---

### 2. Git Config

```bash
# Перевірити git конфігурацію
git config --global --list | grep -E "(credential|user|remote)"

# Перевірити credential helper
git config --global credential.helper
```

**Очікуваний результат**: 
- Покаже поточну конфігурацію credential helper та user

---

### 3. Git Remote

```bash
# Перевірити remote URL
git remote -v
```

**Очікуваний результат**: 
```
origin  https://github.com/platinoff/poolAI.git (fetch)
origin  https://github.com/platinoff/poolAI.git (push)
```

---

### 4. Credentials File

```bash
# Перевірити чи є credentials file
if [ -f ~/.git-credentials ]; then
    echo "✅ ~/.git-credentials file found:"
    cat ~/.git-credentials | sed 's/:[^@]*@/:***@/g'  # Mask password
else
    echo "❌ No ~/.git-credentials file found"
fi
```

**Очікуваний результат**: 
- ✅ Якщо є - покаже файл з замаскованим паролем
- ❌ Якщо немає - `No ~/.git-credentials file found`

---

### 5. SSH Test

```bash
# Перевірити SSH з'єднання з GitHub
ssh -T git@github.com
```

**Очікуваний результат**: 
- ✅ Якщо SSH налаштовано: `Hi platinoff! You've successfully authenticated...`
- ❌ Якщо не налаштовано: `Permission denied` або `Host key verification failed`

---

### 6. Git Status

```bash
# Перевірити git статус
git status -sb
```

**Очікуваний результат**: 
```
## main...origin/main [ahead 8]
nothing to commit, working tree clean
```

---

### 7. Rust Version

```bash
# Перевірити Rust версію
rustc --version
```

**Очікуваний результат**: 
- ✅ `rustc 1.92.0` (або новіша)
- ⚠️ Якщо `rustc 1.87.0` - потрібно оновити до 1.92.0

---

## 📊 Підсумок

Після виконання перевірки, ти побачиш:

1. **SSH Keys**: Чи є SSH ключі для GitHub
2. **Git Config**: Яка конфігурація credential helper
3. **Git Remote**: Який remote URL використовується
4. **Credentials File**: Чи є збережені credentials
5. **SSH Test**: Чи працює SSH з GitHub
6. **Git Status**: Скільки комітів готові до push
7. **Rust Version**: Яка версія Rust встановлена

---

## 🎯 Наступні Кроки

На основі результатів перевірки:

- **Якщо є SSH ключі та SSH працює**: Використай `docs/PUSH_FINAL_SOLUTION.md` → Варіант 1 (SSH)
- **Якщо немає SSH**: Використай `docs/PUSH_FINAL_SOLUTION.md` → Варіант 2 (Credentials File) або Варіант 3 (PAT в URL)
- **Якщо Rust версія 1.87.0**: Використай `docs/troubleshooting/RUST_VERSION_FIX_2026-01-22.md`

---

**Скрипт**: `scripts/check_system.sh`
