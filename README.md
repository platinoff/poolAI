# PoolAI - AI Mining Pool Management System

PoolAI is a comprehensive distributed system for managing AI mining pools with integration of generative models, GPU optimization, and automated resource management.

## 🎯 Development Status

**Current Phase: MVP Development**  
**Target: Working mining pool with basic management**

### 🚀 Development Roadmap

#### MVP (Weeks 1-3) - PRIORITY 1
- ✅ **Core Module** - Basic structures and traits
- 🔄 **Pool Module** - Pool and worker management  
- ⏳ **Monitoring Module** - Basic metrics and monitoring

#### Stage 2 (Weeks 4-9) - PRIORITY 2
- **Network Module** - REST API and WebSocket
- **Platform Module** - GPU management and optimization
- **TGBot Module** - Telegram bot for management

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
- **Telegram Bot**: Remote management via Telegram
- **Web Interface**: Modern dashboard for system management
- **Advanced Monitoring**: Comprehensive metrics and analytics

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --test integration
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
