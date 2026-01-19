# Review Command

Review code changes and check for issues.

1. Check staged changes with `git diff --cached`
2. Run `cargo clippy` to check for linting issues
3. Check for common Rust issues:
   - Unused imports
   - Unused variables
   - Missing error handling
   - Missing documentation
   - Performance issues
4. Summarize findings and suggest improvements

Return review summary.
