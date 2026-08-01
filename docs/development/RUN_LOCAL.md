# Локальний запуск PoolAI

**Канон Windows:**

| Що | Команда |
|----|---------|
| **Запуск / stop** (PowerShell) | `.\bin\run-poolai.ps1` |
| **Bash-скрипти** з PowerShell | `.\bin\poolai-msys.ps1 …` (inline: `-lc` або `-Command "cargo test-ci"`) |
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

**Last updated:** 2026-07-28 (PH-S1608 band 96 · `--monitoring-loc-audit` · `VERIFY_MONITORING_LOC_AUDIT`)

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

`quick` restores `data/dev/last_run.json` port when present (PH-S1014), runs light build unless `--skip-build`, starts `single --bg`, waits for `/api/v1/health`. Optional **`--stand-smoke`** (PH-S1095) runs `poolai-http-stand-smoke --run-local-smoke` after health OK. Optional **`--migration-advisory`** (PH-S1104) runs `poolai-loc-audit --migration-advisory` after health OK. Optional **`--stable-touchup`** (PH-S1114) runs `poolai-loc-audit --stable-touchup` after health OK. Optional **`--edge-verification`** (PH-S1125) runs `poolai-loc-audit --edge-verification-advisory` after health OK. Optional **`--pre-push-canon`** (PH-S1134) runs `poolai-loc-audit --pre-push-canon` after health OK. Optional **`--ci-canon`** (PH-S1143) runs `poolai-loc-audit --ci-canon` + `poolai-openapi-gap-audit` after health OK. Optional **`--tenant-persist`** (PH-S1154) runs `poolai-loc-audit --tenant-persist` after health OK. Optional **`--tenant-store`** (PH-S1162) runs `poolai-loc-audit --tenant-store` after health OK. Optional **`--tenant-api`** (PH-S1175) runs `poolai-loc-audit --tenant-api` after health OK. Optional **`--tenant-admin-ops`** (PH-S1184) runs `poolai-loc-audit --tenant-admin-ops` after health OK. Optional **`--tenant-stand-smoke`** (PH-S1195) runs live `poolai-http-stand-smoke --tenant-stand-smoke` + `poolai-loc-audit --tenant-stand-smoke` after health OK. Optional **`--tenant-loc-audit`** (PH-S1202) runs `poolai-loc-audit --tenant-loc-audit` after health OK. Optional **`--tenant-docs-canon`** (PH-S1212) runs `poolai-loc-audit --tenant-docs-canon` after health OK. Optional **`--tenant-vision-sync`** (PH-S1222) runs `poolai-loc-audit --tenant-vision-sync` after health OK. Optional **`--tenant-ratio-advisory`** (PH-S1232) runs `poolai-loc-audit --tenant-ratio-advisory` after health OK. Optional **`--tenant-horizon`** (PH-S1242) runs `poolai-loc-audit --tenant-horizon` after health OK. Optional **`--sso`** (PH-S1252) runs `poolai-loc-audit --sso` after health OK. Optional **`--sso-store`** (PH-S1262) runs `poolai-loc-audit --sso-store` after health OK.

```bash
/usr/bin/bash bin/run-poolai.sh quick --stand-smoke
/usr/bin/bash bin/run-poolai.sh quick --migration-advisory
/usr/bin/bash bin/run-poolai.sh quick --stable-touchup
/usr/bin/bash bin/run-poolai.sh quick --edge-verification
/usr/bin/bash bin/run-poolai.sh quick --pre-push-canon
/usr/bin/bash bin/run-poolai.sh quick --ci-canon
/usr/bin/bash bin/run-poolai.sh quick --tenant-persist
/usr/bin/bash bin/run-poolai.sh quick --tenant-store
/usr/bin/bash bin/run-poolai.sh quick --tenant-api
/usr/bin/bash bin/run-poolai.sh quick --tenant-admin-ops
/usr/bin/bash bin/run-poolai.sh quick --tenant-stand-smoke
/usr/bin/bash bin/run-poolai.sh quick --tenant-loc-audit
/usr/bin/bash bin/run-poolai.sh quick --tenant-docs-canon
/usr/bin/bash bin/run-poolai.sh quick --tenant-vision-sync
/usr/bin/bash bin/run-poolai.sh quick --tenant-ratio-advisory
/usr/bin/bash bin/run-poolai.sh quick --tenant-horizon
/usr/bin/bash bin/run-poolai.sh quick --sso
/usr/bin/bash bin/run-poolai.sh quick --sso-store
/usr/bin/bash bin/run-poolai.sh quick --sso-api
/usr/bin/bash bin/run-poolai.sh quick --sso-admin-ops
/usr/bin/bash bin/run-poolai.sh quick --sso-stand-smoke
/usr/bin/bash bin/run-poolai.sh quick --sso-loc-audit
/usr/bin/bash bin/run-poolai.sh quick --sso-docs-canon
/usr/bin/bash bin/run-poolai.sh quick --sso-vision-sync
/usr/bin/bash bin/run-poolai.sh quick --sso-ratio-advisory
/usr/bin/bash bin/run-poolai.sh quick --sso-horizon
/usr/bin/bash bin/run-poolai.sh quick --audit-docs-canon
/usr/bin/bash bin/run-poolai.sh quick --audit-vision-sync
/usr/bin/bash bin/run-poolai.sh quick --audit-ratio-advisory
/usr/bin/bash bin/run-poolai.sh quick --audit-horizon
/usr/bin/bash bin/run-poolai.sh quick --policy
/usr/bin/bash bin/run-poolai.sh quick --policy-store
/usr/bin/bash bin/run-poolai.sh quick --policy-api
/usr/bin/bash bin/run-poolai.sh quick --policy-admin-ops
/usr/bin/bash bin/run-poolai.sh quick --policy-stand-smoke
/usr/bin/bash bin/run-poolai.sh quick --policy-loc-audit
/usr/bin/bash bin/run-poolai.sh quick --policy-docs-canon
/usr/bin/bash bin/run-poolai.sh quick --policy-vision-sync
/usr/bin/bash bin/run-poolai.sh quick --policy-ratio-advisory
/usr/bin/bash bin/run-poolai.sh quick --policy-horizon
/usr/bin/bash bin/run-poolai.sh quick --monitoring-store
/usr/bin/bash bin/run-poolai.sh quick --monitoring-api
/usr/bin/bash bin/run-poolai.sh quick --monitoring-admin-ops
/usr/bin/bash bin/run-poolai.sh quick --monitoring-stand-smoke
/usr/bin/bash bin/run-poolai.sh quick --monitoring-loc-audit
/usr/bin/bash bin/run-poolai.sh quick --monitoring-horizon
# PowerShell:
.\bin\run-poolai.ps1 quick -StandSmoke
.\bin\run-poolai.ps1 quick -MigrationAdvisory
.\bin\run-poolai.ps1 quick -StableTouchup
.\bin\run-poolai.ps1 quick -TenantStandSmoke
.\bin\run-poolai.ps1 quick -TenantVisionSync
.\bin\run-poolai.ps1 quick -TenantRatioAdvisory
```

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

# RUN_LOCAL quick subset (health + monitoring + vm + ops; PH-S1093):
cargo run --bin poolai-http-stand-smoke -- --run-local-smoke

# Після verify-dev-stand (опційно):
VERIFY_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
```

| Env | Призначення |
|-----|-------------|
| `POOLAI_BASE_URL` | Base URL stand (default `http://127.0.0.1:8080`) |
| `POOLAI_E2E_STAND_ROOT` | Каталог e2e stand з `restart.sh` (для `--raid-restart` / `--raid`) |
| `POOLAI_STAND_SMOKE_RAID_RESTART=1` | Альтернатива прапорцю `--raid-restart` |
| `POOLAI_STAND_SMOKE_LEASE_RENEW=1` | Альтернатива прапорцю `--lease-renew` |
| `POOLAI_STAND_SMOKE_RAID=1` | Альтернатива прапорцю `--raid` (full suite + raid) |
| `POOLAI_STAND_SMOKE_RUN_LOCAL=1` | Альтернатива `--run-local-smoke` (PH-S1093) |
| `VERIFY_STAND_SMOKE=1` | `verify-dev-stand.sh` → `--run-local-smoke` після bootstrap (PH-S1094) |
| `VERIFY_MIGRATION_ADVISORY=1` | `verify-dev-stand.sh` → `poolai-loc-audit --migration-advisory` (PH-S1103) |
| `VERIFY_STABLE_TOUCHUP=1` | `verify-dev-stand.sh` → `poolai-loc-audit --stable-touchup` (PH-S1113) |
| `VERIFY_EDGE_VERIFICATION=1` | `verify-dev-stand.sh` → `poolai-loc-audit --edge-verification-advisory` (PH-S1125) |
| `VERIFY_PRE_PUSH_CANON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --pre-push-canon` (PH-S1134) |
| `VERIFY_CI_CANON=1` | `verify-dev-stand.sh` → openapi-gap-audit + `poolai-loc-audit --ci-canon` + rust-ratio advisory (PH-S1142) |
| `VERIFY_TENANT_PERSIST=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-persist` (PH-S1153) |
| `VERIFY_TENANT_STORE=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-store` (PH-S1162) |
| `VERIFY_TENANT_API=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-api` (PH-S1175) |
| `VERIFY_TENANT_ADMIN_OPS=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-admin-ops` (PH-S1184) |
| `VERIFY_TENANT_STAND_SMOKE=1` | `verify-dev-stand.sh` → live `--tenant-stand-smoke` + loc-audit (PH-S1195) |
| `VERIFY_TENANT_LOC_AUDIT=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-loc-audit` (PH-S1202) |
| `VERIFY_TENANT_DOCS_CANON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-docs-canon` (PH-S1212) |
| `VERIFY_TENANT_VISION_SYNC=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-vision-sync` (PH-S1222) |
| `VERIFY_TENANT_RATIO_ADVISORY=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-ratio-advisory` (PH-S1232) |
| `VERIFY_TENANT_HORIZON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --tenant-horizon` (PH-S1242) |
| `VERIFY_SSO=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso` (PH-S1252) |
| `VERIFY_SSO_STORE=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-store` (PH-S1262) |
| `VERIFY_SSO_API=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-api` (PH-S1275) |
| `VERIFY_SSO_ADMIN_OPS=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-admin-ops` (PH-S1284) |
| `VERIFY_SSO_STAND_SMOKE=1` | `verify-dev-stand.sh` → live `--sso-stand-smoke` + loc-audit (PH-S1295) |
| `VERIFY_SSO_LOC_AUDIT=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-loc-audit` (PH-S1302) |
| `VERIFY_SSO_DOCS_CANON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-docs-canon` (PH-S1312) |
| `VERIFY_SSO_VISION_SYNC=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-vision-sync` (PH-S1322) |
| `VERIFY_SSO_RATIO_ADVISORY=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-ratio-advisory` (PH-S1332) |
| `VERIFY_SSO_HORIZON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --sso-horizon` (PH-S1342) |
| `VERIFY_AUDIT=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit` (PH-S1352) |
| `VERIFY_AUDIT_STORE=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-store` (PH-S1362) |
| `VERIFY_AUDIT_API=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-api` (PH-S1374) |
| `VERIFY_AUDIT_ADMIN_OPS=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-admin-ops` (PH-S1384) |
| `VERIFY_AUDIT_STAND_SMOKE=1` | `verify-dev-stand.sh` → live `--audit-stand-smoke` + loc-audit (PH-S1395) |
| `VERIFY_AUDIT_LOC_AUDIT=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-loc-audit` (PH-S1402) |
| `VERIFY_AUDIT_DOCS_CANON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-docs-canon` (PH-S1412) |
| `VERIFY_AUDIT_VISION_SYNC=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-vision-sync` (PH-S1422) |
| `VERIFY_AUDIT_RATIO_ADVISORY=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-ratio-advisory` (PH-S1432) |
| `VERIFY_AUDIT_HORIZON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --audit-horizon` (PH-S1442) |
| `VERIFY_POLICY=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy` (PH-S1452) |
| `VERIFY_POLICY_STORE=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-store` (PH-S1462) |
| `VERIFY_POLICY_API=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-api` (PH-S1474) |
| `VERIFY_POLICY_ADMIN_OPS=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-admin-ops` (PH-S1484) |
| `VERIFY_POLICY_STAND_SMOKE=1` | `verify-dev-stand.sh` → `poolai-http-stand-smoke --policy-stand-smoke` + `poolai-loc-audit --policy-stand-smoke` (PH-S1495) |
| `VERIFY_POLICY_LOC_AUDIT=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-loc-audit` (PH-S1502) |
| `VERIFY_POLICY_DOCS_CANON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-docs-canon` (PH-S1512) |
| `VERIFY_POLICY_VISION_SYNC=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-vision-sync` (PH-S1522) |
| `VERIFY_POLICY_RATIO_ADVISORY=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-ratio-advisory` (PH-S1532) |
| `VERIFY_POLICY_HORIZON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --policy-horizon` (PH-S1544) |
| `VERIFY_MONITORING=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring` (PH-S1552) |
| `VERIFY_MONITORING_STORE=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-store` (PH-S1562) |
| `VERIFY_MONITORING_API=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-api` (PH-S1574) |
| `VERIFY_MONITORING_ADMIN_OPS=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-admin-ops` (PH-S1584) |
| `VERIFY_MONITORING_STAND_SMOKE=1` | `verify-dev-stand.sh` → `poolai-http-stand-smoke --monitoring-stand-smoke` + `poolai-loc-audit --monitoring-stand-smoke` (PH-S1595) |
| `VERIFY_MONITORING_LOC_AUDIT=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-loc-audit` (PH-S1602) |
| `VERIFY_MONITORING_DOCS_CANON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-docs-canon` (PH-S1612) |
| `VERIFY_MONITORING_VISION_SYNC=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-vision-sync` (PH-S1622) |
| `VERIFY_MONITORING_RATIO_ADVISORY=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-ratio-advisory` (PH-S1632) |
| `VERIFY_MONITORING_HORIZON=1` | `verify-dev-stand.sh` → `poolai-loc-audit --monitoring-horizon` (PH-S1644) |
| `POOLAI_VISION_BASE_URL` | Vision static server for PH-S208 header check (default `http://127.0.0.1:8765`; `open-docs-vision.ps1`) |

### PH-S1100: Rust migration advisory (band 46)

Registry ui_js → wasm targets + archived e2e → Rust wire canon; stretch **96%** spirit hold advisory.

```bash
cargo run --bin poolai-loc-audit -- --migration-advisory
cargo run --bin poolai-loc-audit -- --migration-advisory --advisory --min-ratio 0.95

VERIFY_MIGRATION_ADVISORY=1 bash bin/verify-dev-stand.sh
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `migration_advisory_mode` | `true` when `--migration-advisory` (PH-S1100) |
| `migration_candidate_total` | ui_js + archived e2e registry count |
| `migration_ui_js_candidate_count` | Admin JS glue files pending wasm (PH-S1102) |
| `migration_e2e_archived_count` | Archived Playwright API specs with Rust canon (PH-S1103) |

Module: [`rust_migration_advisory_depth.rs`](../../crates/poolai-ui-core/src/rust_migration_advisory_depth.rs) · tests: `rust_migration_advisory_audit.rs`, `galaxy_horizon_s1099_integration.rs`.

### PH-S1110: STABLE touch-up (band 47)

Maintenance-mode STABLE criteria registry touch-up; validates canonical doc markers for product-complete checklist.

```bash
cargo run --bin poolai-loc-audit -- --stable-touchup
cargo run --bin poolai-loc-audit -- --stable-touchup --advisory --min-ratio 0.95

VERIFY_STABLE_TOUCHUP=1 bash bin/verify-dev-stand.sh
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `stable_touchup_mode` | `true` when `--stable-touchup` (PH-S1110) |
| `stable_criteria_total` | STABLE maintenance criteria registry size (PH-S1112) |
| `stable_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`stable_state_touchup_depth.rs`](../../crates/poolai-ui-core/src/stable_state_touchup_depth.rs) · tests: `stable_state_touchup_audit.rs`, `galaxy_horizon_s1109_integration.rs`.

### PH-S1120: Edge verification horizon (band 48)

Galaxy §6.6 edge verification criteria registry; validates fraud-proof/capability/TEE wire markers and `GET /api/v1/grid/edge-verification-metrics`.

```bash
cargo run --bin poolai-loc-audit -- --edge-verification-advisory
cargo run --bin poolai-loc-audit -- --edge-verification-advisory --advisory --min-ratio 0.95

VERIFY_EDGE_VERIFICATION=1 bash bin/verify-dev-stand.sh
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `edge_verification_advisory_mode` | `true` when `--edge-verification-advisory` (PH-S1120) |
| `edge_verification_criteria_total` | Edge verification criteria registry size (PH-S1121) |
| `edge_verification_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`galaxy_edge_verification_depth.rs`](../../crates/poolai-ui-core/src/galaxy_edge_verification_depth.rs) · tests: `galaxy_edge_verification_audit.rs`, `galaxy_horizon_s1119_integration.rs`.

### PH-S1130: Pre-push vision canon gate (band 49)

Git pre-push hook criteria registry; validates `bin/pre-push-hook.sh`, `poolai-vision-sync` canon doc sync, and `cargo fmt` gate.

```bash
cargo run --bin poolai-loc-audit -- --pre-push-canon
cargo run --bin poolai-loc-audit -- --pre-push-canon --advisory --min-ratio 0.95

bash bin/install-pre-push-hook.sh
VERIFY_PRE_PUSH_CANON=1 bash bin/verify-dev-stand.sh
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `pre_push_canon_mode` | `true` when `--pre-push-canon` (PH-S1130) |
| `pre_push_criteria_total` | Pre-push canon criteria registry size (PH-S1131) |
| `pre_push_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`pre_push_hook_depth.rs`](../../crates/poolai-ui-core/src/pre_push_hook_depth.rs) · tests: `pre_push_hook_audit.rs`, `galaxy_horizon_s1129_integration.rs` · docs: [`PRE_PUSH_HOOK.md`](./PRE_PUSH_HOOK.md).

### PH-S1140: CI canon gate (band 50)

Local dual-gate workflow mirroring GitHub CI: `cargo test-ci` + `poolai-openapi-gap-audit` + rust-ratio advisory.

```bash
cargo run --bin poolai-loc-audit -- --ci-canon
cargo run --bin poolai-loc-audit -- --ci-canon --advisory --min-ratio 0.95
cargo run --bin poolai-openapi-gap-audit

VERIFY_CI_CANON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --ci-canon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `ci_canon_mode` | `true` when `--ci-canon` (PH-S1140) |
| `ci_canon_criteria_total` | CI canon criteria registry size (PH-S1141) |
| `ci_canon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`ci_canon_depth.rs`](../../crates/poolai-ui-core/src/ci_canon_depth.rs) · tests: `ci_canon_audit.rs`, `galaxy_horizon_s1139_integration.rs` · docs: [`CI_CANON.md`](./CI_CANON.md).

### PH-S1150: Tenant persistence (band 51)

Enterprise phase A scaffold: `POOLAI_TENANT_STORE` + loc-audit criteria for durable tenant store horizon.

```bash
cargo run --bin poolai-loc-audit -- --tenant-persist
cargo run --bin poolai-loc-audit -- --tenant-persist --advisory --min-ratio 0.95

VERIFY_TENANT_PERSIST=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-persist
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_persist_mode` | `true` when `--tenant-persist` (PH-S1150) |
| `tenant_persist_criteria_total` | Tenant persist criteria registry size (PH-S1151) |
| `tenant_persist_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_persistence_depth.rs`](../../crates/poolai-ui-core/src/tenant_persistence_depth.rs) · tests: `tenant_persistence_audit.rs`, `galaxy_horizon_s1149_integration.rs` · docs: [`TENANT_PERSIST.md`](./TENANT_PERSIST.md) · roadmap: [`PH_S_ENTERPRISE_ROADMAP_2026-07-19.md`](./PH_S_ENTERPRISE_ROADMAP_2026-07-19.md).

### PH-S1164: Tenant store wire (band 52)

Enterprise phase A store wire: `POOLAI_TENANT_DATA_DIR` + `tenant_store_wire()` durable-path stub (no restart-safe CRUD yet).

```bash
cargo run --bin poolai-loc-audit -- --tenant-store
cargo run --bin poolai-loc-audit -- --tenant-store --advisory --min-ratio 0.95

VERIFY_TENANT_STORE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-store
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_store_mode` | `true` when `--tenant-store` (PH-S1164) |
| `tenant_store_criteria_total` | Tenant store-wire criteria registry size |
| `tenant_store_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_depth.rs`](../../crates/poolai-ui-core/src/tenant_depth.rs) · tests: `tenant_store_audit.rs`, `tenant_store_wire_integration.rs`, `galaxy_horizon_s1159_integration.rs` · docs: [`TENANT_STORE.md`](./TENANT_STORE.md).

### PH-S1176: Tenant HTTP API contracts (band 53)

Enterprise phase A HTTP contracts: CRUD / quota / isolation + `GET /tenants/store` wire read.

```bash
cargo run --bin poolai-loc-audit -- --tenant-api
cargo run --bin poolai-loc-audit -- --tenant-api --advisory --min-ratio 0.95

VERIFY_TENANT_API=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-api
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_api_mode` | `true` when `--tenant-api` (PH-S1176) |
| `tenant_api_criteria_total` | Tenant HTTP API criteria registry size |
| `tenant_api_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/tenant_api_contracts_depth.rs) · tests: `tenant_api_contracts_integration.rs`, `galaxy_horizon_s1169_integration.rs` · docs: [`TENANT_API.md`](./TENANT_API.md).

### PH-S1185: Tenant admin/ops glue (band 54)

Enterprise phase A admin UI + ops hooks: store-wire strip, usage refresh, quota probe.

```bash
cargo run --bin poolai-loc-audit -- --tenant-admin-ops
cargo run --bin poolai-loc-audit -- --tenant-admin-ops --advisory --min-ratio 0.95

VERIFY_TENANT_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-admin-ops
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_admin_ops_mode` | `true` when `--tenant-admin-ops` (PH-S1185) |
| `tenant_admin_ops_criteria_total` | Tenant admin/ops criteria registry size |
| `tenant_admin_ops_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/tenant_admin_ops_depth.rs) · tests: `tenant_admin_ops_integration.rs`, `galaxy_horizon_s1179_integration.rs` · docs: [`TENANT_ADMIN_OPS.md`](./TENANT_ADMIN_OPS.md).

### PH-S1194: Tenant live stand smoke (band 55)

Enterprise phase A live HTTP stand smoke for tenants (store / CRUD / usage+quota) plus loc-audit gate.

```bash
export POOLAI_BASE_URL=http://127.0.0.1:8080
cargo run --bin poolai-http-stand-smoke -- --tenant-stand-smoke
# or: POOLAI_STAND_SMOKE_TENANT=1

cargo run --bin poolai-loc-audit -- --tenant-stand-smoke
cargo run --bin poolai-loc-audit -- --tenant-stand-smoke --advisory --min-ratio 0.95

VERIFY_TENANT_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-stand-smoke
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_stand_smoke_mode` | `true` when `--tenant-stand-smoke` (PH-S1194) |
| `tenant_stand_smoke_criteria_total` | Tenant stand-smoke criteria registry size |
| `tenant_stand_smoke_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/tenant_stand_smoke_depth.rs) · tests: `tenant_stand_smoke_integration.rs`, `galaxy_horizon_s1189_integration.rs` · docs: [`TENANT_STAND_SMOKE.md`](./TENANT_STAND_SMOKE.md).

### PH-S1204: Tenant loc-audit aggregate (band 56)

Enterprise phase A aggregate gate for band 51–55 `--tenant-*` loc-audit slices.

```bash
cargo run --bin poolai-loc-audit -- --tenant-loc-audit
cargo run --bin poolai-loc-audit -- --tenant-loc-audit --advisory --min-ratio 0.95

VERIFY_TENANT_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-loc-audit
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_loc_audit_mode` | `true` when `--tenant-loc-audit` (PH-S1204) |
| `tenant_loc_audit_criteria_total` | Tenant loc-audit criteria registry size |
| `tenant_loc_audit_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/tenant_loc_audit_depth.rs) · tests: `tenant_loc_audit_integration.rs`, `galaxy_horizon_s1199_integration.rs` · docs: [`TENANT_LOC_AUDIT.md`](./TENANT_LOC_AUDIT.md).

### PH-S1214: Tenant docs canon (band 57)

Enterprise phase A aggregate gate for band 51–56 `TENANT_*.md` canon docs.

```bash
cargo run --bin poolai-loc-audit -- --tenant-docs-canon
cargo run --bin poolai-loc-audit -- --tenant-docs-canon --advisory --min-ratio 0.95

VERIFY_TENANT_DOCS_CANON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-docs-canon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_docs_canon_mode` | `true` when `--tenant-docs-canon` (PH-S1214) |
| `tenant_docs_canon_criteria_total` | Tenant docs-canon criteria registry size |
| `tenant_docs_canon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/tenant_docs_canon_depth.rs) · tests: `tenant_docs_canon_integration.rs`, `galaxy_horizon_s1209_integration.rs` · docs: [`TENANT_DOCS_CANON.md`](./TENANT_DOCS_CANON.md).

### PH-S1224: Tenant vision sync (band 58)

Enterprise phase A aggregate gate for `docs/vision/*` + prior `TENANT_DOCS_CANON.md`.

```bash
cargo run --bin poolai-loc-audit -- --tenant-vision-sync
cargo run --bin poolai-loc-audit -- --tenant-vision-sync --advisory --min-ratio 0.95

VERIFY_TENANT_VISION_SYNC=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-vision-sync
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_vision_sync_mode` | `true` when `--tenant-vision-sync` (PH-S1224) |
| `tenant_vision_sync_criteria_total` | Tenant vision-sync criteria registry size |
| `tenant_vision_sync_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/tenant_vision_sync_depth.rs) · tests: `tenant_vision_sync_integration.rs`, `galaxy_horizon_s1219_integration.rs` · docs: [`TENANT_VISION_SYNC.md`](./TENANT_VISION_SYNC.md).

### PH-S1234: Tenant ratio advisory (band 59)

Enterprise phase A aggregate gate for prior `--tenant-*` slices + restart-safe SQLite CRUD.

```bash
cargo run --bin poolai-loc-audit -- --tenant-ratio-advisory
cargo run --bin poolai-loc-audit -- --tenant-ratio-advisory --advisory --min-ratio 0.95

VERIFY_TENANT_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-ratio-advisory
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_ratio_advisory_mode` | `true` when `--tenant-ratio-advisory` (PH-S1234) |
| `tenant_ratio_advisory_criteria_total` | Tenant ratio-advisory criteria registry size |
| `tenant_ratio_advisory_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/tenant_ratio_advisory_depth.rs) · tests: `tenant_ratio_advisory_integration.rs`, `tenant_sqlite_durable_integration.rs`, `galaxy_horizon_s1229_integration.rs` · docs: [`TENANT_RATIO_ADVISORY.md`](./TENANT_RATIO_ADVISORY.md).

### PH-S1244: Tenant horizon close (band 60)

Enterprise phase A close gate aggregating bands 51–59 `--tenant-*` slices.

```bash
cargo run --bin poolai-loc-audit -- --tenant-horizon
cargo run --bin poolai-loc-audit -- --tenant-horizon --advisory --min-ratio 0.95

VERIFY_TENANT_HORIZON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --tenant-horizon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `tenant_horizon_mode` | `true` when `--tenant-horizon` (PH-S1244) |
| `tenant_horizon_criteria_total` | Tenant horizon criteria registry size |
| `tenant_horizon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`tenant_horizon_depth.rs`](../../crates/poolai-ui-core/src/tenant_horizon_depth.rs) · tests: `tenant_horizon_integration.rs`, `galaxy_horizon_s1239_integration.rs` · docs: [`TENANT_HORIZON.md`](./TENANT_HORIZON.md).

### PH-S1254: SSO depth scaffold (band 61)

Enterprise phase B SSO depth gate — `POOLAI_SSO_STORE` + SAML audience/NotOnOrAfter stub.

```bash
cargo run --bin poolai-loc-audit -- --sso
cargo run --bin poolai-loc-audit -- --sso --advisory --min-ratio 0.95

VERIFY_SSO=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_mode` | `true` when `--sso` (PH-S1254) |
| `sso_criteria_total` | SSO criteria registry size |
| `sso_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_depth.rs`](../../crates/poolai-ui-core/src/sso_depth.rs) · tests: `sso_depth_audit.rs`, `galaxy_horizon_s1249_integration.rs` · docs: [`SSO_DEPTH.md`](./SSO_DEPTH.md).

### PH-S1264: SSO store wire (band 62)

Enterprise phase B SSO store-wire gate — `sso_store_wire()` + `POOLAI_SSO_DATA_DIR` durable path stub.

```bash
cargo run --bin poolai-loc-audit -- --sso-store
cargo run --bin poolai-loc-audit -- --sso-store --advisory --min-ratio 0.95

VERIFY_SSO_STORE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-store
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_store_mode` | `true` when `--sso-store` (PH-S1264) |
| `sso_store_criteria_total` | SSO store-wire criteria registry size |
| `sso_store_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_store_depth.rs`](../../crates/poolai-ui-core/src/sso_store_depth.rs) · tests: `sso_store_wire_integration.rs`, `galaxy_horizon_s1259_integration.rs` · docs: [`SSO_STORE.md`](./SSO_STORE.md).

### PH-S1276: SSO HTTP API contracts (band 63)

Enterprise phase B SSO HTTP API contracts — OAuth2/SAML CRUD + `GET /security/sso/store` + callback fixtures.

```bash
cargo run --bin poolai-loc-audit -- --sso-api
cargo run --bin poolai-loc-audit -- --sso-api --advisory --min-ratio 0.95

VERIFY_SSO_API=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-api
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_api_mode` | `true` when `--sso-api` (PH-S1276) |
| `sso_api_criteria_total` | SSO HTTP API criteria registry size |
| `sso_api_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/sso_api_contracts_depth.rs) · tests: `sso_api_contracts_integration.rs`, `galaxy_horizon_s1269_integration.rs` · docs: [`SSO_API.md`](./SSO_API.md).

### PH-S1285: SSO admin/ops glue (band 64)

Admin store-wire strip + OAuth2/SAML refresh glue + verify/loc-audit hooks for phase B SSO.

```bash
cargo run --bin poolai-loc-audit -- --sso-admin-ops
cargo run --bin poolai-loc-audit -- --sso-admin-ops --advisory --min-ratio 0.95

VERIFY_SSO_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-admin-ops
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_admin_ops_mode` | `true` when `--sso-admin-ops` (PH-S1285) |
| `sso_admin_ops_criteria_total` | SSO admin/ops criteria registry size |
| `sso_admin_ops_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/sso_admin_ops_depth.rs) · tests: `sso_admin_ops_integration.rs`, `galaxy_horizon_s1279_integration.rs` · docs: [`SSO_ADMIN_OPS.md`](./SSO_ADMIN_OPS.md).

### PH-S1294: SSO stand smoke (band 65)

Live store / OAuth2+SAML CRUD / callback fixtures stand smoke + verify/loc-audit hooks for phase B SSO.

```bash
cargo run --bin poolai-http-stand-smoke -- --sso-stand-smoke
# or: POOLAI_STAND_SMOKE_SSO=1

cargo run --bin poolai-loc-audit -- --sso-stand-smoke
cargo run --bin poolai-loc-audit -- --sso-stand-smoke --advisory --min-ratio 0.95

VERIFY_SSO_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-stand-smoke
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_stand_smoke_mode` | `true` when `--sso-stand-smoke` (PH-S1294) |
| `sso_stand_smoke_criteria_total` | SSO stand-smoke criteria registry size |
| `sso_stand_smoke_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/sso_stand_smoke_depth.rs) · tests: `sso_stand_smoke_integration.rs`, `galaxy_horizon_s1289_integration.rs` · docs: [`SSO_STAND_SMOKE.md`](./SSO_STAND_SMOKE.md).

### PH-S1304: SSO loc-audit aggregate (band 66)

Aggregate gate for band 61–65 `--sso*` loc-audit slices.

```bash
cargo run --bin poolai-loc-audit -- --sso-loc-audit
cargo run --bin poolai-loc-audit -- --sso-loc-audit --advisory --min-ratio 0.95

VERIFY_SSO_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-loc-audit
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_loc_audit_mode` | `true` when `--sso-loc-audit` (PH-S1304) |
| `sso_loc_audit_criteria_total` | SSO loc-audit criteria registry size |
| `sso_loc_audit_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/sso_loc_audit_depth.rs) · tests: `sso_loc_audit_integration.rs`, `galaxy_horizon_s1299_integration.rs` · docs: [`SSO_LOC_AUDIT.md`](./SSO_LOC_AUDIT.md).

### PH-S1314: SSO docs-canon aggregate (band 67)

Docs-canon gate for band 61–66 `SSO_*.md` slice docs.

```bash
cargo run --bin poolai-loc-audit -- --sso-docs-canon
cargo run --bin poolai-loc-audit -- --sso-docs-canon --advisory --min-ratio 0.95

VERIFY_SSO_DOCS_CANON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-docs-canon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_docs_canon_mode` | `true` when `--sso-docs-canon` (PH-S1314) |
| `sso_docs_canon_criteria_total` | SSO docs-canon criteria registry size |
| `sso_docs_canon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/sso_docs_canon_depth.rs) · tests: `sso_docs_canon_integration.rs`, `galaxy_horizon_s1309_integration.rs` · docs: [`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md).

### PH-S1324: SSO vision-sync aggregate (band 68)

Vision-sync gate for `docs/vision/*` + prior [`SSO_DOCS_CANON.md`](./SSO_DOCS_CANON.md).

```bash
cargo run --bin poolai-loc-audit -- --sso-vision-sync
cargo run --bin poolai-loc-audit -- --sso-vision-sync --advisory --min-ratio 0.95

VERIFY_SSO_VISION_SYNC=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-vision-sync
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_vision_sync_mode` | `true` when `--sso-vision-sync` (PH-S1324) |
| `sso_vision_sync_criteria_total` | SSO vision-sync criteria registry size |
| `sso_vision_sync_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/sso_vision_sync_depth.rs) · tests: `sso_vision_sync_integration.rs`, `galaxy_horizon_s1319_integration.rs` · docs: [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md).

### PH-S1334: SSO ratio-advisory (band 69)

Ratio-advisory gate aggregating prior `--sso*` + [`SSO_VISION_SYNC.md`](./SSO_VISION_SYNC.md).

```bash
cargo run --bin poolai-loc-audit -- --sso-ratio-advisory
cargo run --bin poolai-loc-audit -- --sso-ratio-advisory --advisory --min-ratio 0.95

VERIFY_SSO_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-ratio-advisory
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_ratio_advisory_mode` | `true` when `--sso-ratio-advisory` (PH-S1334) |
| `sso_ratio_advisory_criteria_total` | SSO ratio-advisory criteria registry size |
| `sso_ratio_advisory_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/sso_ratio_advisory_depth.rs) · tests: `sso_ratio_advisory_integration.rs`, `galaxy_horizon_s1329_integration.rs` · docs: [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md).

### PH-S1344: SSO horizon close (band 70)

Horizon gate aggregating phase-B `--sso*` + [`SSO_RATIO_ADVISORY.md`](./SSO_RATIO_ADVISORY.md).

```bash
cargo run --bin poolai-loc-audit -- --sso-horizon
cargo run --bin poolai-loc-audit -- --sso-horizon --advisory --min-ratio 0.95

VERIFY_SSO_HORIZON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --sso-horizon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `sso_horizon_mode` | `true` when `--sso-horizon` (PH-S1344) |
| `sso_horizon_criteria_total` | SSO horizon criteria registry size |
| `sso_horizon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`sso_horizon_depth.rs`](../../crates/poolai-ui-core/src/sso_horizon_depth.rs) · tests: `sso_horizon_integration.rs`, `galaxy_horizon_s1339_integration.rs` · docs: [`SSO_HORIZON.md`](./SSO_HORIZON.md).

### PH-S1354: Audit depth scaffold (band 71)

Enterprise phase C audit depth gate — `POOLAI_AUDIT_STORE` + event metadata stub.

```bash
cargo run --bin poolai-loc-audit -- --audit
cargo run --bin poolai-loc-audit -- --audit --advisory --min-ratio 0.95

VERIFY_AUDIT=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_mode` | `true` when `--audit` (PH-S1354) |
| `audit_criteria_total` | Audit criteria registry size |
| `audit_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_depth.rs`](../../crates/poolai-ui-core/src/audit_depth.rs) · tests: `audit_depth_audit.rs`, `galaxy_horizon_s1349_integration.rs` · docs: [`AUDIT_DEPTH.md`](./AUDIT_DEPTH.md).

### PH-S1364: Audit store wire (band 72)

Enterprise phase C audit store-wire gate — durable path stub via `POOLAI_AUDIT_DATA_DIR`.

```bash
cargo run --bin poolai-loc-audit -- --audit-store
cargo run --bin poolai-loc-audit -- --audit-store --advisory --min-ratio 0.95

VERIFY_AUDIT_STORE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-store
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_store_mode` | `true` when `--audit-store` (PH-S1364) |
| `audit_store_criteria_total` | Audit store criteria registry size |
| `audit_store_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_store_depth.rs`](../../crates/poolai-ui-core/src/audit_store_depth.rs) · tests: `audit_store_wire_integration.rs`, `galaxy_horizon_s1359_integration.rs` · docs: [`AUDIT_STORE.md`](./AUDIT_STORE.md).

### PH-S1375: Audit HTTP API contracts (band 73)

Enterprise phase C audit HTTP API contracts gate — query + store-wire + field fixtures.

```bash
cargo run --bin poolai-loc-audit -- --audit-api
cargo run --bin poolai-loc-audit -- --audit-api --advisory --min-ratio 0.95

VERIFY_AUDIT_API=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-api
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_api_mode` | `true` when `--audit-api` (PH-S1375) |
| `audit_api_criteria_total` | Audit API criteria registry size |
| `audit_api_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/audit_api_contracts_depth.rs) · tests: `audit_api_contracts_integration.rs`, `galaxy_horizon_s1369_integration.rs` · docs: [`AUDIT_API.md`](./AUDIT_API.md).

### PH-S1384 / PH-S1385: Audit admin/ops glue (band 74)

```bash
cargo run --bin poolai-loc-audit -- --audit-admin-ops
cargo run --bin poolai-loc-audit -- --audit-admin-ops --advisory --min-ratio 0.95

VERIFY_AUDIT_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-admin-ops
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_admin_ops_mode` | `true` when `--audit-admin-ops` (PH-S1385) |
| `audit_admin_ops_criteria_total` | Audit admin/ops criteria registry size |
| `audit_admin_ops_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/audit_admin_ops_depth.rs) · tests: `audit_admin_ops_integration.rs`, `galaxy_horizon_s1379_integration.rs` · docs: [`AUDIT_ADMIN_OPS.md`](./AUDIT_ADMIN_OPS.md).

### PH-S1394: Audit stand smoke (band 75)

Live store / events query / event-field validation fixtures stand smoke + verify/loc-audit hooks for phase C Audit.

```bash
cargo run --bin poolai-http-stand-smoke -- --audit-stand-smoke
# or: POOLAI_STAND_SMOKE_AUDIT=1

cargo run --bin poolai-loc-audit -- --audit-stand-smoke
cargo run --bin poolai-loc-audit -- --audit-stand-smoke --advisory --min-ratio 0.95

VERIFY_AUDIT_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-stand-smoke
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_stand_smoke_mode` | `true` when `--audit-stand-smoke` (PH-S1394) |
| `audit_stand_smoke_criteria_total` | Audit stand-smoke criteria registry size |
| `audit_stand_smoke_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/audit_stand_smoke_depth.rs) · tests: `audit_stand_smoke_integration.rs`, `galaxy_horizon_s1389_integration.rs` · docs: [`AUDIT_STAND_SMOKE.md`](./AUDIT_STAND_SMOKE.md).

### PH-S1404: Audit loc-audit aggregate (band 76)

Aggregate gate for band 71–75 `--audit*` loc-audit slices.

```bash
cargo run --bin poolai-loc-audit -- --audit-loc-audit
cargo run --bin poolai-loc-audit -- --audit-loc-audit --advisory --min-ratio 0.95

VERIFY_AUDIT_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-loc-audit
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_loc_audit_mode` | `true` when `--audit-loc-audit` (PH-S1404) |
| `audit_loc_audit_criteria_total` | Registry size (10) |
| `audit_loc_audit_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/audit_loc_audit_depth.rs) · tests: `audit_loc_audit_integration.rs`, `galaxy_horizon_s1399_integration.rs` · docs: [`AUDIT_LOC_AUDIT.md`](./AUDIT_LOC_AUDIT.md).

### PH-S1414: Audit docs canon (band 77)

Docs-canon matrix gate for band 71–76 `AUDIT_*.md` slices.

```bash
cargo run --bin poolai-loc-audit -- --audit-docs-canon
cargo run --bin poolai-loc-audit -- --audit-docs-canon --advisory --min-ratio 0.95

VERIFY_AUDIT_DOCS_CANON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-docs-canon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_docs_canon_mode` | `true` when `--audit-docs-canon` (PH-S1414) |
| `audit_docs_canon_criteria_total` | Registry size (10) |
| `audit_docs_canon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/audit_docs_canon_depth.rs) · tests: `audit_docs_canon_integration.rs`, `galaxy_horizon_s1409_integration.rs` · docs: [`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md).

### PH-S1424: Audit vision sync (band 78)

Vision-sync matrix gate for `docs/vision/*` + prior [`AUDIT_DOCS_CANON.md`](./AUDIT_DOCS_CANON.md).

```bash
cargo run --bin poolai-loc-audit -- --audit-vision-sync
cargo run --bin poolai-loc-audit -- --audit-vision-sync --advisory --min-ratio 0.95

VERIFY_AUDIT_VISION_SYNC=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-vision-sync
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_vision_sync_mode` | `true` when `--audit-vision-sync` (PH-S1424) |
| `audit_vision_sync_criteria_total` | Registry size (10) |
| `audit_vision_sync_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/audit_vision_sync_depth.rs) · tests: `audit_vision_sync_integration.rs`, `galaxy_horizon_s1419_integration.rs` · docs: [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md).

### PH-S1434: Audit ratio advisory (band 79)

Ratio-advisory gate aggregating prior `--audit*` + [`AUDIT_VISION_SYNC.md`](./AUDIT_VISION_SYNC.md).

```bash
cargo run --bin poolai-loc-audit -- --audit-ratio-advisory
cargo run --bin poolai-loc-audit -- --audit-ratio-advisory --advisory --min-ratio 0.95

VERIFY_AUDIT_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-ratio-advisory
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_ratio_advisory_mode` | `true` when `--audit-ratio-advisory` (PH-S1434) |
| `audit_ratio_advisory_criteria_total` | Registry size (10) |
| `audit_ratio_advisory_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/audit_ratio_advisory_depth.rs) · tests: `audit_ratio_advisory_integration.rs`, `galaxy_horizon_s1429_integration.rs` · docs: [`AUDIT_RATIO_ADVISORY.md`](./AUDIT_RATIO_ADVISORY.md).

### PH-S1444: Audit horizon close (band 80)

```bash
cargo run --bin poolai-loc-audit -- --audit-horizon
cargo run --bin poolai-loc-audit -- --audit-horizon --advisory --min-ratio 0.95

VERIFY_AUDIT_HORIZON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --audit-horizon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `audit_horizon_mode` | `true` when `--audit-horizon` (PH-S1444) |
| `audit_horizon_criteria_total` | Registry size (10) |
| `audit_horizon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`audit_horizon_depth.rs`](../../crates/poolai-ui-core/src/audit_horizon_depth.rs) · tests: `audit_horizon_integration.rs`, `galaxy_horizon_s1439_integration.rs` · docs: [`AUDIT_HORIZON.md`](./AUDIT_HORIZON.md).

### PH-S1454: Policies depth scaffold (band 81)

```bash
cargo run --bin poolai-loc-audit -- --policy
cargo run --bin poolai-loc-audit -- --policy --advisory --min-ratio 0.95

VERIFY_POLICY=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_mode` | `true` when `--policy` (PH-S1454) |
| `policy_criteria_total` | Registry size (8) |
| `policy_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_depth.rs`](../../crates/poolai-ui-core/src/policy_depth.rs) · tests: `policy_depth_audit.rs`, `galaxy_horizon_s1449_integration.rs` · docs: [`POLICIES_DEPTH.md`](./POLICIES_DEPTH.md).

### PH-S1464: Policies store wire (band 82)

```bash
cargo run --bin poolai-loc-audit -- --policy-store
cargo run --bin poolai-loc-audit -- --policy-store --advisory --min-ratio 0.95

VERIFY_POLICY_STORE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-store
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_store_mode` | `true` when `--policy-store` (PH-S1464) |
| `policy_store_criteria_total` | Registry size (7) |
| `policy_store_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_store_depth.rs`](../../crates/poolai-ui-core/src/policy_store_depth.rs) · tests: `policy_store_wire_integration.rs`, `galaxy_horizon_s1459_integration.rs` · docs: [`POLICIES_STORE.md`](./POLICIES_STORE.md).

### PH-S1475: Policies HTTP API contracts (band 83)

```bash
cargo run --bin poolai-loc-audit -- --policy-api
cargo run --bin poolai-loc-audit -- --policy-api --advisory --min-ratio 0.95

VERIFY_POLICY_API=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-api
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_api_mode` | `true` when `--policy-api` (PH-S1475) |
| `policy_api_criteria_total` | Registry size (9) |
| `policy_api_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/policy_api_contracts_depth.rs) · tests: `policy_api_contracts_integration.rs`, `galaxy_horizon_s1469_integration.rs` · docs: [`POLICIES_API.md`](./POLICIES_API.md).

### PH-S1485: Policies admin/ops glue (band 84)

```bash
cargo run --bin poolai-loc-audit -- --policy-admin-ops
cargo run --bin poolai-loc-audit -- --policy-admin-ops --advisory --min-ratio 0.95

VERIFY_POLICY_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-admin-ops
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_admin_ops_mode` | `true` when `--policy-admin-ops` (PH-S1485) |
| `policy_admin_ops_criteria_total` | Registry size (10) |
| `policy_admin_ops_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/policy_admin_ops_depth.rs) · tests: `policy_admin_ops_integration.rs`, `galaxy_horizon_s1479_integration.rs` · docs: [`POLICIES_ADMIN_OPS.md`](./POLICIES_ADMIN_OPS.md).

### PH-S1495: Policies stand smoke (band 85)

```bash
export POOLAI_BASE_URL=http://127.0.0.1:8080
cargo run --bin poolai-http-stand-smoke -- --policy-stand-smoke
cargo run --bin poolai-loc-audit -- --policy-stand-smoke
cargo run --bin poolai-loc-audit -- --policy-stand-smoke --advisory --min-ratio 0.95

VERIFY_POLICY_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-stand-smoke
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_stand_smoke_mode` | `true` when `--policy-stand-smoke` (PH-S1494) |
| `policy_stand_smoke_criteria_total` | Registry size (10) |
| `policy_stand_smoke_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/policy_stand_smoke_depth.rs) · tests: `policy_stand_smoke_integration.rs`, `galaxy_horizon_s1489_integration.rs` · docs: [`POLICIES_STAND_SMOKE.md`](./POLICIES_STAND_SMOKE.md).

### PH-S1504: Policies loc-audit aggregate (band 86)

Aggregate gate for band 81–85 `--policy*` loc-audit slices.

```bash
cargo run --bin poolai-loc-audit -- --policy-loc-audit
cargo run --bin poolai-loc-audit -- --policy-loc-audit --migration-advisory --advisory --min-ratio 0.95

VERIFY_POLICY_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-loc-audit
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_loc_audit_mode` | `true` when `--policy-loc-audit` (PH-S1504) |
| `policy_loc_audit_criteria_total` | Registry size (10) |
| `policy_loc_audit_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/policy_loc_audit_depth.rs) · tests: `policy_loc_audit_integration.rs`, `galaxy_horizon_s1499_integration.rs` · docs: [`POLICIES_LOC_AUDIT.md`](./POLICIES_LOC_AUDIT.md).

### PH-S1514: Policies docs-canon aggregate (band 87)

Aggregate gate for band 81–86 `POLICIES_*.md` canon docs.

```bash
cargo run --bin poolai-loc-audit -- --policy-docs-canon
cargo run --bin poolai-loc-audit -- --policy-docs-canon --migration-advisory --advisory --min-ratio 0.95

VERIFY_POLICY_DOCS_CANON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-docs-canon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_docs_canon_mode` | `true` when `--policy-docs-canon` (PH-S1514) |
| `policy_docs_canon_criteria_total` | Registry size (10) |
| `policy_docs_canon_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/policy_docs_canon_depth.rs) · tests: `policy_docs_canon_integration.rs`, `galaxy_horizon_s1509_integration.rs` · docs: [`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md).

### PH-S1524: Policies vision-sync aggregate (band 88)

Aggregate gate for `docs/vision/*` + prior [`POLICIES_DOCS_CANON.md`](./POLICIES_DOCS_CANON.md).

```bash
cargo run --bin poolai-loc-audit -- --policy-vision-sync
cargo run --bin poolai-loc-audit -- --policy-vision-sync --migration-advisory --advisory --min-ratio 0.95

VERIFY_POLICY_VISION_SYNC=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-vision-sync
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_vision_sync_mode` | `true` when `--policy-vision-sync` (PH-S1524) |
| `policy_vision_sync_criteria_total` | Registry size (10) |
| `policy_vision_sync_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/policy_vision_sync_depth.rs) · tests: `policy_vision_sync_integration.rs`, `galaxy_horizon_s1519_integration.rs` · docs: [`POLICIES_VISION_SYNC.md`](./POLICIES_VISION_SYNC.md).

### PH-S1534: Policies ratio-advisory aggregate (band 89)

Aggregate gate for prior `--policy*` slices + [`POLICIES_VISION_SYNC.md`](./POLICIES_VISION_SYNC.md).

```bash
cargo run --bin poolai-loc-audit -- --policy-ratio-advisory
cargo run --bin poolai-loc-audit -- --policy-ratio-advisory --migration-advisory --advisory --min-ratio 0.95

VERIFY_POLICY_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --policy-ratio-advisory
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `policy_ratio_advisory_mode` | `true` when `--policy-ratio-advisory` (PH-S1534) |
| `policy_ratio_advisory_criteria_total` | Registry size (10) |
| `policy_ratio_advisory_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`policy_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/policy_ratio_advisory_depth.rs) · tests: `policy_ratio_advisory_integration.rs`, `galaxy_horizon_s1529_integration.rs` · docs: [`POLICIES_RATIO_ADVISORY.md`](./POLICIES_RATIO_ADVISORY.md).

### PH-S1554: Monitoring depth (band 91)

Enterprise phase E monitoring depth scaffold — `POOLAI_MONITORING_DATA_DIR` + alert field stub.

```bash
cargo run --bin poolai-loc-audit -- --monitoring
cargo run --bin poolai-loc-audit -- --monitoring --advisory --min-ratio 0.95

VERIFY_MONITORING=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_mode` | `true` when `--monitoring` (PH-S1554) |
| `monitoring_criteria_total` | Registry size (8) |
| `monitoring_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`monitoring_depth.rs`](../../crates/poolai-ui-core/src/monitoring_depth.rs) · tests: `monitoring_depth_audit.rs`, `galaxy_horizon_s1549_integration.rs` · docs: [`MONITORING_DEPTH.md`](./MONITORING_DEPTH.md).

### PH-S1564: Monitoring store wire (band 92)

Enterprise phase E monitoring store-wire gate — durable path wire via `POOLAI_MONITORING_STORE` + `POOLAI_MONITORING_DATA_DIR`.

```bash
cargo run --bin poolai-loc-audit -- --monitoring-store
cargo run --bin poolai-loc-audit -- --monitoring-store --advisory --min-ratio 0.95

VERIFY_MONITORING_STORE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-store
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_store_mode` | `true` when `--monitoring-store` (PH-S1564) |
| `monitoring_store_criteria_total` | Registry size (7) |
| `monitoring_store_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`monitoring_store_depth.rs`](../../crates/poolai-ui-core/src/monitoring_store_depth.rs) · tests: `monitoring_store_wire_integration.rs`, `galaxy_horizon_s1559_integration.rs` · docs: [`MONITORING_STORE.md`](./MONITORING_STORE.md).

### PH-S1575: Monitoring HTTP API contracts (band 93)

Enterprise phase E monitoring HTTP API contracts — query lifecycle, `GET /monitoring/store`, alert-rule field fixtures.

```bash
cargo run --bin poolai-loc-audit -- --monitoring-api
cargo run --bin poolai-loc-audit -- --monitoring-api --advisory --min-ratio 0.95

VERIFY_MONITORING_API=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-api
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_api_mode` | `true` when `--monitoring-api` (PH-S1575) |
| `monitoring_api_criteria_total` | Registry size (9) |
| `monitoring_api_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`monitoring_api_contracts_depth.rs`](../../crates/poolai-ui-core/src/monitoring_api_contracts_depth.rs) · tests: `monitoring_api_contracts_integration.rs`, `galaxy_horizon_s1569_integration.rs` · docs: [`MONITORING_API.md`](./MONITORING_API.md).

### PH-S1585: Monitoring admin/ops glue (band 94)

Enterprise phase E monitoring admin/ops glue — `#monitoring-store-badge`, `refreshMonitoring`, verify/loc-audit hooks.

```bash
cargo run --bin poolai-loc-audit -- --monitoring-admin-ops
cargo run --bin poolai-loc-audit -- --monitoring-admin-ops --advisory --min-ratio 0.95

VERIFY_MONITORING_ADMIN_OPS=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-admin-ops
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_admin_ops_mode` | `true` when `--monitoring-admin-ops` (PH-S1585) |
| `monitoring_admin_ops_criteria_total` | Registry size (10) |
| `monitoring_admin_ops_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`monitoring_admin_ops_depth.rs`](../../crates/poolai-ui-core/src/monitoring_admin_ops_depth.rs) · tests: `monitoring_admin_ops_integration.rs`, `galaxy_horizon_s1579_integration.rs` · docs: [`MONITORING_ADMIN_OPS.md`](./MONITORING_ADMIN_OPS.md).

### Monitoring stand smoke (band 95, PH-S1589…S1598)

Enterprise phase E monitoring live stand smoke — store wire, alerts query, alert-rules validate fixtures.

```bash
cargo run --bin poolai-http-stand-smoke -- --monitoring-stand-smoke
cargo run --bin poolai-loc-audit -- --monitoring-stand-smoke
cargo run --bin poolai-loc-audit -- --monitoring-stand-smoke --advisory --min-ratio 0.95
VERIFY_MONITORING_STAND_SMOKE=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-stand-smoke
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_stand_smoke_mode` | `true` when `--monitoring-stand-smoke` (PH-S1594) |
| `monitoring_stand_smoke_criteria_total` | Registry size (10) |
| `monitoring_stand_smoke_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`monitoring_stand_smoke_depth.rs`](../../crates/poolai-ui-core/src/monitoring_stand_smoke_depth.rs) · tests: `monitoring_stand_smoke_integration.rs`, `galaxy_horizon_s1589_integration.rs` · docs: [`MONITORING_STAND_SMOKE.md`](./MONITORING_STAND_SMOKE.md).

### Monitoring loc-audit aggregate (band 96, PH-S1599…S1608)

Enterprise phase E monitoring loc-audit aggregate — consolidates band 91–95 `--monitoring*` slices.

```bash
cargo run --bin poolai-loc-audit -- --monitoring-loc-audit
cargo run --bin poolai-loc-audit -- --monitoring-loc-audit --advisory --min-ratio 0.95
VERIFY_MONITORING_LOC_AUDIT=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-loc-audit
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_loc_audit_mode` | `true` when `--monitoring-loc-audit` (PH-S1604) |
| `monitoring_loc_audit_criteria_total` | Registry size (10) |
| `monitoring_loc_audit_criteria_met_count` | Criteria with marker present in canonical doc path |

Module: [`monitoring_loc_audit_depth.rs`](../../crates/poolai-ui-core/src/monitoring_loc_audit_depth.rs) · tests: `monitoring_loc_audit_integration.rs`, `galaxy_horizon_s1599_integration.rs` · docs: [`MONITORING_LOC_AUDIT.md`](./MONITORING_LOC_AUDIT.md).

Default stand smoke includes **`vision_revision_parity`** (PH-S208, PH-S235): repo `manifest.revision` vs FM §5.12 `Vision rev`, `extensions.active_sprint` vs `manifest.next_sprint`, then `GET /docs/vision/manifest.json` with `X-PoolAI-Vision-Revision` header vs JSON body.

### Monitoring docs-canon (band 97, PH-S1609…S1618)

Enterprise phase E monitoring docs-canon — consolidates band 91–95 `MONITORING_*.md` slices.

```bash
cargo run --bin poolai-loc-audit -- --monitoring-docs-canon
cargo run --bin poolai-loc-audit -- --monitoring-docs-canon --advisory --min-ratio 0.95
VERIFY_MONITORING_DOCS_CANON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-docs-canon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_docs_canon_mode` | `true` when `--monitoring-docs-canon` (PH-S1614) |
| `monitoring_docs_canon_criteria_total` | Registry size (10) |
| `monitoring_docs_canon_criteria_met_count` | Criteria with marker present in canonical doc paths |

Module: [`monitoring_docs_canon_depth.rs`](../../crates/poolai-ui-core/src/monitoring_docs_canon_depth.rs) · tests: `monitoring_docs_canon_integration.rs`, `galaxy_horizon_s1609_integration.rs` · docs: [`MONITORING_DOCS_CANON.md`](./MONITORING_DOCS_CANON.md).

### Monitoring vision-sync (band 98, PH-S1619…S1628)

Enterprise phase E monitoring vision-sync — consolidates `docs/vision/*` + prior [`MONITORING_DOCS_CANON.md`](./MONITORING_DOCS_CANON.md).

```bash
cargo run --bin poolai-loc-audit -- --monitoring-vision-sync
cargo run --bin poolai-loc-audit -- --monitoring-vision-sync --migration-advisory --advisory --min-ratio 0.95
VERIFY_MONITORING_VISION_SYNC=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-vision-sync
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_vision_sync_mode` | `true` when `--monitoring-vision-sync` (PH-S1624) |
| `monitoring_vision_sync_criteria_total` | Registry size (10) |
| `monitoring_vision_sync_criteria_met_count` | Criteria with marker present in canonical doc paths |

Module: [`monitoring_vision_sync_depth.rs`](../../crates/poolai-ui-core/src/monitoring_vision_sync_depth.rs) · tests: `monitoring_vision_sync_integration.rs`, `galaxy_horizon_s1619_integration.rs` · docs: [`MONITORING_VISION_SYNC.md`](./MONITORING_VISION_SYNC.md).

### Monitoring ratio-advisory (band 99, PH-S1629…S1638)

Enterprise phase E monitoring ratio-advisory — consolidates the loc-audit ratio hold floor (`rust_ratio.json` + `--min-ratio`) with the ratio strategy and prior [`MONITORING_VISION_SYNC.md`](./MONITORING_VISION_SYNC.md).

```bash
cargo run --bin poolai-loc-audit -- --monitoring-ratio-advisory
cargo run --bin poolai-loc-audit -- --monitoring-ratio-advisory --migration-advisory --advisory --min-ratio 0.95
VERIFY_MONITORING_RATIO_ADVISORY=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-ratio-advisory
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_ratio_advisory_mode` | `true` when `--monitoring-ratio-advisory` (PH-S1634) |
| `monitoring_ratio_advisory_criteria_total` | Registry size (10) |
| `monitoring_ratio_advisory_criteria_met_count` | Criteria with marker present in canonical doc paths |

Module: [`monitoring_ratio_advisory_depth.rs`](../../crates/poolai-ui-core/src/monitoring_ratio_advisory_depth.rs) · tests: `monitoring_ratio_advisory_integration.rs`, `galaxy_horizon_s1629_integration.rs` · docs: [`MONITORING_RATIO_ADVISORY.md`](./MONITORING_RATIO_ADVISORY.md).

---

## Monitoring horizon close (PH-S1642 / band 100)

```bash
VERIFY_MONITORING_HORIZON=1 bash bin/verify-dev-stand.sh
/usr/bin/bash bin/run-poolai.sh quick --monitoring-horizon
```

| Field (`rust_ratio.json`) | Призначення |
|---------------------------|-------------|
| `monitoring_horizon_mode` | `true` when `--monitoring-horizon` (PH-S1644) |
| `monitoring_horizon_criteria_total` | Registry size (10) |
| `monitoring_horizon_criteria_met_count` | Criteria with marker present in canonical doc paths |

Module: [`monitoring_horizon_depth.rs`](../../crates/poolai-ui-core/src/monitoring_horizon_depth.rs) · tests: `monitoring_horizon_integration.rs`, `galaxy_horizon_s1639_integration.rs` · docs: [`MONITORING_HORIZON.md`](./MONITORING_HORIZON.md).

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
