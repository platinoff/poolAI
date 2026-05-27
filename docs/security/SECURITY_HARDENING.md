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

#### Galaxy governance cross-links (PH-S69)

Use these pointers instead of duplicating governance prose in this guide:

- Signed release flow and trust model: [`POOLAI_GALAXY_GRID.md` §9.2](../concept/POOLAI_GALAXY_GRID.md#92-signed-releases-канон-ph-s63)
- Protocol compatibility matrix and rollout constraints: [`POOLAI_GALAXY_GRID.md` §9.3](../concept/POOLAI_GALAXY_GRID.md#93-protocol-versioning-та-compat-matrix)
- Runtime wire guardrails already implemented in code/docs: `protocol_version` + `build_id` registration checks (PH-S65) in [`FUNCTION_MANAGEMENT.md` §5.12](../catalog/FUNCTION_MANAGEMENT.md#512-research-backlog-ph-s65-2026-05-27)
- Verification CLI entrypoint: `cargo run --bin poolai-verify-release -- --manifest <path> --signature <path> [--artifact <path>]`

#### Operator quickstart: verify signed release (PH-S71)

Minimal verification sequence for operators before rollout:

1. Prepare trust root (`maintainer_keys.json`) or pass a pinned key via `--public-key-hex`.
2. Verify release manifest signature:
   `cargo run --bin poolai-verify-release -- --manifest <release-manifest.json> --signature <release-manifest.sig>`
3. Verify artifact integrity against manifest (recommended):
   `cargo run --bin poolai-verify-release -- --manifest <release-manifest.json> --signature <release-manifest.sig> --artifact <poolai-binary-or-archive>`
4. Roll out only after signature + artifact verification are both successful.

For policy and compatibility constraints, use canonical references:
- Signed release model: [`POOLAI_GALAXY_GRID.md` §9.2](../concept/POOLAI_GALAXY_GRID.md#92-signed-releases-канон-ph-s63)
- Protocol compatibility matrix: [`POOLAI_GALAXY_GRID.md` §9.3](../concept/POOLAI_GALAXY_GRID.md#93-protocol-versioning-та-compat-matrix)

#### Operator checklist: protocol compatibility triage (PH-S72)

Use this quick checklist when worker registration fails because of protocol mismatch:

1. Confirm worker sends expected wire fields (`protocol_version`, `build_id`) via discovery register flow (PH-S65 baseline).
2. If coordinator returns HTTP `403` or `426`, treat it as compat negotiation failure and stop rollout for that worker build.
3. Check returned `compat_status` and compare it with the active protocol window from Galaxy compat matrix (`§9.3`).
4. Verify that the candidate build belongs to an approved signed release before retrying registration.
5. Retry only after either worker build or coordinator protocol window is aligned.

Canonical references for investigation:
- Compatibility model and rollout constraints: [`POOLAI_GALAXY_GRID.md` §9.3](../concept/POOLAI_GALAXY_GRID.md#93-protocol-versioning-та-compat-matrix)
- Wire implementation status (PH-S65): [`FUNCTION_MANAGEMENT.md` §5.12](../catalog/FUNCTION_MANAGEMENT.md#512-research-backlog-ph-s65-2026-05-27)
- Signed artifact verification flow: `poolai-verify-release` quickstart (section above)

#### Protocol reject troubleshooting pointer (PH-S73)

Use this escalation path for repeated registration rejects (`compat_status`, HTTP `403`/`426`):

1. Verify signed build first (`poolai-verify-release` quickstart) to rule out untrusted artifact issues.
2. Compare worker `protocol_version` against coordinator compatibility window (Galaxy §9.3 matrix).
3. Confirm reject reason from `compat_status` and classify it as either worker upgrade needed or coordinator window update needed.
4. Apply one controlled change at a time (worker build or coordinator protocol window), then retry registration.
5. If reject persists, pause rollout and document the mismatch tuple (`build_id`, worker protocol, coordinator protocol window) for ops review.

Fast links:
- Signed release verify flow: section `Operator quickstart: verify signed release (PH-S71)` above
- Compatibility model: [`POOLAI_GALAXY_GRID.md` §9.3](../concept/POOLAI_GALAXY_GRID.md#93-protocol-versioning-та-compat-matrix)
- Wire baseline: [`FUNCTION_MANAGEMENT.md` §5.12 (PH-S65)](../catalog/FUNCTION_MANAGEMENT.md#512-research-backlog-ph-s65-2026-05-27)

#### Advisory and update-policy link hygiene (PH-S74)

Use pointer-only links for operator actions around advisories and key updates:

- Security advisory lifecycle and key rotation canon: [`POOLAI_GALAXY_GRID.md` §9.6](../concept/POOLAI_GALAXY_GRID.md#96-security-advisories-та-key-rotation)
- Signed releases and trust root context: [`POOLAI_GALAXY_GRID.md` §9.2](../concept/POOLAI_GALAXY_GRID.md#92-signed-releases-канон-ph-s63)
- Protocol rollout constraints: [`POOLAI_GALAXY_GRID.md` §9.3](../concept/POOLAI_GALAXY_GRID.md#93-protocol-versioning-та-compat-matrix)
- Dependency advisory workflow: [`DEPENDENCY_SECURITY.md`](./DEPENDENCY_SECURITY.md)

Operator note: process signed advisory events (`CVE-*`, `key_transition`, `protocol_sunset`) through local change management; no remote-exec path is implied by advisory handling.

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
**Version**: 1.5 - Advisory/update-policy link hygiene (PH-S74)
