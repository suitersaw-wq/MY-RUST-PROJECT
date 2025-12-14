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

## Setup

After cloning, run `./scripts/setup-hooks.sh` to enable pre-commit hooks.

## Docker

- `docker build -t my-rust-project .` - Build Docker image
- `docker run -p 3000:3000 my-rust-project` - Run container

## Docker Compose

- `docker compose up -d` - Start application
- `docker compose down` - Stop application
- `docker compose logs -f` - View logs

## CI

GitHub Actions runs on push/PR to master:
- Format check, clippy, build, and test
- Code coverage with cargo-tarpaulin (uploaded as artifact)
- Docker build verification

## Architecture

This is a Rust web server using Axum framework.

- `src/main.rs` - Main entry point with Axum server and routes
- Tests are inline using `#[cfg(test)]` module

## Web Server

The application runs an Axum web server on port 3000.

### API Endpoints

- `GET /` - Returns "Hello, World!"
- `GET /health` - Returns `{"status": "ok"}`
- `GET /greet/{name}` - Returns personalized greeting
- `POST /ai/chat` - Chat with Claude AI (requires `ANTHROPIC_API_KEY`)

### Environment Variables

- `ANTHROPIC_API_KEY` - Required for `/ai/chat` endpoint

### Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `serde` / `serde_json` - JSON serialization
- `reqwest` - HTTP client for Anthropic API
