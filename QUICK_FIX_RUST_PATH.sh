#!/bin/bash
# Quick fix script to add Rust to MSYS2 UCRT64 PATH
# Run this in MSYS2 UCRT64 terminal: bash QUICK_FIX_RUST_PATH.sh

echo "🔧 Adding Rust to MSYS2 UCRT64 PATH..."

# Add to current session
export PATH="/c/Users/$USER/.cargo/bin:$PATH"

# Add to ~/.bashrc for permanent setup
if ! grep -q "\.cargo/bin" ~/.bashrc 2>/dev/null; then
    echo "" >> ~/.bashrc
    echo "# Rust/Cargo PATH for MSYS2 UCRT64" >> ~/.bashrc
    echo 'if [ -d "/c/Users/$USER/.cargo/bin" ]; then' >> ~/.bashrc
    echo '    export PATH="/c/Users/$USER/.cargo/bin:$PATH"' >> ~/.bashrc
    echo "fi" >> ~/.bashrc
    echo "✅ Added Rust PATH to ~/.bashrc"
else
    echo "ℹ️  Rust PATH already in ~/.bashrc"
fi

# Verify
echo ""
echo "📋 Verification:"
if command -v cargo &> /dev/null; then
    echo "✅ Cargo found: $(cargo --version)"
else
    echo "❌ Cargo not found. Please check Rust installation."
fi

if command -v rustc &> /dev/null; then
    echo "✅ Rustc found: $(rustc --version)"
else
    echo "❌ Rustc not found. Please check Rust installation."
fi

echo ""
echo "💡 To apply changes in current terminal, run:"
echo "   source ~/.bashrc"
echo ""
echo "💡 Or restart MSYS2 UCRT64 terminal"

