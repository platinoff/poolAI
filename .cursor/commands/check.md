# Check Command

Run comprehensive code checks before committing.

Recommended sequence (run in **MSYS2 bash**):

1. `cargo fmt --all --check` (перевірка формату без змін)
2. `cargo clippy --all-targets --all-features` (linting)
3. `cargo check --all-targets --all-features` (компіляція)
4. `cargo test --all-features` (повні тести)
5. `cargo build --all-features` (збірка з features)

If step `cargo fmt --all --check` fails:
1. Run `cargo fmt --all`
2. Re-run the checks from step 1.

Return summary of results.
