# Scripts Structure Rules

## 🔧 Scripts Structure - STRICT RULES

### ⚠️ CRITICAL: All shell scripts MUST be in `scripts/` directory

**NEVER create `.sh` or `.ps1` files in the root directory**

### Scripts Directory Structure

```
scripts/
├── README.md                    # Scripts documentation
├── fix_cargo_now.sh             # Cargo fix script
├── fix_gcc.sh                   # GCC fix script
├── install_gcc.sh               # GCC installation
├── setup_rust_path.sh           # Rust PATH setup
├── setup_msvc_environment.ps1  # MSVC environment setup
├── setup_rust_environment.ps1  # Automatic Rust environment setup
├── QUICK_FIX_RUST_PATH.sh       # Quick Rust PATH fix
├── verify_build.sh              # Build verification
└── PUSH_COMMANDS.sh             # Git push helper
```

### Rules for Creating New Scripts

1. **Always create in `scripts/` directory**:
   - Build scripts → `scripts/`
   - Setup scripts → `scripts/`
   - Utility scripts → `scripts/`
   - CI/CD scripts → `scripts/` (or `scripts/ci/` for multiple)
   - PowerShell scripts (`.ps1`) → `scripts/`
   - Bash scripts (`.sh`) → `scripts/`

2. **File naming**:
   - Use lowercase with underscores: `setup_rust_path.sh`
   - Use descriptive names: `fix_cargo_now.sh`
   - Use UPPERCASE for important scripts: `PUSH_COMMANDS.sh` (optional)
   - PowerShell scripts: `setup_msvc_environment.ps1`

3. **When user asks to create a script**:
   - Always create in `scripts/`
   - Add description in `scripts/README.md`
   - Make executable: `chmod +x scripts/script_name.sh` (for `.sh` files)

### Script References

- In documentation: `scripts/script_name.sh` or `scripts/script_name.ps1`
- In Rust code comments: `scripts/script_name.sh`
- In CI/CD configs: `scripts/script_name.sh`
- **When listing files**: Use descriptive names, NOT `.ps1` or `.ps` extensions in lists

### ⚠️ File Listing Rules

**When creating file lists or inventories:**
- ❌ **NEVER** use `.ps1` or `.ps` extensions in file lists
- ❌ **NEVER** use PowerShell-specific syntax in documentation lists
- ✅ Use descriptive names: `setup_msvc_environment.ps1` → "MSVC environment setup script"
- ✅ Group by category: "PowerShell scripts", "Bash scripts"
- ✅ Use markdown lists or tables, not PowerShell command output

**Examples:**
- ❌ Bad: `setup_msvc_environment.ps1`, `setup_rust_environment.ps1`
- ✅ Good: "MSVC environment setup script", "Rust environment setup script"
- ✅ Good: "PowerShell scripts: setup_msvc_environment.ps1, setup_rust_environment.ps1"

## 🚫 What NOT to Do with Scripts

- ❌ NEVER create `.sh` or `.ps1` files in root
- ❌ NEVER leave scripts without documentation
- ❌ NEVER create scripts without proper shebang (`#!/bin/bash` for `.sh`)
- ❌ NEVER use `.ps1` or `.ps` extensions in file lists or documentation

## ✅ What to Do with Scripts

- ✅ Always create in `scripts/`
- ✅ Add to `scripts/README.md` documentation
- ✅ Use proper shebang and error handling
- ✅ Make executable with `chmod +x` (for `.sh` files)
- ✅ Use descriptive names in lists, not file extensions

## 📚 Quick Reference

**Scripts location**: `scripts/`
**Documentation**: `scripts/README.md`

**Remember**: Rust Architect wants CLEAN structure - all scripts in `scripts/`, and descriptive names in lists!
