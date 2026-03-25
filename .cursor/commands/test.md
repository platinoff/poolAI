# Test Command

Run tests for the project.

Recommended:
1. Run `cargo test --all-features` to run full test suite
2. For context memory changes: `cargo test --test context_memory_integration`
3. For enterprise audit logger changes: `cargo test --test enterprise_audit_integration --features enterprise`
4. For enterprise monitoring changes: `cargo test --test enterprise_monitoring_integration --features enterprise` and `cargo test --test enterprise_monitoring_sqlite_integration --features enterprise`
5. For cloud mock provider changes: `cargo test --test cloud_mock_integration --features cloud,cloud-sdk -- --test-threads=1` (requires `rustc >= 1.88` for AWS SDK; ensure enough free disk space to build dependencies)
6. Report test results and any failures
7. If tests fail, analyze the failures and suggest fixes

Return test summary.
