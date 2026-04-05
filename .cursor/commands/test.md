# Test Command

Run tests for the project.

**Required CI parity** (matches `.github/workflows/ci.yml` required step; set before `cargo` on Windows PowerShell: `$env:K8S_OPENAPI_ENABLED_VERSION='1.28'`):

1. `cargo test --lib --tests --features ml,enterprise,cloud`

Then, as needed:

2. Run `cargo test --all-features` for a fuller matrix (heavy; on Windows MSVC may hit compiler stack issues — prefer GNU toolchain from `rust-toolchain.toml` or Linux CI)
3. For context memory changes: `cargo test --test context_memory_integration`
4. For enterprise audit logger changes: `cargo test --test enterprise_audit_integration --features enterprise`
5. For enterprise monitoring changes: `cargo test --test enterprise_monitoring_integration --features enterprise` and `cargo test --test enterprise_monitoring_sqlite_integration --features enterprise`
6. For cloud mock provider changes: `cargo test --test cloud_mock_integration --features cloud,cloud-sdk -- --test-threads=1` (requires `rustc >= 1.88` for AWS SDK; ensure enough free disk space to build dependencies)
7. Report test results and any failures
8. If tests fail, analyze the failures and suggest fixes

Return test summary.
