# my-rust-project

[![CI](https://github.com/suitersaw-wq/my-rust-project/actions/workflows/ci.yml/badge.svg)](https://github.com/suitersaw-wq/my-rust-project/actions/workflows/ci.yml)

A Rust web server with Axum.

## Getting Started

```bash
# Enable pre-commit hooks
./scripts/setup-hooks.sh

# Build and run
cargo build
cargo run
```

The server runs at http://localhost:3000

## API Endpoints

- `GET /` - Returns greeting
- `GET /health` - Health check endpoint
- `GET /greet/{name}` - Personalized greeting

## Commands

- `cargo build` - Compile the project
- `cargo build --release` - Compile with optimizations
- `cargo run` - Build and run
- `cargo test` - Run tests
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

## Docker

```bash
# Build image
docker build -t my-rust-project .

# Run container
docker run -p 3000:3000 my-rust-project
```

## Docker Compose

```bash
# Start the application
docker compose up -d

# View logs
docker compose logs -f

# Stop the application
docker compose down
```
