# AI Assistant Rules

## 📚 Quick Reference - Key Documents for AI Assistant

### ⚠️ CRITICAL: Always check these documents first when answering questions

**Main active documents** (ALWAYS reference these):
- **PRIMARY Concept Document**: `docs/concept/poolAI_concept_root.txt` (USE THIS FIRST - most comprehensive)
- **Alternative Concept**: `docs/concept/poolAI_concept.txt` (Ukrainian, detailed dev environment)
- **Current Project Status**: `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` (most up-to-date)
- **Stable State**: `docs/status/STABLE_STATE_SUMMARY.md`
- **Development Plans**: `docs/development/NEXT_STEPS_PLAN.md` or `docs/development/NEXT_STEPS_2026-01-19.md`
- **Project README**: `README.md` (in root)
- **Documentation Index**: `docs/README.md`
- **Helper File**: `file_list.csv` (108254+ lines, file inventory for navigation)

**When answering questions about**:
- **Module status/completion**: Check `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` FIRST
- **Development plans**: Check `docs/development/NEXT_STEPS_PLAN.md` or latest `NEXT_STEPS_*.md`
- **Project concept**: Check `docs/concept/poolAI_concept_root.txt` FIRST (PRIMARY concept document)
- **Architecture**: Check `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` and `docs/concept/poolAI_concept_root.txt`
- **Current progress**: Check `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md`
- **Next steps**: Check `docs/development/NEXT_STEPS_2026-01-19.md`

### Rules for AI Assistant (Rust Architect Mode)

1. **ALWAYS read relevant docs** from `docs/` BEFORE answering questions about:
   - Module completion status
   - Development plans
   - Project architecture
   - Current state of features
   - Next development steps

2. **Use exact file paths** when referencing:
   - `docs/concept/poolAI_concept_root.txt` (PRIMARY - USE FIRST, not just "concept file")
   - `docs/concept/poolAI_concept.txt` (Alternative, Ukrainian version)
   - `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` (for current status)
   - `docs/development/NEXT_STEPS_2026-01-19.md` (for latest plans)
   - `file_list.csv` (for file navigation and structure analysis)
   - `README.md` (in root - project overview)

3. **Sync information** - If user asks to update concept/plans:
   - Update `docs/concept/poolAI_concept_root.txt` FIRST (PRIMARY working document)
   - Sync with `docs/concept/poolAI_concept.txt` if needed (Ukrainian version)
   - Sync with `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` (ensure status matches)
   - Sync with `README.md` if needed (project overview)
   - Ensure version numbers match: `Cargo.toml` → `src/version.rs` → concept files
   - Ensure consistency across all documents

4. **Terminal Commands** (CRITICAL):
   - ✅ ALWAYS use MSYS2 bash: `C:\msys64\usr\bin\bash.exe`
   - ❌ NEVER use PowerShell for development tasks
   - ✅ All cargo commands in MSYS2 bash
   - ✅ Git operations in MSYS2 bash (preferred)

5. **File Navigation**:
   - Use `file_list.csv` to find files by name or path
   - Check `docs/concept/poolAI_concept_root.txt` for file structure
   - All documentation in `docs/` directory
   - All scripts in `scripts/` directory (bash scripts)

**Remember**: Rust Architect wants CLEAN structure - all docs in `docs/`!
