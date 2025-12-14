# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

- `cargo build` - Compile the project (debug mode)
- `cargo build --release` - Compile with optimizations
- `cargo run` - Build and run the application
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run a specific test
- `cargo clippy` - Run linter
- `cargo fmt` - Format code
- `cargo tarpaulin` - Run tests with coverage (requires cargo-tarpaulin)

## CI

GitHub Actions runs on push/PR to master:
- Format check, clippy, build, and test
- Code coverage with cargo-tarpaulin (uploaded as artifact)

## Architecture

This is a Rust binary application. Entry point: `src/main.rs`
