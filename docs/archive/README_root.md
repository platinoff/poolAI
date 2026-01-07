# PoolAI

AI Mining Pool Management System

## Features
- Modular Rust architecture: core, pool, monitoring, network, platform, tgbot
- Configurable via `config.toml` ([system], [gpu], [pool], [monitoring], [version], [health])
- Automatic build time via build.rs
- HTTPS server with self-signed certificates (dev)
- JWT authentication & RBAC (planned)
- Live metrics via WebSocket (planned)
- CI/CD via GitHub Actions (planned)
- Swagger/OpenAPI (planned)

## Quick Start
1. Generate self-signed certs: see docs/ or use `openssl`
2. Edit `config.toml` (see example in repo)
3. Run with HTTPS:
   ```sh
   cargo run --features "stage2 https"
   ```

## Security
- Never commit private keys to git!
- Use env vars for secrets in production
- For dev: use self-signed certs

## API
- `/api/v1/status` — status (HTML/JSON)
- `/api/v1/metrics` — metrics
- `/api/v1/models` — models
- `/api/v1/gpu` — GPU info
- `/api/v1/workers` — workers
- `/ws/metrics` — live metrics (WebSocket, planned)

## Roadmap
- Healthcheck endpoint
- JWT/RBAC
- Swagger/OpenAPI
- CI/CD workflow
- UI/UX improvements

## Documentation
- [README.uk.md](./README.uk.md) — українською
- [poolAI_concept.txt](./poolAI_concept.txt) — концепт

---

© 2025 PoolAI Team 