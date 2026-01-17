# Certificate Management Security Guide

## Overview

This document provides guidelines for secure certificate management in PoolAI, covering TLS certificates, private keys, certificate validation, rotation, and monitoring.

## Current Implementation

### Certificate Storage

PoolAI currently stores certificates in:

1. **File System**:
   - Certificate files (`certs/cert.pem`)
   - Private key files (`certs/key.pem`)
   - Self-signed certificates for development

2. **Configuration**:
   - `config.toml` → `[security]` → `cert_path`, `key_path`
   - Environment variables: `HTTPS_CERT_PATH`, `HTTPS_KEY_PATH`

3. **Let's Encrypt**:
   - Documented but requires manual setup
   - No automatic renewal implemented

### Certificate Types

#### 1. TLS Server Certificates
- **Location**: `certs/cert.pem`
- **Usage**: HTTPS/TLS server authentication
- **Current Status**: ⚠️ Self-signed for development (production should use Let's Encrypt)
- **Key Size**: 4096-bit RSA (recommended) or EC (P-256, P-384)

#### 2. TLS Private Keys
- **Location**: `certs/key.pem`
- **Usage**: TLS server key exchange
- **Current Status**: ⚠️ File permissions may be insecure
- **Key Format**: PEM format (PKCS#8)

## Security Best Practices

### 1. Certificate Generation

#### Development (Self-Signed)

```bash
# Generate self-signed certificate for development
openssl req -x509 -newkey rsa:4096 \
    -keyout certs/key.pem \
    -out certs/cert.pem \
    -days 365 \
    -nodes \
    -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"

# Set secure permissions
chmod 600 certs/key.pem  # Private key: owner only
chmod 644 certs/cert.pem # Certificate: readable by all
```

#### Production (Let's Encrypt)

```bash
# Install certbot
sudo apt-get update
sudo apt-get install certbot

# Obtain certificate (standalone mode)
sudo certbot certonly --standalone \
    -d poolai.example.com \
    -d www.poolai.example.com

# Certificates stored in:
# /etc/letsencrypt/live/poolai.example.com/fullchain.pem
# /etc/letsencrypt/live/poolai.example.com/privkey.pem
```

### 2. Certificate Validation

#### Current Implementation

**Status**: ✅ Certificate validation handled by `rustls`

**Validation Checks**:
- Certificate chain validation
- Expiration date check
- Signature verification
- Certificate authority (CA) validation

#### Recommended Enhancements

1. **Certificate Expiration Monitoring**:
   - Check expiration date on startup
   - Alert 30 days before expiration
   - Automatic renewal workflow

2. **Certificate Pinning**:
   - Pin certificate for API clients
   - Reduce MITM attack surface
   - Implementation required

3. **OCSP Stapling**:
   - Improve certificate validation performance
   - Reduce OCSP server load
   - Implementation required

### 3. Certificate Rotation

#### Current Status

**Status**: ⚠️ Manual rotation required

**Current Process**:
1. Generate new certificate
2. Update config file
3. Restart application

#### Recommended Improvements

1. **Automatic Renewal** (Let's Encrypt):
```bash
# Crontab entry for auto-renewal
0 0 * * * certbot renew --quiet --deploy-hook "systemctl reload poolai"
```

2. **Graceful Certificate Rotation**:
   - Support dual certificates (old + new)
   - Zero-downtime rotation
   - Rollback capability

3. **Certificate Monitoring**:
   - Alert 30 days before expiration
   - Alert 7 days before expiration
   - Alert on expiration

### 4. Private Key Security

#### Current Status

**Status**: ⚠️ File permissions may be insecure

#### Recommendations

1. **File Permissions**:
```bash
# Secure private key
chmod 600 /etc/poolai/certs/key.pem  # Owner read/write only
chown poolai:poolai /etc/poolai/certs/key.pem

# Secure certificate (readable by all, but not writable)
chmod 644 /etc/poolai/certs/cert.pem
chown poolai:poolai /etc/poolai/certs/cert.pem

# Secure directory
chmod 700 /etc/poolai/certs/  # Owner only
chown poolai:poolai /etc/poolai/certs/
```

2. **Key Storage**:
   - Store in secure directory (`/etc/poolai/certs/`)
   - Use hardware security module (HSM) for production (future)
   - Encrypt keys at rest (full disk encryption)

3. **Key Rotation**:
   - Rotate keys when certificate expires
   - Generate new keys for each certificate
   - Securely delete old keys

### 5. Certificate Chain Validation

#### Current Implementation

**Status**: ✅ Handled by `rustls`

**Validation**:
- Full chain validation
- Root CA validation
- Intermediate CA validation

#### Recommended Enhancements

1. **Certificate Transparency (CT)**:
   - Monitor CT logs for certificates
   - Detect unauthorized certificate issuance
   - Implementation required

2. **OCSP Stapling**:
   - Reduce certificate validation latency
   - Improve performance
   - Implementation required

3. **Certificate Pinning**:
   - Pin certificates for API clients
   - Reduce MITM risk
   - Implementation required

## Certificate Management Workflow

### Development

1. **Generate Self-Signed Certificate**:
```bash
openssl req -x509 -newkey rsa:4096 \
    -keyout certs/key.pem \
    -out certs/cert.pem \
    -days 365 -nodes
```

2. **Configure Application**:
```toml
# config.toml
[security]
https_enabled = true
cert_path = "certs/cert.pem"
key_path = "certs/key.pem"
```

3. **Run Application**:
```bash
cargo run --release --features https
```

### Staging

1. **Obtain Let's Encrypt Certificate**:
```bash
certbot certonly --standalone -d staging.poolai.example.com
```

2. **Configure Application**:
```toml
# config.staging.toml
[security]
https_enabled = true
cert_path = "/etc/letsencrypt/live/staging.poolai.example.com/fullchain.pem"
key_path = "/etc/letsencrypt/live/staging.poolai.example.com/privkey.pem"
```

3. **Set Auto-Renewal**:
```bash
# Crontab
0 0 * * * certbot renew --quiet --deploy-hook "systemctl reload poolai"
```

### Production

1. **Obtain Let's Encrypt Certificate**:
```bash
certbot certonly --standalone \
    -d poolai.example.com \
    -d www.poolai.example.com
```

2. **Configure Application**:
```bash
# Environment variables
export HTTPS_CERT_PATH="/etc/letsencrypt/live/poolai.example.com/fullchain.pem"
export HTTPS_KEY_PATH="/etc/letsencrypt/live/poolai.example.com/privkey.pem"
```

3. **Set Auto-Renewal with Monitoring**:
```bash
# Crontab with monitoring
0 0 * * * certbot renew --quiet --deploy-hook "systemctl reload poolai && /opt/poolai/scripts/notify-renewal.sh"
```

4. **Monitor Certificate Expiration**:
   - Alert 30 days before expiration
   - Alert 7 days before expiration
   - Alert on expiration

## Certificate Monitoring

### Expiration Monitoring

**Recommendations**:
1. Check certificate expiration on startup
2. Periodic checks (daily/weekly)
3. Alert 30 days before expiration
4. Alert 7 days before expiration

**Implementation** (Future):
```rust
// Check certificate expiration
fn check_certificate_expiration(cert_path: &Path) -> Result<(), AppError> {
    let cert = std::fs::read(cert_path)?;
    let cert = rustls::Certificate(cert);
    // Parse expiration date
    // Check if expires in < 30 days
    // Alert if necessary
}
```

### Certificate Validation

**Current Checks**:
- ✅ Certificate chain validation (rustls)
- ✅ Expiration date check (rustls)
- ✅ Signature verification (rustls)
- ✅ CA validation (rustls)

**Missing Checks**:
- [ ] Certificate Transparency monitoring
- [ ] OCSP validation
- [ ] Certificate pinning
- [ ] CRL (Certificate Revocation List) checking

## Certificate Security Checklist

### Certificate Generation
- [x] Use strong private keys (4096-bit RSA or EC)
- [x] Use Let's Encrypt for production
- [ ] Generate new keys for each certificate
- [ ] Use hardware security module (HSM) for production (future)

### Certificate Storage
- [ ] Secure file permissions (600 for private keys, 644 for certificates)
- [ ] Store in secure directory (`/etc/poolai/certs/`)
- [ ] Use full disk encryption
- [ ] Use secret management services (for private keys)

### Certificate Rotation
- [x] Document manual rotation process
- [ ] Implement automatic renewal (Let's Encrypt)
- [ ] Implement graceful rotation (zero-downtime)
- [ ] Implement rollback capability

### Certificate Monitoring
- [ ] Check expiration on startup
- [ ] Periodic expiration checks
- [ ] Alert 30 days before expiration
- [ ] Alert 7 days before expiration
- [ ] Alert on expiration

### Certificate Validation
- [x] Certificate chain validation
- [x] Expiration date check
- [x] Signature verification
- [x] CA validation
- [ ] Certificate Transparency monitoring
- [ ] OCSP stapling
- [ ] Certificate pinning

## Certificate Lifecycle

### 1. Generation
- Generate private key (4096-bit RSA or EC)
- Generate certificate signing request (CSR)
- Submit CSR to CA (Let's Encrypt)
- Receive certificate from CA

### 2. Installation
- Store certificate and key securely
- Set appropriate file permissions
- Configure application to use certificate
- Test certificate functionality

### 3. Validation
- Verify certificate chain
- Check expiration date
- Validate certificate signature
- Test HTTPS connection

### 4. Monitoring
- Monitor certificate expiration
- Check certificate validity
- Monitor certificate usage
- Alert on issues

### 5. Rotation
- Generate new certificate before expiration
- Install new certificate
- Restart application (or use graceful rotation)
- Verify new certificate works
- Remove old certificate

## Tools and Resources

### Certificate Management Tools
- **certbot**: Let's Encrypt certificate management
- **openssl**: Certificate generation and validation
- **rustls**: TLS library for Rust

### Certificate Validation Tools
```bash
# Check certificate expiration
openssl x509 -in cert.pem -noout -dates

# Check certificate chain
openssl verify -CAfile ca-chain.pem cert.pem

# Check certificate details
openssl x509 -in cert.pem -noout -text

# Test HTTPS connection
openssl s_client -connect poolai.example.com:443 -showcerts
```

### Certificate Monitoring Tools
- **Prometheus**: Certificate expiration metrics
- **Grafana**: Certificate expiration dashboards
- **Alertmanager**: Certificate expiration alerts

## Compliance

### Security Standards

- [ ] TLS 1.2+ for all HTTPS connections
- [ ] Strong cipher suites only
- [ ] Certificate expiration monitoring
- [ ] Certificate rotation procedures

### Audit Requirements

- [ ] Certificate inventory
- [ ] Certificate expiration tracking
- [ ] Certificate rotation logs
- [ ] Certificate security incidents

## Security Incidents

### Certificate Compromise Response

1. **Immediate Actions**:
   - Revoke compromised certificate
   - Generate new certificate
   - Update application configuration
   - Restart application

2. **Investigation**:
   - Determine compromise scope
   - Check for unauthorized access
   - Review certificate usage logs
   - Document incident

3. **Prevention**:
   - Review certificate management procedures
   - Implement certificate pinning
   - Enhance certificate monitoring
   - Security training for team

## Resources

- [Let's Encrypt Documentation](https://letsencrypt.org/docs/)
- [certbot Documentation](https://certbot.eff.org/)
- [OpenSSL Documentation](https://www.openssl.org/docs/)
- [TLS 1.3 RFC](https://www.rfc-editor.org/rfc/rfc8446)
- [Certificate Transparency](https://www.certificate-transparency.org/)

---

**Last Updated**: 2026-01-16  
**Version**: 1.0 - Initial certificate management security guide
