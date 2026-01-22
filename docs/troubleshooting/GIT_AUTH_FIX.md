# Fix: Git Authentication Failed для GitHub Push

## Проблема

При виконанні `git push origin main`:
```
remote: No anonymous write access.
fatal: Authentication failed for 'https://github.com/platinoff/poolAI.git/'
```

## Рішення

### Варіант 1: Personal Access Token (PAT) для HTTPS (Рекомендовано)

#### Крок 1: Створити Personal Access Token на GitHub

1. Перейди на GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Натисни "Generate new token (classic)"
3. Назва: `poolAI-push` (або будь-яка інша)
4. Expiration: вибери термін (наприклад, 90 днів або No expiration)
5. Scopes: обирай `repo` (повний доступ до репозиторіїв)
6. Натисни "Generate token"
7. **ВАЖЛИВО**: Скопіюй токен одразу (він більше не буде показаний)

#### Крок 2: Налаштувати git credential helper

В MSYS2 bash:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Налаштувати credential helper (зберігає токен)
git config --global credential.helper store

# Або для Windows Credential Manager (безпечніше)
git config --global credential.helper wincred
```

#### Крок 3: Push з токеном

При першому push git запитає credentials:
- **Username**: твій GitHub username (наприклад, `platinoff`)
- **Password**: встав Personal Access Token (НЕ пароль від GitHub!)

```bash
git push origin main
```

Після успішного push, credential helper збереже токен для майбутніх операцій.

---

### Варіант 2: SSH Authentication (Альтернатива)

#### Крок 1: Перевірити чи є SSH ключ

```bash
ls -la ~/.ssh/id_rsa.pub
```

Якщо файлу немає, створи новий ключ:

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# Натисни Enter для всіх питань (або вкажи пароль)
```

#### Крок 2: Додати SSH ключ до GitHub

1. Скопіюй публічний ключ:
```bash
cat ~/.ssh/id_ed25519.pub
```

2. GitHub → Settings → SSH and GPG keys → New SSH key
3. Встав публічний ключ
4. Натисни "Add SSH key"

#### Крок 3: Змінити remote на SSH

```bash
cd /s/rust/poolAI
git remote set-url origin git@github.com:platinoff/poolAI.git
```

#### Крок 4: Перевірити SSH з'єднання

```bash
ssh -T git@github.com
```

Має показати: `Hi platinoff! You've successfully authenticated...`

#### Крок 5: Push через SSH

```bash
git push origin main
```

---

### Варіант 3: GitHub CLI (gh) - Найпростіше

#### Крок 1: Встановити GitHub CLI

```bash
# В MSYS2 (якщо доступно)
pacman -S github-cli

# Або завантажити з https://cli.github.com/
```

#### Крок 2: Авторизуватися

```bash
gh auth login
```

Слідуй інструкціям:
- GitHub.com → HTTPS → Login with a web browser
- Скопіюй код та відкрий посилання в браузері
- Авторизуйся через браузер

#### Крок 3: Push

```bash
git push origin main
```

GitHub CLI автоматично використає збережені credentials.

---

## Швидке виправлення (HTTPS + PAT)

Якщо вже є Personal Access Token:

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI
git config --global credential.helper store
git push origin main
# Коли запитає:
# Username: platinoff
# Password: <встав PAT токен>
```

---

## Перевірка налаштувань

### Перевірити remote URL:

```bash
git remote -v
```

Має показати:
- HTTPS: `https://github.com/platinoff/poolAI.git`
- SSH: `git@github.com:platinoff/poolAI.git`

### Перевірити credential helper:

```bash
git config --global credential.helper
```

### Перевірити збережені credentials (Windows):

```powershell
# PowerShell
cmdkey /list | findstr git
```

---

## Профілактика

1. **Використовуй Personal Access Token** замість пароля
2. **Встанови expiration** для токенів (90 днів або менше)
3. **Використовуй SSH** для постійної розробки (безпечніше)
4. **Не коміть токени** в код або конфігурацію

---

## Детальніше

- [GitHub: Creating a personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
- [GitHub: Adding a new SSH key](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)
- [GitHub CLI documentation](https://cli.github.com/manual/)

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-22
