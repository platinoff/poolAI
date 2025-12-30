# Security Best Practices

## Overview

This guide covers security best practices for deploying and operating PoolAI in production environments.

## Authentication & Authorization

### JWT Configuration

#### Strong Secret Keys

```toml
[security]
jwt_secret = "CHANGE_THIS_TO_RANDOM_32_BYTE_HEX_STRING"  # Use openssl rand -hex 32
token_expiry_seconds = 3600  # 1 hour
refresh_token_expiry_seconds = 86400  # 24 hours
```

Generate secure secret:

```bash
openssl rand -hex 32
```

#### Token Rotation

- Implement token rotation for long-lived sessions
- Use refresh tokens for extended sessions
- Revoke tokens on logout

### RBAC Configuration

#### Principle of Least Privilege

```toml
[security]
default_role = "viewer"  # Most restrictive default
admin_users = ["admin@example.com"]
operator_users = ["ops@example.com"]
```

#### Role Definitions

- **Admin**: Full system access
- **Operator**: Read/write operations, no system config
- **Viewer**: Read-only access

## Network Security

### HTTPS/TLS

#### Certificate Management

```toml
[security]
https_enabled = true
cert_path = "/opt/poolai/certs/cert.pem"
key_path = "/opt/poolai/certs/key.pem"
require_https = true  # Redirect HTTP to HTTPS
```

#### Certificate Best Practices

1. **Use Let's Encrypt** for production:
```bash
certbot certonly --standalone -d poolai.example.com
```

2. **Auto-renewal**:
```bash
# Add to crontab
0 0 * * * certbot renew --quiet --deploy-hook "systemctl reload poolai"
```

3. **Strong Cipher Suites**:
```toml
[security]
tls_min_version = "1.2"
tls_cipher_suites = ["TLS_AES_256_GCM_SHA384", "TLS_CHACHA20_POLY1305_SHA256"]
```

### Firewall Configuration

#### Linux (UFW)

```bash
# Allow only necessary ports
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 443/tcp   # HTTPS
sudo ufw deny 8080/tcp   # Block HTTP if HTTPS enabled
sudo ufw enable
```

#### Network Segmentation

- Separate public-facing API from internal services
- Use private networks for Raft cluster communication
- Implement network policies in Kubernetes

### Rate Limiting

```toml
[api]
rate_limit_per_minute = 60
rate_limit_per_hour = 1000
rate_limit_burst = 10
```

## Data Security

### Encryption at Rest

#### Disk Encryption

```bash
# Use LUKS for disk encryption
sudo cryptsetup luksFormat /dev/sdb
sudo cryptsetup luksOpen /dev/sdb poolai-data
sudo mkfs.ext4 /dev/mapper/poolai-data
```

#### Application-Level Encryption

For sensitive data:

```toml
[security]
encryption_enabled = true
encryption_key = "CHANGE_THIS_TO_RANDOM_32_BYTE_HEX_STRING"
```

### Encryption in Transit

- Always use HTTPS for API communication
- Use TLS 1.2+ for Raft cluster communication
- Encrypt inter-node replication traffic

### Data Sanitization

```toml
[logging]
sanitize_logs = true  # Remove sensitive data from logs
sanitize_fields = ["password", "token", "secret", "key"]
```

## Access Control

### IP Whitelisting

```toml
[security]
allowed_ips = ["10.0.0.0/8", "192.168.1.0/24"]
blocked_ips = []
```

### API Key Management

- Rotate API keys regularly
- Use different keys for different services
- Revoke compromised keys immediately

### Audit Logging

```toml
[audit]
enabled = true
log_all_requests = true
log_sensitive_operations = true
retention_days = 90
```

## Container Security

### Docker Security

#### Non-Root User

```dockerfile
RUN useradd -r -s /bin/false poolai
USER poolai
```

#### Read-Only Filesystem

```yaml
services:
  poolai:
    read_only: true
    tmpfs:
      - /tmp
```

#### Resource Limits

```yaml
services:
  poolai:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G
```

### Kubernetes Security

#### Pod Security Policy

```yaml
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: poolai-psp
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
    - ALL
  runAsUser:
    rule: 'MustRunAsNonRoot'
```

#### Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: poolai-netpol
spec:
  podSelector:
    matchLabels:
      app: poolai
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080
```

## Secrets Management

### Environment Variables

```bash
# Use environment variables for secrets
export POOLAI_JWT_SECRET=$(openssl rand -hex 32)
export POOLAI_DB_PASSWORD=$(openssl rand -base64 32)
```

### Secret Management Tools

#### HashiCorp Vault

```bash
# Store secrets in Vault
vault kv put secret/poolai jwt_secret="$(openssl rand -hex 32)"
```

#### Kubernetes Secrets

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: poolai-secrets
type: Opaque
stringData:
  jwt_secret: "CHANGE_THIS"
```

## Vulnerability Management

### Dependency Scanning

```bash
# Use cargo-audit
cargo install cargo-audit
cargo audit

# Use cargo-deny
cargo install cargo-deny
cargo deny check
```

### Regular Updates

- Keep dependencies up to date
- Monitor security advisories
- Apply security patches promptly

### Security Scanning

```bash
# Use trivy for container scanning
trivy image poolai:latest

# Use snyk for dependency scanning
snyk test
```

## Incident Response

### Security Monitoring

- Monitor failed login attempts
- Track unusual API access patterns
- Alert on security events

### Incident Response Plan

1. **Detection**: Identify security incidents
2. **Containment**: Isolate affected systems
3. **Eradication**: Remove threats
4. **Recovery**: Restore normal operations
5. **Lessons Learned**: Document and improve

### Logging and Monitoring

```toml
[security]
log_failed_auth = true
log_suspicious_activity = true
alert_on_brute_force = true
brute_force_threshold = 5  # Failed attempts
brute_force_window_seconds = 300  # 5 minutes
```

## Compliance

### Data Protection

- Implement GDPR compliance for EU users
- Encrypt personal data
- Provide data export/deletion capabilities

### Audit Requirements

- Maintain audit logs for 90+ days
- Log all administrative actions
- Track data access

## Security Checklist

### Pre-Deployment

- [ ] Strong JWT secret configured
- [ ] HTTPS enabled with valid certificates
- [ ] Firewall rules configured
- [ ] Rate limiting enabled
- [ ] RBAC properly configured
- [ ] Secrets stored securely
- [ ] Non-root user configured
- [ ] Audit logging enabled

### Post-Deployment

- [ ] Security monitoring active
- [ ] Regular security scans scheduled
- [ ] Incident response plan documented
- [ ] Backup and recovery tested
- [ ] Access logs reviewed regularly
- [ ] Security updates applied

### Ongoing

- [ ] Regular security audits
- [ ] Dependency updates
- [ ] Certificate renewal automated
- [ ] Security patches applied
- [ ] Access reviews conducted
- [ ] Incident response tested

## Security Tools

### Recommended Tools

- **cargo-audit**: Dependency vulnerability scanning
- **cargo-deny**: License and security policy enforcement
- **trivy**: Container security scanning
- **snyk**: Dependency vulnerability management
- **fail2ban**: Brute force protection
- **certbot**: SSL certificate management

## Reporting Security Issues

If you discover a security vulnerability:

1. **Do NOT** create a public GitHub issue
2. Email security@poolai.example.com
3. Include detailed description and steps to reproduce
4. Allow time for fix before public disclosure

## References

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://rust-lang.github.io/rust-clippy/master/index.html)
- [CIS Benchmarks](https://www.cisecurity.org/cis-benchmarks/)

