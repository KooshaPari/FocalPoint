---
title: Contribution Guide
description: How to contribute code, documentation, and connectors to FocalPoint.
---

# Contribution Guide

FocalPoint welcomes contributions! Here's how to get involved.

## Before You Start

1. **Check existing issues**: Search [GitHub Issues](https://github.com/KooshaPari/FocalPoint/issues) for related work
2. **Discuss large changes**: Open an issue or discussion first to align on approach
3. **Sign the CLA**: First-time contributors sign a Contributor License Agreement (CLA)

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- iOS 17 SDK (for iOS testing)
- `cargo`, `rustfmt`, `clippy`

### Clone & Build

```bash
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint

cargo build --release
cargo test --workspace
cargo clippy -- -D warnings
cargo fmt --check
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feat/your-feature
# or
git checkout -b fix/your-bugfix
```

### 2. Make Your Changes

- Write code following Rust style guidelines
- Add tests for new functionality
- Update documentation

### 3. Run Quality Checks

```bash
cargo test --workspace
cargo clippy -- -D warnings
cargo fmt
cargo doc --no-deps --open
```

### 4. Commit with Clear Messages

```bash
git commit -m "feat: add Canvas assignment notifications

- Emit canvas.assignment.due_soon event
- Support 24h, 1h, and overdue thresholds
- Add integration test for Canvas sync

Closes #123"
```

### 5. Push and Open a PR

```bash
git push origin feat/your-feature
```

Then open a pull request on GitHub with:

- Clear title and description
- Link to related issue (if any)
- Test instructions
- Screenshots (for UI changes)

## Code Review Process

1. **Automated checks** run on every PR:
   - `cargo test --workspace`
   - `cargo clippy`
   - `cargo fmt --check`

2. **Maintainers review** for:
   - Correctness and design
   - Test coverage
   - Documentation
   - Accessibility

3. **Approval and merge** once feedback is addressed

## Contribution Types

### Code

- Bug fixes
- New features
- Refactoring
- Performance improvements

### Documentation

- Usage guides
- API documentation
- Examples
- Architecture notes

### Connectors

- New integrations (Canvas, GitHub, etc.)
- Connector bug fixes
- Connector documentation

### Rule Packs

- Curated rule templates
- Student/developer/wellness packs
- Community-contributed rules

## Testing Requirements

All PRs must include tests:

- **New features**: >=1 unit test + >=1 integration test
- **Bug fixes**: >=1 regression test
- **Refactoring**: Maintain existing test coverage
- **Documentation**: No tests required; lint checks only

Minimum coverage: **70%** of changed lines.

## License

By contributing, you agree that your contributions are licensed under **MIT OR Apache-2.0**.

## Code of Conduct

Please review and follow our [Code of Conduct](./coc).

## Questions?

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Email**: hello@focalpoint.local

Thank you for contributing! 🎉
