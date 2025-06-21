# PoolAI - Инструкции по сборке для Linux (Pop!_OS)

## 🐧 Поддерживаемые дистрибутивы

- **Pop!_OS 22.04+** (рекомендуется)
- **Ubuntu 20.04+**
- **Debian 11+**
- **Fedora 36+**
- **Arch Linux**

## 📋 Системные требования

### Минимальные требования
- **CPU**: 4 ядра, 2.0 GHz
- **RAM**: 8 GB
- **GPU**: NVIDIA GTX 1060+ или AMD RX 580+
- **Диск**: 20 GB свободного места
- **Сеть**: Стабильное интернет-соединение

### Рекомендуемые требования
- **CPU**: 8 ядер, 3.0 GHz+
- **RAM**: 16 GB+
- **GPU**: NVIDIA RTX 4090+ или AMD RX 7900 XTX+
- **Диск**: 50 GB+ SSD
- **Сеть**: 1 Gbps+

## 🚀 Быстрая установка (Pop!_OS)

### 1. Обновление системы

```bash
sudo apt update && sudo apt upgrade -y
sudo apt autoremove -y
```

### 2. Установка Rust

```bash
# Установка Rust через rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Перезагрузка shell или выполнение:
source ~/.cargo/env

# Проверка установки
rustc --version
cargo --version
```

### 3. Установка системных зависимостей

```bash
# Основные зависимости
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    cmake \
    git \
    curl \
    wget \
    unzip \
    libudev-dev \
    libusb-1.0-0-dev \
    libpulse-dev \
    libasound2-dev \
    libx11-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libgles2-mesa-dev \
    libegl1-mesa-dev \
    libwayland-dev \
    libxkbcommon-dev \
    libdbus-1-dev \
    libudev-dev \
    libinput-dev \
    libxss-dev \
    libxtst-dev

# Для GPU поддержки (NVIDIA)
sudo apt install -y \
    nvidia-driver-535 \
    nvidia-cuda-toolkit \
    nvidia-cuda-dev

# Для GPU поддержки (AMD)
sudo apt install -y \
    mesa-utils \
    mesa-vulkan-drivers \
    vulkan-tools \
    vulkan-validationlayers
```

### 4. Установка дополнительных инструментов

```bash
# Установка дополнительных утилит
sudo apt install -y \
    htop \
    iotop \
    nvtop \
    nvidia-smi \
    radeontop \
    lm-sensors \
    fancontrol \
    smartmontools \
    hdparm \
    nvme-cli \
    fio \
    stress-ng \
    sysbench \
    glmark2 \
    glxgears \
    vkmark \
    vkcube

# Настройка мониторинга
sudo sensors-detect --auto
```

### 5. Клонирование репозитория

```bash
# Клонирование проекта
git clone https://github.com/platinoff/poolAI.git
cd poolAI

# Переключение на нужную ветку
git checkout beta_bolvanka_v1
```

### 6. Сборка проекта

```bash
# Очистка предыдущих сборок
cargo clean

# Сборка в release режиме
cargo build --release

# Проверка сборки
cargo test --release

# Проверка кода
cargo clippy --release
cargo fmt --check
```

### 7. Создание конфигурации

```bash
# Создание конфигурационного файла
cat > config.toml << 'EOF'
[system]
version = "Beta_bolvanka_v1"
debug = false
log_level = "info"

[gpu]
enabled = true
optimization = true
memory_limit = 16384  # MB
cuda_version = "12.0"

[models]
default_model = "gpt-3.5-turbo"
max_models = 10
auto_load = true
model_path = "./models"

[pool]
name = "PoolAI Beta"
description = "AI-powered mining pool"
max_workers = 1000
reward_algorithm = "ai_optimized"

[telegram]
enabled = true
token = "YOUR_BOT_TOKEN"
admin_users = ["your_telegram_id"]

[web]
enabled = true
host = "0.0.0.0"
port = 8080
tls_enabled = false

[monitoring]
enabled = true
metrics_port = 9090
alert_email = "admin@poolai.com"

[raid]
enabled = true
replication_factor = 3
auto_repair = true
EOF
```

### 8. Запуск системы

```bash
# Запуск в обычном режиме
cargo run --release -- --config config.toml

# Запуск в фоновом режиме
nohup cargo run --release -- --config config.toml > poolai.log 2>&1 &

# Запуск с отладкой
RUST_LOG=debug cargo run --release -- --config config.toml

# Запуск с профилированием
RUST_LOG=info cargo run --release -- --config config.toml
```

## 🔧 Дополнительная настройка

### Настройка GPU

#### NVIDIA GPU

```bash
# Проверка драйверов
nvidia-smi

# Установка CUDA (если не установлено)
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-keyring_1.0-1_all.deb
sudo dpkg -i cuda-keyring_1.0-1_all.deb
sudo apt-get update
sudo apt-get install cuda

# Настройка переменных окружения
echo 'export PATH=/usr/local/cuda/bin:$PATH' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc
```

#### AMD GPU

```bash
# Проверка драйверов
glxinfo | grep "OpenGL renderer"
vulkaninfo | grep "GPU"

# Установка ROCm (для AMD)
wget -q -O - https://repo.radeon.com/rocm/rocm.gpg.key | sudo apt-key add -
echo 'deb [arch=amd64] https://repo.radeon.com/rocm/apt/debian ubuntu main' | sudo tee /etc/apt/sources.list.d/rocm.list
sudo apt update
sudo apt install rocm-hip-sdk
```

### Настройка мониторинга

```bash
# Установка Prometheus
wget https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
tar xvf prometheus-*.tar.gz
cd prometheus-*

# Создание конфигурации Prometheus
cat > prometheus.yml << 'EOF'
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'poolai'
    static_configs:
      - targets: ['localhost:9090']
EOF

# Запуск Prometheus
./prometheus --config.file=prometheus.yml
```

### Настройка systemd сервиса

```bash
# Создание systemd сервиса
sudo tee /etc/systemd/system/poolai.service << 'EOF'
[Unit]
Description=PoolAI Mining Pool System
After=network.target

[Service]
Type=simple
User=poolai
WorkingDirectory=/opt/poolai
ExecStart=/opt/poolai/target/release/poolai --config /opt/poolai/config.toml
Restart=always
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

# Создание пользователя
sudo useradd -r -s /bin/false poolai
sudo mkdir -p /opt/poolai
sudo cp target/release/poolai /opt/poolai/
sudo cp config.toml /opt/poolai/
sudo chown -R poolai:poolai /opt/poolai

# Включение и запуск сервиса
sudo systemctl daemon-reload
sudo systemctl enable poolai
sudo systemctl start poolai
sudo systemctl status poolai
```

## 🐳 Docker установка

### Создание Dockerfile

```dockerfile
FROM rust:1.75 as builder

WORKDIR /usr/src/poolai
COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/poolai/target/release/poolai /usr/local/bin/poolai

EXPOSE 8080 9090

CMD ["poolai"]
```

### Сборка и запуск Docker

```bash
# Сборка образа
docker build -t poolai:beta-bolvanka-v1 .

# Запуск контейнера
docker run -d \
  --name poolai \
  --gpus all \
  -p 8080:8080 \
  -p 9090:9090 \
  -v /path/to/config:/app/config \
  -v /path/to/models:/app/models \
  poolai:beta-bolvanka-v1
```

## 🔍 Диагностика проблем

### Проверка системы

```bash
# Проверка Rust
rustc --version
cargo --version

# Проверка GPU
nvidia-smi  # для NVIDIA
radeontop   # для AMD

# Проверка памяти
free -h
df -h

# Проверка сети
ping -c 4 8.8.8.8
curl -I https://github.com

# Проверка портов
netstat -tulpn | grep :8080
netstat -tulpn | grep :9090
```

### Логи и отладка

```bash
# Просмотр логов
tail -f poolai.log

# Просмотр systemd логов
sudo journalctl -u poolai -f

# Запуск с подробным логированием
RUST_LOG=debug cargo run --release -- --config config.toml

# Проверка производительности
cargo bench
```

### Частые проблемы

#### Ошибка: "linker 'cc' not found"
```bash
sudo apt install build-essential
```

#### Ошибка: "libssl not found"
```bash
sudo apt install libssl-dev pkg-config
```

#### Ошибка: "CUDA not found"
```bash
# Установка CUDA toolkit
sudo apt install nvidia-cuda-toolkit
export PATH=/usr/local/cuda/bin:$PATH
export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH
```

#### Ошибка: "Permission denied"
```bash
# Проверка прав доступа
sudo chown -R $USER:$USER /opt/poolai
chmod +x /opt/poolai/poolai
```

## 📊 Мониторинг производительности

### Системные метрики

```bash
# Мониторинг CPU
htop
top

# Мониторинг GPU
nvidia-smi -l 1
nvtop

# Мониторинг памяти
free -h
cat /proc/meminfo

# Мониторинг диска
iotop
df -h
```

### Сетевые метрики

```bash
# Мониторинг сети
iftop
nethogs
ss -tulpn

# Тест пропускной способности
iperf3 -s  # сервер
iperf3 -c localhost  # клиент
```

## 🚀 Оптимизация производительности

### Настройка ядра

```bash
# Добавление в /etc/sysctl.conf
echo 'vm.swappiness=10' | sudo tee -a /etc/sysctl.conf
echo 'vm.dirty_ratio=15' | sudo tee -a /etc/sysctl.conf
echo 'vm.dirty_background_ratio=5' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

### Настройка GPU

```bash
# Для NVIDIA
sudo nvidia-smi -pm 1  # включение persistent mode
sudo nvidia-smi -ac 1215,1410  # установка частот

# Для AMD
echo 'high' | sudo tee /sys/class/drm/card0/device/power_dpm_force_performance_level
```

### Настройка сети

```bash
# Оптимизация TCP
echo 'net.core.rmem_max=134217728' | sudo tee -a /etc/sysctl.conf
echo 'net.core.wmem_max=134217728' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv4.tcp_rmem=4096 87380 134217728' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv4.tcp_wmem=4096 65536 134217728' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

## 📞 Поддержка

Если у вас возникли проблемы:

1. **Проверьте логи**: `tail -f poolai.log`
2. **Проверьте системные требования**
3. **Создайте issue на GitHub**: https://github.com/platinoff/poolAI/issues
4. **Напишите на email**: platinovubuntu@gmail.com
5. **Telegram**: @platinov

---

**Удачной сборки PoolAI на Pop!_OS!** 🐧🚀 