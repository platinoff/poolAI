# Запуск PoolAI (Windows)

## Не працює (чому)

| Команда | Проблема |
|---------|----------|
| `bash bin/run-poolai.sh` у **PowerShell** | Викликає **WSL** (`Windows Subsystem for Linux has no installed distributions`) |
| `bash …` у **MSYS2**, якщо в PATH є WindowsApps | Той самий WSL-stub замість `/usr/bin/bash` |

## PowerShell (рекомендовано для run/stop)

```powershell
cd S:\rust\poolAI

bash bin/run-poolai.sh build
bash bin/run-poolai.sh single -Background -SkipBuild
bash bin/run-poolai.sh status
bash bin/run-poolai.sh stop
```

Якщо ExecutionPolicy блокує скрипти:

```powershell
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
# або разово:
powershell -ExecutionPolicy Bypass -File bash bin/run-poolai.sh single -Background -SkipBuild
```

| URL | |
|-----|--|
| Login | http://127.0.0.1:8080/ui/login |
| Admin / Jobs | http://127.0.0.1:8080/ui/admin/jobs |
| Credentials | `admin` / `admin123` |

## Bash-скрипти з PowerShell (e2e, verify, cargo test-ci)

```powershell
bash bin/poolai-msys.sh bin/run-poolai.sh status
bash bin/poolai-msys.sh bin/e2e-playwright.sh --start
bash bin/e2e-playwright.sh -Start
bash bin/poolai-msys.sh -lc 'export K8S_OPENAPI_ENABLED_VERSION=1.28; cargo test-ci'
```

## MSYS2 UCRT64 (зовнішнє вікно з меню Пуск)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI

/usr/bin/bash bin/run-poolai.sh single --bg --skip-build
/usr/bin/bash bin/run-poolai.sh stop
```

**Не** використовуй голе слово `bash` — лише `/usr/bin/bash` або `bash bin/poolai-msys.sh`.

## Git / push

Лише **зовнішнє** вікно MSYS2 — [git-push.md](./git-push.md). Не `git` з PowerShell Cursor.

Повна дока: [`docs/development/RUN_LOCAL.md`](../../docs/development/RUN_LOCAL.md)
