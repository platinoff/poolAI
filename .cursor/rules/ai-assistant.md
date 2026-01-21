# AI Assistant Rules

## 📚 Quick Reference - Key Documents for AI Assistant

### ⚠️ CRITICAL: Always check these documents first when answering questions

**Main active documents** (ALWAYS reference these):
- **Current Project Status**: `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` (most up-to-date)
- **Stable State**: `docs/status/STABLE_STATE_SUMMARY.md`
- **Development Plans**: `docs/development/NEXT_STEPS_PLAN.md` or `docs/development/NEXT_STEPS_2026-01-19.md`
- **Main Concept Document**: `docs/concept/poolAI_concept_root.txt` (PRIMARY concept file - USE THIS FIRST)
- **Alternative Concept**: `docs/concept/poolAI_concept.txt` (if root not available)
- **Project README**: `README.md` (in root)
- **Documentation Index**: `docs/README.md`

**When answering questions about**:
- **Module status/completion**: Check `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` FIRST
- **Development plans**: Check `docs/development/NEXT_STEPS_PLAN.md` or latest `NEXT_STEPS_*.md`
- **Project concept**: Check `docs/concept/poolAI_concept_root.txt` FIRST (PRIMARY concept document)
- **Architecture**: Check `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` and `docs/concept/poolAI_concept_root.txt`
- **Current progress**: Check `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md`
- **Next steps**: Check `docs/development/NEXT_STEPS_2026-01-19.md`

### Rules for AI Assistant
1. **ALWAYS read relevant docs** from `docs/` BEFORE answering questions about:
   - Module completion status
   - Development plans
   - Project architecture
   - Current state of features
   - Next development steps

2. **Use exact file paths** when referencing:
   - `docs/concept/poolAI_concept_root.txt` (PRIMARY - not just "concept file")
   - `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` (for current status)
   - `docs/development/NEXT_STEPS_2026-01-19.md` (for latest plans)
   - `README.md` (in root - project overview)

3. **Sync information** - If user asks to update concept/plans:
   - Update `docs/concept/poolAI_concept_root.txt` FIRST (primary working document)
   - Sync with `docs/status/PROJECT_STATUS_REPORT_2026-01-19.md` (ensure status matches)
   - Sync with `README.md` if needed (project overview)
   - Ensure consistency across all documents

**Remember**: Rust Architect wants CLEAN structure - all docs in `docs/`!
