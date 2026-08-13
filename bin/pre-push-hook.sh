#!/usr/bin/env bash
# Canonical pre-push gate: vision canon sync + drift check + cargo fmt.
# Installed into .git/hooks/pre-push via bin/install-pre-push-hook.sh

set -euo pipefail

echo "🔍 Running pre-push checks..."

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

export PATH="${HOME}/.cargo/bin:/ucrt64/bin:/usr/bin:${PATH:-}"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable-x86_64-pc-windows-gnu}"

if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ cargo not found in PATH (use MSYS2 UCRT64 or add ~/.cargo/bin)"
    exit 1
fi

if [ ! -f Cargo.toml ]; then
    echo "⚠️  Cargo.toml not found. Skipping checks."
    exit 0
fi

echo "🔄 Vision sync (poolai-vision-sync)..."
if ! cargo run --quiet --bin poolai-vision-sync; then
    echo "❌ poolai-vision-sync failed"
    exit 1
fi

CANON_CHANGED=0
CANON_PATHS=(
    README.md
    docs/INDEX_2026-03-17.md
    docs/development/README.md
    docs/development/NEXT_SESSION_PROMPT.md
    GSV/docs/vision/manifest.json
    GSV/docs/vision/feed.json
    GSV/docs/vision/extensions.json
    GSV/docs/vision/vision.svg
)

for f in "${CANON_PATHS[@]}"; do
    if [ -f "$f" ] && ! git diff --quiet -- "$f" 2>/dev/null; then
        CANON_CHANGED=1
        echo "  📝 modified by sync: $f"
    fi
done

if [ "$CANON_CHANGED" -eq 1 ]; then
    echo ""
    echo "❌ Vision/canon files were updated. Stage and commit before push:"
    echo "   git add README.md docs/INDEX_2026-03-17.md docs/development/README.md docs/development/NEXT_SESSION_PROMPT.md GSV/docs/vision/"
    echo "   git commit -m 'docs: vision canon sync'"
    echo "   git push"
    exit 1
fi

echo "🔎 Vision drift check..."
if ! cargo run --quiet --bin poolai-vision-sync -- --check; then
    echo "❌ vision drift check failed"
    exit 1
fi

echo "📝 Running cargo fmt --all --check..."
if cargo fmt --all -- --check; then
    echo "✅ Pre-push checks passed!"
    exit 0
fi

echo "❌ Code formatting check failed!"
echo ""
echo "Running cargo fmt --all to fix formatting..."
cargo fmt --all
echo ""
echo "⚠️  Code has been auto-formatted. Please review changes and commit them:"
echo "   git add -A"
echo "   git commit -m 'style: auto-format code'"
echo "   git push"
exit 1
