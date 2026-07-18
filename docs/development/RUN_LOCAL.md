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

**Last updated:** 2026-07-18 (PH-S1018 band 37 · quick preset · `--light` · vision launch)

### PH-S1011 / PH-S1012: Light compile + quick preset

```powershell
# fastest dev loop — light features + background + health wait
.\bin\run-poolai.ps1 quick

# explicit light build
.\bin\run-poolai.ps1 build -Light
.\bin\run-poolai.ps1 single -Background -SkipBuild -Light
```

```bash
/usr/bin/bash bin/run-poolai.sh quick
/usr/bin/bash bin/run-poolai.sh build --light
/usr/bin/bash bin/run-poolai.sh single --bg --skip-build --light
```

| Preset | Features (default) | Notes |
|--------|-------------------|--------|
| **full** | `enterprise,ml,cloud,test-utils` | `run-poolai build` default |
| **light** (`--light`) | `enterprise,test-utils` | PH-S1011 faster compile |

`quick` restores `data/dev/last_run.json` port when present (PH-S1014), runs light build unless `--skip-build`, starts `single --bg`, waits for `/api/v1/health`.

### PH-S1013: Vision easy launch

| Shell | Command |
|-------|---------|
| PowerShell | `.\bin\open-docs-vision.ps1` |
| MSYS2 | `/usr/bin/bash bin/open-docs-vision.sh` |

URL: `http://127.0.0.1:8765/docs/vision/index.html` — see README § Galaxy docs vision.

### PH-S1015 / PH-S1016: Admin power UI + API

Admin toolbar **⏻** → modal «Виключити» / «Перезавантажити» (`/ui/admin`). Wire: `POST /api/v1/ops/power` with `{"action":"shutdown"|"reboot"}` — dev-stand safe (host reboot skipped). Rust integration: `tests/ops_power_integration.rs`.

### PH-S55 / PH-S856: RAID jobs preset (single / lan one-liner)

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

### Verify signed release (PH-S85, після `build`)

Перевірка CLI **`poolai-verify-release`** на repo fixtures (dev key `poolai-dev`, не production):

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
cd /s/rust/poolAI
FIX=tests/fixtures/release/dev

cargo run --bin poolai-verify-release -- \
  --manifest "$FIX/release-manifest.json" \
  --signature "$FIX/release-manifest.json.sig" \
  --trust-root "$FIX/maintainer_keys.json" \
  --artifact "$FIX/poolai-sample.bin" \
  --artifact-name poolai
```

PowerShell (після `.\bin\run-poolai.ps1 build`):

```powershell
$fix = "tests/fixtures/release/dev"
cargo run --bin poolai-verify-release -- `
  --manifest "$fix/release-manifest.json" `
  --signature "$fix/release-manifest.json.sig" `
  --trust-root "$fix/maintainer_keys.json" `
  --artifact "$fix/poolai-sample.bin" `
  --artifact-name poolai
```

Операторський quickstart і політика — [`SECURITY_HARDENING.md`](../security/SECURITY_HARDENING.md) (Galaxy §9.2 hub, без дублювання governance). Файли fixtures — [`tests/fixtures/release/dev/README.md`](../../tests/fixtures/release/dev/README.md).

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

## HTTP stand smoke (Rust, PH-S145)

API wire перевірки проти **live** coordinator без Playwright (канон для stand; CI без stand — `cargo test-ci` + `tests/*_integration.rs`).

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export POOLAI_BASE_URL=http://127.0.0.1:8080

# Coordinator уже на :8080 (run-poolai single --bg)
cargo run --bin poolai-http-stand-smoke

# RAID restart only (PH-S156; replaces Playwright jobs_raid):
export POOLAI_E2E_STAND_ROOT=/tmp/poolai-e2e-NNN   # шлях з логу --start
cargo run --bin poolai-http-stand-smoke -- --raid-restart

# Job lease renew suite (PH-S196; replaces Playwright jobs_lease):
cargo run --bin poolai-http-stand-smoke -- --lease-renew

# Full suite incl. raid restart:
cargo run --bin poolai-http-stand-smoke -- --raid

# JSON звіт на stdout:
cargo run --bin poolai-http-stand-smoke -- --json
```

| Env | Призначення |
|-----|-------------|
| `POOLAI_BASE_URL` | Base URL stand (default `http://127.0.0.1:8080`) |
| `POOLAI_E2E_STAND_ROOT` | Каталог e2e stand з `restart.sh` (для `--raid-restart` / `--raid`) |
| `POOLAI_STAND_SMOKE_RAID_RESTART=1` | Альтернатива прапорцю `--raid-restart` |
| `POOLAI_STAND_SMOKE_LEASE_RENEW=1` | Альтернатива прапорцю `--lease-renew` |
| `POOLAI_STAND_SMOKE_RAID=1` | Альтернатива прапорцю `--raid` (full suite + raid) |
| `POOLAI_VISION_BASE_URL` | Vision static server for PH-S208 header check (default `http://127.0.0.1:8765`; `open-docs-vision.ps1`) |

Default stand smoke includes **`vision_revision_parity`** (PH-S208, PH-S235): repo `manifest.revision` vs FM §5.12 `Vision rev`, `extensions.active_sprint` vs `manifest.next_sprint`, then `GET /docs/vision/manifest.json` with `X-PoolAI-Vision-Revision` header vs JSON body.

---

## Admin UI WASM POC (PH-S147)

Grid-pricing / lease helpers з [`crates/poolai-ui-core`](../../crates/poolai-ui-core) → [`crates/poolai-ui-wasm`](../../crates/poolai-ui-wasm) для браузера.

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu   # MSYS2 UCRT64

# Потрібно: rustup target add wasm32-unknown-unknown
#           cargo install wasm-bindgen-cli --version 0.2.108
bash bin/build-ui-wasm.sh
```

Артефакти: `src/ui/wasm/poolai_ui_wasm_bg.wasm` + `poolai_ui_wasm.js` (gitignored; див. [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) §2).

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

**Last updated:** 2026-06-21 (PH-S156 `--raid-restart` · PH-S853 `jobs_store_backend` smoke)
