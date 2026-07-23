# PoolAI - AI Mining Pool Management System

> 🇺🇦 Українськомовні матеріали: концепція та індекс у [`docs/concept/`](docs/concept/) та [`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md) (окремого `README.uk.md` у корені немає).

PoolAI is a comprehensive distributed system for managing AI mining pools with integration of generative models, GPU optimization, and automated resource management.

## Galaxy docs vision (інтерактивна карта доків)

Після серії спринтів **Galaxy Grid** (концепт у [`docs/concept/POOLAI_GALAXY_GRID.md`](docs/concept/POOLAI_GALAXY_GRID.md)) у репо з’явилася **жива карта**: як пов’язані концепт, HANDOFF, FM і код (`galaxy_fee_split`, virtual nodes тощо).

| Що | Де |
|----|-----|
| **Запуск** | PowerShell: `.\bin\open-docs-vision.ps1` · MSYS2: `/usr/bin/bash bin/open-docs-vision.sh` |
| **URL** | `http://127.0.0.1:8765/docs/vision/index.html` (Cursor Simple Browser — лише localhost, не `S:/…`) |
| **Панелі** | 3D-шари **L0–L5** (concept → workspace TOML) · Galaxy map · граф зв’язків · preview |
| **Map UX** | pan/zoom · **Eco** (GPU save) · **Layers/Types** filters · **⊟ Folders** · **◎ Sprint** |
| **Auto-reload** | **Auto** (1.5 s eco / 4 s): manifest без F5 · **Reload** → `__sync` нових файлів |

Деталі: [`docs/vision/README.md`](docs/vision/README.md) · правило агента [`.cursor/rules/docs-vision.mdc`](.cursor/rules/docs-vision.mdc). Статична схема: [`docs/vision/vision.svg`](docs/vision/vision.svg).

## Documentation map (canonical order)

Один порядок у всіх точках входу: [`docs/README.md`](docs/README.md), [`docs/status/STABLE_STATE_SUMMARY.md`](docs/status/STABLE_STATE_SUMMARY.md), [`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md), [`docs/development/HANDOFF_NEW_SESSION.md`](docs/development/HANDOFF_NEW_SESSION.md), [`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md), [`docs/catalog/FUNCTION_MANAGEMENT.md`](docs/catalog/FUNCTION_MANAGEMENT.md).

**Таксономія каталогу `docs/`** — [`docs/STRUCTURE.md`](docs/STRUCTURE.md). **Правила агента для доків** — [`.cursor/rules/documentation.md`](.cursor/rules/documentation.md).

### Technology stack

| Layer | Stack |
|-------|--------|
| **Product (target 90–95%)** | **Rust** — `src/`, `tests/`, `crates/`, `src/bin/` |
| **Admin UI** | HTML + JS glue; **wasm32** shared logic (horizon) — `src/ui/` |
| **Browser regression only** | Playwright — `e2e/` (smoke, admin, axe, visual) |
| **Ops** | `bin/` (launch/LAN/verify), `scripts/` (toolchain), MSYS2 |

**Strategy:** [`docs/development/RUST_RATIO_STRATEGY_2026-06-13.md`](docs/development/RUST_RATIO_STRATEGY_2026-06-13.md) · testing [`.cursor/rules/poolai-testing-policy.mdc`](.cursor/rules/poolai-testing-policy.mdc).

**No Python** in the repository (0× `.py`; ML/TurboQuant — `src/ml/` on Rust). OpenAPI route audit: `cargo run --bin poolai-openapi-gap-audit`. Cursor agents: [`.cursor/rules/runtime-stack-policy.mdc`](.cursor/rules/runtime-stack-policy.mdc).

1. **Кореневий [`README.md`](README.md)** (цей файл) — швидкий старт, збірка, CI, посилання нижче.
2. **[`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md)** — карта всього каталогу `docs/` (концепція, статус, ML, cloud, troubleshooting).
3. **[`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md)** — план Rust Architect (P1–P6, TurboQuant, узгодження з CI).
4. **[`docs/development/HANDOFF_NEW_SESSION.md`](docs/development/HANDOFF_NEW_SESSION.md)** — старт **нової сесії**: гілка `main`, порядок доків, git-push, зріз зробленого, наступні кроки.
5. **Концепція** — [`docs/concept/poolAI_concept_root.txt`](docs/concept/poolAI_concept_root.txt); Grid / Memory / Job: [`POOLAI_GRID_NODE.md`](docs/concept/POOLAI_GRID_NODE.md), **[`POOLAI_GALAXY_GRID.md`](docs/concept/POOLAI_GALAXY_GRID.md)** (федеративна мережа), [`POOLAI_MEMORY_LAYER.md`](docs/concept/POOLAI_MEMORY_LAYER.md), [`JOB_LAYER_CONCEPT_2026-03-17.md`](docs/development/JOB_LAYER_CONCEPT_2026-03-17.md), [`GRID_PROTOCOL_CONCEPT_2026-04-06.md`](docs/development/GRID_PROTOCOL_CONCEPT_2026-04-06.md), [`SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](docs/development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md). **Інтерактивна карта:** [Galaxy docs vision](#galaxy-docs-vision-інтерактивна-карта-доків) → [`docs/vision/index.html`](docs/vision/index.html).
6. **Архітектура** — [`docs/ARCHITECTURE_REVIEW.md`](docs/ARCHITECTURE_REVIEW.md), [`docs/ARCHITECTURE_BEST_PRACTICES.md`](docs/ARCHITECTURE_BEST_PRACTICES.md).
7. **Продуктивність** — [`docs/performance/BENCHMARKS.md`](docs/performance/BENCHMARKS.md), [`docs/performance/PROFILING.md`](docs/performance/PROFILING.md); опційні прогони Criterion: [`.github/workflows/benchmarks.yml`](.github/workflows/benchmarks.yml); HTTP health load — in-tree **`poolai_health_load`** (опційно **`--json`** на stdout для baseline; див. `BENCHMARKS.md`).
8. **CI** — обов’язкові перевірки: [`.github/workflows/ci.yml`](.github/workflows/ci.yml).
9. **Інвентар** — [`file_list.csv`](file_list.csv) (ручний зріз; оновлюй після змін у `src/services/`, `src/network/`, `.github/workflows/`, `.cursor/`, `docs/catalog/`, **`docs/vision/`**); повний список: `git ls-files`.
10. **Git push (Windows)** — [`.cursor/commands/git-push.md`](.cursor/commands/git-push.md) (MSYS2 bash, PATH, змінні для cloud-sdk).
11. **Витяг функціоналу** — [`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) (зведення можливостей за доками та кодом; OpenAPI може бути неповним).
12. **Керування функціоналом** — [`docs/catalog/FUNCTION_MANAGEMENT.md`](docs/catalog/FUNCTION_MANAGEMENT.md) (індекс vs сталевий стан, прогалини, чернетки тікетів `FM-*`); правило агента — [`.cursor/rules/functionality-management.mdc`](.cursor/rules/functionality-management.mdc).

## Release and status

**Версія в репозиторії:** `0.2.2` (див. `Cargo.toml`). **Робоча гілка:** `main`.

Зрілий **MVP і модулі Stage 1–3** здані й покриті тестами; **Galaxy Grid wire:** PH-S65…S1298 ✅ (phase B SSO admin/ops glue). **Rust ratio:** **94.82%** (hold **95%** advisory). **Vision:** manifest rev **358**. **§5.12:** **10** відкритих (band 64 ✅) · наступна сесія **`абракадабра`** → band 66 — [`NEXT_SESSION_PROMPT.md`](docs/development/NEXT_SESSION_PROMPT.md).

**Репозиторій:** [github.com/platinoff/poolAI](https://github.com/platinoff/poolAI)

### Останні релізні нотатки (скорочено)

**v0.2.2 (2026-05)** — **FM-016** virtual nodes: `poolai-worker`, `poolai-telegram-bot` (`tgbot`), discovery/register-remote, Telegram bind/webhook, pool join, `raid_artifact_probe`, `bin/verify-dev-stand.*`; **OpenAPI** S14–S20 (v1 + enterprise REST); **FM-012** OAuth/Telegram; **FM-019** pa11y baseline (18 auth URLs, `a11y.yml`). **2026-04-10:** Clippy `-D warnings` по CI-матрицях. **2026-04:** Cloud LB routing rules; service layer + **FM-005** JSON errors.

**v0.2.1** — Cloud auto-scaling (metrics API, `evaluate_and_scale`, `ScalingAction`); pre-push `cargo fmt --all --check`; правила `.cursor/rules/`, MSYS2.

Наступні кроки: [Next Focus](#next-focus) та [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md). Довгий нумерований чеклист архітектурних пунктів 2025 року прибрано з README як дубль — вони відображені в коді, CI та `docs/status/`.

---

## 🎯 Development Status
**Current Phase**: Stage 4.3 Cloud Integration (SDK + operator work continues) and **Stage 4.4 AI/ML** (pipeline orchestration, enterprise HTTP API, versioning/experiments — active development)  
**Target**: Advanced AI Mining Pool with Enterprise Features and Cloud/ML optimization  
For a detailed status view see `docs/status/STABLE_STATE_SUMMARY.md`. Documentation entry points: [Documentation map](#documentation-map-canonical-order), [`docs/README.md`](docs/README.md), [`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md).

### ✅ Current Build/Test Status (2026-04-10)
- `cargo fmt --all` — CI / before push  
- **Required in CI** (`.github/workflows/ci.yml`): `cargo test --lib --tests --features ml,enterprise,cloud,test-utils` with `K8S_OPENAPI_ENABLED_VERSION=1.28` — **passing** (верифікація включно з `-j 1` та `--test-threads=1` на Windows при обмеженій RAM / OOM лінкера).  
- **`cargo clippy` з `-D warnings`** узгоджено з матрицями CI (`.github/workflows/ci.yml`): `--all-targets` + `--no-default-features`, `--features jwt,https`, `--features cloud,cloud-sdk` (для останнього потрібен `K8S_OPENAPI_ENABLED_VERSION=1.28`) — **чисто на `main` (2026-04-10)**. Повний `--all-features` локально може відрізнятися за набором крейтів; орієнтир — ті самі три кроки, що в CI.  
- `cargo test --all-features` — на **Windows MSVC** можливі каскадні помилки компіляції тестів і/або `STATUS_STACK_BUFFER_OVERRUN` у `rustc` через обсяг фіч (cloud-sdk тощо); для повного матрицю краще **GNU toolchain** з `rust-toolchain.toml` або **Linux CI**. Інтеграційні тести ML прунінгу та SAML узгоджені з поточною семантикою `PruningResult` / унікальними іменами SAML-провайдерів.
- **Архітектурні інкременти (`main`, 2026-04–05)**: **`RaidService`** + **`VirtualNode*`** services (**FM-016** ✅); ML pipeline + **TurboQuant**; **P3 / FM-005** — `json_errors.rs`, **`HttpAppError`/`RestError`** по REST, **`raid*`**, **`enterprise_api/`**, auth/WS/rate-limit ✅; **OpenAPI** enterprise sync (S14–S20); бінарі **`poolai-worker`**, **`poolai-telegram-bot`**, **`poolai_health_load`**; dev stand — `bin/verify-dev-stand.*`, `core::dev_stand`; ML-тести — **`[[test]]` + `required-features = ["ml"]`**; P2b wire — `tests/distributed_raid_wire_integration.rs`.

### Next Focus (2026-07-22)

**Product-complete:** PH-S1010 ✅ · FM **§5.15** ✅ · **maintenance mode**.

**Service (сьогодні):** Cursor **3.12.30** rules re-check · FM **§5.16** PH-SVC21…SVC30 ✅ · vision queue/feed enterprise bands · [`CURSOR_UPDATE_RESEARCH_2026-07-22.md`](docs/development/CURSOR_UPDATE_RESEARCH_2026-07-22.md).

**Наступна сесія (owner):** **`абракадабра`** — project scan → band 66 **PH-S1299…S1308** · [`NEXT_SESSION_PROMPT.md`](docs/development/NEXT_SESSION_PROMPT.md).

**§5.12:** **10** відкритих (band 64 ✅) · vision **rev 358** · last **PH-S1298** · next **PH-S1299**.

**Ops (поза чергою):** **FM-003** LAN §4 **BLOCKED** (2 хости) · **FM-041** Cloud SDK **Deferred**.

**Старт сесії:** [`HANDOFF_NEW_SESSION.md`](docs/development/HANDOFF_NEW_SESSION.md) · copy-paste — [`NEXT_SESSION_PROMPT.md`](docs/development/NEXT_SESSION_PROMPT.md) · карта — [`docs/vision/`](docs/vision/) (`.\bin\open-docs-vision.ps1`).

**Звірка «не зроблено»:** [`FUNCTION_MANAGEMENT.md`](docs/catalog/FUNCTION_MANAGEMENT.md) **§5.3**.

**Контекст за пріоритетами Architect (P\*)**

- **P5 / P6 (доки)**: **закриті** на рівні плану — архівні статуси з банером, інвентар TODO у `src/`, концепти Grid + Solana; див. [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md).
- **P4**: повний прогін Criterion + baseline у `BENCHMARKS.md` на **референс-хост**; **`GET /api/v1/health`** — **`poolai_health_load`** (опційно **`--json`**) або **`wrk`**; workflow [`.github/workflows/benchmarks.yml`](.github/workflows/benchmarks.yml); [`docs/performance/PROFILING.md`](docs/performance/PROFILING.md).
- **P2b**: TurboQuant фаза 1 у коді ✅; відкритий чекбокс у Architect-плані — **LAN-заміри** (див. пункт 1 вище); Criterion `raid_replication_engine` уже є.
- **P2 (опційно)**: основні домени через сервіси ✅; дрібні edge cases міграції handlers → `services/*` за потреби.
- **P3 / FM-005** ✅: узгоджений JSON по **`auth`** / **`ws`** / **`rate_limit`**, REST + **`raid*`**, **`enterprise_api`**, **`login`/`refresh`**, **`check_permission`**, **`auth_middleware`** — **`HttpAppError`/`RestError`** (`src/network/json_errors.rs`).
- **P1 (опційно)**: Raft wire ✅ (PH-S04…S06) — `AppState::raft_node`, `raft_rpc`, `cargo test-raft-ci`; production mount `/raft/*` на coordinator — за потреби.
- **UI / UX:** [`docs/development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](docs/development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md).
- **Документація:** таксономія — [`docs/STRUCTURE.md`](docs/STRUCTURE.md); застарілі плоскі `docs/*.md` — опційно в [`docs/archive/`](docs/archive/).
- **Старт сесії:** [`docs/development/HANDOFF_NEW_SESSION.md`](docs/development/HANDOFF_NEW_SESSION.md) §4.

### 🚀 Development Roadmap

#### ✅ MVP (Stage 1) - COMPLETED
- ✅ **Core Module** - Basic structures and traits
- ✅ **Pool Module** - Pool and worker management  
- ✅ **Monitoring Module** - Basic metrics and monitoring

#### ✅ Stage 2 - COMPLETED
- ✅ **Network Module** - REST API and WebSocket with HTTPS/TLS support
- ✅ **Platform Module** - GPU management and optimization
- ✅ **TGBot Module** - Telegram bot for management
- ✅ **Security Module** - JWT authentication, rate limiting, and certificate management

#### ✅ Stage 3 - COMPLETED! 🎉
- ✅ **Runtime Module** - Lifecycle management and process control
- ✅ **Libs Module** - Model library management and version control
- ✅ **VM Module** - Virtualization and isolation support (100% complete) 🎉
  - ✅ Process runner integration
  - ✅ Resource limits (Linux cgroups, Windows Job Objects)
  - ✅ Health checks with auto-recovery (exponential backoff)
  - ✅ Resource monitoring with alerts and history tracking
  - ✅ Network isolation (loopback interface setup, veth pairs)
  - ✅ Filesystem isolation (bind mounts, read-only mounts)
  - ✅ Firewall rules setup (nftables/iptables)
  - ✅ Enhanced documentation with detailed implementation notes
  - ✅ 24 integration tests passing
- ✅ **RAID Module** - Fault tolerance and data replication (100% complete) 🎉
  - ✅ Local artifact storage
  - ✅ Distributed RAID with Raft consensus
  - ✅ Event sourcing, circuit breaker, replication strategies
  - ✅ BurstRAID Strategy (100% complete)
    - ✅ Burst detection with metrics
    - ✅ Adaptive replication factor
    - ✅ Integration tests with real artifacts
  - ✅ SmallWorld Network Strategy (100% complete)
    - ✅ Network topology-based replication
    - ✅ Clustering coefficient calculation
    - ✅ Integration tests with real artifacts
  - ✅ 139+ tests passing (122+ base + 17+ new integration tests)
- ✅ **UI Module** - Web interface and dashboard (100% complete)
  - ✅ Dashboard pages with write operations
  - ✅ JWT authentication and RBAC
  - ✅ UI Components Library
  - ✅ Theme customization (dark, light, high-contrast)
  - ✅ Accessibility features (keyboard navigation, ARIA labels, skip links)
  - ✅ Additional UI components (dropdowns, tooltips, tabs, accordion)
  - ✅ UX improvements (skeleton loaders, error handling, search & filtering)
  - ✅ Responsive design (mobile navigation, touch optimizations)
  - ✅ UI alignment improvements (box-sizing, table containers)
  - ✅ Global search functionality (Ctrl+K/Cmd+K shortcut)
- ✅ **Rewards System** - Endorphin-based achievement system
- ✅ **WebSocket Security** - Real-time updates with JWT authentication
- ✅ **Enhanced API** - Comprehensive REST endpoints (67+ endpoints)
- ✅ **Documentation Improvements** - Enhanced TODO comments with detailed implementation notes
  - ✅ Runtime and platform modules documentation
  - ✅ RAID distributed handlers documentation
  - ✅ VM isolation and resources documentation
  - ✅ Auth and libs integration documentation

#### ✅ Stage 4.1: Advanced Runtime - COMPLETED ✅
- ✅ Process management, resource orchestration
- ✅ Task scheduling, caching system
- ✅ Storage management, health monitoring
- ✅ Auto-scaling capabilities

#### ✅ Stage 4.2: Enterprise Features - COMPLETED ✅ 🎉
- ✅ **Multi-tenancy** - Resource quotas, usage tracking, tenant isolation
- ✅ **Advanced Security** - OAuth2, SAML SSO, security policies
- ✅ **Audit Logging** - File-based logging, rotation, query, cleanup
- ✅ **Advanced Monitoring** - Real-time dashboards, alerts, metrics aggregation
- ✅ **Admin Panel** - Comprehensive admin interface with full functionality
- ✅ **Enterprise API** - REST API for all enterprise features
- ✅ **16 enterprise tests passing**

#### 🔄 Stage 4.3: Cloud Integration - IN PROGRESS 🔄 (Infrastructure 100% ✅)
- ✅ **Module Structure** - Complete cloud module architecture
- ✅ **Kubernetes Support** - Infrastructure ready (operator, CRDs placeholders)
- ✅ **Cloud Providers** - AWS, Azure, GCP integration infrastructure ready
- ✅ **Auto-scaling** - Auto-scaling module structure ready
- ✅ **Load Balancing** - Load balancing module structure ready
- ✅ **Unit Tests** - 8 comprehensive tests passing
- 🔄 **SDK Integration** - Full implementation with cloud SDKs (planned)
- 🔄 **Kubernetes Operator** - Full operator implementation (planned)

#### 🔄 Stage 4.4: AI/ML Enhancement — IN PROGRESS 🔄
- ✅ ML.1–ML.6 scaffolding in `src/ml` (optimization, AutoML, federated, context memory, versioning, experiments, **pipeline** with steps such as **FederatedAggregation**)
- ✅ After AutoML step, default registration with **ModelVersionManager** / **ExperimentTracker** (disable via pipeline **`automl_skip_registry`** when needed)
- ✅ Enterprise REST (**features `enterprise` + `ml`**): `/api/enterprise/ai-ml/pipeline` (list, create, get, execute), `/pipeline/demo` — shared **`MLPipelineManager`** on **`ApiContext`**
- 🔄 Hardening: production-grade step implementations, metrics, and documentation of operational playbooks

---

## 🌟 New Stage 3 Features

### 🎁 **Rewards System**
- **Endorphin-based rewards** for performance and collaboration
- **Achievement system** with badges and levels
- **Progress tracking** and user statistics
- **Performance bonuses** and streak rewards

### 🔐 **Enhanced Security**
- **JWT authentication** with role-based access control
- **WebSocket security** with token validation
- **HTTPS/TLS support** with self-signed certificates
- **Rate limiting** and DDoS protection

### 🌐 **Real-time Communication**
- **WebSocket endpoints** for live metrics
- **Real-time updates** for system status
- **Live monitoring** with instant notifications
- **Secure communication** protocols

### 📊 **Advanced API**
- **Health check endpoints** for monitoring
- **Comprehensive metrics** collection
- **User management** and authentication
- **Resource monitoring** and optimization

---

## 📋 Requirements

### System Requirements
- **OS**: Linux (Ubuntu 20.04+) or Windows 10+
- **CPU**: 4+ cores recommended
- **RAM**: 8GB+ recommended
- **Storage**: 50GB+ available space
- **GPU**: NVIDIA GPU with CUDA support (optional)

### Rust Requirements
- **Rust**: див. **`rust-toolchain.toml`** (наприклад, `1.92.0` stable); MSRV для окремих залежностей — у коментарях `Cargo.toml`.
- **Cargo**: разом із Rust
- **Edition**: 2021
- **Toolchain**: для Windows у репозиторії задано `x86_64-pc-windows-gnu` у `rust-toolchain.toml`; на Linux — типовий stable для цільової ОС.

### Software Requirements
- **Rust**: як у `rust-toolchain.toml` / stable для вашої платформи
- **MSYS2** (Windows): For native dependencies (gcc, dlltool)
- **CUDA**: 11.0+ (optional, for GPU support)
- **OpenSSL**: 1.1.1+ (for HTTPS/TLS support)
- **Certbot**: For Let's Encrypt certificates (production)

## 🛠️ Installation

### Windows Setup (Required for jwt/https features)

If you're building on Windows with `jwt` or `https` features, you need MSYS2 tools (`dlltool.exe`, `gcc`) in your PATH:

#### Quick Setup (MSYS2 bash)
Open an **MSYS2 UCRT64 bash** terminal and run:
```bash
export MSYSTEM=UCRT64
export PATH=/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH

# Verify tools are available
dlltool --version
gcc --version
```

#### Manual Setup (permanent)
If the MSYS2 dirs are not on your Windows `PATH`, add permanently:
- `C:\msys64\ucrt64\bin`
- `C:\msys64\usr\bin`

Then restart your terminal.

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/platinoff/poolAI.git
   cd poolAI
   ```

2. **Windows: Setup MSYS2 PATH** (if using jwt/https features)
   ```bash
   export MSYSTEM=UCRT64
   export PATH=/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH
   ```

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Generate certificates (for HTTPS)**
   ```bash
   mkdir certs
   openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem -days 365 -nodes
   ```

5. **Run the application** — канон: [`docs/development/RUN_LOCAL.md`](docs/development/RUN_LOCAL.md)
   ```powershell
   # Windows PowerShell (без WSL)
   cd S:\rust\poolAI
   .\bin\run-poolai.ps1 build
   .\bin\run-poolai.ps1 single -Background -SkipBuild
   # Login http://127.0.0.1:8080/ui/login  →  admin / admin123
   ```
   ```bash
   # MSYS2 UCRT64 (зовнішнє вікно; не голе "bash" — WSL stub)
   export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
   cd /s/rust/poolAI
   /usr/bin/bash bin/run-poolai.sh single --bg --skip-build
   ```

## 🚀 Usage

### Starting the System

**Єдиний лаунчер** (single / LAN / virtual-node / Docker):

```powershell
.\bin\run-poolai.ps1 help
.\bin\run-poolai.ps1 single -Background
.\bin\run-poolai.ps1 stop
.\bin\poolai-msys.ps1 bin/e2e-playwright.sh --start   # bash-скрипти з PS
```

MSYS2: `/usr/bin/bash bin/run-poolai.sh …` — див. [`.cursor/commands/run-poolai.md`](.cursor/commands/run-poolai.md).

Повний runbook: [`docs/development/RUN_LOCAL.md`](docs/development/RUN_LOCAL.md).

**Альтернатива — cargo напряму:**

```bash
cargo run --features enterprise,ml,cloud
# Admin: http://localhost:8080/ui/admin
# HTTPS + JWT: cargo run --features enterprise,https,jwt
POOLAI_CONFIG_PATH=./custom_config.toml RUST_LOG=debug cargo run --features enterprise
```

### Default users and first login

On first startup the server seeds built-in accounts (see `UserManager::initialize` in `src/core/user_manager.rs`): **Admin** `admin` / `admin123`, plus operator and viewer test users. For a fresh dev install you can sign in at `/ui/auth` with those credentials; you do **not** need to create the primary admin via API first.

Successful **`POST /api/v1/login`** and **`POST /api/v1/refresh`** responses include optional JSON field **`bootstrap_default_admin`** (`true` when the logged-in user still matches the default seeded admin credentials). The web UI shows a first-run banner until the admin password is changed (e.g. under **Users** in the enterprise admin panel). Change all default passwords before production.

```bash
# Optional: create an additional admin or user via API (when permitted)
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"username": "admin2", "password": "change-me", "role": "Admin"}'

# Login to get JWT token
curl -X POST http://localhost:8080/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'
```

The JWT token is stored in localStorage (and refreshed via `/api/v1/refresh`) for the admin UI.

### Current Features (Stage 3)

- **Pool Management**: Advanced worker pool with intelligent load balancing
- **Model Integration**: Core model interface and processing with library management
- **Advanced Monitoring**: System metrics, health checks, and real-time updates
- **Resource Management**: GPU and memory allocation with optimization
- **Security**: JWT authentication, HTTPS/TLS, role-based access control
- **Rewards System**: Achievement-based motivation system
- **WebSocket**: Real-time communication and live metrics
- **API**: Comprehensive REST endpoints with documentation

### Planned / Ongoing Work (Stage 4.x)

- **Enterprise Features**: Multi-tenancy, advanced security, audit logging (**implemented**, see Enterprise docs)
- **Cloud Integration**: Kubernetes support, cloud providers, auto-scaling, load balancing (see Cloud SDK docs and status)
- **AI/ML Enhancement**: Model optimization, AutoML integration, federated learning (Stage 4.4 plan + stubs)
- **Advanced UI**: Modern dashboard with real-time monitoring (**implemented**, further polish optional)
- **CI/CD**: Automated testing and deployment pipelines (**configured**, can be extended with perf benchmarks)

## 🔒 Security & HTTPS

### Security Architecture

PoolAI implements a comprehensive security model with multiple deployment options:

#### Development Mode (HTTPS)
- HTTPS on localhost with self-signed certificates
- JWT authentication for API access
- CORS enabled for local development

#### Production Mode (HTTPS)
- TLS 1.3 encryption for all communications
- Automatic certificate management with Let's Encrypt
- HSTS headers for enhanced security
- Rate limiting and DDoS protection

### Security Features

- **Authentication**: JWT-based API authentication ✅
- **Authorization**: Role-based access control (Admin, Operator, Viewer) ✅
- **Encryption**: TLS 1.3 for transport, AES-256 for data at rest ✅
- **Rate Limiting**: Configurable request limits ✅
- **CORS**: Configurable cross-origin resource sharing ✅
- **Security Headers**: HSTS, CSP, X-Frame-Options ✅
- **WebSocket Security**: WSS with JWT authentication ✅

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --test integration
```

### Security Tests

```bash
# Run security audit
cargo audit

# Test HTTPS endpoints
curl -k https://localhost:8080/api/v1/status

# Test WebSocket secure connection
wscat -c wss://localhost:8080/ws/metrics

# Test rewards system
curl -k https://localhost:8080/api/v1/rewards
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](.github/CONTRIBUTING.md) for details.

### Quick Start

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following our [coding standards](.github/CONTRIBUTING.md#coding-standards)
4. Commit using [Conventional Commits](docs/GIT_COMMIT_GUIDELINES.md) format
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request using our [PR template](.github/PULL_REQUEST_TEMPLATE.md)

### Development Guidelines

- Follow the Stage 4 roadmap approach
- Focus on enterprise features and cloud integration
- Maintain clean, documented code
- Write tests for new functionality
- Follow [Rust Book](https://doc.rust-lang.org/book/) best practices

## Documentation

Увесь текстовий корпус — у [`docs/`](./docs/).

| Що відкрити | Призначення |
|-------------|-------------|
| [Documentation map](#documentation-map-canonical-order) (зверху) | Канонічні кроки 1–12 |
| [`docs/README.md`](docs/README.md) | Той самий порядок + короткі вказівки |
| [`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md) | Повна навігація по дереву `docs/` |
| [`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) | Витяг функціоналу (крок 11) |
| [`docs/catalog/FUNCTION_MANAGEMENT.md`](docs/catalog/FUNCTION_MANAGEMENT.md) | Керування функціоналом, тікети FM-* (крок 12) |
| [`docs/development/README.md`](docs/development/README.md) | Індекс планів у `development/` |
| [`docs/openapi.yaml`](docs/openapi.yaml) | OpenAPI (REST) |

**Додатково (історія та зрізи):** [`docs/status/CURRENT_STATUS.md`](docs/status/CURRENT_STATUS.md), [`docs/status/STABLE_STATE_SUMMARY.md`](docs/status/STABLE_STATE_SUMMARY.md), [`docs/development/NEXT_DEVELOPMENT_PHASE.md`](docs/development/NEXT_DEVELOPMENT_PHASE.md), [`docs/development/NEXT_STEPS_PLAN.md`](docs/development/NEXT_STEPS_PLAN.md), [`docs/concept/poolAI_concept.txt`](docs/concept/poolAI_concept.txt).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/platinoff/poolAI/issues)
- **Discussions**: [GitHub Discussions](https://github.com/platinoff/poolAI/discussions)

## 💰 Support the Project (Solana Donations)

**Madevinc** welcomes donations in Solana (SOL) to support the development of this crypto project.

**Solana Address**: `GcdgNtdE8NEk3z9sQ5jXv2tqguZjSYqPqNAtjsjPNJx8`

All donations help fund:
- Continued development and maintenance
- Infrastructure costs
- Feature enhancements
- Community support

Thank you for supporting PoolAI! 🙏

## 🙏 Acknowledgments

- **Madevinc** - Project creator and maintainer (one developer with Cursor AI)
- Rust community for the excellent ecosystem
- NVIDIA for CUDA and GPU computing tools
- All contributors and users of PoolAI
- GitHub Copilot and Cursor AI for development assistance

---

**PoolAI** — distributed AI mining pool management.  
**Version:** 0.2.2 (`Cargo.toml`) · **Docs updated:** 2026-05-19 · **Repository:** [github.com/platinoff/poolAI](https://github.com/platinoff/poolAI)  
**Наступні орієнтири:** Stage 4.3–4.4 (cloud / ML); канонічний план — [`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md); старт сесії — [`docs/development/HANDOFF_NEW_SESSION.md`](docs/development/HANDOFF_NEW_SESSION.md); витяг функціоналу — [`docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md); беклог функцій — [`docs/catalog/FUNCTION_MANAGEMENT.md`](docs/catalog/FUNCTION_MANAGEMENT.md).
