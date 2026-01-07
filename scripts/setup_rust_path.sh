#!/bin/bash
# Quick setup script for Rust/Cargo in MSYS2 UCRT64
# Run this in MSYS2 UCRT64 terminal: bash setup_rust_path.sh

echo "🔧 Setting up Rust/Cargo PATH for MSYS2 UCRT64..."

# Detect Windows username
if [ -n "$USER" ]; then
    WIN_USER="$USER"
elif [ -n "$USERNAME" ]; then
    WIN_USER="$USERNAME"
else
    WIN_USER=$(whoami)
fi

# Windows Rust path
WIN_RUST_PATH="/c/Users/$WIN_USER/.cargo/bin"

# Check if Rust is installed
if [ -f "$WIN_RUST_PATH/cargo.exe" ]; then
    echo "✅ Found Rust installation at: $WIN_RUST_PATH"
else
    echo "❌ Rust not found at: $WIN_RUST_PATH"
    echo "💡 Installing Rust via rustup..."
    
    # Install rustup if not found
    if ! command -v rustup &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

# Add to current session
export PATH="$WIN_RUST_PATH:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"

# Add to ~/.bashrc for permanent setup
BASHRC="$HOME/.bashrc"
RUST_PATH_ADDED=false

# Check if already added
if grep -q "\.cargo/bin" "$BASHRC" 2>/dev/null; then
    echo "ℹ️  Rust PATH already configured in ~/.bashrc"
    RUST_PATH_ADDED=true
else
    echo "" >> "$BASHRC"
    echo "# Rust/Cargo PATH for MSYS2 UCRT64" >> "$BASHRC"
    echo "# Added by setup_rust_path.sh on $(date)" >> "$BASHRC"
    echo "if [ -d \"$WIN_RUST_PATH\" ]; then" >> "$BASHRC"
    echo "    export PATH=\"$WIN_RUST_PATH:\$PATH\"" >> "$BASHRC"
    echo "fi" >> "$BASHRC"
    echo "if [ -d \"\$HOME/.cargo/bin\" ]; then" >> "$BASHRC"
    echo "    export PATH=\"\$HOME/.cargo/bin:\$PATH\"" >> "$BASHRC"
    echo "fi" >> "$BASHRC"
    echo "✅ Added Rust PATH to ~/.bashrc"
    RUST_PATH_ADDED=true
fi

# Set default toolchain if rustup is available
if command -v rustup &> /dev/null; then
    echo ""
    echo "🔧 Setting up GNU toolchain..."
    rustup default stable-x86_64-pc-windows-gnu 2>/dev/null || echo "ℹ️  Toolchain already set or needs manual setup"
fi

# Verify installation
echo ""
echo "📋 Verification:"
echo "=================="

if command -v cargo &> /dev/null; then
    echo "✅ Cargo found: $(cargo --version 2>/dev/null || echo 'version check failed')"
else
    echo "❌ Cargo not found"
    echo "💡 Try: source ~/.bashrc"
fi

if command -v rustc &> /dev/null; then
    echo "✅ Rustc found: $(rustc --version 2>/dev/null || echo 'version check failed')"
else
    echo "❌ Rustc not found"
    echo "💡 Try: source ~/.bashrc"
fi

if command -v rustup &> /dev/null; then
    echo "✅ Rustup found: $(rustup --version 2>/dev/null | head -n1 || echo 'version check failed')"
    echo ""
    echo "📊 Current toolchain:"
    rustup show 2>/dev/null | head -n5 || echo "Run 'rustup show' manually"
else
    echo "❌ Rustup not found"
fi

echo ""
echo "💡 Next steps:"
echo "   1. Reload shell: source ~/.bashrc"
echo "   2. Or restart MSYS2 UCRT64 terminal"
echo "   3. Verify: cargo --version"
echo "   4. Test build: cd /s/rust/poolAI && cargo check"

