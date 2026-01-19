# Project Structure Rules

## Directory Organization

- `src/` - Source code (all Rust modules)
- `tests/` - Integration tests (all test files)
- `docs/` - Documentation (all markdown files)
- `scripts/` - Build/deployment scripts (all shell scripts)
- `docker/` - Docker files (Dockerfile, docker-compose.yml)
- `.cursor/` - Cursor agent configuration (rules, commands, plans)

## File Naming

- Rust files: `snake_case.rs`
- Test files: `test_*.rs` or `*_test.rs`
- Documentation: `UPPER_CASE.md` for important docs, `lowercase.md` for guides
- Scripts: `snake_case.sh` or `snake_case.ps1`

## Module Organization

- Each module has a `mod.rs` file
- Sub-modules in separate files (e.g., `src/raid/burst_raid.rs`)
- Public API exported through `pub use` in `mod.rs`
- See `src/lib.rs` for public API structure

## Documentation

- Main docs in `docs/` directory
- Status reports in `docs/status/`
- Development plans in `docs/development/`
- Architecture docs in `docs/ARCHITECTURE_*.md`
- Never create markdown files in root (except README.md, LICENSE)

## Scripts

- All scripts MUST be in `scripts/` directory
- Never create `.sh` files in root directory
- See `scripts/README.md` for script documentation

## Git

- Use Conventional Commits format
- One logical change per commit
- See `.cursorrules` for detailed commit format
