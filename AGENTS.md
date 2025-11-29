# AGENTS.md

This file provides guidelines for agentic coding agents operating in this Rust repository.

## Build, Lint, and Test Commands

- Build: `cargo build`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Test: `cargo test`
- Run single test: `cargo test <test_name>`
- Run all tests with coverage: `cargo test -- --nocapture`

## Code Style Guidelines

- Follow Rust naming conventions (snake_case for functions and variables, PascalCase for types)
- Use `rustfmt` for formatting with settings in `rustfmt.toml`
- Reorder imports alphabetically
- Maximum line width: 100 characters
- Use spaces, not tabs (4 spaces per tab)
- All public APIs should have documentation comments
- Prefer `anyhow` for error handling
- Use `tracing` for logging and debugging

## Testing

- Use `rstest` for parameterized tests
- Tests should be in the same file as the code they test when appropriate
- Integration tests should be in their own module or file

## Project Structure

- Source files in `src/` directory
- REPL in `src/bin/repl.rs`
- Core modules: lexer, parser, AST, evaluator, environment, object