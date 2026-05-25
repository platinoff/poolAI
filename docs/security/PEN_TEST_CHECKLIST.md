# Penetration Test Checklist (PoolAI)

**PH-S24** · **Last updated:** 2026-05-24  
**Scope:** Coordinator HTTP API, admin UI, enterprise auth, RAID/VM admin, TLS/metrics sidecars.

Use with [`OWASP_TOP10_CHECKLIST.md`](./OWASP_TOP10_CHECKLIST.md) and [`SECRETS_MANAGEMENT.md`](./SECRETS_MANAGEMENT.md).

---

## Pre-test setup

| Step | Action |
|------|--------|
| 1 | Run against a **non-production** stand (`bin/run-poolai.sh` or `bin/verify-dev-stand.sh`) |
| 2 | Record build: `git rev-parse HEAD`, `cargo --version`, feature flags |
| 3 | Enable audit log dir (`data/audit/`) and note `POOLAI_*` env (no secrets in report) |
| 4 | Obtain admin + viewer test accounts (default dev stand) |
| 5 | Optional: `cargo run --bin poolai-openapi-gap-audit` for route inventory |

---

## 1. Authentication and session

| ID | Test | Pass criteria |
|----|------|----------------|
| AUTH-01 | `GET /api/v1/workers` without `Authorization` | 401 / 403 |
| AUTH-02 | Expired or malformed Bearer token | 401 |
| AUTH-03 | `POST /api/v1/login` brute force (rate limit) | 429 after threshold (`rate_limit` middleware) |
| AUTH-04 | `POST /api/v1/refresh` with revoked/unknown token | 401 |
| AUTH-05 | JWT signed with wrong secret | Rejected when `jwt` feature enabled |
| AUTH-06 | During JWT rotation: token signed with **previous** secret within grace | Accepted if `POOLAI_JWT_SECRET_PREVIOUS` set |

---

## 2. Authorization (RBAC)

| ID | Test | Pass criteria |
|----|------|----------------|
| RBAC-01 | Viewer calls `POST /api/v1/users` | 403 |
| RBAC-02 | Viewer calls `DELETE /api/v1/vm/instances/{id}` | 403 |
| RBAC-03 | Operator cannot call `POST /api/v1/admin/secrets/rotate` | 403 |
| RBAC-04 | Admin can call `GET /api/v1/admin/secrets/rotation` | 200, no secret values in JSON |

---

## 3. Input validation and injection

| ID | Test | Pass criteria |
|----|------|----------------|
| INJ-01 | Path traversal in RAID/library paths | 400 / blocked (`validation.rs`) |
| INJ-02 | SSRF in URL fields (webhooks, cloud callbacks) | Rejected private IPs |
| INJ-03 | Oversized JSON body on write endpoints | 413 / 400 |
| INJ-04 | XSS payloads in admin UI labels (stored) | Escaped in DOM / no execution |

---

## 4. Transport and headers

| ID | Test | Pass criteria |
|----|------|----------------|
| TLS-01 | HTTPS deployment: TLS 1.3 preferred | Scanner shows TLS 1.3 |
| TLS-02 | HTTP response security headers | `security_headers` middleware present |
| TLS-03 | Cert reload (`HTTPS_CERT_RELOAD_SECS`) | No downtime; see [`TLS.md`](./TLS.md) |
| TLS-04 | HSTS only when HTTPS enabled | No HSTS on plain HTTP dev |

---

## 5. Secrets and rotation (PH-S24)

| ID | Test | Pass criteria |
|----|------|----------------|
| SEC-01 | Repo scan: no live secrets in git | `git grep -i password\|jwt_secret` clean |
| SEC-02 | `POST /api/v1/admin/secrets/rotate` `{"kind":"jwt"}` as admin | 200; `rotation_count` increments |
| SEC-03 | Rotate without auth | 401 |
| SEC-04 | Env: set `POOLAI_JWT_SECRET` + `POOLAI_JWT_SECRET_PREVIOUS`, reload hooks | Old tokens valid during grace |
| SEC-05 | `POOLAI_TELEGRAM_WEBHOOK_SECRET` rotation hook | Hook reports configured when env set |

---

## 6. API surface and information disclosure

| ID | Test | Pass criteria |
|----|------|----------------|
| INFO-01 | `GET /metrics` (Prometheus feature) | No credentials in labels |
| INFO-02 | Error JSON bodies | No stack traces in production mode |
| INFO-03 | `GET /api/v1/status` HTML | No internal paths/secrets |
| INFO-04 | OpenAPI vs live routes | Gap audit documented |

---

## 7. Enterprise and admin UI

| ID | Test | Pass criteria |
|----|------|----------------|
| UI-01 | pa11y/axe CI routes (18+ admin URLs) | Zero critical violations |
| UI-02 | Playwright admin flows (tenants, security, audit) | Pass in `e2e/tests/admin.spec.ts` |
| UI-03 | CSRF: state-changing admin forms | Session/auth required |
| UI-04 | OAuth2/SAML test endpoints | Invalid state rejected |

---

## 8. Distributed / ops (when enabled)

| ID | Test | Pass criteria |
|----|------|----------------|
| OPS-01 | Raft RPC without membership | Rejected / unreachable |
| OPS-02 | LAN discovery spoofed peer | No trust without auth |
| OPS-03 | Worker join with bad attestation | Rejected |

---

## Reporting template

```markdown
## PoolAI pen-test — YYYY-MM-DD
- Target: <host:port>
- Commit: <hash>
- Tester:

### Findings
| ID | Severity | Summary | Remediation |
|----|----------|---------|-------------|
| AUTH-01 | Low | ... | ... |

### Evidence
- Request/response redacted snippets
- Screenshot or HAR (no secrets)
```

---

## Automation references

```bash
# Contract / unit (MSYS2 bash)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
cd /s/rust/poolAI
cargo test-ci
cargo test --test secret_rotation_integration --features enterprise -- --test-threads=1

# E2E
bash bin/e2e-playwright.sh --start
```

---

## Sign-off

| Role | Name | Date |
|------|------|------|
| Security reviewer | | |
| Engineering lead | | |
