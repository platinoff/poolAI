# ⚡ Push Зараз: SSH або PAT
## Дата: 2026-01-22

**Проблема**: `credential-wincred` не працює в MSYS2 bash

**Рішення**: Використати SSH або PAT безпосередньо в URL

**Статус**: 
- ✅ Коміт створено (`58a755f`)
- ✅ 8 комітів готові до push

---

## 🚀 Швидкий Push: SSH (Рекомендовано)

### Якщо SSH вже налаштовано:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Змінити remote на SSH
git remote set-url origin git@github.com:platinoff/poolAI.git

# Push
git push origin main
```

### Якщо SSH не налаштовано:

**Крок 1**: Створити SSH ключ (якщо немає):
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# Натисни Enter для всіх питань
```

**Крок 2**: Показати публічний ключ:
```bash
cat ~/.ssh/id_ed25519.pub
```

**Крок 3**: Додати ключ до GitHub:
1. GitHub → Settings → SSH and GPG keys → New SSH key
2. Встав публічний ключ
3. Натисни "Add SSH key"

**Крок 4**: Перевірити SSH:
```bash
ssh -T git@github.com
```

**Крок 5**: Push:
```bash
git remote set-url origin git@github.com:platinoff/poolAI.git
git push origin main
```

---

## 🚀 Швидкий Push: PAT в URL (Тимчасово)

⚠️ **УВАГА**: Не зберігай PAT в URL постійно!

### Крок 1: Створити Personal Access Token

1. GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Generate new token (classic)
3. Назва: `poolAI-push`
4. Scopes: `repo` (повний доступ)
5. Generate token
6. **Скопіюй токен одразу!**

### Крок 2: Push з PAT

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Заміни YOUR_PAT на твій Personal Access Token
git push https://platinoff:YOUR_PAT@github.com/platinoff/poolAI.git main
```

### Крок 3: Після push, видалити PAT з URL

```bash
# Повернути звичайний URL
git remote set-url origin https://github.com/platinoff/poolAI.git
```

---

## 🔧 Альтернатива: Git Credential Store

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Використати credential store (зберігає в ~/.git-credentials)
git config --global credential.helper store

# Push (має запитати credentials)
git push origin main
```

Коли запитає:
- Username: `platinoff`
- Password: **Personal Access Token** (не пароль!)

---

## ✅ Перевірка Після Push

```bash
git log --oneline -5
git status
```

Має показати: `Your branch is up to date with 'origin/main'`

---

## 📝 Рекомендації

1. **SSH** - найбезпечніший варіант для постійного використання
2. **PAT в URL** - швидкий варіант для одноразового push (не забудь видалити після)
3. **Credential Store** - зручний для MSYS2

---

**Детальніше**: 
- `docs/archive/PUSH_SSH_OR_PAT.md` - детальний гайд
- `docs/troubleshooting/GIT_AUTH_FIX.md` - повна інструкція
