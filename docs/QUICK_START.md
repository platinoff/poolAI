# PoolAI Quick Start Guide

## Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Cargo**: Included with Rust
- **Git**: For cloning the repository

### Optional (for production):
- **Native toolchain**: For JWT and HTTPS features
  - Windows: MSVC or MinGW (gcc.exe, dlltool.exe) - **Use MSYS2 setup script**
  - Linux: gcc, make
  - macOS: Xcode Command Line Tools

### Windows Setup (Required for jwt/https features)

If you're building on Windows with `jwt` or `https` features, you need MSYS2 tools in your PATH:

```powershell
# Run setup script to add MSYS2 to PATH
.\scripts\setup_msys2_path.ps1

# Verify dlltool is available
dlltool --version
```

**Note**: The `setup_msys2_path.ps1` script adds MSYS2 to PATH only for the current PowerShell session. For a permanent solution, add `C:\msys64\usr\bin` to your system PATH environment variable.

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/poolai/poolai.git
cd poolai
```

### 2. Windows: Setup MSYS2 PATH (if using jwt/https features)

**Before building with jwt or https features on Windows:**

```powershell
.\scripts\setup_msys2_path.ps1
```

This adds `C:\msys64\usr\bin` to PATH for access to `dlltool.exe` and other MinGW tools.

### 3. Build the project

#### Basic build (no security features):
```bash
cargo build --release
```

#### With JWT authentication:
```bash
cargo build --release --features jwt
```

#### With HTTPS/TLS:
```bash
cargo build --release --features https
```

#### With all security features:
```bash
cargo build --release --features jwt,https
```

### 4. Run the server

#### Basic mode (HTTP, no JWT):
```bash
cargo run --release
```

#### With features:
```bash
cargo run --release --features jwt,https
```

The server will start on:
- **HTTP**: `http://localhost:8080`
- **HTTPS**: `https://localhost:8080` (with self-signed certificate)

## Configuration

### Basic Configuration

Create a `config.toml` file in the project root:

```toml
[server]
host = "0.0.0.0"
port = 8080
https_enabled = false

[security]
jwt_secret = "your-secret-key-here"
jwt_expiration = 3600  # seconds

[logging]
level = "info"
```

### HTTPS Configuration

For HTTPS, you need to generate certificates:

```bash
# Generate self-signed certificate (development only)
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
```

Then update `config.toml`:

```toml
[server]
https_enabled = true
cert_file = "cert.pem"
key_file = "key.pem"
```

## API Usage

### 1. Check Server Status

```bash
curl http://localhost:8080/api/v1/status
```

### 2. Health Check

```bash
curl http://localhost:8080/api/v1/health
```

### 3. Authentication (if JWT enabled)

```bash
curl -X POST http://localhost:8080/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "password"}'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "role": "Admin"
}
```

### 4. Access Protected Endpoints

```bash
curl http://localhost:8080/api/v1/metrics \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

### 5. Create VM Instance

```bash
curl -X POST http://localhost:8080/api/v1/vm/instances \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-vm",
    "resources": {
      "cpu_cores": 4,
      "memory_mb": 8192,
      "gpu_required": true
    },
    "isolation": "ProcessSandbox"
  }'
```

### 6. List VM Instances

```bash
curl http://localhost:8080/api/v1/vm/instances \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

### 7. Install Library

```bash
curl -X POST http://localhost:8080/api/v1/libraries/my-library/install \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{"version": "1.0.0"}'
```

## Web Interface

Access the web dashboard at:
- **HTTP**: `http://localhost:8080/ui`
- **HTTPS**: `https://localhost:8080/ui`

### Features:
- Real-time metrics dashboard
- VM instance management
- Library management
- System monitoring
- User authentication

## WebSocket API

Connect to real-time metrics stream:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws/metrics');
ws.onmessage = (event) => {
  const metrics = JSON.parse(event.data);
  console.log('Metrics:', metrics);
};
```

With authentication:
```javascript
const token = 'YOUR_TOKEN_HERE';
const ws = new WebSocket(`ws://localhost:8080/ws/metrics?token=${token}`);
```

## Testing

### Run all tests:
```bash
cargo test
```

### Run with features:
```bash
cargo test --features jwt,https
```

### Run specific test:
```bash
cargo test test_name
```

### Run integration tests:
```bash
cargo test --test integration_test_name
```

## Development

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Build Documentation

```bash
cargo doc --open
```

## Troubleshooting

### Build Errors

**Problem**: `error: linker 'cc' not found`
**Solution**: Install native toolchain:
- Windows: Install MSVC or MinGW
- Linux: `sudo apt-get install build-essential`
- macOS: `xcode-select --install`

**Problem**: `error: failed to compile ring`
**Solution**: This is a known issue with `ring` on Windows. Use MSVC toolchain or disable JWT/HTTPS features.

### Runtime Errors

**Problem**: `Address already in use`
**Solution**: Change the port in `config.toml` or stop the process using port 8080.

**Problem**: `Certificate file not found`
**Solution**: Generate certificates or disable HTTPS in `config.toml`.

### Authentication Issues

**Problem**: `401 Unauthorized`
**Solution**: 
- Check that JWT feature is enabled
- Verify token is valid and not expired
- Ensure `Authorization: Bearer TOKEN` header is included

**Problem**: `403 Forbidden`
**Solution**: Your user role doesn't have required permissions. Check RBAC configuration.

## Next Steps

- Read the [API Documentation](openapi.yaml) for complete endpoint reference
- Check [Architecture Documentation](../ARCHITECTURE.md) for system design
- Review [Security Guide](../SECURITY.md) for production deployment
- Explore [Examples](../examples/) for code samples

## Support

- **GitHub Issues**: [Report bugs](https://github.com/poolai/poolai/issues)
- **Documentation**: [Full docs](https://github.com/poolai/poolai/tree/main/docs)
- **API Reference**: [OpenAPI Spec](openapi.yaml)

