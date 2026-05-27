# Security Hardening Guide

## Overview

This document provides a comprehensive guide for security hardening of PoolAI, covering OWASP Top 10 vulnerabilities, input validation, dependency security, and production security best practices.

## Security Audit Checklist

### 1. Dependency Security Audit

#### Install Security Audit Tools

```bash
# Install cargo-audit
cargo install cargo-audit

# Install cargo-deny (license compliance)
cargo install cargo-deny
```

#### Run Security Audits

```bash
# Check for vulnerable dependencies
cargo audit

# Check for license issues
cargo deny check licenses

# Check for duplicate dependencies
cargo deny check duplicates

# Check for banned crates
cargo deny check bans
```

#### CI/CD Integration

Security audits are automatically run in CI pipeline:
- `cargo audit` - Checks for known vulnerabilities
- `cargo-deny` - Checks licenses and bans

**Status**: ✅ Integrated in `.github/workflows/ci.yml`

### 2. OWASP Top 10 Security Review

#### A01:2021 – Broken Access Control

**Current Implementation**:
- ✅ JWT-based authentication
- ✅ Role-Based Access Control (RBAC)
- ✅ Permission middleware for API endpoints
- ✅ Token expiration and validation

**Review Checklist**:
- [x] API endpoints require authentication
- [x] Role-based access enforced (`check_permission()`)
- [x] Token expiration checked
- [x] Permission validation in middleware
- [ ] Rate limiting on authentication endpoints
- [ ] Token revocation mechanism

**Recommendations**:
- Add rate limiting to `/api/v1/login` endpoint (max 5 attempts per minute)
- Implement token blacklist for logout/revocation
- Add audit logging for failed authentication attempts

#### A02:2021 – Cryptographic Failures

**Current Implementation**:
- ✅ JWT tokens signed with HS256/RS256
- ✅ HTTPS/TLS support with strong ciphers
- ✅ Secure secret management (environment variables)

**Review Checklist**:
- [x] Secrets not hardcoded
- [x] TLS 1.2+ support
- [x] Strong cipher suites
- [ ] Secrets rotation mechanism
- [ ] Certificate auto-renewal

**Recommendations**:
- Implement secrets rotation for JWT keys
- Use secure secret storage (e.g., HashiCorp Vault, AWS Secrets Manager)
- Enable certificate auto-renewal with Let's Encrypt

#### A03:2021 – Injection

**Current Implementation**:
- ✅ Input validation in API handlers
- ✅ Serialized types for request/response (serde)
- ✅ Type-safe parsing

**Review Checklist**:
- [x] No raw SQL queries (no database used)
- [x] Request deserialization with serde
- [x] Type validation for all inputs
- [ ] Input sanitization for file uploads
- [ ] Path traversal validation for file operations

**Recommendations**:
- Add path traversal validation for file paths
- Sanitize library upload file names
- Validate file sizes and types

#### A04:2021 – Insecure Design

**Current Implementation**:
- ✅ Multi-layer security architecture
- ✅ Defense in depth
- ✅ Secure defaults

**Review Checklist**:
- [x] Security architecture documented
- [x] Threat modeling considered
- [x] Secure defaults configured
- [ ] Security review for new features
- [ ] Security testing in CI/CD

**Recommendations**:
- Add security review checklist for pull requests
- Include security testing in CI/CD pipeline
- Regular security architecture reviews

#### A05:2021 – Security Misconfiguration

**Current Implementation**:
- ✅ Configuration via TOML files
- ✅ Environment variable overrides
- ✅ Secure default configuration

**Review Checklist**:
- [x] No default passwords
- [x] Error messages don't leak sensitive info
- [x] Unnecessary features disabled by default
- [ ] Security headers configured
- [ ] CORS properly configured

**Recommendations**:
- Add security headers middleware (HSTS, CSP, X-Frame-Options)
- Configure CORS whitelist for production
- Disable debug endpoints in production

#### A06:2021 – Vulnerable and Outdated Components

**Current Implementation**:
- ✅ `cargo audit` for vulnerability scanning
- ✅ Regular dependency updates
- ✅ `Cargo.lock` committed for reproducible builds

**Review Checklist**:
- [x] Dependencies audited regularly
- [x] `Cargo.lock` committed
- [ ] Dependency update schedule
- [ ] Automated dependency updates

**Recommendations**:
- Enable Dependabot for automated dependency updates
- Schedule monthly dependency review
- Monitor `cargo audit` reports

#### A07:2021 – Identification and Authentication Failures

**Current Implementation**:
- ✅ JWT-based authentication
- ✅ Password hashing (bcrypt recommended)
- ✅ Token expiration

**Review Checklist**:
- [x] JWT tokens with expiration
- [x] Role-based authorization
- [ ] Password strength requirements
- [ ] Account lockout after failed attempts
- [ ] Multi-factor authentication (MFA)

**Recommendations**:
- Add password strength validation
- Implement account lockout after 5 failed login attempts
- Consider adding MFA support (TOTP)

#### A08:2021 – Software and Data Integrity Failures

**Current Implementation**:
- ✅ Library integrity checks (checksums)
- ✅ Secure file uploads
- ✅ Code signing (Git commits)

**Review Checklist**:
- [x] File integrity checks for library uploads
- [x] Checksum verification
- [x] Signed releases (Galaxy §9.2 — **`poolai-verify-release`**, PH-S66)
- [ ] Dependency pinning

**Recommendations**:
- Add GPG/minisign release pipeline in CI (verify via `cargo run --bin poolai-verify-release`)
- Implement signed dependency verification
- Add checksum verification for distributed artifacts

**Implemented (PH-S66):** [`src/release/`](../../src/release/) + binary **`poolai-verify-release`** — ed25519 signature over raw manifest JSON, optional artifact SHA-256 vs manifest `artifacts[]`. Trust root: `maintainer_keys.json` (`key_id` → `public_key_hex`) or `--public-key-hex`.

#### Galaxy governance canonical pointers (PH-S69, PH-S77)

**Single source** for Galaxy §9.2 / §9.3 / §9.6 policy links in `docs/security/`. Operator runbooks below reference this hub — do not duplicate governance prose elsewhere in this file.

**INDEX (PH-S87):** canonical [крок 8 (CI)](../INDEX_2026-03-17.md#0-канонічний-порядок-читання) · [§7 Monitoring / Security / CI](../INDEX_2026-03-17.md#7-monitoring--performance--security--ci) — [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md).

| Topic | Galaxy (canon) | PoolAI ops |
|-------|----------------|------------|
| **§9.2** Signed releases | [`POOLAI_GALAXY_GRID.md` §9.2](../concept/POOLAI_GALAXY_GRID.md#92-signed-releases-канон-ph-s63) | `poolai-verify-release` (quickstart below); [`src/release/`](../../src/release/) |
| **§9.3** Protocol compat | [`POOLAI_GALAXY_GRID.md` §9.3](../concept/POOLAI_GALAXY_GRID.md#93-protocol-versioning-та-compat-matrix) | PH-S65 wire (`protocol_version`, `build_id`); FM [§5.12](../catalog/FUNCTION_MANAGEMENT.md#512-research-backlog-ph-s65-galaxy-wire--ops-2026-05-27) |
| **§9.6** Advisories & keys | [`POOLAI_GALAXY_GRID.md` §9.6](../concept/POOLAI_GALAXY_GRID.md#96-security-advisories-та-key-rotation) | [`DEPENDENCY_SECURITY.md`](./DEPENDENCY_SECURITY.md) (RustSec / `cargo audit` workflow) |

**Verify-release CLI:** `cargo run --bin poolai-verify-release -- --manifest <path> --signature <path> [--artifact <path>]`

#### Operator quickstart: verify signed release (PH-S71)

Minimal verification sequence before rollout (policy: [Galaxy §9.2](#galaxy-governance-canonical-pointers-ph-s69-ph-s77)):

1. Prepare trust root (`maintainer_keys.json`) or pass a pinned key via `--public-key-hex`.
2. Verify release manifest signature:
   `cargo run --bin poolai-verify-release -- --manifest <release-manifest.json> --signature <release-manifest.sig>`
3. Verify artifact integrity against manifest (recommended):
   `cargo run --bin poolai-verify-release -- --manifest <release-manifest.json> --signature <release-manifest.sig> --artifact <poolai-binary-or-archive>`
4. Roll out only after signature + artifact verification are both successful.

#### Dev fixtures (PH-S85)

Repo-local sample paths (ed25519 key **`poolai-dev`**, dev-only — **не** production trust root):

| Path | Role |
|------|------|
| [`tests/fixtures/release/dev/maintainer_keys.json`](../../tests/fixtures/release/dev/maintainer_keys.json) | Trust root |
| [`tests/fixtures/release/dev/release-manifest.json`](../../tests/fixtures/release/dev/release-manifest.json) | Manifest |
| [`tests/fixtures/release/dev/release-manifest.json.sig`](../../tests/fixtures/release/dev/release-manifest.json.sig) | Signature envelope |
| [`tests/fixtures/release/dev/poolai-sample.bin`](../../tests/fixtures/release/dev/poolai-sample.bin) | Sample artifact |

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
cd /s/rust/poolAI
FIX=tests/fixtures/release/dev

cargo run --bin poolai-verify-release -- \
  --manifest "$FIX/release-manifest.json" \
  --signature "$FIX/release-manifest.json.sig" \
  --trust-root "$FIX/maintainer_keys.json"

cargo run --bin poolai-verify-release -- \
  --manifest "$FIX/release-manifest.json" \
  --signature "$FIX/release-manifest.json.sig" \
  --trust-root "$FIX/maintainer_keys.json" \
  --artifact "$FIX/poolai-sample.bin" \
  --artifact-name poolai
```

Локальний запуск coordinator після `run-poolai` build — [`RUN_LOCAL.md`](../development/RUN_LOCAL.md) § verify-release. Регенерація fixtures: `cargo test --lib release::verify::tests::write_dev_release_fixtures -- --ignored --exact` (див. [`tests/fixtures/release/dev/README.md`](../../tests/fixtures/release/dev/README.md)).

#### Operator checklist: protocol compatibility triage (PH-S72)

When worker registration fails because of protocol mismatch (policy: [Galaxy §9.3](#galaxy-governance-canonical-pointers-ph-s69-ph-s77)):

1. Confirm worker sends expected wire fields (`protocol_version`, `build_id`) via discovery register flow (PH-S65 baseline).
2. If coordinator returns HTTP `403` or `426`, treat it as compat negotiation failure and stop rollout for that worker build.
3. Check returned `compat_status` against the active protocol window (Galaxy §9.3 matrix).
4. Verify the candidate build with [verify signed release](#operator-quickstart-verify-signed-release-ph-s71) before retrying registration.
5. Retry only after either worker build or coordinator protocol window is aligned.

#### Protocol reject troubleshooting (PH-S73)

Escalation for repeated registration rejects (`compat_status`, HTTP `403`/`426`):

1. [Verify signed build](#operator-quickstart-verify-signed-release-ph-s71) first to rule out untrusted artifact issues.
2. Compare worker `protocol_version` against the coordinator compatibility window ([§9.3](#galaxy-governance-canonical-pointers-ph-s69-ph-s77)).
3. Classify `compat_status` as worker upgrade vs coordinator window update.
4. Apply one controlled change at a time, then retry registration.
5. If reject persists, pause rollout and record (`build_id`, worker protocol, coordinator protocol window) for ops review.

#### Operator actions: signed release advisories (PH-S76)

When a signed release advisory is published (`CVE-*`, `key_transition`, `protocol_sunset` — policy: [§9.6](#galaxy-governance-canonical-pointers-ph-s69-ph-s77)):

1. Validate advisory authenticity and release signature ([verify quickstart](#operator-quickstart-verify-signed-release-ph-s71)).
2. Map advisory class to rollout urgency (hold / staged / immediate patch).
3. For dependency-only CVEs, follow [`DEPENDENCY_SECURITY.md`](./DEPENDENCY_SECURITY.md); for release/protocol advisories, use the [canonical pointer table](#galaxy-governance-canonical-pointers-ph-s69-ph-s77).
4. If protocol window changes are involved, run [protocol compatibility triage](#operator-checklist-protocol-compatibility-triage-ph-s72) before enabling new workers.
5. Record the final decision and artifact hash in the local change log.

Process signed advisories through local change management only; no remote-exec path is implied (PH-S74 hygiene — folded into this hub).

#### A09:2021 – Security Logging and Monitoring Failures

**Current Implementation**:
- ✅ Structured logging with `tracing`
- ✅ Audit logging for critical operations
- ✅ Health check endpoints

**Review Checklist**:
- [x] Logging infrastructure
- [x] Audit logging for security events
- [ ] Security alerting
- [ ] Log retention policy
- [ ] Centralized logging

**Recommendations**:
- Add security alerting (failed logins, privilege escalations)
- Implement log retention policy
- Set up centralized logging (e.g., ELK stack)

#### A10:2021 – Server-Side Request Forgery (SSRF)

**Current Implementation**:
- ✅ No external HTTP requests from user input
- ✅ Validated URLs for library downloads

**Review Checklist**:
- [x] No user-controlled URLs in HTTP requests
- [x] URL validation for downloads
- [ ] IP whitelist for external requests
- [ ] Request timeout limits

**Recommendations**:
- Add IP whitelist for external HTTP requests
- Set strict timeout limits for external requests
- Validate URLs against allowed domains

## Input Validation

### Current Implementation

PoolAI uses `serde` for type-safe deserialization, which provides built-in validation:

```rust
// Example: ModelRequest with validation
#[derive(Deserialize)]
pub struct ModelRequest {
    pub input: String,  // Validated by serde
    pub parameters: ModelParameters,  // Nested validation
    pub priority: u8,  // Type validation
    pub timeout: Option<u64>,
}
```

### Validation Checklist

- [x] Type validation (serde)
- [x] Required fields validation
- [x] Numeric range validation
- [ ] String length validation
- [ ] Pattern validation (regex)
- [ ] File upload validation
- [ ] Path traversal validation

### Recommendations

1. **Add String Length Validation**:
```rust
// Validate input length
if request.input.len() > MAX_INPUT_LENGTH {
    return Err(AppError::ValidationError("Input too long".to_string()));
}
```

2. **Add Path Traversal Validation**:
```rust
// Validate file paths
fn validate_path(path: &Path) -> Result<(), AppError> {
    let canonical = path.canonicalize()?;
    let base = Path::new("/var/lib/poolai");
    if !canonical.starts_with(base) {
        return Err(AppError::ValidationError("Path traversal detected".to_string()));
    }
    Ok(())
}
```

3. **Add File Upload Validation**:
```rust
// Validate file uploads
fn validate_file_upload(file: &[u8], max_size: usize, allowed_types: &[&str]) -> Result<(), AppError> {
    if file.len() > max_size {
        return Err(AppError::ValidationError("File too large".to_string()));
    }
    // Check file type (magic bytes)
    // ...
    Ok(())
}
```

## Security Headers

### Current Implementation

Security headers are partially implemented in HTTPS configuration. Full implementation needed.

### Recommended Security Headers

```rust
// Security headers middleware
const SECURITY_HEADERS: &[(&str, &str)] = &[
    ("Strict-Transport-Security", "max-age=31536000; includeSubDomains"),
    ("X-Frame-Options", "DENY"),
    ("X-Content-Type-Options", "nosniff"),
    ("X-XSS-Protection", "1; mode=block"),
    ("Referrer-Policy", "strict-origin-when-cross-origin"),
    ("Content-Security-Policy", "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';"),
    ("Permissions-Policy", "geolocation=(), microphone=(), camera=()"),
];
```

### Implementation Checklist

- [ ] HSTS header
- [ ] X-Frame-Options header
- [ ] X-Content-Type-Options header
- [ ] Content-Security-Policy header
- [ ] Referrer-Policy header
- [ ] Permissions-Policy header

## Rate Limiting

### Current Implementation

Rate limiting is not yet implemented.

### Recommendations

Implement rate limiting using `tower-http::limit`:

```rust
use tower_http::limit::RateLimitLayer;
use tower::time::Duration;

// Rate limiting middleware
let rate_limit = RateLimitLayer::new(
    100,  // 100 requests
    Duration::from_secs(60),  // per minute
);

// Apply to specific routes
let app = Router::new()
    .route("/api/v1/login", login_handler)
    .layer(rate_limit);
```

### Rate Limit Configuration

- **Authentication endpoints**: 5 requests per minute
- **API endpoints**: 100 requests per minute per IP
- **File upload endpoints**: 10 requests per minute
- **Admin endpoints**: 50 requests per minute

## Secrets Management

### Current Implementation

- Secrets stored in environment variables
- Configuration files with secrets (`.env`, `config.toml`)

### Recommendations

1. **Use Secret Management Service**:
   - HashiCorp Vault
   - AWS Secrets Manager
   - Azure Key Vault
   - Kubernetes Secrets

2. **Secret Rotation**:
   - Implement automatic JWT key rotation
   - Rotate certificates before expiration
   - Rotate API keys periodically

3. **Secrets in Configuration**:
   - Never commit secrets to Git
   - Use `.env` files with `.gitignore`
   - Use secret management in production

## Certificate Management

### Current Implementation

- Self-signed certificates for development
- Let's Encrypt support documented

### Recommendations

1. **Production Certificates**:
   - Use Let's Encrypt for production
   - Enable auto-renewal
   - Monitor certificate expiration

2. **Certificate Security**:
   - Use strong private keys (4096-bit RSA or EC)
   - Enable OCSP stapling
   - Use certificate pinning for API clients

## Security Testing

### Manual Testing Checklist

- [ ] Authentication bypass attempts
- [ ] Authorization bypass attempts
- [ ] Input validation testing
- [ ] SQL injection attempts (if applicable)
- [ ] XSS attempts
- [ ] CSRF attempts
- [ ] Path traversal attempts
- [ ] File upload abuse
- [ ] Rate limiting verification

### Automated Security Testing

```bash
# OWASP ZAP scan
docker run -t owasp/zap2docker-stable zap-baseline.py -t http://localhost:8080

# Security header check
curl -I https://poolai.example.com | grep -i security

# SSL/TLS check
openssl s_client -connect poolai.example.com:443 -showcerts
```

## Security Monitoring

### Recommended Monitoring

1. **Failed Login Attempts**: Alert on > 10 failed attempts per minute
2. **Privilege Escalation**: Alert on role changes
3. **Suspicious API Usage**: Alert on unusual request patterns
4. **Certificate Expiration**: Alert 30 days before expiration
5. **Dependency Vulnerabilities**: Alert on new `cargo audit` findings

## Compliance

### Security Standards

- [ ] OWASP Top 10 compliance
- [ ] PCI DSS (if handling payments)
- [ ] GDPR compliance (if handling EU data)
- [ ] SOC 2 (if required)

## Security Incident Response

### Response Plan

1. **Detection**: Automated alerts + manual monitoring
2. **Containment**: Isolate affected systems
3. **Eradication**: Remove threat and patch vulnerabilities
4. **Recovery**: Restore services with enhanced security
5. **Lessons Learned**: Document and improve

## Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Advisory Database](https://rustsec.org/)
- [Cargo Audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [Security Best Practices](BEST_PRACTICES.md)

---

**Last Updated**: 2026-05-27  
**Version**: 1.9 - PH-S87 INDEX step-8 / §7 cross-link to this hub (bidirectional)

**Version**: 1.8 - PH-S85 dev fixtures paths (`tests/fixtures/release/dev`); RUN_LOCAL verify-release pointer

**Version**: 1.7 - Galaxy §9.2/§9.3/§9.6 canonical pointer hub (PH-S77); deduplicated PH-S71–S76 link blocks
