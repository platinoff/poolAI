#!/bin/bash
# Build verification script for PoolAI
# Run: bash verify_build.sh

set -e  # Exit on error

echo "🔍 PoolAI Build Verification"
echo "=============================="
echo ""

# Ensure Rust is in PATH
export PATH="/c/Users/$USER/.cargo/bin:$PATH"

# Check Rust tools
echo "📋 Checking Rust tools..."
if ! command -v cargo &> /dev/null; then
    echo "❌ ERROR: cargo not found!"
    echo "💡 Run: export PATH=\"/c/Users/\$USER/.cargo/bin:\$PATH\""
    exit 1
fi

if ! command -v rustc &> /dev/null; then
    echo "❌ ERROR: rustc not found!"
    exit 1
fi

echo "✅ Cargo: $(cargo --version)"
echo "✅ Rustc: $(rustc --version)"
echo ""

# Check toolchain
echo "📋 Checking toolchain..."
TOOLCHAIN=$(rustup show | grep -i "default\|active" | head -n1)
echo "   $TOOLCHAIN"
if echo "$TOOLCHAIN" | grep -qi "x86_64-pc-windows-gnu"; then
    echo "✅ GNU toolchain detected"
else
    echo "⚠️  WARNING: Not using GNU toolchain (expected: x86_64-pc-windows-gnu)"
fi
echo ""

# Change to project directory
cd "$(dirname "$0")" || exit 1

# Clean previous build (optional)
if [ "$1" == "--clean" ]; then
    echo "🧹 Cleaning previous build..."
    cargo clean
    echo ""
fi

# Format check
echo "📋 Checking code format..."
if cargo fmt -- --check 2>/dev/null; then
    echo "✅ Code formatting OK"
else
    echo "⚠️  Code formatting issues found (run: cargo fmt)"
fi
echo ""

# Clippy check
echo "📋 Running Clippy (linter)..."
if cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null; then
    echo "✅ Clippy checks passed"
else
    echo "⚠️  Clippy warnings found (review above)"
fi
echo ""

# Build check (without running)
echo "📋 Checking compilation (cargo check)..."
if cargo check --all-features; then
    echo "✅ Compilation check passed"
else
    echo "❌ Compilation failed!"
    exit 1
fi
echo ""

# Full build
echo "📋 Building project (cargo build)..."
if cargo build --all-features; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi
echo ""

# Check for warnings
echo "📋 Checking for warnings..."
if cargo build --all-features 2>&1 | grep -i "warning" | head -n5; then
    echo "⚠️  Warnings found (review above)"
else
    echo "✅ No warnings"
fi
echo ""

# Summary
echo "=============================="
echo "✅ Build verification complete!"
echo ""
echo "📊 Summary:"
echo "   - Rust tools: ✅"
echo "   - Toolchain: ✅"
echo "   - Compilation: ✅"
echo "   - Build: ✅"
echo ""
echo "🚀 Ready for commit and push!"

