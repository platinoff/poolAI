# Fix Issue Command

Fix a GitHub issue.

1. Fetch issue details using `gh issue view <number>`
2. Find relevant code using codebase search
3. Implement the fix following project patterns
4. Add tests for the fix
5. Run `cargo fmt`, `cargo clippy`, `cargo test`
6. Create a commit with Conventional Commits format
7. Open a pull request with `gh pr create`

Return PR URL when done.
