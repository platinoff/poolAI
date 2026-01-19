# Pull Request Command

Create a pull request for the current changes.

1. Check current branch with `git branch --show-current`
2. Look at staged and unstaged changes with `git diff` and `git diff --cached`
3. Write a clear commit message based on what changed (use Conventional Commits format)
4. Commit changes with `git commit -m "<message>"`
5. Push to current branch with `git push`
6. Use `gh pr create` to open a pull request with:
   - Title: Based on commit message
   - Description: Summary of changes, reference related issues
   - Labels: Based on change type (if applicable)
7. Return the PR URL when done

If there are uncommitted changes, ask user if they want to commit them first.
