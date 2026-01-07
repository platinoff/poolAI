#!/bin/bash
# Immediate fix for cargo in MSYS2 UCRT64
# Run: bash fix_cargo_now.sh

echo "🔧 Fixing cargo PATH in MSYS2 UCRT64..."

# Get Windows username
WIN_USER=${USER:-${USERNAME:-$(whoami)}}

# Windows Rust path
WIN_RUST_PATH="/c/Users/$WIN_USER/.cargo/bin"

# Add to current session immediately
export PATH="$WIN_RUST_PATH:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"

echo "✅ Added Rust to PATH for current session"
echo "   Windows path: $WIN_RUST_PATH"
echo "   Home path: $HOME/.cargo/bin"

# Check if cargo is now available
if command -v cargo &> /dev/null; then
    echo ""
    echo "✅ SUCCESS! Cargo is now available:"
    cargo --version
    rustc --version 2>/dev/null || echo "rustc not found"
    echo ""
    echo "💡 To make this permanent, run:"
    echo "   bash setup_rust_path.sh"
    echo "   source ~/.bashrc"
else
    echo ""
    echo "❌ Cargo still not found. Checking installation..."
    echo ""
    echo "Checking Windows installation:"
    ls "$WIN_RUST_PATH/cargo.exe" 2>/dev/null && echo "✅ Found: $WIN_RUST_PATH/cargo.exe" || echo "❌ Not found: $WIN_RUST_PATH/cargo.exe"
    echo ""
    echo "Checking MSYS2 installation:"
    ls "$HOME/.cargo/bin/cargo" 2>/dev/null && echo "✅ Found: $HOME/.cargo/bin/cargo" || echo "❌ Not found: $HOME/.cargo/bin/cargo"
    echo ""
    echo "💡 If Rust is not installed, run:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

