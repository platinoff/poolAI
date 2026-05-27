# Локальний запуск PoolAI

**Канон Windows:**

| Що | Команда |
|----|---------|
| **Запуск / stop** (PowerShell) | `.\bin\run-poolai.ps1` |
| **Bash-скрипти** з PowerShell | `.\bin\poolai-msys.ps1 …` |
| **MSYS2 UCRT64** (зовнішнє вікно) | `/usr/bin/bash bin/run-poolai.sh …` |
| **Git / cargo test-ci** | зовнішнє MSYS2 — [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md) |

**Не працює:** `bash bin/run-poolai.sh` у PowerShell або в MSYS2 з WSL-stub → *«WSL has no installed distributions»*. Див. [`.cursor/commands/run-poolai.md`](../../.cursor/commands/run-poolai.md).

---

## Швидкий старт — PowerShell (рекомендовано)

```powershell
cd S:\rust\poolAI

.\bin\run-poolai.ps1 build
.\bin\run-poolai.ps1 single -Background -SkipBuild
.\bin\run-poolai.ps1 status
.\bin\run-poolai.ps1 stop
```

### PH-S55: RAID jobs preset (single / lan one-liner)

```powershell
# single node + RAID job store preset
.\bin\run-poolai.ps1 single -Background -SkipBuild -RaidJobs

# lan: one-liner with explicit env (node 1; repeat with another RAID path for node 2)
$env:POOLAI_RAID_BASE_PATH = "$PWD\data\dev\lan\node1\raid"; $env:POOLAI_JOB_STORE = "raid"; .\bin\run-poolai.ps1 lan
```

| URL | Призначення |
|-----|-------------|
| http://127.0.0.1:8080/ui/login | **Спочатку логін** |
| http://127.0.0.1:8080/ui/admin/jobs | Jobs + badge store (`json`/`raid`) |
| http://127.0.0.1:8080/ui/admin | Admin dashboard |
| http://127.0.0.1:8080/api/v1/health | Health JSON |

**Логін:** `admin` / `admin123`

---

## Швидкий старт — MSYS2 UCRT64 (зовнішнє вікно)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI

/usr/bin/bash bin/run-poolai.sh build
/usr/bin/bash bin/run-poolai.sh single --bg --skip-build
/usr/bin/bash bin/run-poolai.sh stop
```

```bash
# single node + RAID job store preset
/usr/bin/bash bin/run-poolai.sh single --bg --skip-build --raid-jobs

# lan: one-liner with explicit env (node 1; repeat with another RAID path for node 2)
POOLAI_RAID_BASE_PATH="$PWD/data/dev/lan/node1/raid" POOLAI_JOB_STORE=raid /usr/bin/bash bin/run-poolai.sh lan
```

**Не** пиши голе `bash` — використовуй `/usr/bin/bash` (інакше може викликатись WSL).

---

## Режими запуску

| Команда | Що піднімає | Коли використовувати |
|---------|-------------|-------------------|
| `single` | 1× `poolai` | Розробка UI, API, admin, ML enterprise (`--raid-jobs` для job RAID preset) |
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
# опційно RAID job persist (PH-S54): coordinator з raid store, потім:
# VERIFY_RAID_JOB_STORE=1 bash bin/verify-dev-stand.sh
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
| `POOLAI_RAID_BASE_PATH` | poolai | Сховище RAID-артефактів (обов’язково для `POOLAI_JOB_STORE=raid`) |
| `POOLAI_JOB_DATA_DIR` | coordinator | JSON/SQLite jobs (`data/jobs` → `jobs.json` або `jobs.db`) |
| `POOLAI_JOB_STORE` | coordinator | `sqlite` (feature `job-store-sqlite`), `raid` (snapshot у RAID), інакше JSON |
| `POOLAI_CONFIG_PATH` | poolai | TOML (default `config.toml`) |
| `POOLAI_VIRTUAL_NODE_DATA_DIR` | coordinator | Персистентні VN tasks/bindings |
| `POOLAI_COORDINATOR_URL` | worker, bot | Base URL coordinator |
| `POOLAI_WORKER_PORT` | worker | Health порту worker (default 9090) |
| `RUST_LOG` | усі | `info`, `debug`, … |

Приклад конфігу: [`config.example.toml`](../../config.example.toml) → скопіювати в `config.toml`.

### Job store: RAID snapshot (PH-S48 / PH-S49)

Jobs зберігаються як RAID-артефакт (логічне ім’я у `src/job/store.rs`). **Порядок:** спочатку `POOLAI_RAID_BASE_PATH`, потім `POOLAI_JOB_STORE=raid`, потім старт coordinator.

```bash
export POOLAI_RAID_BASE_PATH="$PWD/data/dev/raid"
export POOLAI_JOB_STORE=raid
bash bin/run-poolai.sh single
# POST /api/v1/jobs → restart → GET /api/v1/jobs/{id} має зберегти запис
```

Для LAN (`bash bin/run-poolai.sh lan`) — **різні** `POOLAI_RAID_BASE_PATH` на кожен вузол (див. [`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md)). Тест: `tests/job_store_raid_persistence.rs` (`--features test-utils`).

---

## E2E / pa11y (окремо від сервера)

```powershell
# PowerShell:
.\bin\e2e-playwright.ps1 -Start
# або:
.\bin\poolai-msys.ps1 bin/e2e-playwright.sh --start
```

```bash
# MSYS2:
/usr/bin/bash bin/e2e-playwright.sh --start
/usr/bin/bash bin/pa11y-ci.sh
```

---

## Усунення проблем

| Симптом | Дія |
|---------|-----|
| WSL / «no installed distributions» на `bash` | PowerShell: `.\bin\run-poolai.ps1`; bash-скрипти: `.\bin\poolai-msys.ps1` або `/usr/bin/bash` у MSYS2 |
| `link.exe` / `link: extra operand` у PowerShell `cargo` | Не `cargo` з PS — `.\bin\run-poolai.ps1 build` (MSYS2 GNU) або зовнішнє MSYS2 |
| `poolai.exe` не знайдено | `.\bin\run-poolai.ps1 build` |
| Порт зайнятий | `.\bin\run-poolai.ps1 stop` або `-Port 8082` |
| `/ui/admin` порожній / редірект | Спочатку http://127.0.0.1:8080/ui/login (`admin` / `admin123`) |
| OOM при тестах Windows | `.\bin\poolai-msys.ps1 -lc 'cargo test-ci'` з `-j 1` (див. README) |
| Admin 404 / порожній API | Запуск з `enterprise` (лаунчер робить це за замовчуванням) |

**Last updated:** 2026-05-26 (PowerShell / WSL bash fix)
