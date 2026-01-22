# 🔐 Push через SSH або PAT
## Дата: 2026-01-22

**Проблема**: `credential-wincred` не працює в MSYS2 bash, Git не запитує credentials

**Рішення**: Використати SSH або PAT безпосередньо в URL

**Статус**: 
- ✅ Коміт створено (`58a755f`)
- ✅ 8 комітів готові до push
- ⚠️ Потрібна аутентифікація

---

## ✅ Варіант 1: SSH (Рекомендовано)

### Крок 1: Перевірити чи є SSH ключ

В MSYS2 bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Перевірити чи є SSH ключ
ls -la ~/.ssh/id_ed25519.pub
# або
ls -la ~/.ssh/id_rsa.pub
```

### Крок 2: Якщо немає SSH ключа, створити

```bash
# Створити новий SSH ключ
ssh-keygen -t ed25519 -C "your_email@example.com"

# Натисни Enter для всіх питань (або вкажи пароль)
```

### Крок 3: Додати SSH ключ до GitHub

```bash
# Показати публічний ключ
cat ~/.ssh/id_ed25519.pub
```

Потім:
1. GitHub → Settings → SSH and GPG keys → New SSH key
2. Встав публічний ключ
3. Натисни "Add SSH key"

### Крок 4: Перевірити SSH з'єднання

```bash
ssh -T git@github.com
```

Має показати: `Hi platinoff! You've successfully authenticated...`

### Крок 5: Змінити remote на SSH та Push

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Змінити remote на SSH
git remote set-url origin git@github.com:platinoff/poolAI.git

# Push
git push origin main
```

---

## ✅ Варіант 2: PAT в URL (Тимчасово)

⚠️ **УВАГА**: Не зберігай PAT в URL постійно!

### Крок 1: Створити Personal Access Token

1. GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Generate new token (classic)
3. Назва: `poolAI-push`
4. Expiration: вибери термін (наприклад, 90 днів)
5. Scopes: обирай `repo` (повний доступ до репозиторіїв)
6. Натисни "Generate token"
7. **ВАЖЛИВО**: Скопіюй токен одразу (він більше не буде показаний)

### Крок 2: Push з PAT в URL

В MSYS2 bash:

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

## ✅ Варіант 3: Git Credential Store (MSYS2)

### Крок 1: Налаштувати credential store

В MSYS2 bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Використати credential store (зберігає в ~/.git-credentials)
git config --global credential.helper store

# Або тимчасово (не зберігає)
git config --global credential.helper cache
```

### Крок 2: Push (має запитати credentials)

```bash
git push origin main
```

Коли запитає:
- Username: `platinoff`
- Password: **Personal Access Token** (не пароль!)

---

## ✅ Варіант 4: Git Credential Fill (Інтерактивно)

В MSYS2 bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Видалити credential helper
git config --global --unset credential.helper

# Інтерактивно заповнити credentials
echo "protocol=https
host=github.com
username=platinoff
password=YOUR_PAT" | git credential fill

# Push
git push origin main
```

---

## 🔧 Перевірка Після Push

```bash
git log --oneline -5
git status
```

Має показати: `Your branch is up to date with 'origin/main'`

---

## 📝 Рекомендації

1. **SSH** - найбезпечніший варіант для постійного використання
2. **PAT в URL** - швидкий варіант для одноразового push (не забудь видалити після)
3. **Credential Store** - зручний для MSYS2, але зберігає credentials в файлі

---

**Детальніше**: 
- `docs/troubleshooting/GIT_AUTH_FIX.md` - повна інструкція з аутентифікації
- `docs/FIX_AUTH_AND_PUSH.md` - виправлення аутентифікації
