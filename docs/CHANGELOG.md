# Changelog

All notable changes to PoolAI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Автопрогін 2026-05-20 (AUTO_RUN_SESSION)** — **FM-015 ✅**: admin contracts фаза 3 (`/instance`, `/raid/artifacts`, smallworld metrics; 20 tests).
- **Автопрогін 2026-05-19 (AUTO_RUN_SESSION)** — **FM-014 ✅**: admin contracts (config, users, topology/nodes; 15 tests); `rewards.rs` → `HttpAppError` (FM-005).
- **Автопрогін 2026-05-18 (AUTO_RUN_SESSION)** — **FM-013 ✅**: розширено `tests/admin_ui_api_contracts.rs` (libraries, topology, VM, workers; 12 tests); admin libs UI — статус «Installed» з `metadata.installed_at`.
- **Автопрогін 2026-05-17 (AUTO_RUN_SESSION)** — P0: [`AUTO_DEV_PATTERNS.md`](development/AUTO_DEV_PATTERNS.md) (25 записів `path:line`); оркестратор [`.cursor/rules/autonomous-orchestrator.mdc`](../../.cursor/rules/autonomous-orchestrator.mdc); FM-003 runbook звірка без LAN-стенду.
- **Автопрогін 2026-05-16 (AUTO_RUN_SESSION)** — FM-012 Telegram/OAuth ✅; FM-007/008 wire ✅; FM-002 service audit ✅; FM-011 `cargo test-ci` ✅; [`LAN_BENCHMARK_RUNBOOK.md`](performance/LAN_BENCHMARK_RUNBOOK.md) (FM-003 ops).
- **FM-012 — UI i18n UA/EN (2026-04 → 2026-05)** — `i18n_core.js`, `/ui/auth`, layout `/ui/*`, write-flow Workers/Libs/VM/RAID, enterprise admin (`src/ui/admin/*`, `admin_common.js`); shared shell у `mod.rs` (глобальний пошук, confirm/retry, error boundary, валідація форм, ролі).
- **FM-012 — перший вхід** — банер для сідженого `admin`, поле **`bootstrap_default_admin`** у **`POST /api/v1/login`** / **`POST /api/v1/refresh`**.
- **Enterprise Telegram OAuth** — HMAC query віджета, `auth_date`, allowlist `telegram_allow_user_ids`, audit; widget HTML UA/EN (`Accept-Language` / `?lang=`); unit-тести allowlist/expiry/RBAC; нові користувачі → Viewer.
- **Distributed RAID (FM-007 / FM-008, wire)** — sync каталогів, `conflicts` за remote versions, LeaveCluster з replication/membership; інтеграційні тести `distributed_raid_wire_integration`.
- **FM-011** — alias **`cargo test-ci`** (`--lib` + `--tests`, без doctests); профіль **`[profile.test] debug = 1`**.
- **P4 / perf** — рядки baseline у `BENCHMARKS.md` (Criterion, `poolai_health_load --json`).
- **FM-005 ✅** — узгоджений JSON помилок (`HttpAppError` / `AppError::RestError`) по REST, RAID, enterprise API, auth/login/refresh, middleware.
- **P2 service layer** — `SystemService`, `UiService`, `RaidDistributedProtocolService`, `ChatCompletionService`, `RewardsService`, `WorkerPoolService`, `TopologyService`, `DiscoveryService`, тощо; тонкі handlers у `network/api/*`.
- **HPA (Horizontal Pod Autoscaler) init for Kubernetes** 🎉
  - `KubernetesManager::hpa_exists(name)`, `create_hpa(name, deployment, min, max, target_cpu%)`
  - `AutoScaler::ensure_hpa_for(deployment_name)` — create HPA from scaler min/max, CPU 70%
  - HPA v2 API: `autoscaling/v2`, CPU-based scaling
  - Initialize logs "HPA support (use ensure_hpa_for)" when k8s_manager set
- **Mock server integration harness** 🎉
  - `tests/cloud_mock_integration.rs` wires `tests/integration/cloud/` (mockito: AWS, Azure, GCP)
  - Run: `cargo test --test cloud_mock_integration --features cloud,cloud-sdk` (Rust 1.88+)
  - CLOUD_SDK_STATUS, NEXT_STEPS updated
- **Configurable base URL override (Azure)** 🎉
  - `AzureManager::set_base_url_override(Option<String>)` for Management API (e.g. mock server)
  - `create_vm_scale_set` uses override when set; e2e test `test_azure_vmss_e2e_with_mock_server`
- **Configurable base URL override (GCP)** 🎉
  - `GcpManager::set_base_url_override(Option<String>)` for metadata + Compute API
  - `get_token_from_metadata_server` and `create_compute_instance` use override when set
  - E2E test `test_gcp_compute_e2e_with_mock_server`; fix instance `id` parse (str or u64)
- **Configurable base URL override (AWS)** 🎉
  - `AwsManager::set_ec2_base_url_override`, `set_ecs_base_url_override`; SigV4 skipped when set
  - E2E tests `test_aws_ec2_e2e_with_mock_server`, `test_aws_ecs_e2e_with_mock_server`
- **Stage 4.4 AI/ML scaffolding (v0.3.0+)** 🎉
  - `src/ml` module, feature `ml`, `AiMlStatus` stub
  - `GET /api/enterprise/ai-ml` and `GET /api/enterprise/ai-ml/status` when `enterprise` + `ml` enabled
  - Placeholders: Model Optimization, AutoML, Federated Learning, etc.
- **ML.1 Model Optimization stub** 🎉
  - `src/ml/optimization.rs`: `QuantizationLevel`, `OptimizationProfile`, `default_fast` / `default_balanced`
  - `GET /api/enterprise/ai-ml/optimization` returns default profile
- **ML.1 profiling, tuning, quantization (implementation)** 🎉
  - `ModelProfile`, `profile_model()` — latency_ms, memory_mb, flops stubs
  - `TuningConfig`, `TuningResult`, `suggest_hyperparams()` — hyperparameter stub
  - `QuantizationResult`, `apply_quantization(profile)` — compression stub
  - `GET /ai-ml/optimization/profile`, `/optimization/tuning`, `/optimization/quantization-result`
- **ML.2 AutoML stub** 🎉 — `src/ml/automl.rs`: `AutomlConfig`, `GET /ai-ml/automl`
- **ML.3 Federated Learning stub** 🎉 — `src/ml/federated.rs`: `AggregationMode`, `FederatedConfig`, `GET /ai-ml/federated`

### Planned
- GlobalState manager for centralized state management
- ErrorContext for structured error handling
- Additional performance optimizations

## [0.2.2] - 2026-01-22

### Added
- **Load Balancing – routing rules & cloud LB init** 🎉
  - `RoutingRule` (path_prefix, host, priority), default rule `/*` on init
  - `add_routing_rule()`, `get_routing_rules()`
  - `set_cloud_lb_config(deployment, ports)` for K8s LoadBalancer Service
  - Cloud LB init: create `{deployment}-lb` Service when k8s_manager + config set
  - Fix `check_backend_health_static` param names (backend, config)
  - Tests: `test_routing_rules_default_after_init`, `test_add_and_get_routing_rules`
  - Cloud SDK 100% complete

### Changed
- **Documentation** (2026-01-22): Stable state, roadmap, next steps (Rust Architect)
  - Concept root: Cloud SDK 100%, Next Goal v0.2.2 → v0.3.0+
  - DEVELOPMENT_ROADMAP: Cloud 100%, P1 (v0.2.2), P2 (v0.3.0+)
  - NEXT_STEPS, STABLE_STATE_SUMMARY, CLOUD_SDK_STATUS, FUTURE_DEVELOPMENT_ROADMAP aligned

## [0.2.1] - 2026-01-22

### Added
- **Cloud SDK Auto-scaling - Metrics Collection** 🎉
  - Real metrics collection from Kubernetes Metrics API
  - `get_pod_metrics()` method in KubernetesManager
  - PodMetrics structure (CPU millicores, memory Kibibytes)
  - Helper functions: `parse_cpu_millicores()`, `parse_memory_kibibytes()`
  - Integration with AutoScaler.get_metrics() for real-time metrics
  - Fallback to placeholder metrics when Metrics API unavailable
- **Cloud SDK Auto-scaling - Automatic Scaling Rules** 🎉
  - `evaluate_and_scale()` method for automatic scaling based on policies
  - ScalingAction structure to track scaling operations
  - Automatic scale up/down based on policy thresholds
  - Support for CPU, Memory, and RequestRate metrics
- **Pre-push Hook** 🎉
  - Automatic `cargo fmt --all --check` before git push
  - Auto-formatting if formatting fails
  - Documentation: `docs/development/PRE_PUSH_HOOK.md`
- **Rust Architect Rules** 🎉
  - `.cursor/rules/rust-architect.md` - Complete workflow guide
  - Rules for using concept files, file_list.csv, MSYS2 bash
  - Document synchronization guidelines

### Changed
- **Version Update**: Updated `APP_VERSION` in `src/version.rs` from 0.1.0 to 0.2.1 to match `Cargo.toml`
- **Git Configuration**: Updated `.gitignore` to exclude development directories:
  - `.cursor/` - Cursor IDE settings and cache
  - `docs/` - Local documentation (not synced)
  - `.vscode/` - VS Code settings (except important configs)
  - `scripts/` - Local development scripts
- **File Organization**:
  - Moved `.cursorrules` to `.cursor/rules/.cursorrules`
  - Moved `QUICK_FIX_MSYS2.md` to `docs/troubleshooting/`
- **Documentation**: Updated concept files with:
  - Current Rust version (1.92.0)
  - Updated test counts (437+ tests passing)
  - MSYS2 commands and tools section
  - Git configuration documentation
  - file_list.csv usage information
- **Cloud SDK Progress**: 95% → 99% (Metrics collection + Scaling rules implemented)

### Fixed
- Git working tree cleanup - removed all modified (M) flags from files
- Version synchronization between `Cargo.toml` and `src/version.rs`
- AutoScaler.get_metrics() now uses real Kubernetes Metrics API instead of placeholders

## [0.2.1] - 2026-01-19

### Added
- **Enterprise Features - 100% Complete** 🎉
  - **SAML SSO Implementation** - Full SAML 2.0 authentication flow
    - SAML auth handler (`/auth/saml/{provider}`) - redirects to Identity Provider SSO URL
    - SAML callback handler (`/auth/saml/{provider}/callback`) - processes SAML response
    - SAML assertion validation with attribute extraction
    - User creation/mapping from SAML attributes
    - JWT token generation after successful SAML authentication
    - Support for attribute mapping configuration
    - RelayState support for custom redirect URLs
  - **SQLite Persistence Integration Tests** - Comprehensive test coverage
    - 10 integration tests for SQLite persistence
    - Tests for metrics persistence, historical queries with filters
    - Tests for automatic cleanup of old metrics (30 days)
    - Tests for fallback to in-memory history when database unavailable
    - Tests for tags serialization/deserialization
    - Tests for multiple metrics handling
    - Tests for error handling with invalid database paths

### Changed
- **Enterprise Features**: Status updated from 95% to 100% complete
- **Test Coverage**: Increased from 427+ to 437+ tests (102 unit + 335+ integration)
  - Added 10 new integration tests for SQLite persistence

### Fixed
- SAML route placement to avoid conflict with OAuth2 telegram callback

## [0.2.0] - 2026-01-19

### Added
- **RAID Strategy Enhancements** - 100% Complete 🎉
  - **BurstRAID Strategy** - Full implementation with metrics and integration tests
    - Burst detection with adaptive replication factor (2-5)
    - Automatic rebalancing with artifact movement tracking
    - Comprehensive metrics collection (`BurstRaidMetrics`, `ArtifactBurstStats`)
    - Integration tests with real artifacts (6 tests)
  - **SmallWorld Network Strategy** - Full implementation with metrics and integration tests
    - Network topology-based replication
    - Clustering coefficient calculation
    - Short-path routing optimization
    - Comprehensive metrics collection (`SmallWorldMetrics`)
    - Integration tests with real artifacts (6 tests)
  - **Cross-Strategy Integration** - Complete support for strategy switching
    - Strategy switching tests (5 tests)
    - Status tracking (`last_rebalance_time`, `artifacts_moved`)
    - Real metrics exposure through API
- **Enterprise Features Enhancement** - 95% Complete
  - **SQLite Persistence for Monitoring** - 100% Complete
    - Database schema for `metrics_history` table
    - Automatic cleanup of old metrics (30 days retention)
    - Historical metrics query API with filters (metric, time range, tenant_id, limit)
    - Async-safe operations using `spawn_blocking`
    - Fallback to in-memory history if database unavailable
  - **GitHub OAuth2 Flow** - 100% Complete
    - In-memory state storage with TTL (10 minutes)
    - CSRF protection via state parameter verification
    - Complete OAuth2 flow (authorization → callback → token generation)
    - User mapping and JWT token generation
- **Cloud SDK Improvements** - 90% Complete
  - AWS SDK initialization (EC2, ECS, S3 clients) - 100%
  - GCP token refresh and caching - 100%
  - Azure token acquisition (Environment, CLI, Managed Identity) - 100%
  - Extended integration tests with timeout configuration - 85%
  - Cross-provider integration tests

### Changed
- **RAID Module**: Updated `rebalance()` to return `usize` (number of artifacts moved)
- **RAID Module**: Added `last_rebalance_time` tracking in `RaidManager`
- **Enterprise Monitoring**: Refactored to use transient SQLite connections via `spawn_blocking` for async safety
- **Enterprise API**: Enhanced OAuth2 flow with state management and CSRF protection
- **Test Coverage**: Increased from 410+ to 427+ tests (102 unit + 325+ integration)
  - Added 17 new integration tests for RAID strategies

### Fixed
- SQLite connection handling in async context (using `spawn_blocking`)
- SmallWorld strategy test access to private methods (made `update_clustering_coefficients()` public)
- Cross-strategy integration test isolation

### Security
- Enhanced OAuth2 flow with CSRF protection
- State parameter validation for OAuth2 callbacks
- Secure state storage with automatic TTL cleanup

## [0.1.0] - 2025-01-09

### Added
- **Complete Core Infrastructure** - 15 modules fully implemented (100%)
- **Admin Panel** - 100% UI and functionality complete
  - User Management (CRUD)
  - Tenant Management (CRUD)
  - Worker Management (CRUD)
  - VM Management (CRUD)
  - Security Management (OAuth2/SAML/Policies)
  - System Configuration (6 tabs: General, Performance, GPU, Security, Monitoring, Health)
  - Library Management (Upload, Install, Update)
  - RAID Management (Snapshot, Restore, Sync, GC)
  - Monitoring Dashboard (Real-time metrics, alerts, dashboards)
- **VM Module Enhancements**
  - GPU scheduling policies (RoundRobin, PriorityBased, LoadBased, Exclusive)
  - Advanced resource monitoring (percentiles P50/P95/P99, variance)
- **RAID Module Enhancements**
  - Snapshot & Restore functionality
  - Advanced actions (sync, GC, restore)
- **Library Management**
  - Upload functionality (base64-encoded archives)
  - Complete installation pipeline
- **Enterprise Features**
  - Multi-tenancy support
  - Audit logging (comprehensive audit trails)
  - Security management (OAuth2/SAML providers, security policies)
  - Monitoring manager (real-time metrics, alerts, dashboards)
- **Cloud Integration**
  - Kubernetes operator
  - Auto-scaling (metrics-based)
  - Load balancing (multiple strategies)
  - Multi-cloud support (AWS, Azure, GCP)
- **Docker Deployment**
  - Dockerfile (multi-stage build)
  - docker-compose.yml
  - .dockerignore
- **Kubernetes Deployment**
  - Helm charts
  - CRD definitions
  - Operator implementation
- **Deployment Testing**
  - Integration tests (15 tests)
  - Testing scripts (bash + PowerShell)
  - Testing checklist and results
- **Documentation**
  - Production deployment guides (Docker, Kubernetes, Bare Metal)
  - API documentation (OpenAPI)
  - Architecture documentation
  - Configuration guides
  - Troubleshooting guides
  - Security best practices
  - Performance tuning guides
- **Testing**
  - 410+ tests passing (102 unit + 308+ integration)
  - Comprehensive test coverage
  - Deployment integration tests
  - Failure scenario tests
  - Load tests
  - Performance benchmark tests
- **Toolchain Configuration**
  - rust-toolchain.toml
  - DLLTOOL fix documentation

### Changed
- **Project Structure**
  - API modularization (8 modules)
  - Admin Panel modularization (11 modules)
  - Improved code organization
- **Error Handling**
  - Enhanced error messages with context and suggestions
  - Structured error handling across all modules
- **Dependencies**
  - Updated all dependencies to latest versions
  - Fixed breaking changes (rand 0.9, axum 0.8, etc.)
- **Overall Progress**: 100% (all 15 modules complete)

### Fixed
- Compiler warnings (unused imports, unused variables)
- DLLTOOL issue on Windows (GNU toolchain)
- All breaking changes in dependencies
- Code formatting and linting issues

### Security
- JWT authentication
- HTTPS/TLS support
- RBAC (Role-Based Access Control)
- OAuth2/SAML integration
- Security policies
- Audit logging

## [0.1.0-pre] - 2025-12-30

### Added
- Core Module - Base structures and traits
- Pool Module - Worker pool management
- Monitoring Module - Basic metrics and monitoring
- Network Module - REST API and WebSocket with HTTPS/TLS
- Platform Module - GPU management and optimization
- TGBot Module - Telegram bot for management
- Security Module - JWT authentication, rate limiting, RBAC
- Runtime Module - Lifecycle management and process control
- Libs Module - Model library management (95% complete)
- VM Module - Virtualization and isolation (99% complete)
- RAID Module - Fault tolerance and data replication (90% complete)
- UI Module - Web interface and dashboard (99% complete)
- Rewards System - Endorphin-based achievement system
- WebSocket Security - Real-time updates with JWT authentication
- Enhanced API - Comprehensive REST endpoints (50+ endpoints)

### Changed
- Project structure organized (docs/, scripts/)
- Documentation aligned with Rust Book 2024/2025
- Git commit guidelines implemented

### Security
- JWT authentication
- HTTPS/TLS support
- Role-based access control
- Rate limiting

---

[Unreleased]: https://github.com/platinoff/poolAI/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/platinoff/poolAI/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/platinoff/poolAI/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/platinoff/poolAI/releases/tag/v0.1.0

