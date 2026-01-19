# Cursor Agent Configuration

This directory contains configuration for Cursor's AI coding agent, following best practices from [Cursor's agent guide](https://cursor.com/blog/agent-best-practices).

## Structure

- `rules/` - Static context that applies to every conversation
  - `rust.md` - Rust coding standards and patterns
  - `project-structure.md` - Project organization rules
- `commands/` - Reusable workflows triggered with `/` in agent input
  - `check.md` - Run comprehensive code checks
  - `test.md` - Run tests
  - `review.md` - Review code changes
  - `fix-issue.md` - Fix a GitHub issue
  - `pr.md` - Create a pull request
- `plans/` - Saved implementation plans (created via Plan Mode)
- `hooks/` - Agent hooks for extended workflows
  - `check-tests.ps1` - Verify tests pass before stopping (optional)

## Usage

### Rules
Rules are automatically included in every agent conversation. They provide persistent instructions about:
- Commands to run
- Code patterns to follow
- Pointers to canonical examples

### Commands
Invoke commands with `/` prefix:
- `/check` - Run cargo fmt, clippy, check, and test
- `/test` - Run tests
- `/review` - Review code changes
- `/fix-issue <number>` - Fix a GitHub issue
- `/pr` - Create a pull request

### Plans
Use Plan Mode (Shift+Tab) for complex features. Plans are saved here for:
- Team documentation
- Resuming interrupted work
- Context for future agents

## Best Practices

1. **Start with plans** - Use Plan Mode for complex features
2. **Let the agent find context** - Don't manually tag every file
3. **Start new conversations** - When moving to different tasks
4. **Use rules sparingly** - Only add rules when agent makes repeated mistakes
5. **Review carefully** - AI-generated code needs review

See [Cursor's agent best practices](https://cursor.com/blog/agent-best-practices) for more details.
