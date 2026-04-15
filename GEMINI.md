# GEMINI.md - Project Context

## Project Overview
**rust101** is a foundational Rust project, likely intended for learning or as a basic template for Rust development. It utilizes the Rust 2024 edition.

- **Main Technologies:** Rust, Cargo.
- **Architecture:** Standard Rust binary application structure with a single entry point in `src/main.rs`.

## Building and Running
As a standard Cargo project, the following commands are available:

- **Build:** `cargo build` to compile the project.
- **Run:** `cargo run` to compile and run the binary.
- **Test:** `cargo test` to execute any unit or integration tests (none currently implemented).
- **Linting:** `cargo clippy` for idiomatic Rust suggestions.
- **Formatting:** `cargo fmt` to ensure consistent code style.

## Development Conventions
- **Standard Tooling:** Use `cargo` for all dependency management and build tasks.
- **Coding Style:** Follow standard Rust conventions as enforced by `rustfmt` and `clippy`.
- **Documentation:** Use doc comments (`///`) for public functions and modules as the project grows.
