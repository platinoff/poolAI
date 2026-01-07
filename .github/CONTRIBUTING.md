# 🤝 Contributing to PoolAI

Thank you for your interest in contributing to PoolAI! This document provides guidelines and instructions for contributing.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Documentation](#documentation)

## 📜 Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code.

## 🚀 Getting Started

### Prerequisites

- **Rust**: 1.70+ (Recommended: 1.83+)
- **Cargo**: Included with Rust
- **Git**: For version control

### Setup

1. **Fork the repository**
   ```bash
   git clone https://github.com/your-username/poolai.git
   cd poolai
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

## 🔄 Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-description
```

### 2. Make Changes

- Follow coding standards (see below)
- Write tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test

# Check compilation
cargo check
```

### 4. Commit Your Changes

Follow [Conventional Commits](../docs/GIT_COMMIT_GUIDELINES.md):

```bash
git commit -m "feat(module): add new feature"
```

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## 📝 Coding Standards

### Rust Style

- Follow [Rust Book](https://doc.rust-lang.org/book/) best practices
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow project's module structure

### Code Organization

- **Documentation**: All `.md` files in `docs/`
- **Scripts**: All `.sh` files in `scripts/`
- **Code**: Follow existing module structure in `src/`

### Error Handling

- Use `Result<T, AppError>` for recoverable errors
- Use `?` operator for error propagation
- Define error types in `core::error`

### Testing

- Write unit tests in `#[cfg(test)]` modules
- Write integration tests in `tests/` directory
- Aim for high test coverage

## 📋 Commit Guidelines

We use [Conventional Commits](../docs/GIT_COMMIT_GUIDELINES.md):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Code style
- `refactor` - Refactoring
- `test` - Tests
- `chore` - Other changes

**Examples**:
- ✅ `feat(vm): add network isolation`
- ✅ `fix(ui): correct modal focus trap`
- ✅ `docs: update project structure`

## 🔍 Pull Request Process

1. **Update Documentation**
   - Update README if needed
   - Add/update docs in `docs/` if applicable
   - Update CHANGELOG.md

2. **Ensure Tests Pass**
   - All existing tests pass
   - New tests added for new features
   - Integration tests pass

3. **Code Review**
   - Address review comments
   - Keep PR focused and atomic
   - Respond to feedback promptly

4. **Merge**
   - Squash and merge (preferred)
   - Or merge commit (for complex PRs)

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific test
cargo test test_name
```

### Writing Tests

- Use descriptive test names
- Test both success and failure cases
- Mock external dependencies
- Use property-based testing where applicable

## 📚 Documentation

### Code Documentation

- Use `///` for public API documentation
- Use `//!` for module-level documentation
- Include code examples in docs
- Generate docs with `cargo doc --open`

### Project Documentation

- All documentation in `docs/` directory
- Follow structure in `docs/STRUCTURE.md`
- Update relevant docs when making changes

## 🐛 Reporting Bugs

Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md).

Include:
- Clear description
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Relevant logs

## 💡 Suggesting Features

Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md).

Include:
- Clear description
- Problem statement
- Proposed solution
- Alternatives considered

## ❓ Questions?

- Open an issue for questions
- Check existing documentation in `docs/`
- Review [Rust Book](https://doc.rust-lang.org/book/)

---

Thank you for contributing to PoolAI! 🎉

