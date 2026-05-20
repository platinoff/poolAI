# Rust Project Rules

**Stack policy:** PoolAI product code is **Rust-only** (plus `src/ui/` JS). Do not add Python runtime, `requirements.txt`, or Python sidecars. See `.cursor/rules/runtime-stack-policy.mdc`.

## Commands

- `cargo check`: Check code compilation without building
- `cargo build`: Build the project
- `cargo build --release`: Build optimized release version
- `cargo test`: Run all tests (prefer single test files for speed: `cargo test --test <test_file>`)
- `cargo fmt`: Format code (always run before commit)
- `cargo clippy`: Run linter (always run before commit)
- `cargo doc --open`: Generate and open documentation
- `cargo run`: Run the main binary
- `cargo run --features enterprise`: Run with enterprise features
- `cargo run --features jwt`: Run with JWT authentication
- `cargo run --features https`: Run with HTTPS support

## Code Style

- Use Rust Edition 2021 conventions
- Follow Rust naming conventions: `snake_case` for functions/variables, `PascalCase` for types
- Use `pub` only when necessary - prefer private by default
- Use `Arc<RwLock<T>>` for shared mutable state across async tasks
- Use `Option<T>` and `Result<T, E>` for error handling - never use unwrap() in production code
- Use `async/await` for I/O operations, prefer `tokio` runtime
- Use `tracing` for logging: `info!()`, `warn!()`, `error!()`, `debug!()`
- See `src/core/error.rs` for canonical error handling patterns
- See `src/raid/mod.rs` for canonical module structure with `mod.rs`

## Module Structure

- Each module has a `mod.rs` file
- Sub-modules in separate files (e.g., `src/raid/burst_raid.rs`)
- Public API exported through `pub use` in `mod.rs`
- Private implementation details stay in sub-modules
- See `src/lib.rs` for public API structure

## Error Handling

- Use `AppError` from `crate::core::error::AppError`
- Always provide context and suggestions in error messages
- Use `?` operator for error propagation
- Never use `unwrap()` or `expect()` in production code
- Use `Result<T, AppError>` for fallible operations

## Async Patterns

- Use `tokio::spawn` for background tasks
- Store `JoinHandle` in `Arc<RwLock<Option<JoinHandle<()>>>>` for task management
- Use `tokio::time::interval` for periodic tasks
- Use `Arc::clone()` for sharing data across tasks
- Always implement `shutdown()` methods to abort background tasks

## Testing

- Before heavy runs (`cargo test --all-features`, full clippy matrices): optional `bash scripts/check_target_disk.sh` — warns if free space on the repo volume is below **POOLAI_MIN_FREE_DISK_GB** (default 12) or `target/` exceeds **POOLAI_MAX_TARGET_DIR_GB** (default 48). Use `--enforce` or `POOLAI_ENFORCE_DISK_LIMIT=1` to fail fast. See `rust-architect.md` (target disk policy).
- Unit tests in `tests/` directory
- Use `#[tokio::test]` for async tests
- Use `tempfile` crate for temporary directories in tests
- Test error cases, not just happy paths
- See `tests/raid_*` for canonical test patterns

## Documentation

- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Include examples in doc comments when helpful
- Update `docs/` directory for architectural changes
- See `docs/ARCHITECTURE_BEST_PRACTICES.md` for patterns

## Workflow

- Always run `cargo fmt` after making code changes
- Always run `cargo clippy` before committing
- Always run `cargo test` before committing
- Use Conventional Commits format (see `.cursorrules`)
- Update documentation when adding new features
- Check `docs/status/CURRENT_STATUS.md` for project status

## Feature Flags

- Use `#[cfg(feature = "...")]` for optional features
- Document feature requirements in `Cargo.toml`
- Test with and without features enabled
- See `Cargo.toml` for available features

## Git Workflow

- Use Conventional Commits: `feat(scope): description`
- One logical change per commit
- Include tests for new features
- Update documentation for new features
- See `.cursorrules` for detailed commit format
