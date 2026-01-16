# Terminal Setup - MSYS2 UCRT64 (Cursor/VS Code)
## Updated practices for Windows development

**Last updated**: 2026-01-16  
**Scope**: Cursor/VS Code integrated terminal on Windows

---

## ✅ Goal
Open MSYS2 UCRT64 automatically **in the project folder** and keep a consistent build toolchain for Rust GNU.

---

## ✅ Required settings

File: `.vscode/settings.json`

- **Default shell**: `MSYS2 UCRT64`
- **Project working directory**: `${workspaceFolder}/poolAI`
- **Environment**: `MSYSTEM=UCRT64`, `CHERE_INVOKING=1`

These settings ensure:
- MSYS2 opens in the project directory (not `~`)
- UCRT64 toolchain is selected
- PATH includes MSYS2 + Cargo

---

## ✅ Troubleshooting

### MSYS2 opens in home folder
- Verify `CHERE_INVOKING=1` is set in profile or `terminal.integrated.env.windows`.
- Verify `terminal.integrated.cwd` points to the **project folder**.

### MSYS2 profile ignored
- Check `terminal.integrated.defaultProfile.windows` is set to `MSYS2 UCRT64`.
- Confirm MSYS2 is installed at `C:\msys64`.

### Rust not found in MSYS2
```bash
echo 'export PATH="/c/Users/$USER/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

## ✅ Recommended verification
```bash
which rustc
rustc --version
cargo --version
```

