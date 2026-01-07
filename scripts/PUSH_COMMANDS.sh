#!/bin/bash
# Git push commands for fix/unsafe-global-state-and-compilation branch
# Run: bash PUSH_COMMANDS.sh

set -e

echo "🚀 Preparing Git Push"
echo "======================"
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ ERROR: Not in a git repository"
    exit 1
fi

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📋 Current branch: $CURRENT_BRANCH"
echo ""

# Check if there are uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  WARNING: There are uncommitted changes"
    echo ""
    echo "Uncommitted files:"
    git status --short
    echo ""
    read -p "Do you want to continue? (y/n) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Aborted"
        exit 1
    fi
fi

# Create new branch if not already on it
NEW_BRANCH="fix/unsafe-global-stage-and-compilation"
if [ "$CURRENT_BRANCH" != "$NEW_BRANCH" ]; then
    echo "🔀 Creating new branch: $NEW_BRANCH"
    git checkout -b "$NEW_BRANCH"
    echo "✅ Branch created"
else
    echo "✅ Already on branch: $NEW_BRANCH"
fi

echo ""
echo "📊 Staging files..."
git add -A

echo ""
echo "📋 Staged files:"
git status --short

echo ""
echo "📝 Creating commit..."
if [ -f "COMMIT_MESSAGE.md" ]; then
    git commit -F COMMIT_MESSAGE.md
else
    git commit -m "fix: replace unsafe global state with OnceLock and fix compilation issues"
fi

echo ""
echo "✅ Commit created"
echo ""
echo "🚀 Ready to push!"
echo ""
echo "To push, run:"
echo "  git push -u origin $NEW_BRANCH"
echo ""
read -p "Do you want to push now? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "📤 Pushing to remote..."
    git push -u origin "$NEW_BRANCH"
    echo ""
    echo "✅ Push completed!"
    echo ""
    echo "🔗 Branch URL:"
    echo "  https://github.com/YOUR_REPO/compare/$NEW_BRANCH"
else
    echo "ℹ️  Skipped push. Run manually:"
    echo "  git push -u origin $NEW_BRANCH"
fi

