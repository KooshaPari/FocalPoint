# pheno-domain — Agent Instructions

## Project

Canonical domain primitives for the `pheno-*` fleet.
EntityId, Timestamp, Slug, Email, Money — newtype wrappers with
validation, serde, and Display so they can cross API boundaries,
persist to JSON, and parse from CLI arguments or config files without
ceremony.

## Stack

- Language: Rust (edition 2021, rust-version 1.75)
- Build: Cargo
- License: MIT OR Apache-2.0
- Dependencies: serde, thiserror, uuid, chrono, regex, rust_decimal

## Build

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

## Test

```bash
cargo test --all-features
cargo test --doc
```

## Style

- `cargo fmt` nightly is authoritative.
- Every public type must have a doc comment and a `#![warn(missing_docs)]` guard.
- Newtype constructors should validate eagerly; use `TryFrom` for fallible paths.
- `Display` and `Debug` must be hand-implemented to expose the wrapped value.
- `FromStr` should return `DomainError` with a descriptive message.
- Keep `serde` impls transparent; the inner type serializes naturally.

## PR Conventions

- Branch naming: `feat/`, `fix/`, `chore/`, `docs/`, `refactor/`.
- One logical change per PR (e.g. one new primitive per PR).
- All PRs must pass `clippy`, `fmt`, and `test`.
- Update `CHANGELOG.md` under `[Unreleased]`.
- Squash-merge with a conventional commit message.

## Do-Not-Touch

- `Cargo.lock` — managed by Cargo; never hand-edit.
- `target/` — ephemeral build artifacts.
- Validation regexes in `Slug` and `Email` — changing them is a breaking change.
- `Money` rounding policy (2 dp) — changing this changes monetary semantics.

## Scope

This crate holds only universal domain primitives. Do not add
application-specific types (e.g. `OrderId`, `UserName`) here; create
a downstream crate that depends on `pheno-domain` and defines those.
