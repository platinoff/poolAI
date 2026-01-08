# Changelog

All notable changes to PoolAI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Docker deployment support (Dockerfile, docker-compose.yml, .dockerignore)
- Deployment integration tests (15 tests)
- Deployment testing scripts (bash + PowerShell)
- Deployment testing checklist and results documentation
- Final validation report
- Production deployment guides (Docker, Kubernetes, Bare Metal)
- Event sourcing batch operations for RAID module
- Circuit breaker performance optimizations
- UI polling optimization with request deduplication and retry logic
- Enhanced error handling in dependency resolution
- GitHub issue templates (bug report, feature request)
- Pull request template
- Contributing guidelines
- Security policy
- MIT License file
- Changelog

### Changed
- Updated project structure documentation
- Improved automation review
- RAID Module optimization (98% complete)
- VM Module infrastructure ready (99.5% complete)
- Libs Module completion (100% complete)
- Overall project progress: ~94% (was ~92%)
- Test coverage: 351+ tests passing (was 336+)
- Updated all dependencies to latest versions
- Fixed breaking changes in dependencies (rand 0.9, axum 0.8, etc.)
- Enhanced error messages with context and suggestions

## [0.1.0] - 2025-12-30

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

[Unreleased]: https://github.com/platinoff/poolAI/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/platinoff/poolAI/releases/tag/v0.1.0

