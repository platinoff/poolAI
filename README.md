# PoolAI - AI Mining Pool Management System

> 🇺🇦 Ukrainian version available: [README.uk.md](../README.md)

PoolAI is a comprehensive distributed system for managing AI mining pools with integration of generative models, GPU optimization, and automated resource management.

## 🎉 **STAGE 3 COMPLETED!** 🚀

**Current Status**: Stage 3 fully implemented with advanced features  
**Next Target**: Stage 4 - Enterprise Features & Cloud Integration

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
10. **UI/UX** — Copy buttons, security links, favicon/logo, status page improvements ✅ **BASIC COMPLETED**

---

## 🎯 Development Status

**Current Phase: Stage 3 COMPLETED** 🎉  
**Target: Advanced AI Mining Pool with Enterprise Features**

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
- ✅ **VM Module** - Virtualization and isolation support
- ✅ **RAID Module** - Fault tolerance and data replication
- ✅ **UI Module** - Web interface and dashboard
- ✅ **Rewards System** - Endorphin-based achievement system
- ✅ **WebSocket Security** - Real-time updates with JWT authentication
- ✅ **Enhanced API** - Comprehensive REST endpoints

#### 🔄 Stage 4 - IN DEVELOPMENT (Q2 2025)
- **Stage 4.1: Advanced Runtime** - Process management, resource orchestration
- **Stage 4.2: Enterprise Features** - Multi-tenancy, advanced security, audit logging
- **Stage 4.3: Cloud Integration** - Kubernetes support, cloud providers, auto-scaling
- **Stage 4.4: AI/ML Enhancement** - Model optimization, AutoML, federated learning

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

### Software Requirements
- **Rust**: 1.70+ (latest stable)
- **MSYS2** (Windows): For native dependencies
- **CUDA**: 11.0+ (optional, for GPU support)
- **OpenSSL**: 1.1.1+ (for HTTPS/TLS support)
- **Certbot**: For Let's Encrypt certificates (production)

## 🛠️ Installation

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/poolai/poolai.git
   cd poolai
   ```

2. **Install dependencies**
   ```bash
   cargo build --features "stage3 https"
   ```

3. **Generate certificates (for HTTPS)**
   ```bash
   mkdir certs
   openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem -days 365 -nodes
   ```

4. **Run with Stage 3 features**
   ```bash
   cargo run --features "stage3 https"
   ```

## 🚀 Usage

### Starting the System

```bash
# Stage 3 with HTTPS
cargo run --features "stage3 https"

# With specific config
POOLAI_CONFIG_PATH=./custom_config.toml cargo run --features "stage3 https"

# With logging
RUST_LOG=debug cargo run --features "stage3 https"
```

### Current Features (Stage 3)

- **Pool Management**: Advanced worker pool with intelligent load balancing
- **Model Integration**: Core model interface and processing with library management
- **Advanced Monitoring**: System metrics, health checks, and real-time updates
- **Resource Management**: GPU and memory allocation with optimization
- **Security**: JWT authentication, HTTPS/TLS, role-based access control
- **Rewards System**: Achievement-based motivation system
- **WebSocket**: Real-time communication and live metrics
- **API**: Comprehensive REST endpoints with documentation

### Planned Features (Stage 4)

- **Enterprise Features**: Multi-tenancy, advanced security, audit logging
- **Cloud Integration**: Kubernetes support, cloud providers, auto-scaling
- **AI/ML Enhancement**: Model optimization, AutoML integration, federated learning
- **Advanced UI**: Modern dashboard with real-time monitoring
- **CI/CD**: Automated testing and deployment pipelines

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

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow the Stage 4 roadmap approach
- Focus on enterprise features and cloud integration
- Maintain clean, documented code
- Write tests for new functionality

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Issues**: [GitHub Issues](https://github.com/poolai/poolai/issues)
- **Discussions**: [GitHub Discussions](https://github.com/poolai/poolai/discussions)

## 🙏 Acknowledgments

- Rust community for the excellent ecosystem
- NVIDIA for CUDA and GPU computing tools
- All contributors and users of PoolAI

---

**PoolAI** - Empowering AI with distributed computing 🚀  
**Status**: Stage 3 COMPLETED! 🎯  
**Next Goal**: Stage 4 - Enterprise Features & Cloud Integration 🚀
