# Dependency Security and Updates

## Overview

This document outlines the strategy for managing dependencies securely, including security audits, version updates, and vulnerability management.

## Security Audit Tools

### Cargo Audit

**Installation:**
```bash
cargo install cargo-audit
```

**Usage:**
```bash
# Check for known vulnerabilities
cargo audit

# Audit with JSON output
cargo audit --json

# Fix automatically (if possible)
cargo audit fix
```

**Status**: ✅ Integrated in CI pipeline (`.github/workflows/ci.yml`)

### Cargo Deny

**Installation:**
```bash
cargo install cargo-deny
```

**Usage:**
```bash
# Check for license issues
cargo deny check licenses

# Check for duplicate dependencies
cargo deny check duplicates

# Check for banned crates
cargo deny check bans

# Check for security advisories
cargo deny check advisories
```

## Dependency Update Strategy

### Current Versions (2026-01-16)

#### Core Dependencies
- `tokio`: `1.49` ✅ (Latest stable)
- `axum`: `0.8` ✅ (Latest stable)
- `serde`: `1.0` ✅ (Latest stable)
- `tracing`: `0.1` ✅ (Latest stable)
- `reqwest`: `0.13` ✅ (Latest stable)

#### Security-Critical Dependencies
- `jsonwebtoken`: `10.2` ✅ (With `rust_crypto` feature)
- `sha2`: `0.10` ✅ (Latest stable)
- `chrono`: `0.4` ✅ (Latest stable)
- `uuid`: `1.19` ✅ (Latest stable)

#### Optional Dependencies
- `async-raft`: `0.6.1` ✅ (Latest stable)
- `k8s-openapi`: `0.21` ✅ (Latest stable)
- `azure_core`: `0.30` ✅ (Latest stable)
- `azure_identity`: `0.30` ✅ (Latest stable)

### Update Policy

#### Major Version Updates
- Review changelog and breaking changes
- Test thoroughly before updating
- Update incrementally (one major version at a time)

#### Minor/Patch Updates
- Apply automatically via Dependabot
- Review security advisories
- Test in CI before merging

#### Security Updates
- **Priority**: Apply immediately
- **Process**: 
  1. Review `cargo audit` output
  2. Check RustSec advisory database
  3. Update to patched version
  4. Test in CI
  5. Deploy to production

## Automated Dependency Updates

### GitHub Dependabot

**Configuration** (`.github/dependabot.yml`):
```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/poolAI"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
    reviewers:
      - "poolai-team"
    labels:
      - "dependencies"
      - "security"
```

**Status**: 🔄 To be enabled

### CI/CD Integration

Security audits run automatically on:
- Every pull request
- Weekly scheduled runs
- On-demand via GitHub Actions

## Vulnerability Response Process

### 1. Detection
- `cargo audit` in CI pipeline
- GitHub Dependabot alerts
- RustSec advisory notifications

### 2. Assessment
- Check severity (critical, high, medium, low)
- Review affected functionality
- Determine exploitability

### 3. Remediation
- Update to patched version (if available)
- Apply workaround (if update not possible)
- Remove dependency (if necessary)

### 4. Verification
- Run full test suite
- Security testing for affected features
- Performance testing

### 5. Deployment
- Deploy patch to production
- Monitor for issues
- Document in security log

### Governance pointers (Galaxy §9.6)

**Galaxy policy links** (§9.2 / §9.3 / §9.6) and operator runbooks (`poolai-verify-release`, protocol triage, release advisories) live in one hub — do not duplicate here:

[`SECURITY_HARDENING.md` — Galaxy governance canonical pointers](./SECURITY_HARDENING.md#galaxy-governance-canonical-pointers-ph-s69-ph-s77)

This document owns: `cargo audit` / RustSec workflow, Dependabot, and the vulnerability response steps above.

## Known Vulnerabilities

### Currently None

All dependencies are up-to-date with no known vulnerabilities (as of 2026-01-16).

## Dependency Pinning

### Cargo.lock

`Cargo.lock` is committed to the repository for:
- Reproducible builds
- Security audit consistency
- Dependency version tracking

**Note**: `Cargo.lock` should be updated regularly with `cargo update`.

## Security Best Practices

### 1. Minimize Dependencies
- Only include necessary dependencies
- Remove unused dependencies regularly
- Consider alternatives for heavy dependencies

### 2. Use Trusted Sources
- Prefer crates.io packages
- Verify maintainer activity
- Check download statistics

### 3. Regular Updates
- Review dependencies monthly
- Apply security patches immediately
- Test updates thoroughly

### 4. Version Constraints
- Use semantic versioning (`^` for compatible updates)
- Pin critical dependencies (exact version if needed)
- Document version requirements

### 5. Feature Flags
- Use feature flags for optional dependencies
- Disable unused features to reduce attack surface
- Document feature dependencies

## Dependency Review Checklist

When adding a new dependency:

- [ ] Check RustSec advisory database
- [ ] Review maintainer activity (recent commits, issues)
- [ ] Check download statistics and popularity
- [ ] Review license compatibility
- [ ] Verify feature flags (only enable needed features)
- [ ] Test with security audit (`cargo audit`)
- [ ] Document dependency purpose and usage

## License Compliance

### License Check

```bash
cargo deny check licenses
```

### License Policy

- **Permissive licenses**: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause ✅
- **Copyleft licenses**: GPL, AGPL (review case-by-case) ⚠️
- **Commercial licenses**: Require approval ❌

### License Attribution

All licenses are documented in `LICENSE` file and dependency metadata.

## Resources

- [RustSec Advisory Database](https://rustsec.org/advisories/)
- [Cargo Audit Documentation](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [Cargo Deny Documentation](https://github.com/EmbarkStudios/cargo-deny)
- [Dependabot Documentation](https://docs.github.com/en/code-security/dependabot)

---

**Last Updated**: 2026-05-27  
**Version**: 1.2 - Pointer to SECURITY_HARDENING Galaxy hub (PH-S77); no duplicate §9.2/§9.6 prose
