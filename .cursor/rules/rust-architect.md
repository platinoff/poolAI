# Rust Architect Rules - PoolAI Development

## 🏗️ Rust Architect Workflow

### ⚠️ CRITICAL: MSYS2 Bash only — No PowerShell, No cmd

**Terminal**: `C:\msys64\usr\bin\bash.exe` (ЗАВЖДИ для автономної роботи)  
**Environment**: MSYS2 UCRT64, `rust-toolchain.toml` (1.92.0, GNU)  
**Do NOT use**: PowerShell, cmd.exe, Cursor integrated terminal для git

**Автономна робота**: Всі команди виконуються через `C:\msys64\usr\bin\bash.exe`

### Key Documents for Rust Architect

**PRIMARY Concept Document** (ALWAYS check first):
- `docs/concept/poolAI_concept_root.txt` - PRIMARY concept document (USE THIS FIRST)
- Contains: Complete architecture, module status, implementation details

**Alternative Concept Documents**:
- `docs/concept/poolAI_concept.txt` - Ukrainian version, detailed development environment info
- `docs/concept/poolAI_concept_workspace.txt` - Workspace-specific context

**Status & Planning Documents**:
- `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` - Current project status
- `docs/status/STABLE_STATE_UPDATE_2026-01-19.md` - Stable state reference (v0.2.2)
- `docs/concept/CONCEPT_UPDATE_2026-01-19.md` - Concept update (v7)
- `docs/development/NEXT_STEPS_2026-01-19.md` - Latest next steps (v0.2.2 → v0.3.0+)
- `docs/development/NEXT_STEPS_2026-01-22.md` - Current development roadmap

**Helper Files**:
- `file_list.csv` - Complete project file inventory (108254+ lines)
  - Location: `S:\rust\poolAI\file_list.csv`
  - Use for: File navigation, structure analysis, finding files by name/path
  - Format: CSV with `FullName`, `Length`, `LastWriteTime` columns

### Workflow Rules

1. **Before Starting Any Task**:
   - ✅ Read `docs/concept/poolAI_concept_root.txt` for architecture context
   - ✅ Check `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` for current status
   - ✅ Review relevant module documentation in `docs/`
   - ✅ Use `file_list.csv` to locate files if needed

2. **When Updating Concept/Plans**:
   - ✅ Update `docs/concept/poolAI_concept_root.txt` FIRST (PRIMARY document)
   - ✅ Sync changes to `docs/concept/CONCEPT_UPDATE_2026-01-19.md` (v7)
   - ✅ Update `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` for status changes
   - ✅ Update `docs/status/STABLE_STATE_UPDATE_2026-01-19.md` for stable state
   - ✅ Update `docs/development/NEXT_STEPS_2026-01-19.md` for plan changes
   - ✅ Ensure version numbers match: `Cargo.toml` (v0.2.2) → `src/version.rs` → docs

3. **When Answering Questions**:
   - ✅ Check concept files FIRST before making assumptions
   - ✅ Reference exact file paths: `docs/concept/poolAI_concept_root.txt`
   - ✅ Verify information against status documents
   - ✅ Use `file_list.csv` to find specific files

4. **Terminal** (MSYS2 bash only; no PS, no cmd, **без .sh** для git push):
   - **Patches**: `rust-toolchain.toml`, `.cursor`, `.vscode`, `scripts/`
   - Git: copy-paste блок з `.cursor/commands/git-push.md` (fmt, add, commit, push).
   ```bash
   cargo check && cargo test && cargo fmt --all
   git status --short
   git add <paths> && git commit -m "type(scope): subject" && git push origin main
   ```
   See `git-workflow.md`, `git-push.md`.

5. **File Organization**:
   - ✅ All documentation in `docs/` directory
   - ✅ Concept files in `docs/concept/`
   - ✅ Status reports in `docs/status/`
   - ✅ Development plans in `docs/development/`
   - ✅ Scripts in `scripts/` (bash only; no PowerShell, no cmd)
   - ✅ NEVER create `.md` files in root (except README files)

6. **Error Prevention**:
   - ✅ Always check concept files before implementing features
   - ✅ Verify module completion status in status documents
   - ✅ Check for existing implementations before adding new code
   - ✅ Use `file_list.csv` to find related files
   - ✅ Follow Rust patterns from `src/core/error.rs` and `src/raid/mod.rs`

### Document Synchronization

**When updating version numbers**:
- `Cargo.toml` → `src/version.rs` → `docs/concept/poolAI_concept_root.txt` → `docs/concept/poolAI_concept.txt` → `README.md`

**When updating module status**:
- `docs/concept/poolAI_concept_root.txt` → `docs/status/PROJECT_STATUS_REPORT_*.md` → `README.md`

**When updating development plans**:
- `docs/development/NEXT_STEPS_*.md` → `docs/concept/poolAI_concept_root.txt` (if architecture changes)

### MSYS2 Environment Setup

**CRITICAL**: Always use MSYS2 bash, NOT PowerShell:
- Terminal path: `C:\msys64\usr\bin\bash.exe`
- Environment: MSYS2 UCRT64
- PATH includes: `/c/msys64/ucrt64/bin:/c/msys64/usr/bin`

**For building with features**:
```bash
# In MSYS2 bash terminal
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
export CC="gcc"
export CXX="g++"
cargo build --features enterprise,https,jwt
```

**Troubleshooting**:
- See `docs/troubleshooting/QUICK_FIX_MSYS2.md` for MSYS2 issues (quick fix guide)
- See `docs/troubleshooting/GCC_DLLTOOL_NOT_FOUND.md` for compilation issues
- See `docs/troubleshooting/RUST_VERSION_ISSUE.md` for Rust version issues
- All troubleshooting guides in `docs/troubleshooting/`

### Git Workflow

**Commit Format** (Conventional Commits):
```
type(scope): description

- Detailed change 1
- Detailed change 2
```

**Types**: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `style`

**Before Committing**:
- ✅ Run `cargo fmt --all` (or let pre-push hook handle it)
- ✅ Run `cargo clippy --all-targets --all-features`
- ✅ Run `cargo test` (or specific test file)
- ✅ Update relevant documentation
- ✅ Check concept files are synchronized

**Before Pushing** (Automatic):
- ✅ Pre-push hook automatically runs `cargo fmt --all --check`
- ✅ If formatting fails, hook will auto-format and require commit
- ✅ Manual format: `cargo fmt --all` (in MSYS2 bash)

### Current Project State (2026-01-22)

**Version**: v0.2.2 Production Ready ✅  
**Status**: STABLE - All core modules 100% complete  
**Tests**: 437+ passing (102 unit + 325+ integration)  
**Rust Toolchain**: 1.92.0 (GNU target: x86_64-pc-windows-gnu)

**Completed Modules (15/15 - 100%)**:
- ✅ Core, Pool, Monitoring, Network, Platform, Runtime, Rewards, TGBot, Security
- ✅ Enterprise (100% - SQLite, OAuth2, SAML SSO)
- ✅ Cloud (100% - AWS/Azure/GCP, Auto-scaling, Load Balancing, HPA)
- ✅ RAID (100% - BurstRAID, SmallWorld, Admin Control Plane)
- ✅ VM, UI, Libs (100%)

**Next Steps (v0.3.0+)**:
- ⏸️ Stage 4.4 AI/ML: Model Optimization (ML.1 ✅), AutoML (ML.2 stub), Federated Learning (ML.3 stub)
- ⏸️ ML.2–ML.3 implementation (pipeline, aggregation)
- ⏸️ ML.1 pruning strategies

**Patch Tools Development**:
- Adapt scripts in `scripts/` for patch tools on machine
- Use MSYS2 bash for all operations (no PowerShell, no cmd, no .sh scripts for git)
- Format: `cargo fmt --all` before git operations
- Git workflow: external MSYS2 bash terminal (see `git-push.md`)

### Remember

- ✅ **PRIMARY concept**: `docs/concept/poolAI_concept_root.txt` (USE FIRST)
- ✅ **Terminal**: MSYS2 bash (`C:\msys64\usr\bin\bash.exe`), NOT PowerShell, NOT cmd
- ✅ **Helper file**: `file_list.csv` for file navigation
- ✅ **Always sync**: Concept files ↔ Status documents ↔ README
- ✅ **Clean structure**: All docs in `docs/`, scripts in `scripts/`
- ✅ **Current version**: v0.2.2 (check `Cargo.toml` and `src/version.rs`)
