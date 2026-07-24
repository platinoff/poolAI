# Local TLS certificates (dev only)

**Do not commit** `*.pem` / `*.key` here. Private keys and PEMs are gitignored (security hygiene PH-SVC55).

## Generate self-signed certs for local HTTPS

```bash
mkdir -p certs
openssl req -x509 -newkey rsa:2048 \
  -keyout certs/key.pem -out certs/cert.pem \
  -days 365 -nodes -subj "/CN=localhost"
chmod 600 certs/key.pem
chmod 644 certs/cert.pem
```

Env overrides: `HTTPS_CERT_PATH`, `HTTPS_KEY_PATH` (see `docs/security/TLS.md`).

If these files were ever pushed historically, regenerate local material and treat the old key as compromised (rotate). History rewrite requires explicit OWNER approval (PH-SVC36).
