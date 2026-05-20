# Project Structure Rules

## Runtime stack (канон)

- **Primary:** Rust — `src/`, `tests/`, `crates/` (див. `.cursor/rules/runtime-stack-policy.mdc`, **alwaysApply**).
- **UI:** JavaScript у `src/ui/`; E2E — `e2e/` (TypeScript).
- **Python:** **заборонено** в репозиторії (0× `.py`); OpenAPI audit — `cargo run --bin poolai-openapi-gap-audit`.
- **Java:** у репо немає; не додавати без явного запиту.
- Архівні docs з Python — ігнорувати для імплементації.

## Repository layout (люди vs Cargo)

**Повний опис:** `docs/development/REPOSITORY_LAYOUT.md`.

| Шлях | Призначення |
|------|-------------|
| `src/` | Rust product code (library + `main`) |
| `src/bin/` | **Cargo binaries only** — `cargo run --bin NAME` |
| `bin/` | **Dev/ops launchers** — `.sh` / `.ps1` (run, LAN, verify, e2e) |
| `scripts/` | **Toolchain / deploy** — MSYS PATH, gcc, build verify, git helpers |
| `tests/` | Integration tests (Rust convention) |
| `crates/*/src/` | Workspace members (e.g. `poolai-solana-adapter`) |

**Не плутати:** кореневий `bin/` ≠ `src/bin/`. Rust `.rs` binaries **тільки** в `src/bin/`.

## Directory Organization

- `src/` - Source code (all Rust modules)
- `tests/` - Integration tests (all test files)
- `docs/` - Documentation (all markdown files)
  - `docs/troubleshooting/` - Troubleshooting guides (QUICK_FIX_MSYS2.md, etc.)
- `bin/` - Dev/ops shell scripts (launch, LAN stand, verify, cargo helpers)
- `scripts/` - Toolchain, MSYS setup, deployment helpers
- `docker/` - Docker files (Dockerfile, docker-compose.yml)
- `.cursor/` - Cursor agent configuration (rules, commands, plans, project skills)
  - `.cursor/rules/` - All Cursor rules (including `.cursorrules` moved here)
  - `.cursor/skills/` - Project Agent Skills (e.g. `poolai-documentation/SKILL.md` — карта доків 1–11 та витяг функціоналу)

## File Naming

- Rust files: `snake_case.rs`
- Test files: `test_*.rs` or `*_test.rs`
- Documentation: `UPPER_CASE.md` for important docs, `lowercase.md` for guides
- Ops scripts: `snake_case.sh` / `kebab-case.ps1` in `bin/` or `scripts/`

## Module Organization

- Each module has a `mod.rs` file
- Sub-modules in separate files (e.g., `src/raid/burst_raid.rs`)
- Public API exported through `pub use` in `mod.rs`
- See `src/lib.rs` for public API structure
- Horizon wire modules: `src/grid/`, `src/job/`, `src/memory/`; workspace crate `crates/poolai-solana-adapter/`

## Documentation

- Main docs in `docs/` directory
- **Catalog / functionality digest:** `docs/catalog/` (canonical step **11** in root README)
- **Layout for humans:** `docs/development/REPOSITORY_LAYOUT.md`
- Status reports in `docs/status/`
- Development plans in `docs/development/`
- Architecture docs in `docs/ARCHITECTURE_*.md`
- Never create markdown files in root (except README.md, LICENSE)

## Scripts policy

| Куди | Що |
|------|-----|
| **`bin/`** | run-poolai, LAN nodes, verify-dev-stand, e2e-playwright, pa11y, capture metrics |
| **`scripts/`** | setup_rust_path, fix_gcc, verify_build, deployment, git-push shell helpers |

- Never create `.sh` / `.ps1` in repo root.
- No duplicate logic: one canonical file; old path → forwarder with `DEPRECATED` comment.
- See `bin/README.md` and `scripts/README.md`.

## File Listing Rules

**When creating file lists or inventories:**
- ❌ **NEVER** use `.ps1` or `.ps` extensions in file lists
- ✅ Use descriptive names: "MSYS environment setup script" instead of `setup_msvc_environment.ps1`
- ✅ Group by category: e.g. "Bash scripts", "Build helpers"
- ✅ Use markdown lists or tables, not PowerShell command output

## Git

- Use Conventional Commits format
- One logical change per commit
- See `git-workflow.md` for detailed commit format and workflow rules
