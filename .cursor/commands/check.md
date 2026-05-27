# Check Command

Run comprehensive code checks before committing (**MSYS2 bash**).

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
```

Recommended sequence:

1. `cargo fmt --all --check` (or `cargo fmt --all` to fix)
2. `cargo test-ci` (канон PoolAI; `.cargo/config.toml`)
3. After API changes: `cargo run --bin poolai-openapi-gap-audit` (expect **0** missing)
4. Optional: `cargo clippy --all-targets --all-features` (CI parity)
5. After `src/ui/` or `e2e/`: `cd e2e && npm run test:ci`

Return summary of results.
