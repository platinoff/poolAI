# PoolAI - AI Mining Pool Management System

> 🇺🇦 Ukrainian version available: [README.uk.md](../README.md)

PoolAI is a comprehensive distributed system for managing AI mining pools with integration of generative models, GPU optimization, and automated resource management.

## ⚡️ Architectural Improvement Plan (2025)

1. **Healthcheck endpoint** — /api/v1/health for CI/CD and monitoring
2. **Global version/uptime state** — implemented via `version.rs` module (read-only, no global mutable static, Rust Book best practices)
3. **Public API exported only from lib.rs** — all internals private, rustdoc for public traits/structs
4. **JWT & RBAC** — middleware for token and role checks (admin/operator/viewer)
5. **Endpoint access restriction** — /metrics, /workers, /shutdown only for authorized users
6. **CI/CD** — GitHub Actions workflow for tests and builds
7. **Swagger/OpenAPI** — API spec generation and publication
8. **Documentation** — Quick Start, curl examples, security section
9. **Live metrics (WebSocket)** — /ws/metrics for real-time monitoring
10. **UI/UX** — Copy buttons, security links, favicon/logo, status page improvements

---

## Example version.rs
```rust
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = option_env!("BUILD_TIME").unwrap_or("unknown");
```

## Example rustdoc for public trait
```rust
/// Interface for all models (trait-based OOP)
pub trait ModelInterface: Send + Sync {
    fn process(&self, input: &ModelInput) -> ModelOutput;
    fn info(&self) -> ModelInfo;
}
```

---

## Roadmap (updated)

- **MVP**: core, pool, monitoring
- **Stage 2**: network (REST+WebSocket+HTTPS), platform, tgbot, security (CORS, rate limiting)
- **Stage 2.1**: Healthcheck endpoint, global version/uptime state
- **Stage 2.2**: JWT+RBAC, endpoint access restriction
- **Stage 2.3**: CI/CD, Swagger/OpenAPI, documentation improvements
- **Stage 2.4**: Live metrics (WebSocket), UI/UX improvements
- **Stage 3**: runtime, libs, vm, raid, ui (web interface)

---

## 🎯 Development Status

**Current Phase: MVP Development**  
**Target: Working mining pool with basic management**

### 🚀 Development Roadmap

#### MVP (Weeks 1-3) - PRIORITY 1
- ✅ **Core Module** - Basic structures and traits
- 🔄 **Pool Module** - Pool and worker management  
- ⏳ **Monitoring Module** - Basic metrics and monitoring

#### Stage 2 (Weeks 4-9) - PRIORITY 2
- **Network Module** - REST API and WebSocket with HTTPS/TLS support
- **Platform Module** - GPU management and optimization
- **TGBot Module** - Telegram bot for management
- **Security Module** - JWT authentication, rate limiting, and certificate management

#### Stage 3 (Weeks 10+) - PRIORITY 3
- **Runtime Module** - Lifecycle management
- **Libs Module** - Model library management
- **VM Module** - Virtualization and isolation
- **RAID Module** - Fault tolerance and replication
- **UI Module** - Web interface

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

2. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **Build the project**
   ```bash
   cargo build --release
   ```

4. **Run PoolAI**
   ```bash
   ./target/release/poolai
   ```

### Windows Setup

For Windows users, ensure MSYS2 is installed and the toolchain is properly configured:

```bash
# Install MSYS2 from https://www.msys2.org/
# Add MSYS2 to PATH
# Install required packages
pacman -S mingw-w64-x86_64-toolchain
pacman -S mingw-w64-x86_64-cmake
pacman -S mingw-w64-x86_64-pkg-config
```

## 🏗️ Architecture

### Current MVP Modules

```
poolai/
├── core/           # Core functionality and interfaces
├── pool/           # Pool management and load balancing
└── monitoring/     # Metrics collection and health monitoring
```

### Planned Modules (Future Stages)

```
poolai/
├── network/        # API and network communication
├── platform/       # Platform-specific optimizations
├── tgbot/          # Telegram bot integration
├── runtime/        # Model runtime and instance management
├── libs/           # Model library management
├── vm/             # Virtual machine management
├── raid/           # Storage and RAID management
└── ui/             # Web interface and dashboard
```

### Key Components

- **Model Interface**: Unified interface for all AI models
- **Resource Manager**: Intelligent resource allocation
- **Load Balancer**: Request distribution across workers
- **Health Monitor**: Real-time system health tracking

## 🔧 Configuration

### Basic Configuration

Create `config.toml` in the project root:

```toml
[pool]
max_workers = 10
max_queue_size = 1000
auto_scaling = true
scaling_threshold = 0.8

[monitoring]
enable_metrics = true
metrics_interval = 30

[gpu]
enable_gpu_passthrough = true
max_gpu_memory = 8192

[security]
# HTTPS/TLS Configuration (Stage 2+)
enable_https = false  # Set to true for production
cert_path = "/etc/poolai/certs/cert.pem"
key_path = "/etc/poolai/certs/key.pem"
redirect_http_to_https = true

# Authentication
jwt_secret = "your-super-secret-key-change-in-production"
jwt_expiration = 3600
rate_limit_requests = 100
rate_limit_window = 60

# CORS Settings
cors_origins = ["http://localhost:3000", "https://poolai.example.com"]
cors_methods = ["GET", "POST", "PUT", "DELETE"]
cors_headers = ["Authorization", "Content-Type"]
```

### Environment Variables

```bash
export POOLAI_LOG_LEVEL=info
export POOLAI_CONFIG_PATH=./config.toml
export POOLAI_DATA_DIR=./data
```

## 📊 Usage

### Starting the System

```bash
# Basic startup
cargo run

# With specific config
POOLAI_CONFIG_PATH=./custom_config.toml cargo run

# With logging
RUST_LOG=debug cargo run
```

### Current Features (MVP)

- **Pool Management**: Basic worker pool with load balancing
- **Model Integration**: Core model interface and processing
- **Basic Monitoring**: System metrics and health checks
- **Resource Management**: GPU and memory allocation

### Planned Features (Future Stages)

- **REST API**: Full API endpoints for model management
- **WebSocket**: Real-time updates and communication
- **HTTPS/TLS**: Secure communication with automatic certificate management
- **Telegram Bot**: Remote management via Telegram
- **Web Interface**: Modern dashboard for system management
- **Advanced Monitoring**: Comprehensive metrics and analytics

## 🔒 Security & HTTPS

### Security Architecture

PoolAI implements a comprehensive security model with multiple deployment options:

#### Development Mode (HTTP)
- HTTP on localhost for development
- Basic authentication
- CORS enabled for local development

#### Production Mode (HTTPS)
- TLS 1.3 encryption for all communications
- Automatic certificate management with Let's Encrypt
- HSTS headers for enhanced security
- Rate limiting and DDoS protection

### Deployment Options

#### Option A: Built-in HTTPS (Recommended)
```
Internet → PoolAI (HTTPS:443) → Internal Services
```
- TLS termination within PoolAI
- Automatic certificate renewal
- Simplified deployment

#### Option B: Reverse Proxy (Enterprise)
```
Internet → Nginx/Apache (HTTPS:443) → PoolAI (HTTP:8080)
```
- Centralized certificate management
- Additional security layers
- Load balancing capabilities

### Security Features

- **Authentication**: JWT-based API authentication
- **Authorization**: Role-based access control (Admin, Operator, Viewer)
- **Encryption**: TLS 1.3 for transport, AES-256 for data at rest
- **Rate Limiting**: Configurable request limits
- **CORS**: Configurable cross-origin resource sharing
- **Security Headers**: HSTS, CSP, X-Frame-Options
- **WebSocket Security**: WSS with JWT authentication

### Certificate Management

```bash
# Generate self-signed certificate for development
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Let's Encrypt automatic setup (production)
# PoolAI will automatically handle certificate renewal
```

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
```

### Development Setup

```bash
# Install development dependencies
cargo install cargo-watch
cargo install cargo-audit

# Run with hot reload
cargo watch -x run

# Check for security vulnerabilities
cargo audit
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow the MVP-first approach
- Focus on core functionality before adding features
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
