#!/bin/bash
# Fix dlltool path for MSYS2 terminal
# Run: bash scripts/fix_dlltool_msys2.sh

echo "🔧 Fixing dlltool path for MSYS2..."

# Add MSYS2 UCRT64 bin to PATH (contains gcc.exe)
export PATH="/c/msys64/ucrt64/bin:$PATH"
export PATH="/c/msys64/usr/bin:$PATH"

# Set CC environment variables for cargo
export CC="gcc"
export CC_x86_64_pc_windows_gnu="gcc"
export AR="ar"
export AR_x86_64_pc_windows_gnu="ar"

# Check if tools are available
echo ""
echo "📋 Checking tools..."
if command -v gcc &> /dev/null; then
    echo "✅ GCC found: $(gcc --version | head -n1)"
    echo "   Location: $(which gcc)"
else
    echo "❌ GCC not found"
fi

if command -v dlltool &> /dev/null; then
    echo "✅ dlltool found: $(dlltool --version | head -n1)"
    echo "   Location: $(which dlltool)"
else
    echo "❌ dlltool not found"
    echo ""
    echo "💡 Installing dlltool via MSYS2 pacman..."
    echo "   Run in MSYS2 UCRT64 terminal:"
    echo "   pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain"
fi

if command -v ar &> /dev/null; then
    echo "✅ ar found: $(ar --version | head -n1)"
    echo "   Location: $(which ar)"
else
    echo "❌ ar not found"
fi

echo ""
echo "📋 Environment variables set:"
echo "   CC=$CC"
echo "   CC_x86_64_pc_windows_gnu=$CC_x86_64_pc_windows_gnu"
echo "   AR=$AR"
echo "   AR_x86_64_pc_windows_gnu=$AR_x86_64_pc_windows_gnu"
echo ""
echo "💡 To make permanent, add to ~/.bashrc:"
echo "   export PATH=\"/c/msys64/ucrt64/bin:/c/msys64/usr/bin:\$PATH\""
echo "   export CC=\"gcc\""
echo "   export CC_x86_64_pc_windows_gnu=\"gcc\""
echo "   export AR=\"ar\""
echo "   export AR_x86_64_pc_windows_gnu=\"ar\""
echo ""
echo "✅ Ready to build! Run: cargo build"
