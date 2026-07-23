# Secrets Management Guide

## Overview

This document provides guidelines for secure secrets management in PoolAI, covering JWT keys, certificates, API keys, and other sensitive credentials.

## Current Implementation

### Secrets Storage

PoolAI currently stores secrets in:

1. **Configuration Files** (`config.toml`):
   - JWT secret keys
   - Certificate paths
   - API keys (if any)

2. **Environment Variables**:
   - `POOLAI_JWT_SECRET`
   - `HTTPS_CERT_PATH`
   - `HTTPS_KEY_PATH`
   - `POOLAI_SECURITY_*` (various security settings)

3. **File System**:
   - Certificate files (`certs/cert.pem`, `certs/key.pem`)
   - Private keys

### Secrets Identified

#### 1. JWT Secret Key
- **Location**: `config.toml` → `[security]` → `jwt_secret`
- **Environment Variable**: `POOLAI_JWT_SECRET`
- **Usage**: Signing and verifying JWT tokens
- **Current Status**: ⚠️ Stored in config file (should use environment variables in production)

#### 2. TLS Certificates
- **Location**: `config.toml` → `[security]` → `cert_path`, `key_path`
- **Environment Variables**: `HTTPS_CERT_PATH`, `HTTPS_KEY_PATH`
- **Usage**: HTTPS/TLS encryption
- **Current Status**: ✅ File paths configured (certificates stored as files)

#### 3. Cloud Provider Credentials
- **Azure**: `DefaultAzureCredential` (environment-based authentication)
- **GCP**: Service account keys or metadata server
- **AWS**: Access keys (when implemented)
- **Current Status**: ⚠️ Environment variables (should use secret management services)

## Security Best Practices

### 1. Never Commit Secrets to Git

**Current Status**: ⚠️ `config.toml` may contain secrets

**Recommendations**:
```bash
# Add to .gitignore
config.toml
*.pem
*.key
.env
secrets/
```

**Template Files**:
- Use `config.example.toml` (without secrets)
- Use `config.https.example.toml` (without actual certificates)

### 2. Use Environment Variables for Production

**Implementation**:
```bash
# Set environment variables
export POOLAI_JWT_SECRET=$(openssl rand -hex 32)
export HTTPS_CERT_PATH="/etc/poolai/certs/cert.pem"
export HTTPS_KEY_PATH="/etc/poolai/certs/key.pem"
```

**Code Support**: ✅ Already implemented
- Configuration system reads environment variables
- Environment variables override config file values

### 4. GitHub Secret Scanning (PH-SVC35/36, 2026-07-23)

**Alert #1 (Atlassian API Token, opened 2023-06-25, Public leak):** detected under historical path `target/…/deps/atlassian_core-…` (build artifact). Current tree: `target/` is in `.gitignore`; no live `target/` tracked files.

| Action | Who | Status |
|--------|-----|--------|
| Revoke the Atlassian API token in Atlassian account | **OWNER** | PH-SVC35 **[ ]** |
| Do **not** rewrite git history (`filter-repo` / BFG) without explicit owner request | Agent | PH-SVC36 ✅ |
| Keep `target/` ignored; never commit build deps | Agent | ✅ |

After revoke, dismiss or resolve the GitHub Secret scanning alert in repo Security settings.

### 3. Use Secret Management Services

#### HashiCorp Vault

**Setup**:
```bash
# Install Vault
curl -fsSL https://apt.releases.hashicorp.com/gpg | sudo apt-key add -
sudo apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main"
sudo apt-get update && sudo apt-get install vault

# Start Vault (dev mode)
vault server -dev
```

**Integration**:
```rust
// Example: Fetch JWT secret from Vault
// TODO: Implement Vault integration
let jwt_secret = vault_client
    .secret("poolai/jwt/secret")
    .read()
    .await?;
```

#### AWS Secrets Manager

**Setup**:
- Create secret in AWS Secrets Manager
- Use IAM roles for authentication
- Access via AWS SDK

**Integration**:
```rust
// Example: Fetch secret from AWS Secrets Manager
// Requires: aws-sdk-secretsmanager (Rust 1.88+)
// TODO: Implement when Rust version upgraded
```

#### Azure Key Vault

**Setup**:
- Create Key Vault in Azure
- Use Managed Identity for authentication
- Access via Azure SDK

**Integration**:
```rust
// Example: Fetch secret from Azure Key Vault
// Using azure_keyvault_secrets crate
// TODO: Implement Azure Key Vault integration
```

#### Kubernetes Secrets

**Setup**:
```yaml
# Create Kubernetes secret
apiVersion: v1
kind: Secret
metadata:
  name: poolai-secrets
type: Opaque
stringData:
  jwt-secret: "your-jwt-secret-here"
  cert-path: "/etc/poolai/certs/cert.pem"
  key-path: "/etc/poolai/certs/key.pem"
```

**Integration**:
- Mount secrets as files in pods
- Use environment variables from secrets
- Access via Kubernetes API (for operators)

### 4. Secret Rotation (PH-S24 ✅)

**Implementation:** `src/security/secret_rotation.rs`, `src/security/jwt_secrets.rs`

| Mechanism | Description |
|-----------|-------------|
| **Rotation hooks** | Pluggable per `SecretKind` (`jwt`, `tls_certificate`, `telegram_webhook`) |
| **Admin API** | `GET /api/v1/admin/secrets/rotation` (status), `POST /api/v1/admin/secrets/rotate` (admin only) |
| **JWT dual-key** | `POOLAI_JWT_SECRET` + `POOLAI_JWT_SECRET_PREVIOUS` during `POOLAI_JWT_ROTATION_GRACE_SECS` (default 86400) |
| **Env poll** | `POOLAI_SECRET_ROTATION_POLL_SECS` — periodic JWT reload from env |
| **TLS reload** | `HTTPS_CERT_RELOAD_SECS` + rotation hook on HTTPS startup (FM-044) |

#### JWT Secret Rotation

**Ops workflow:**

```bash
# 1. Generate new secret; keep old as PREVIOUS for grace window
export POOLAI_JWT_SECRET_PREVIOUS="$POOLAI_JWT_SECRET"
export POOLAI_JWT_SECRET=$(openssl rand -hex 32)
export POOLAI_JWT_ROTATION_GRACE_SECS=86400

# 2. Trigger reload (no full restart) — admin Bearer token required
curl -s -X POST http://127.0.0.1:8080/api/v1/admin/secrets/rotate \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"kind":"jwt"}'

# 3. After grace expires, unset PREVIOUS
unset POOLAI_JWT_SECRET_PREVIOUS
```

**Pen-test:** see [`PEN_TEST_CHECKLIST.md`](./PEN_TEST_CHECKLIST.md) §5.

#### Certificate Rotation

**Current Status**: ⚠️ Manual renewal (Let's Encrypt auto-renewal documented)

**Recommendations**:
1. **Let's Encrypt Auto-Renewal**:
   ```bash
   # Crontab entry
   0 0 * * * certbot renew --quiet --deploy-hook "systemctl reload poolai"
   ```

2. **Certificate Monitoring**:
   - Alert 30 days before expiration
   - Monitor certificate validity
   - Automatic renewal workflow

### 5. File Permissions

**Current Status**: ⚠️ Default file permissions may be insecure

**Recommendations**:
```bash
# Secure certificate files
chmod 600 /etc/poolai/certs/key.pem    # Private key: read/write owner only
chmod 644 /etc/poolai/certs/cert.pem   # Certificate: readable by all
chown poolai:poolai /etc/poolai/certs/

# Secure config files
chmod 600 /etc/poolai/config.toml      # Config with secrets: owner only
chown poolai:poolai /etc/poolai/config.toml
```

### 6. Secret Access Control

**Recommendations**:
- Use least privilege principle
- Restrict access to secret files
- Use service accounts (not root) to run application
- Audit secret access

## Migration Plan

### Phase 1: Immediate (Current)
- ✅ Use environment variables for secrets
- ✅ Use `.gitignore` for config files with secrets
- ✅ Provide `config.example.toml` templates
- ⚠️ Secure file permissions (manual setup required)

### Phase 2: Short-term (1-2 months)
- [ ] Implement HashiCorp Vault integration
- [ ] Add secret rotation for JWT keys
- [ ] Certificate auto-renewal with monitoring
- [ ] Secret access audit logging

### Phase 3: Long-term (3-6 months)
- [ ] Multi-provider secret management (Vault, AWS, Azure, K8s)
- [ ] Automatic secret rotation
- [ ] Secret versioning and rollback
- [ ] Centralized secret management dashboard

## Secret Management Checklist

### Development
- [x] Use `.gitignore` for secrets
- [x] Provide example config files
- [x] Use environment variables
- [ ] Never commit secrets to Git

### Staging
- [ ] Use environment variables or secret management
- [ ] Secure file permissions on certificates
- [ ] Use separate secrets from production
- [ ] Enable secret access logging

### Production
- [ ] Use secret management service (Vault, AWS, Azure)
- [ ] Secure file permissions (600 for private keys)
- [ ] Enable secret rotation
- [ ] Monitor secret access
- [ ] Implement secret backup/restore
- [ ] Document secret recovery procedures

## Secret Types and Recommendations

| Secret Type | Current Storage | Recommended Storage | Rotation Period |
|------------|----------------|---------------------|-----------------|
| JWT Secret | Config file / Env var | Secret Management Service | 90 days |
| TLS Private Key | File system | Secret Management Service | Certificate lifetime |
| TLS Certificate | File system | File system (OK) | Certificate lifetime |
| Cloud Credentials | Environment / IAM | IAM Roles / Managed Identity | As needed |
| API Keys | Environment / Config | Secret Management Service | 180 days |
| Database Passwords | N/A (no database) | Secret Management Service | 90 days |

## Security Incidents

### Secret Exposure Response

1. **Immediate Actions**:
   - Rotate exposed secret immediately
   - Revoke compromised credentials
   - Audit access logs
   - Notify affected users (if applicable)

2. **Investigation**:
   - Determine exposure scope
   - Review access logs
   - Check for unauthorized access
   - Document incident

3. **Prevention**:
   - Review security practices
   - Update secret management procedures
   - Implement additional monitoring
   - Security training for team

## Tools and Resources

### Secret Management Tools
- [HashiCorp Vault](https://www.vaultproject.io/)
- [AWS Secrets Manager](https://aws.amazon.com/secrets-manager/)
- [Azure Key Vault](https://azure.microsoft.com/services/key-vault/)
- [Kubernetes Secrets](https://kubernetes.io/docs/concepts/configuration/secret/)

### Secret Generation
```bash
# Generate secure random strings
openssl rand -hex 32        # 32-byte hex string (64 characters)
openssl rand -base64 32     # 32-byte base64 string
uuidgen -r                  # Random UUID
```

### Secret Validation
```bash
# Check for hardcoded secrets in code
grep -r "jwt_secret.*=" src/
grep -r "password.*=" src/
grep -r "api.*key" src/ -i

# Check for secrets in Git history
git log -p --all -S "jwt_secret" --source --all
```

## Configuration Examples

### Development (Local)

```toml
# config.local.toml
[security]
jwt_secret = "dev-secret-change-in-production"  # ⚠️ Development only
https_enabled = false
```

### Staging (Environment Variables)

```bash
# .env.staging
POOLAI_JWT_SECRET=$(openssl rand -hex 32)
HTTPS_CERT_PATH=/etc/poolai/certs/cert.pem
HTTPS_KEY_PATH=/etc/poolai/certs/key.pem
```

### Production (Secret Management)

```bash
# Fetch from Vault
export POOLAI_JWT_SECRET=$(vault kv get -field=jwt_secret poolai/secrets)
export HTTPS_CERT_PATH=$(vault kv get -field=cert_path poolai/secrets)
export HTTPS_KEY_PATH=$(vault kv get -field=key_path poolai/secrets)
```

## Monitoring and Alerting

### Secret-Related Metrics

1. **Secret Access**:
   - Number of secret reads
   - Secret access failures
   - Unusual access patterns

2. **Secret Rotation**:
   - Last rotation time
   - Days until next rotation
   - Rotation failures

3. **Secret Health**:
   - Secret validity status
   - Certificate expiration dates
   - Secret availability checks

### Alerting Rules

- Alert when secret is accessed from unusual IP
- Alert 30 days before secret rotation deadline
- Alert when secret rotation fails
- Alert when certificate expires in 30 days

## Compliance

### Security Standards

- [ ] OWASP Secrets Management guidelines
- [ ] PCI DSS (if handling payment data)
- [ ] GDPR (if handling EU personal data)
- [ ] SOC 2 (if required)

### Audit Requirements

- [ ] Secret access logs
- [ ] Secret rotation logs
- [ ] Secret change history
- [ ] Secret exposure incidents

## Resources

- [OWASP Secrets Management](https://owasp.org/www-community/vulnerabilities/Use_of_hard-coded_cryptographic_key)
- [12-Factor App: Config](https://12factor.net/config)
- [HashiCorp Vault Best Practices](https://learn.hashicorp.com/tutorials/vault/production-hardening)
- [AWS Secrets Manager Best Practices](https://docs.aws.amazon.com/secretsmanager/latest/userguide/best-practices.html)
- [Azure Key Vault Best Practices](https://docs.microsoft.com/azure/key-vault/general/best-practices)

---

**Last Updated**: 2026-01-16  
**Version**: 1.0 - Initial secrets management guide
