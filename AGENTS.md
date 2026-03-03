# AGENTS.md - Coding Agent Guide for bbqio

## Project Overview

`bbqio` is a minimal Rust library crate providing `embedded_io_async` trait wrappers
around `bbqueue` stream producers/consumers. It targets `no_std` / `no_alloc`
embedded environments. The entire library lives in a single file (`src/lib.rs`, ~80 lines).

- **Language:** Rust (edition 2024)
- **Crate type:** Library (`lib`)
- **Environment:** `no_std`, `no_alloc`
- **License:** MIT

## Build / Check / Test Commands

All commands use standard Cargo. There is no Makefile or custom build script.

```sh
# Build (default features: io_v0-7)
cargo build

# Build with specific feature
cargo build --features io_v0-6 --no-default-features
cargo build --features io_v0-7 --no-default-features

# Type-check only (faster than full build)
cargo check
cargo check --all-features

# Run tests (uses built-in Rust test framework)
cargo test

# Run a single test by name
cargo test <test_name>

# Run tests matching a pattern
cargo test <pattern> -- --exact   # exact match
cargo test <pattern>              # substring match

# Run tests for a specific feature combination
cargo test --features io_v0-6 --no-default-features

# Lint with Clippy
cargo clippy
cargo clippy --all-features

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Generate documentation
cargo doc
cargo doc --open  # opens in browser
```

### Important: `no_std` Behavior

The crate uses `#![cfg_attr(not(test), no_std)]`, meaning:
- During normal builds: `no_std` (no standard library)
- During `cargo test`: `std` is available (to support the test harness)

When writing code, do NOT use `std` types or functions outside of `#[cfg(test)]`
blocks. Use `core::` equivalents instead (e.g., `core::cmp::min`, not `std::cmp::min`).

## Features

```toml
[features]
default = ["io_v0-7"]
io_v0-6 = ["dep:embedded-io-async-v0-6"]   # embedded-io-async 0.6.1
io_v0-7 = ["dep:embedded-io-async-v0-7"]   # embedded-io-async 0.7.0
```

The features are **independent and additive**: both can be active simultaneously.
Each feature gates its own module (`v0_6` or `v0_7`), so there is no conflict.

When adding code, ensure it compiles under both feature flags individually and
together. Test with:
```sh
cargo check --features io_v0-6 --no-default-features
cargo check --features io_v0-7 --no-default-features
cargo check --all-features
```

## Code Style Guidelines

### Formatting

- Use default `rustfmt` settings (no `rustfmt.toml` exists).
- Run `cargo fmt` before committing.
- No line length override; follow rustfmt defaults (100 chars).

### Imports

- Feature-gated conditional imports (`#[cfg(feature = "...")]`) go first at the
  top of the file, immediately after module-level doc comments and attributes.
- Group imports logically: feature aliases, then external crate imports, then
  internal/std imports.
- Use nested brace syntax for multiple items from the same crate:
  ```rust
  use bbqueue::{
      prod_cons::stream::{StreamConsumer, StreamProducer},
      traits::{bbqhdl::BbqHandle, notifier::AsyncNotifier},
  };
  ```
- Do NOT use glob imports (`use foo::*`).

### Naming Conventions

- Types/structs: `PascalCase` (e.g., `CWrap`, `PWrap`)
- Functions/methods/variables: `snake_case` (e.g., `wait_read`, `buf`)
- Feature names: `snake_case` with version hyphens (e.g., `io_v0-7`)
- Struct names in this crate are terse abbreviations (`CWrap` = Consumer Wrap,
  `PWrap` = Producer Wrap). Follow this pattern for new wrapper types.

### Types and Generics

- Generic type parameters use single uppercase letters or short PascalCase names
  (e.g., `Q: BbqHandle`).
- Trait bounds go on the `impl` block, not the struct definition, when the bound
  is only needed for specific impls:
  ```rust
  pub struct CWrap<Q: BbqHandle> { ... }        // bound on struct (always needed)
  impl<Q: BbqHandle> Read for CWrap<Q>
  where Q::Notifier: AsyncNotifier { ... }       // additional bound on impl
  ```
- Use `where` clauses for complex bounds rather than inline bounds.

### Error Handling

- Use `Result<T, Self::Error>` with `embedded_io_async::ErrorKind` as the error type.
- No custom error types -- keep it simple, use the trait's error kind.
- Never use `.unwrap()` or `.expect()` in library code. These are only acceptable
  in test code.
- Propagate errors with `?` where possible.

### Documentation

- Module-level doc comments: `//!` at the top of the file.
- Struct/function doc comments: `///` with Markdown formatting.
- Use backtick-bracketed links for cross-references: `` [`TypeName`] ``.
- No inline comments unless the logic is non-obvious. Prefer self-documenting code.

### Async

- Use native `async fn` in trait impls (Rust 2024 edition supports this natively).
- Do NOT use the `async-trait` procedural macro.

### File Organization

The crate uses a flat, single-file structure with two feature-gated modules. Each
module (`v0_6`, `v0_7`) is self-contained and follows the same internal layout:

1. Version-specific `embedded_io_async` alias (`use embedded_io_async_vX_Y as embedded_io_async`)
2. `bbqueue` imports
3. `embedded_io_async` imports
4. `CWrap` struct + impls (definition → `new()` → `ErrorType` → `Read`)
5. `PWrap` struct + impls (definition → `new()` → `ErrorType` → `Write`)

The two modules are structurally identical; the only difference is the alias on
the first line. Maintain this symmetry when modifying either module.

## Dependencies

| Crate | Version | Required | Purpose |
|-------|---------|----------|---------|
| `bbqueue` | 0.6.2 | Always | Lock-free byte queue for embedded |
| `embedded-io-async` | 0.6.1 | Optional (`io_v0-6`) | Async I/O traits v0.6 |
| `embedded-io-async` | 0.7.0 | Optional (`io_v0-7`) | Async I/O traits v0.7 |

Keep dependencies minimal. This is a `no_std`/`no_alloc` crate -- do not add
dependencies that require `std` or `alloc` unless behind a feature gate.

## CI / Quality Checks

No CI pipeline is configured yet. Before submitting changes, manually verify:

```sh
cargo fmt -- --check
cargo clippy --all-features
cargo test
cargo check --features io_v0-6 --no-default-features
cargo check --features io_v0-7 --no-default-features
cargo doc --no-deps
```

All of these should pass without warnings or errors.
