# ⚡ Швидкий Push - Copy-Paste Команди

## Крок 1: Створи Personal Access Token

1. Відкрий: https://github.com/settings/tokens
2. "Generate new token (classic)"
3. Назва: `poolAI-push`
4. Scopes: `repo` (всі галочки в repo)
5. Generate → **СКОПІЮЙ ТОКЕН**

## Крок 2: Виконай в MSYS2 bash

```bash
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd /s/rust/poolAI

# Видали credential helper тимчасово (щоб git запитав credentials)
git config --global --unset credential.helper

# Push - має запитати username та password
git push origin main
```

**Коли запитає**:
- Username: `platinoff`
- Password: **встав Personal Access Token** (не пароль!)

**Після успішного push**:
```bash
# Поверни credential helper
git config --global credential.helper wincred
```

---

## Альтернатива: SSH (якщо вже є SSH ключ)

```bash
cd /s/rust/poolAI
git remote set-url origin git@github.com:platinoff/poolAI.git
git push origin main
```

---

## Якщо все ще не працює

Перевір чи є старі credentials:
```powershell
# В PowerShell
cmdkey /list | findstr git
```

Якщо є, видали:
```powershell
cmdkey /delete:git:https://github.com
```

Потім спробуй push знову.

---

**Детальніше**: [`GIT_AUTH_FIX.md`](./GIT_AUTH_FIX.md)
