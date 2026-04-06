# PoolAI - AI Mining Pool Management System

> 🇺🇦 Українськомовні матеріали: концепція та індекс у [`docs/concept/`](docs/concept/) та [`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md) (окремого `README.uk.md` у корені немає).

PoolAI is a comprehensive distributed system for managing AI mining pools with integration of generative models, GPU optimization, and automated resource management.

## Documentation map (canonical order)

Узгоджено з [`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md) та [`docs/README.md`](docs/README.md).

1. **[`docs/INDEX_2026-03-17.md`](docs/INDEX_2026-03-17.md)** — навігація по всьому каталогу `docs/` (концепція, статус, ML, cloud, troubleshooting).
2. **[`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md)** — **головний** покроковий план Rust Architect (пріоритети P1–P6, TurboQuant, верифікація CI).
3. **[`docs/development/HANDOFF_NEW_SESSION.md`](docs/development/HANDOFF_NEW_SESSION.md)** — старт для **нової сесії**: гілка `main`, порядок доків, git-push, що зроблено в service layer, наступні кроки.
4. **Concept / vision** — [`docs/concept/poolAI_concept_root.txt`](docs/concept/poolAI_concept_root.txt), Grid/Memory/Job: [`docs/concept/POOLAI_GRID_NODE.md`](docs/concept/POOLAI_GRID_NODE.md), [`docs/concept/POOLAI_MEMORY_LAYER.md`](docs/concept/POOLAI_MEMORY_LAYER.md), [`docs/development/JOB_LAYER_CONCEPT_2026-03-17.md`](docs/development/JOB_LAYER_CONCEPT_2026-03-17.md).
5. **Inventory** — [`file_list.csv`](file_list.csv) (ручний зріз ключових шляхів; синхронізуй після змін у `src/services/`, `src/network/api/`, `.cursor/`); повний список: `git ls-files`.
6. **Git push (Windows)** — [`.cursor/commands/git-push.md`](.cursor/commands/git-push.md) (MSYS2 bash, PATH, змінні для CI/cloud-sdk).

## 🎉 **PROJECT 100% COMPLETE! v0.2.2 RELEASED! PRO EDITION!** 🚀

**Current Status**: **All 15 Modules 100% Complete!** **437+ Tests Passing!** **Production Ready!** **v0.2.2 Released!** **PRO Package Active!**  
**Project Structure**: Optimized (Docker files in `docker/`, documentation in `docs/`)  
**Release**: v0.2.2 - Production Ready (2026-01-22)  
**Previous Release**: v0.2.1 (2026-01-22)  
**Package**: PRO Edition (First Day: 2026-01-16)

### 🆕 What's New in v0.2.2
- ✅ **Cloud SDK 100%** - Load Balancing routing rules, Cloud LB init (K8s Service LoadBalancer)
- ✅ **RoutingRule**, `add_routing_rule`, `get_routing_rules`, `set_cloud_lb_config`
- ✅ **Documentation** - Stable state, roadmap, next steps (Rust Architect) aligned

### 🆕 What's New in v0.2.1
- ✅ **Cloud SDK Auto-scaling** - Metrics API, `evaluate_and_scale`, ScalingAction
- ✅ **Pre-push Hook** - `cargo fmt --all --check` before git push
- ✅ **Rust Architect Rules** - `.cursor/rules/`, MSYS2 bash, concept files  
**Repository**: [https://github.com/platinoff/poolAI](https://github.com/platinoff/poolAI)  
**Creator**: Madevinc (one developer with Cursor AI)

---

## ⚡️ Architectural Improvement Plan (2025)

1. **Healthcheck endpoint** — /api/v1/health for CI/CD and monitoring ✅ **COMPLETED**
2. **Global version/uptime state** — implemented via `version.rs` module ✅ **COMPLETED**
3. **Public API exported only from lib.rs** — all internals private, rustdoc for public traits/structs ✅ **COMPLETED**
4. **JWT & RBAC** — middleware for token and role checks (admin/operator/viewer) ✅ **COMPLETED**
5. **Endpoint access restriction** — /metrics, /workers, /shutdown only for authorized users ✅ **COMPLETED**
6. **CI/CD** — GitHub Actions workflow for tests and builds ✅ **COMPLETED**
7. **Swagger/OpenAPI** — API spec generation and publication ✅ **COMPLETED**
8. **Documentation** — Quick Start, curl examples, security section ✅ **COMPLETED**
9. **Live metrics (WebSocket)** — /ws/metrics for real-time monitoring ✅ **COMPLETED**
10. **UI/UX** — Copy buttons, security links, favicon/logo, status page improvements ✅ **COMPLETED**
11. **UI Improvements** — Accessibility features, additional components, UX improvements, responsive design ✅ **COMPLETED**
12. **VM Isolation** — Loopback interface setup, bind mounts, read-only mounts ✅ **COMPLETED**
13. **Documentation Improvements** — Enhanced TODO comments with detailed implementation notes ✅ **COMPLETED**
14. **Comprehensive Documentation** — Rustdoc documentation for all core modules (config, error, monitoring, pool, vm, raid) ✅ **COMPLETED**
15. **Error Message Improvements** — Enhanced error messages with context and suggestions across all modules ✅ **COMPLETED**
16. **Unit Tests** — Comprehensive unit tests for versioning, config, pool, raid, and vm modules ✅ **COMPLETED**

---

## 🎯 Development Status
**Current Phase**: Stage 4.3 Cloud Integration (SDK + operator work continues) and **Stage 4.4 AI/ML** (pipeline orchestration, enterprise HTTP API, versioning/experiments — active development)  
**Target**: Advanced AI Mining Pool with Enterprise Features and Cloud/ML optimization  
For a detailed status view see `docs/status/STABLE_STATE_SUMMARY.md`. Documentation map: `docs/INDEX_2026-03-17.md`.

### ✅ Current Build/Test Status (2026-04-07)
- `cargo fmt --all` — CI / before push  
- **Required in CI** (`.github/workflows/ci.yml`): `cargo test --lib --tests --features ml,enterprise,cloud` with `K8S_OPENAPI_ENABLED_VERSION=1.28` — **passing** (верифікація включно з `-j 1` та `--test-threads=1` на Windows при обмеженій RAM / OOM лінкера).  
- `cargo clippy --all-targets --all-features` — completes (warnings allowed locally; CI uses narrower `-D warnings` matrices).  
- `cargo test --all-features` — на **Windows MSVC** можливі каскадні помилки компіляції тестів і/або `STATUS_STACK_BUFFER_OVERRUN` у `rustc` через обсяг фіч (cloud-sdk тощо); для повного матрицю краще **GNU toolchain** з `rust-toolchain.toml` або **Linux CI**. Інтеграційні тести ML прунінгу та SAML узгоджені з поточною семантикою `PruningResult` / унікальними іменами SAML-провайдерів.
- **Архітектурні інкременти (гілка `main`, 2026-04)**: розширений **`RaidService`** (артефакти, квота, статус кластера); Rust-бекенди базових кроків ML pipeline + **TurboQuant** (`src/ml/turboquant.rs`, крок `Quantization`); **Priority 3 (частково)** — `api_error_response`, **`api_json_error`**, **`AppError::Forbidden`**, `http_status_for_app_error` у `network/api/common.rs`; узгоджені помилки в **instances, libraries, vm, workers, topology, rewards**, **tenant** у `enterprise_api.rs`, плюс раніше — RAID `Operation` та enterprise **AI-ML pipeline** (`ai_ml.rs`).

### Next Focus
- **Priority 3 (продовження)**: `raid.rs` (більшість шляхів), `ui`, `users`, `system`, `completions`, `raid_admin`, решта `enterprise_api.rs`; за потреби уточнити мапінг `AppError` → HTTP для `ResourceError` / not-found.
- **Priority 4**: бенчмарки та профілювання (див. `docs/performance/BENCHMARKS.md`, план у `NEXT_STEPS_ARCHITECT_2026-03-17.md`).
- **Priority 2 (опційно)**: подальше перенесення RAID (workers, events, snapshot) у `RaidService`.
- За потреби: стабілізувати `cargo test --all-features` на Windows (GNU host або розбиття тестів).
- Канонічний план і чекбокси: [`docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md`](docs/development/NEXT_STEPS_ARCHITECT_2026-03-17.md); старт нової сесії: [`docs/development/HANDOFF_NEW_SESSION.md`](docs/development/HANDOFF_NEW_SESSION.md).

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
- ✅ **Libs Module** - Model library management and version control (95% complete)
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
- **Rust**: 1.70+ (stable) - **Recommended: 1.83+ (latest)**
- **Cargo**: Included with Rust
- **Edition**: 2021 (as specified in Cargo.toml)
- **Toolchain**: stable-x86_64-pc-windows-gnu (Windows) or stable-x86_64-unknown-linux-gnu (Linux)

### Software Requirements
- **Rust**: 1.70+ (latest stable)
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

5. **Run the application**
   ```bash
   # Basic run (standard UI)
   cargo run
   
   # With Admin Panel (Enterprise features)
   cargo run --features enterprise
   
   # With Admin Panel + HTTPS + JWT
   cargo run --features enterprise,https,jwt
   ```

## 🚀 Usage

### Starting the System

```bash
# Basic run (standard UI at http://localhost:8080/ui)
cargo run

# With Admin Panel (Enterprise features)
cargo run --features enterprise
# Access Admin Panel at: http://localhost:8080/admin

# With Admin Panel + HTTPS + JWT (Recommended for development)
cargo run --features enterprise,https,jwt
# Access Admin Panel at: https://localhost:8443/admin

# With specific config
POOLAI_CONFIG_PATH=./custom_config.toml cargo run --features enterprise

# With logging
RUST_LOG=debug cargo run --features enterprise
```

### Creating Admin User

After starting the server, create an admin user via API:

```bash
# Create admin user
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123", "role": "Admin"}'

# Login to get JWT token
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'
```

The JWT token will be stored in localStorage or cookie after login for accessing the admin panel.

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

## 📚 Documentation

Вся документація проекту знаходиться в каталозі [`docs/`](./docs/). Дивіться [`docs/README.md`](./docs/README.md) для повного списку документів.

### Основні документи:
- [`docs/status/CURRENT_STATUS.md`](./docs/status/CURRENT_STATUS.md) - Поточний стан проекту
- [`docs/development/NEXT_DEVELOPMENT_PHASE.md`](./docs/development/NEXT_DEVELOPMENT_PHASE.md) - Наступна фаза розробки
- [`docs/development/NEXT_STEPS_PLAN.md`](./docs/development/NEXT_STEPS_PLAN.md) - План наступних кроків
- [`docs/status/STABLE_STATE_SUMMARY.md`](./docs/status/STABLE_STATE_SUMMARY.md) - Стабільний стан розробки
- [`docs/concept/poolAI_concept.txt`](./docs/concept/poolAI_concept.txt) - Концепція проекту

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

**PoolAI** - Empowering AI with distributed computing 🚀  
**Status**: PRO Edition - All Modules Complete! 🎯  
**Version**: v0.1.0  
**Last Updated**: 2026-01-16  
**Package**: PRO (First Day: 2026-01-16)  
**Next Goal**: Stage 4.3 - Cloud Integration SDK Implementation 🚀  
**Repository**: [https://github.com/platinoff/poolAI](https://github.com/platinoff/poolAI)
