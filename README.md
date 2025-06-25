# PoolAI - AI Mining Pool Management System

PoolAI is a comprehensive distributed system for managing AI mining pools with integration of generative models, GPU optimization, and automated resource management.

## 🚀 Features

### Core Features
- **Model Integration**: Support for multiple generative models (GPT, BERT, T5, LLaMA)
- **GPU Management**: Advanced GPU resource allocation and optimization
- **Distributed Computing**: Scalable pool management with load balancing
- **Real-time Monitoring**: Comprehensive metrics and health monitoring
- **Web Interface**: Modern dashboard for system management
- **API Support**: RESTful API and WebSocket endpoints
- **Telegram Bot**: Remote management via Telegram

### Advanced Features
- **Virtual Machine Management**: VM creation and GPU passthrough
- **RAID Management**: Storage optimization and redundancy
- **Library Management**: Dynamic model library loading
- **Platform Abstraction**: Cross-platform support (Linux, Windows)
- **Auto-scaling**: Intelligent resource scaling based on demand
- **Fault Tolerance**: Automatic recovery and redundancy

## 📋 Requirements

### System Requirements
- **OS**: Linux (Ubuntu 20.04+) or Windows 10+
- **CPU**: 4+ cores recommended
- **RAM**: 8GB+ recommended
- **Storage**: 50GB+ available space
- **GPU**: NVIDIA GPU with CUDA support (optional)

### Software Requirements
- **Rust**: 1.70+ (latest stable)
- **Docker**: 20.10+ (optional, for containerization)
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

### Docker Installation

```bash
# Build Docker image
docker build -t poolai .

# Run with GPU support
docker run --gpus all -p 8080:8080 -p 3000:3000 poolai

# Run without GPU
docker run -p 8080:8080 -p 3000:3000 poolai
```

## 🏗️ Architecture

### Core Modules

```
poolai/
├── core/           # Core functionality and interfaces
├── pool/           # Pool management and load balancing
├── runtime/        # Model runtime and instance management
├── monitoring/     # Metrics collection and health monitoring
├── network/        # API and network communication
├── platform/       # Platform-specific optimizations
├── ui/             # Web interface and dashboard
├── libs/           # Model library management
├── vm/             # Virtual machine management
├── raid/           # Storage and RAID management
└── tgbot/          # Telegram bot integration
```

### Key Components

- **Model Interface**: Unified interface for all AI models
- **Resource Manager**: Intelligent resource allocation
- **Load Balancer**: Request distribution across workers
- **Health Monitor**: Real-time system health tracking
- **API Gateway**: RESTful API and WebSocket endpoints

## 🔧 Configuration

### Basic Configuration

Create `config.toml` in the project root:

```toml
[pool]
max_workers = 10
max_queue_size = 1000
auto_scaling = true
scaling_threshold = 0.8

[network]
host = "127.0.0.1"
port = 8080
enable_ssl = false

[ui]
host = "127.0.0.1"
port = 3000
theme = "dark"

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

### API Endpoints

- **Health Check**: `GET /api/v1/health`
- **System Status**: `GET /api/v1/status`
- **Model List**: `GET /api/v1/models`
- **Generate Text**: `POST /api/v1/models/{model}/generate`
- **Metrics**: `GET /api/v1/metrics`

### Web Interface

Access the dashboard at `http://localhost:3000`

### Telegram Bot

Configure your bot token in the config and use commands:
- `/start` - Start the bot
- `/status` - Get system status
- `/metrics` - Get system metrics
- `/help` - Show available commands

## 🔍 Monitoring

### Metrics Available

- **System Metrics**: CPU, Memory, Disk, Network
- **GPU Metrics**: Utilization, Memory, Temperature
- **Model Metrics**: Throughput, Latency, Accuracy
- **Pool Metrics**: Active workers, Queue size, Success rate

### Dashboard Features

- Real-time system monitoring
- Interactive charts and graphs
- Alert management
- Resource utilization tracking
- Performance analytics

## 🚀 Deployment

### Production Deployment

1. **Build for production**
   ```bash
   cargo build --release --features full
   ```

2. **Set up systemd service**
   ```bash
   sudo cp poolai.service /etc/systemd/system/
   sudo systemctl enable poolai
   sudo systemctl start poolai
   ```

3. **Configure reverse proxy** (Nginx example)
   ```nginx
   server {
       listen 80;
       server_name your-domain.com;
       
       location / {
           proxy_pass http://127.0.0.1:3000;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
       }
       
       location /api {
           proxy_pass http://127.0.0.1:8080;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
       }
   }
   ```

### Docker Compose

```yaml
version: '3.8'
services:
  poolai:
    build: .
    ports:
      - "8080:8080"
      - "3000:3000"
    volumes:
      - ./data:/app/data
      - ./config.toml:/app/config.toml
    environment:
      - RUST_LOG=info
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
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

### Performance Tests

```bash
cargo bench
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

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

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [docs.poolai.dev](https://docs.poolai.dev)
- **Issues**: [GitHub Issues](https://github.com/poolai/poolai/issues)
- **Discussions**: [GitHub Discussions](https://github.com/poolai/poolai/discussions)
- **Email**: support@poolai.dev

## 🙏 Acknowledgments

- Rust community for the excellent ecosystem
- NVIDIA for CUDA and GPU computing tools
- All contributors and users of PoolAI

---

**PoolAI** - Empowering AI with distributed computing 🚀 