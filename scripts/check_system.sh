#!/bin/bash
# Check system configuration for git push
# Usage: bash scripts/check_system.sh

set +e  # Don't exit on errors

export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
cd "$(dirname "$0")/.."

echo "=== SSH Keys ==="
if ls ~/.ssh/*.pub 2>/dev/null; then
    echo "✅ SSH public keys found:"
    ls -la ~/.ssh/*.pub
    echo ""
    echo "Public key content:"
    cat ~/.ssh/id_ed25519.pub 2>/dev/null || cat ~/.ssh/id_rsa.pub 2>/dev/null || echo "Could not read public key"
else
    echo "❌ No SSH public keys found"
fi

echo ""
echo "=== Git Config ==="
if git config --global --list 2>/dev/null | grep -E "(credential|user|remote)"; then
    echo ""
else
    echo "No relevant git config found"
fi

echo ""
echo "=== Git Remote ==="
git remote -v 2>/dev/null || echo "No remote configured"

echo ""
echo "=== Credentials File ==="
if [ -f ~/.git-credentials ]; then
    echo "✅ ~/.git-credentials file found:"
    cat ~/.git-credentials | sed 's/:[^@]*@/:***@/g'  # Mask password
else
    echo "❌ No ~/.git-credentials file found"
fi

echo ""
echo "=== SSH Test ==="
if ssh -T git@github.com 2>&1 | head -1; then
    echo ""
else
    echo "SSH test failed or not configured"
fi

echo ""
echo "=== Git Status ==="
git status -sb 2>/dev/null || echo "Could not get git status"

echo ""
echo "=== Rust Version ==="
rustc --version 2>/dev/null || echo "Rust not found in PATH"
