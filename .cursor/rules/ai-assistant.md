# AI Assistant Rules

## 🧭 Подальша робота з чатом

При **«налаштуйся для роботи з чатом»** або старті нової сесії — **спочатку читай `chat-context.md`**. Там зведено: проект, ключові документи, термінал (MSYS2 only), git (без .sh), патчі, поточний стан і наступні кроки.

---

## 📚 Quick Reference - Key Documents for AI Assistant

### ⚠️ CRITICAL: Always check these documents first when answering questions

**Main active documents** (ALWAYS reference these):
- **Chat context** (session start / «налаштуйся»): `.cursor/rules/chat-context.md`
- **PRIMARY Concept Document**: `docs/concept/poolAI_concept_root.txt` (USE THIS FIRST - most comprehensive)
- **Alternative Concept**: `docs/concept/poolAI_concept.txt` (Ukrainian, detailed dev environment)
- **Current Project Status**: `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` (most up-to-date)
- **Stable State**: `docs/status/STABLE_STATE_SUMMARY.md`
- **Development Plans**: `docs/development/NEXT_STEPS_PLAN.md` or `docs/development/NEXT_STEPS_2026-01-19.md`
- **Cursor & next steps verification**: `docs/CURSOR_AND_NEXT_STEPS_VERIFICATION_2026-03-04.md` (Cursor settings, doc cleanup list, next steps)
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

4. **Terminal** (CRITICAL):
   - ✅ ALWAYS use MSYS2 bash: `C:\msys64\usr\bin\bash.exe`
   - ❌ NEVER use PowerShell or cmd for dev or git
   - ✅ Cargo + git only in MSYS2 bash. **Patches**: `rust-toolchain.toml`, `.cursor`, `.vscode`, `scripts/`

5. **File Navigation**:
   - Use `file_list.csv` to find files by name or path
   - Check `docs/concept/poolAI_concept_root.txt` for file structure
   - All documentation in `docs/` directory
   - All scripts in `scripts/` (bash only; no PS, no cmd)

**Remember**: Rust Architect wants CLEAN structure - all docs in `docs/`!

---

**Session start**: See `chat-context.md` for compact context (project, docs, terminal, git, next steps).
