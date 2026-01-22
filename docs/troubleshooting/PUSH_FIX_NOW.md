# 🔧 Швидке Виправлення Push (Authentication Failed)

**Проблема**: Git не запитує credentials, одразу падає з помилкою.

## Рішення 1: Використати Windows Credential Manager (Рекомендовано)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Видалити старі збережені credentials (якщо є)
# В PowerShell (окреме вікно):
# cmdkey /delete:git:https://github.com

# Використати Windows Credential Manager
git config --global credential.helper wincred

# АБО спробувати manager-core (якщо wincred не працює)
# git config --global credential.helper manager-core

# Спробувати push - тепер має запитати credentials
git push origin main
```

Коли запитає:
- **Username**: `platinoff`
- **Password**: Personal Access Token (НЕ пароль!)

**Якщо все ще не запитує**, спробуй:
```bash
# Видалити credential helper тимчасово
git config --global --unset credential.helper

# Push - має запитати credentials
git push origin main

# Після успішного push, поверни credential helper
git config --global credential.helper wincred
```

---

## Рішення 2: Додати credentials в URL (Тимчасово)

**⚠️ УВАГА**: Не використовуй це для постійної роботи, тільки для тестування!

```bash
cd /s/rust/poolAI

# Створи Personal Access Token на GitHub спочатку!
# Потім використай в URL (заміни YOUR_TOKEN):
git remote set-url origin https://platinoff:YOUR_TOKEN@github.com/platinoff/poolAI.git

# Push
git push origin main

# Після успішного push, поверни нормальний URL:
git remote set-url origin https://github.com/platinoff/poolAI.git
```

---

## Рішення 3: Використати GIT_ASKPASS (MSYS2)

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Створи скрипт для credentials (тимчасово)
cat > /tmp/git-askpass.sh << 'EOF'
#!/bin/bash
echo "$GIT_PASSWORD"
EOF
chmod +x /tmp/git-askpass.sh

# Встанови credentials
export GIT_ASKPASS=/tmp/git-askpass.sh
export GIT_PASSWORD="YOUR_PERSONAL_ACCESS_TOKEN"

# Push
git push origin main

# Очистити
unset GIT_ASKPASS
unset GIT_PASSWORD
```

---

## Рішення 4: Перевірити чи є збережені credentials

```bash
# Перевірити збережені credentials (Windows)
# В PowerShell:
cmdkey /list | findstr git

# Якщо є старі credentials, видали їх:
cmdkey /delete:git:https://github.com
```

Потім спробуй push знову - git має запитати нові credentials.

---

## Рішення 5: Використати SSH (Найкраще для постійної роботи)

### Крок 1: Перевірити чи є SSH ключ

```bash
ls -la ~/.ssh/id_*.pub
```

### Крок 2: Якщо немає, створити новий

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# Натисни Enter для всіх питань
```

### Крок 3: Додати ключ до GitHub

```bash
# Показати публічний ключ
cat ~/.ssh/id_ed25519.pub
```

Скопіюй вивід та додай на GitHub:
- https://github.com/settings/ssh/new
- Title: `poolAI-dev`
- Key: встав скопійований ключ
- Add SSH key

### Крок 4: Змінити remote на SSH

```bash
cd /s/rust/poolAI
git remote set-url origin git@github.com:platinoff/poolAI.git
```

### Крок 5: Перевірити SSH з'єднання

```bash
ssh -T git@github.com
```

Має показати: `Hi platinoff! You've successfully authenticated...`

### Крок 6: Push

```bash
git push origin main
```

---

## Швидкий тест: Перевірити remote URL

```bash
cd /s/rust/poolAI
git remote -v
```

Має показати:
- HTTPS: `https://github.com/platinoff/poolAI.git`
- SSH: `git@github.com:platinoff/poolAI.git`

---

## Рекомендація

**Найкраще рішення**: Використай **SSH** (Рішення 5) - це найбезпечніше та найзручніше для постійної розробки.

Якщо потрібно швидко - використай **Рішення 1** (Windows Credential Manager).

---

**Детальніше**: [`GIT_AUTH_FIX.md`](./GIT_AUTH_FIX.md)
