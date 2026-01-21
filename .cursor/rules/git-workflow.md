# Git Workflow Rules

## 🔄 Git Workflow - Conventional Commits & Push Order

### ⚠️ CRITICAL: Git Push Order - IMPORTANT for AI Assistant

**Problem**: AI assistant cannot read/modify files that are staged or have uncommitted changes when git marks them.

**Solution**: Always follow this order when user requests git operations:

### Git Operation Order (MANDATORY)

1. **Check git status FIRST**:
   ```bash
   git status --short
   ```
   - If files are modified but NOT staged: Read/modify files BEFORE `git add`
   - If files are already staged: Commit first, then you can read them after

2. **Read/Modify files BEFORE staging**:
   - If files need to be read or modified: Do it BEFORE `git add`
   - Once files are staged with `git add`, AI may have issues accessing them

3. **Stage files** (only after reading/modifying if needed):
   ```bash
   git add <file>
   # or
   git add -A  # (use carefully, may take long if many files)
   ```

4. **Commit changes**:
   ```bash
   git commit -m "type(scope): subject" -m "body"
   ```

5. **Push to remote**:
   ```bash
   git push
   ```
   - **If push fails due to MSYS2**: Use PowerShell or remove MSYS2 from PATH

### Rules for AI Assistant
- **NEVER** do `git add` on files you need to read/modify
- **ALWAYS** check `git status` before attempting to read/modify files
- **If files are already staged**: Complete commit first, then files become readable again
- **Use PowerShell for git operations** if MSYS2 causes authentication issues

### Commit Message Format

**ALWAYS use Conventional Commits format**:
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style (formatting, no logic change)
- `refactor` - Code refactoring
- `perf` - Performance improvements
- `test` - Adding/updating tests
- `build` - Build system changes
- `ci` - CI/CD changes
- `chore` - Other changes (not code)
- `revert` - Revert previous commit

### Scope Examples
- `vm` - VM Module
- `ui` - UI Module
- `raid` - RAID Module
- `network` - Network Module
- `docs` - Documentation
- `scripts` - Scripts
- `concept` - Concept documents
- (no scope for general changes)

### Examples
- ✅ `feat(vm): add network isolation`
- ✅ `fix(ui): correct modal focus trap`
- ✅ `docs: update project structure`
- ✅ `docs(concept): update concept document in Ukrainian`
- ✅ `test(vm): add isolation integration tests`
- ❌ `Update README` (no type)
- ❌ `docs: Updated the README` (past tense, too vague)

### Rules
- Subject: max 50 chars, lowercase, imperative mood
- Body: optional, explains "what" and "why", max 72 chars per line
- Footer: breaking changes, issue references
- One logical change per commit
- Include tests for new features/fixes
- Update documentation for new features

### Before Commit Checklist
- [ ] Code compiles (`cargo check`)
- [ ] Tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Linter clean (`cargo clippy`)
- [ ] Documentation updated (if needed)
- [ ] Commit message follows format
- [ ] Changes are atomic and logical
- [ ] Files read/modified BEFORE `git add` (if needed)
