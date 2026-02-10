# Contributing to monogirl

Thank you for your interest in contributing.

## Development Setup

1. Fork the repository
2. Clone your fork
3. Create a feature branch from `main`
4. Make your changes
5. Run tests: `cargo test` and `cd sdk && npm test`
6. Submit a pull request

## Code Standards

- Rust code must pass `cargo clippy` without warnings
- TypeScript code must pass `npm run lint`
- All public functions require documentation comments
- New features require corresponding tests

## Commit Messages

Write descriptive commit messages that explain the change:
- "add account set validation for CPE bundles"
- "fix parallel batch scheduling edge case"
- "update SDK client with bundle submission"

## Pull Request Process

1. Update documentation if needed
2. Add tests for new functionality
3. Ensure CI passes
4. Request review from a maintainer

