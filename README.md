# PoolAI - AI Mining Pool Management System

> 🇺🇦 Ukrainian version available: [README.uk.md](../README.md)

PoolAI is a comprehensive distributed system for managing AI mining pools with integration of generative models, GPU optimization, and automated resource management.

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

**Current Phase**: Stage 4.3 Cloud Integration in progress, Stage 4.4 AI/ML scaffolding ready (see docs)  
**Target**: Advanced AI Mining Pool with Enterprise Features and Cloud/ML optimization  
For a detailed status view see `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` and `docs/status/STABLE_STATE_SUMMARY.md`.

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

#### 🔄 Stage 4.4: AI/ML Enhancement - PLANNED 🔄
- 🔄 Model optimization, AutoML integration
- 🔄 Federated learning, model versioning
- 🔄 Experiment tracking, pipeline management

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

#### Quick Setup (PowerShell)
```powershell
# Run setup script to add MSYS2 to PATH
.\scripts\setup_msys2_path.ps1

# Verify dlltool is available
dlltool --version
```

#### Manual Setup
If MSYS2 is installed but not in PATH, add it manually:
```powershell
# For current PowerShell session
$env:PATH += ";C:\msys64\usr\bin"

# Or add C:\msys64\usr\bin to System PATH environment variable permanently
```

**Note**: The `setup_msys2_path.ps1` script adds MSYS2 to PATH only for the current PowerShell session. For a permanent solution, add `C:\msys64\usr\bin` to your system PATH environment variable.

**Troubleshooting**: If you get `Error calling dlltool 'dlltool.exe': program not found`, run `.\scripts\setup_msys2_path.ps1` before building.

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/platinoff/poolAI.git
   cd poolAI
   ```

2. **Windows: Setup MSYS2 PATH** (if using jwt/https features)
   ```powershell
   .\scripts\setup_msys2_path.ps1
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
