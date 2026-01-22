# Pre-Push Hook - Automatic Code Formatting

## 📝 Overview

The pre-push git hook automatically runs `cargo fmt --all --check` before every `git push` to ensure all code is properly formatted.

## 🔧 Location

- **Hook file**: `.git/hooks/pre-push`
- **Script type**: Bash script (works with MSYS2 bash)

## ⚙️ How It Works

1. **Before push**: Hook automatically runs `cargo fmt --all --check`
2. **If formatting is OK**: Push proceeds normally
3. **If formatting fails**:
   - Hook auto-formats code with `cargo fmt --all`
   - Exits with error code
   - Requires you to commit formatted changes before pushing

## 🚀 Usage

### Normal Push (Formatted Code)
```bash
# In MSYS2 bash terminal
git push
# Output:
# 🔍 Running pre-push checks...
# 📝 Running cargo fmt --all...
# ✅ Code is properly formatted!
# ✅ Pre-push checks passed!
```

### Push with Formatting Issues
```bash
# In MSYS2 bash terminal
git push
# Output:
# 🔍 Running pre-push checks...
# 📝 Running cargo fmt --all...
# ❌ Code formatting check failed!
# 
# Running cargo fmt --all to fix formatting...
# 
# ⚠️  Code has been auto-formatted. Please review changes and commit them:
#    git add -A
#    git commit -m 'style: auto-format code'
#    git push
```

### Bypass Hook (Not Recommended)
```bash
# Only use if absolutely necessary
git push --no-verify
```

## 📋 Manual Formatting

You can manually format code before pushing:

```bash
# In MSYS2 bash terminal
cargo fmt --all
```

## 🔍 Troubleshooting

### Hook Not Running
- Check if file exists: `ls -la .git/hooks/pre-push`
- Ensure file has execute permissions (on Linux/Mac)
- On Windows, git hooks should work without chmod

### Cargo Not Found
- Hook will skip format check if `cargo` is not in PATH
- Ensure MSYS2 bash has cargo in PATH: `export PATH="$HOME/.cargo/bin:$PATH"`

### Formatting Fails
- Review auto-formatted changes: `git diff`
- Commit formatted changes: `git add -A && git commit -m 'style: auto-format code'`
- Push again: `git push`

## 📚 Related Documentation

- **Git Workflow**: `.cursor/rules/git-workflow.md`
- **Rust Architect Rules**: `.cursor/rules/rust-architect.md`
- **MSYS2 Setup**: `docs/troubleshooting/QUICK_FIX_MSYS2.md`

## ✅ Benefits

- ✅ Ensures consistent code formatting across all commits
- ✅ Prevents unformatted code from being pushed
- ✅ Automatic formatting saves time
- ✅ Works seamlessly with MSYS2 bash environment
