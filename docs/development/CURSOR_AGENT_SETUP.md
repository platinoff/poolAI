# Cursor Agent Setup

**Date**: 2026-01-18  
**Status**: ✅ Configured

## Overview

This document describes the Cursor AI agent configuration for the PoolAI project, following best practices from [Cursor's agent guide](https://cursor.com/blog/agent-best-practices).

## Structure

```
.cursor/
├── README.md              # Agent configuration overview
├── rules/                 # Static context for every conversation
│   ├── rust.md           # Rust coding standards
│   └── project-structure.md  # Project organization rules
├── commands/             # Reusable workflows
│   ├── check.md          # Run comprehensive checks
│   ├── test.md           # Run tests
│   ├── review.md         # Review code changes
│   ├── fix-issue.md      # Fix GitHub issues
│   └── pr.md             # Create pull requests
└── plans/                # Saved implementation plans
```

## Rules

Rules provide persistent instructions that shape how the agent works with our codebase.

### `rust.md`
- Rust coding standards and patterns
- Commands to run (`cargo fmt`, `cargo clippy`, `cargo test`)
- Error handling patterns
- Async patterns
- Testing guidelines

### `project-structure.md`
- Directory organization
- File naming conventions
- Module organization
- Documentation structure

## Commands

Commands are reusable workflows that can be invoked with `/` prefix.

### `/check`
Run comprehensive code checks:
- `cargo fmt` - Format code
- `cargo clippy` - Lint code
- `cargo check` - Verify compilation
- `cargo test` - Run tests

### `/test`
Run tests for the project. Can target specific test files.

### `/review`
Review code changes and check for:
- Unused imports
- Missing error handling
- Missing documentation
- Performance issues

### `/fix-issue <number>`
Fix a GitHub issue:
1. Fetch issue details
2. Find relevant code
3. Implement fix
4. Add tests
5. Create PR

### `/pr`
Create a pull request:
1. Check changes
2. Write commit message
3. Commit and push
4. Open PR

## Usage Tips

1. **Use Plan Mode** (Shift+Tab) for complex features
2. **Let the agent find context** - Don't manually tag every file
3. **Start new conversations** when moving to different tasks
4. **Use rules sparingly** - Only add when agent makes repeated mistakes
5. **Review carefully** - AI-generated code needs review

## Best Practices

### Planning
- Use Plan Mode for features requiring multiple files
- Save plans to `.cursor/plans/` for team documentation
- Refine plans before implementation

### Context Management
- Don't manually tag every file - let the agent search
- Use `@Branch` to give context about current work
- Start new conversations for different tasks

### Code Review
- Watch the agent work in diff view
- Use Agent Review after generation
- Run `/review` command for comprehensive checks

## Integration with Project

The agent configuration integrates with our existing project structure:

- **Conventional Commits**: Rules enforce commit message format
- **Rust Standards**: Rules reference our coding patterns
- **Project Structure**: Rules enforce documentation organization
- **Testing**: Commands include test execution

## Updates

When updating rules or commands:
1. Edit files in `.cursor/` directory
2. Test with agent to ensure they work
3. Commit changes to git for team sharing
4. Update this document if structure changes

## References

- [Cursor Agent Best Practices](https://cursor.com/blog/agent-best-practices)
- `.cursorrules` - Main project rules
- `.cursor/README.md` - Detailed agent configuration
