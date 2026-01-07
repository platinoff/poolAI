# 🔒 Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## 🚨 Reporting a Vulnerability

If you discover a security vulnerability, please **DO NOT** open a public issue.

### How to Report

1. **Email**: Send details to [security@poolai.dev] (if available) or create a private security advisory on GitHub
2. **GitHub Security Advisory**: Use GitHub's [Private Vulnerability Reporting](https://github.com/platinoff/poolAI/security/advisories/new)

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Time

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity

## 🔐 Security Best Practices

### For Users

- Keep PoolAI updated to the latest version
- Use HTTPS/TLS in production
- Secure your JWT tokens
- Regularly rotate API keys
- Monitor security advisories

### For Developers

- Follow secure coding practices
- Review dependencies regularly (`cargo audit`)
- Use `cargo clippy` for security checks
- Avoid `unsafe` code unless necessary
- Validate all user inputs

## 🛡️ Security Features

PoolAI includes the following security features:

- ✅ JWT authentication
- ✅ Role-based access control (RBAC)
- ✅ HTTPS/TLS support
- ✅ Rate limiting
- ✅ Input validation
- ✅ Secure error handling

## 📋 Security Checklist

When contributing:

- [ ] No hardcoded secrets
- [ ] Input validation implemented
- [ ] Error messages don't leak sensitive info
- [ ] Dependencies are up-to-date
- [ ] Security tests added
- [ ] Documentation updated

## 🔍 Security Scanning

We use:

- `cargo audit` - Dependency vulnerability scanning
- GitHub Dependabot - Automated dependency updates
- Code review - Manual security review

## 📚 Resources

- [Rust Security Guidelines](https://rust-lang.github.io/rust-clippy/master/index.html#security)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Cargo Security](https://doc.rust-lang.org/cargo/reference/security.html)

---

**Last Updated**: 2025-12-30

