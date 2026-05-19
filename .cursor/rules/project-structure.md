# Project Structure Rules

## Directory Organization

- `src/` - Source code (all Rust modules)
- `tests/` - Integration tests (all test files)
- `docs/` - Documentation (all markdown files)
  - `docs/troubleshooting/` - Troubleshooting guides (QUICK_FIX_MSYS2.md, etc.)
- `scripts/` - Build/deployment scripts (all shell scripts, bash NOT PowerShell)
- `docker/` - Docker files (Dockerfile, docker-compose.yml)
- `.cursor/` - Cursor agent configuration (rules, commands, plans, project skills)
  - `.cursor/rules/` - All Cursor rules (including `.cursorrules` moved here)
  - `.cursor/skills/` - Project Agent Skills (e.g. `poolai-documentation/SKILL.md` — карта доків 1–11 та витяг функціоналу)

## File Naming

- Rust files: `snake_case.rs`
- Test files: `test_*.rs` or `*_test.rs`
- Documentation: `UPPER_CASE.md` for important docs, `lowercase.md` for guides
- Scripts: `snake_case.sh` (у репо — лише bash у `scripts/`)

## Module Organization

- Each module has a `mod.rs` file
- Sub-modules in separate files (e.g., `src/raid/burst_raid.rs`)
- Public API exported through `pub use` in `mod.rs`
- See `src/lib.rs` for public API structure
- Horizon wire modules: `src/grid/`, `src/job/`, `src/memory/`; workspace crate `crates/poolai-solana-adapter/`

## Documentation

- Main docs in `docs/` directory
- **Catalog / functionality digest:** `docs/catalog/` (canonical step **11** in root README)
- Status reports in `docs/status/`
- Development plans in `docs/development/`
- Architecture docs in `docs/ARCHITECTURE_*.md`
- Never create markdown files in root (except README.md, LICENSE)

## Scripts

- All scripts MUST be in `scripts/` directory
- Never create `.sh` or `.ps1` files in root directory
- See `scripts/README.md` for script documentation

## File Listing Rules

**When creating file lists or inventories:**
- ❌ **NEVER** use `.ps1` or `.ps` extensions in file lists
- ✅ Use descriptive names: "MSVC environment setup script" instead of `setup_msvc_environment.ps1`
- ✅ Group by category: e.g. "Bash scripts", "Build helpers"
- ✅ Use markdown lists or tables, not PowerShell command output

## Git

- Use Conventional Commits format
- One logical change per commit
- See `git-workflow.md` for detailed commit format and workflow rules
