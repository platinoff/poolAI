# TLS 1.3 (PH-S08 / FM-044)

**Status:** HTTPS listener uses explicit rustls `ServerConfig` with TLS **1.3** by default; optional TLS **1.2** when `https.tls_min_version = "1.2"`.

## Build

```bash
cargo build --release --features https,enterprise
```

Enable HTTPS in config (`https.enabled = true`) or set paths via env.

## Configuration

| Field / env | Purpose |
|-------------|---------|
| `https.enabled` | Start rustls listener instead of plain HTTP |
| `https.cert_path` / `HTTPS_CERT_PATH` | Server certificate PEM (default `certs/cert.pem`) |
| `https.key_path` / `HTTPS_KEY_PATH` | Private key PEM (default `certs/key.pem`) |
| `https.tls_min_version` | `"1.3"` (default) or `"1.2"` for backward compatibility |
| `https.tls_max_version` | `"1.3"` (default); `"2.0"` rejected until rustls supports it |
| `https.hsts_enabled` / `https.hsts_max_age` | `Strict-Transport-Security` via security-headers middleware |
| `HTTPS_CERT_RELOAD_SECS` | Optional periodic PEM reload (cert rotation without restart) |

## Certificate rotation

1. Replace PEM files on disk (same paths).
2. Either restart `poolai` or set `HTTPS_CERT_RELOAD_SECS` (e.g. `3600`) for automatic reload via `RustlsConfig::reload_from_config`.

## Development certificates

PEMs under `certs/` are **local only** (gitignored). See [`certs/README.md`](../../certs/README.md).

```bash
mkdir -p certs
openssl req -x509 -newkey rsa:2048 -keyout certs/key.pem -out certs/cert.pem -days 365 -nodes -subj "/CN=localhost"
```

TLS integration tests skip when PEMs are absent (`tests/tls_https_integration.rs`).

## Related

- Policy module: `src/network/tls_config.rs`
- Listener: `src/network/mod.rs` (`feature = "https"`)
- Secrets hygiene: [`SECRETS_MANAGEMENT.md`](./SECRETS_MANAGEMENT.md) §1 (PH-SVC55)
- Legacy planning: [`../development/TLS_UPGRADE_PLAN.md`](../development/TLS_UPGRADE_PLAN.md)
- Prometheus scrape (unchanged on HTTP or HTTPS): [`../development/PROMETHEUS_METRICS.md`](../development/PROMETHEUS_METRICS.md)

**Last updated:** 2026-07-24 (PH-SVC55 — untrack PEMs).
