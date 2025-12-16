#!/bin/bash
# Install GCC for MSYS2 UCRT64
# Run: bash install_gcc.sh

echo "🔧 Installing GCC for MSYS2 UCRT64..."

# Check if running in MSYS2
if [ -z "$MSYSTEM" ]; then
    echo "❌ ERROR: This script must be run in MSYS2 UCRT64 terminal"
    echo "💡 Open MSYS2 UCRT64 terminal and run: bash install_gcc.sh"
    exit 1
fi

echo "📋 Current environment: $MSYSTEM"
echo ""

# Update package database
echo "📦 Updating package database..."
pacman -Sy

# Install GCC toolchain
echo ""
echo "📦 Installing GCC toolchain (this may take a few minutes)..."
pacman -S --needed --noconfirm base-devel mingw-w64-ucrt-x86_64-toolchain

# Verify installation
echo ""
echo "📋 Verifying GCC installation..."
if command -v gcc &> /dev/null; then
    echo "✅ GCC installed: $(gcc --version | head -n1)"
    GCC_PATH=$(which gcc)
    echo "   Location: $GCC_PATH"
else
    echo "❌ GCC not found after installation"
    exit 1
fi

# Add to PATH permanently
echo ""
echo "📝 Adding GCC to PATH..."
BASHRC="$HOME/.bashrc"
if ! grep -q "ucrt64/bin" "$BASHRC" 2>/dev/null; then
    echo "" >> "$BASHRC"
    echo "# GCC/MinGW-w64 PATH for MSYS2 UCRT64" >> "$BASHRC"
    echo 'export PATH="/c/msys64/ucrt64/bin:$PATH"' >> "$BASHRC"
    echo "✅ Added GCC PATH to ~/.bashrc"
else
    echo "ℹ️  GCC PATH already in ~/.bashrc"
fi

# Set CC environment variables
if ! grep -q "CC=" "$BASHRC" 2>/dev/null; then
    echo 'export CC="gcc"' >> "$BASHRC"
    echo 'export CC_x86_64_pc_windows_gnu="gcc"' >> "$BASHRC"
    echo "✅ Added CC environment variables"
else
    echo "ℹ️  CC variables already set"
fi

# Add to current session
export PATH="/c/msys64/ucrt64/bin:$PATH"
export CC="gcc"
export CC_x86_64_pc_windows_gnu="gcc"

echo ""
echo "✅ GCC installation complete!"
echo ""
echo "💡 To apply changes in current terminal, run:"
echo "   source ~/.bashrc"
echo ""
echo "💡 Or restart MSYS2 UCRT64 terminal"
echo ""
echo "🚀 Now you can compile with ring/jsonwebtoken:"
echo "   cd /s/rust/poolAI"
echo "   cargo build"

