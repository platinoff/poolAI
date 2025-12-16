#!/bin/bash
# Fix GCC path for MSYS2 UCRT64
# Run: bash fix_gcc.sh

echo "🔧 Fixing GCC path for MSYS2 UCRT64..."

# Add MSYS2 UCRT64 bin to PATH (contains gcc.exe)
export PATH="/c/msys64/ucrt64/bin:$PATH"
export PATH="/c/msys64/usr/bin:$PATH"

# Check if gcc is available
if command -v gcc &> /dev/null; then
    echo "✅ GCC found: $(gcc --version | head -n1)"
    GCC_PATH=$(which gcc)
    echo "   Location: $GCC_PATH"
else
    echo "❌ GCC not found in PATH"
    echo ""
    echo "💡 Installing GCC via MSYS2 pacman..."
    echo "   Run in MSYS2 UCRT64 terminal:"
    echo "   pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain"
    echo ""
    echo "   Or check if GCC exists:"
    echo "   ls /c/msys64/ucrt64/bin/gcc.exe"
    echo "   ls /c/msys64/usr/bin/gcc.exe"
fi

# Set CC environment variable for cargo
export CC="gcc"
export CC_x86_64_pc_windows_gnu="gcc"

echo ""
echo "📋 Environment variables set:"
echo "   CC=$CC"
echo "   CC_x86_64_pc_windows_gnu=$CC_x86_64_pc_windows_gnu"
echo "   PATH includes MSYS2 UCRT64 bin directories"
echo ""
echo "💡 To make permanent, add to ~/.bashrc:"
echo "   export PATH=\"/c/msys64/ucrt64/bin:\$PATH\""
echo "   export CC=\"gcc\""
echo "   export CC_x86_64_pc_windows_gnu=\"gcc\""

