# 🔐 Виправлення Аутентифікації та Push
## Дата: 2026-01-22

**Проблема**: Git не запитує credentials, одразу повертає `Authentication failed`

**Причина**: Старі невалідні credentials в Windows Credential Manager

**Статус**: 
- ✅ Коміт створено (`b8df9b3`)
- ⚠️ Є незакомічені зміни в `docs/README.md`
- ⚠️ 7 комітів готові до push

---

## ✅ Крок 1: Додати Зміни до Коміту

В MSYS2 bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Додати зміни в docs/README.md
git add docs/README.md

# Створити коміт
git commit -m "docs: update README with new troubleshooting guides"
```

---

## ✅ Крок 2: Видалити Старі Credentials

### Варіант 1: PowerShell (окреме вікно)

Відкрий PowerShell (не MSYS2) і виконай:

```powershell
# Перевір чи є старі credentials
cmdkey /list | findstr git

# Якщо є, видали:
cmdkey /delete:git:https://github.com
```

### Варіант 2: Через Windows Credential Manager GUI

1. Відкрий **Windows Credential Manager** (через пошук Windows)
2. Знайди `git:https://github.com` або `github.com`
3. Видали цей запис

---

## ✅ Крок 3: Push з Правильними Credentials

### Варіант 1: Push з Prompt для Credentials

В MSYS2 bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Видалити credential helper тимчасово
git config --global --unset credential.helper

# Push - має запитати credentials
git push origin main
```

**Коли запитає**:
- Username: `platinoff`
- Password: **Personal Access Token** (не пароль!)

**Якщо все ще не запитує**, спробуй явно вказати URL:

```bash
# Явно вказати URL з username
git push https://platinoff@github.com/platinoff/poolAI.git main
```

### Варіант 2: Push з PAT в URL (Тимчасово)

⚠️ **УВАГА**: Не зберігай PAT в URL постійно!

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Заміни YOUR_PAT на твій Personal Access Token
git push https://platinoff:YOUR_PAT@github.com/platinoff/poolAI.git main

# Після push, видали remote URL з PAT
git remote set-url origin https://github.com/platinoff/poolAI.git
```

### Варіант 3: SSH (Якщо Налаштовано)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Змінити remote на SSH
git remote set-url origin git@github.com:platinoff/poolAI.git

# Push
git push origin main
```

---

## ✅ Крок 4: Перевірка

Після успішного push:

```bash
git log --oneline -5
git status
```

Має показати: `Your branch is up to date with 'origin/main'`

---

## 🔧 Якщо Все Ще Не Працює

### Перевір Remote URL

```bash
git remote -v
```

Має показати:
```
origin  https://github.com/platinoff/poolAI.git (fetch)
origin  https://github.com/platinoff/poolAI.git (push)
```

### Перевір Git Config

```bash
git config --global --list | grep credential
```

### Створи Новий Personal Access Token

1. GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Generate new token (classic)
3. Встанови права: `repo` (повний доступ до репозиторіїв)
4. Скопіюй token
5. Використай token як password при push

---

## 📝 Після Успішного Push

```bash
# Поверни credential helper
git config --global credential.helper wincred

# Перевір статус
git status
```

---

## 🎯 Наступні Кроки

Після успішного push:
1. Виправи Rust версію (якщо потрібно)
2. Перейти до Ітерації 1: Моніторинг контекстної пам'яті

**Детальний план**: `docs/NEXT_STEPS_AFTER_PUSH_2026-01-22.md`

---

**Детальніше**: 
- `docs/troubleshooting/GIT_AUTH_FIX.md` - повна інструкція з аутентифікації
- `docs/archive/PUSH_WITH_AUTH_FIX.md` - детальний гайд з push
