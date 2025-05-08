# Contributing to Acton ERN

Thank you for your interest in contributing to Acton ERN! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Development Setup

### Prerequisites

- Rust (stable, minimum 1.70.0)
- Cargo (comes with Rust)
- Git

### Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/acton-ern.git
   cd acton-ern
   ```

3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/govcraft/acton-ern.git
   ```

4. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

5. Install development dependencies:
   ```bash
   cargo build
   ```

### Building and Testing

- Build the project:
  ```bash
  cargo build
  ```

- Run tests:
  ```bash
  cargo test
  ```

- Run benchmarks:
  ```bash
  cargo bench
  ```

- Check code formatting:
  ```bash
  cargo fmt --check
  ```

- Run linting:
  ```bash
  cargo clippy -- -D warnings
  ```

## Coding Standards

### Code Style

- Follow the Rust standard style guide.
- Use `cargo fmt` to format your code before submitting.
- Use `cargo clippy` to catch common mistakes and improve your code.

### Commit Messages

- Use clear, descriptive commit messages.
- Follow the conventional commits format:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `style:` for formatting changes
  - `refactor:` for code refactoring
  - `test:` for adding or modifying tests
  - `chore:` for maintenance tasks

### Documentation

- Document all public APIs.
- Include examples in documentation when appropriate.
- Update the README.md if necessary.
- Add entries to CHANGELOG.md for notable changes.

## Pull Request Process

1. **Before submitting a PR**:
   - Ensure your code builds without errors.
   - Run all tests and make sure they pass.
   - Update documentation if needed.
   - Add tests for new functionality.

2. **Creating a PR**:
   - Submit your PR against the `main` branch.
   - Fill out the PR template completely.
   - Reference any related issues.
   - Provide a clear description of the changes.

3. **PR Review Process**:
   - Address review comments promptly.
   - Keep the PR focused on a single issue or feature.
   - Be open to feedback and suggestions.

4. **After PR Approval**:
   - Squash commits if requested.
   - Ensure CI passes before merging.

## Testing Requirements

### Unit Tests

- Write unit tests for all new functionality.
- Aim for high test coverage (at least 80%).
- Tests should be clear and focused on specific functionality.

### Integration Tests

- Add integration tests for features that interact with other components.
- Test edge cases and error conditions.

### Performance Tests

- Add benchmarks for performance-critical code.
- Ensure changes don't negatively impact performance.

### Test Organization

- Place unit tests in the same file as the code they test, using Rust's `#[cfg(test)]` module.
- Place integration tests in the `tests/` directory.
- Place benchmarks in the `benches/` directory.

## Documentation

- Document all public APIs using rustdoc.
- Include examples in documentation.
- Keep the README.md up to date.
- Update CHANGELOG.md for each release.

## Release Process

1. Update version number in Cargo.toml.
2. Update CHANGELOG.md with the new version and changes.
3. Create a git tag for the new version.
4. Push the tag to GitHub.
5. Publish to crates.io:
   ```bash
   cargo publish
   ```

## License

By contributing to Acton ERN, you agree that your contributions will be licensed under the project's MIT OR Apache-2.0 license.