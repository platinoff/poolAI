# MSYS2 UCRT64 - Rust Setup Guide

## 🔧 Problem: `cargo: command not found` in MSYS2 UCRT64

If you see `bash: cargo: command not found` in MSYS2 UCRT64 terminal, Rust is not in PATH.

---

## ✅ Solution: Add Rust to MSYS2 PATH

### Option 1: Install Rust via rustup (Recommended)

1. **Download and install rustup**:
   ```bash
   # In MSYS2 UCRT64 terminal
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Add Rust to PATH**:
   ```bash
   # Add to ~/.bashrc or ~/.bash_profile
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Install GNU toolchain**:
   ```bash
   rustup default stable-x86_64-pc-windows-gnu
   ```

4. **Verify installation**:
   ```bash
   cargo --version
   rustc --version
   ```

### Option 2: Use Windows Rust Installation

If Rust is already installed on Windows:

1. **Find Rust installation path**:
   ```bash
   # Usually in: C:\Users\<username>\.cargo\bin
   # Or: C:\Program Files\Rust stable MSVC 1.xx\bin
   ```

2. **Add to MSYS2 PATH**:
   ```bash
   # Add to ~/.bashrc
   echo 'export PATH="/c/Users/$USER/.cargo/bin:$PATH"' >> ~/.bashrc
   # Or for MSVC installation:
   # echo 'export PATH="/c/Program Files/Rust stable MSVC 1.xx/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Verify**:
   ```bash
   cargo --version
   ```

---

## 🔍 Troubleshooting

### Check if Rust is installed:
```bash
# Check Windows installation
ls ~/.cargo/bin/cargo.exe 2>/dev/null || echo "Rust not found in ~/.cargo/bin"
ls /c/Users/$USER/.cargo/bin/cargo.exe 2>/dev/null || echo "Rust not found in Windows user directory"
```

### Check PATH:
```bash
echo $PATH | tr ':' '\n' | grep -i rust
echo $PATH | tr ':' '\n' | grep -i cargo
```

### Manual PATH addition (temporary):
```bash
export PATH="$HOME/.cargo/bin:$PATH"
# Or Windows path:
export PATH="/c/Users/$USER/.cargo/bin:$PATH"
```

---

## 📝 Permanent Setup

### Add to ~/.bashrc:
```bash
# Rust/Cargo PATH
if [ -d "$HOME/.cargo/bin" ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
elif [ -d "/c/Users/$USER/.cargo/bin" ]; then
    export PATH="/c/Users/$USER/.cargo/bin:$PATH"
fi

# Rust toolchain for MSYS2 UCRT64
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
```

### Reload shell:
```bash
source ~/.bashrc
```

---

## ✅ Verification Steps

After setup, verify:

```bash
# 1. Check Rust version
rustc --version

# 2. Check Cargo version
cargo --version

# 3. Check active toolchain
rustup show

# 4. Verify GNU toolchain
rustup target list --installed | grep windows-gnu

# 5. Test compilation
cd /s/rust/poolAI
cargo check
```

---

## 🎯 Expected Output

After successful setup:
```
$ cargo --version
cargo 1.75.0 (stable-x86_64-pc-windows-gnu)

$ rustc --version
rustc 1.75.0 (stable-x86_64-pc-windows-gnu)

$ rustup show
Default host: x86_64-pc-windows-gnu
rustup home:  C:\Users\plati\.rustup

installed toolchains
--------------------
stable-x86_64-pc-windows-gnu (active, default)
```

---

## 📚 Additional Resources

- Rustup installation: https://rustup.rs/
- MSYS2 documentation: https://www.msys2.org/
- Rust toolchain guide: https://rust-lang.github.io/rustup/concepts/toolchains.html

---

**Note**: After setting up Rust, restart MSYS2 UCRT64 terminal or run `source ~/.bashrc` to apply changes.

