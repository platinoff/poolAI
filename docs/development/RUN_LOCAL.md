# Локальний запуск PoolAI

**Канон:** один вхід — `bin/run-poolai.sh` (MSYS2) або `bin/run-poolai.ps1` (PowerShell).

**Не використовуй** вбудований термінал Cursor для `cargo`/`git` на Windows — [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md); для запуску достатньо **зовнішнього MSYS2 UCRT64** або PowerShell.

---

## Швидкий старт (один вузол — повний UI + Admin)

```bash
export PATH="$HOME/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI

bash bin/run-poolai.sh build    # перший раз
bash bin/run-poolai.sh single   # foreground на :8080
```

| URL | Призначення |
|-----|-------------|
| http://127.0.0.1:8080/ui | Dashboard |
| http://127.0.0.1:8080/ui/admin | Admin (потрібен `enterprise`) |
| http://127.0.0.1:8080/ui/login | Вхід |
| http://127.0.0.1:8080/api/v1/health | Health JSON |

**Логін за замовчуванням:** `admin` / `admin123` (змінити перед продакшном).

**Фоновий режим:**

```bash
bash bin/run-poolai.sh single --bg
bash bin/run-poolai.sh status
bash bin/run-poolai.sh stop
```

---

## Режими запуску

| Команда | Що піднімає | Коли використовувати |
|---------|-------------|-------------------|
| `single` | 1× `poolai` | Розробка UI, API, admin, ML enterprise |
| `lan` | 2+× `poolai` (8080, 8081, …) | Distributed RAID на одному ПК ([`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md) §5) |
| `virtual-node` | `poolai` + `poolai-worker` | FM-016: VN, tasks, pool join |
| `docker` | Контейнер `poolai:latest` | Ізольований прогін без локального Rust |
| `build` | Збірка бінарників | Перед іншими режимами |
| `stop` / `status` | Зупинка / health | Після `--bg` або `lan` / `virtual-node` |

### Virtual node (coordinator + worker)

```bash
bash bin/run-poolai.sh virtual-node
# після ~50 с:
bash bin/verify-dev-stand.sh
bash bin/run-poolai.sh stop
```

Деталі env: [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) §2a–2b.

### LAN (два вузли)

```bash
bash bin/run-poolai.sh lan
# NODE_COUNT=3 BASE_PORT=8080 bash bin/run-lan-nodes.sh  # низькорівневий скрипт
```

### Docker

```bash
bash bin/run-poolai.sh docker
# або: cd docker && docker compose up -d --build
```

Порти: **8080** (HTTP), **8443** (HTTPS за конфігом). Томи: `poolai-data`, `poolai-config`.

### Опційно: Telegram-бот

Потрібен уже запущений coordinator (`single` або `virtual-node`):

```bash
cargo build --bin poolai-telegram-bot --features tgbot
TELEGRAM_BOT_TOKEN=... POOLAI_COORDINATOR_URL=http://127.0.0.1:8080 \
  target/debug/poolai-telegram-bot
```

---

## Збірка та features

| Режим | Features |
|-------|----------|
| Dev (лаунчер) | `enterprise,ml,cloud,test-utils` |
| Мінімум UI | `cargo run` (без enterprise — без `/ui/admin` enterprise API) |
| HTTPS + JWT | `cargo run --features enterprise,https,jwt` |

```bash
export FEATURES=enterprise,ml,cloud,test-utils
bash bin/run-poolai.sh build
```

Перевірка як CI: `cargo test-ci` (див. `bin/cargo-test.sh`, README).

---

## Змінні середовища (часті)

| Змінна | Компонент | Опис |
|--------|-----------|------|
| `POOLAI_HTTP_PORT` | poolai | Порт API/UI (default 8080) |
| `POOLAI_DATA_PATH` | poolai | Каталог даних |
| `POOLAI_RAID_BASE_PATH` | poolai | Сховище RAID-артефактів |
| `POOLAI_CONFIG_PATH` | poolai | TOML (default `config.toml`) |
| `POOLAI_VIRTUAL_NODE_DATA_DIR` | coordinator | Персистентні VN tasks/bindings |
| `POOLAI_COORDINATOR_URL` | worker, bot | Base URL coordinator |
| `POOLAI_WORKER_PORT` | worker | Health порту worker (default 9090) |
| `RUST_LOG` | усі | `info`, `debug`, … |

Приклад конфігу: [`config.example.toml`](../../config.example.toml) → скопіювати в `config.toml`.

---

## E2E / pa11y (окремо від сервера)

```bash
# poolai вже на :8080 або:
bash bin/e2e-playwright.sh --start
bash bin/pa11y-ci.sh
```

---

## Усунення проблем

| Симптом | Дія |
|---------|-----|
| `poolai.exe` не знайдено | `bash bin/run-poolai.sh build` |
| Порт зайнятий | `bash bin/run-poolai.sh stop` або `--port 8082` |
| OOM при тестах Windows | `cargo test-ci` з `-j 1` (див. README) |
| Admin 404 / порожній API | Запуск з `enterprise` (лаунчер робить це за замовчуванням) |

**Last updated:** 2026-05-19
