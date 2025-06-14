# Rust DDD Template with GraphQL

A template for building Rust backend applications following Domain-Driven Design (DDD) principles with a GraphQL API.

## Project Structure

The project follows a DDD architecture with clear separation of concerns:

```
src/
├── domain/                 # Domain layer - core business logic
│   ├── entities/           # Business entities
│   ├── value_objects/      # Value objects
│   ├── repositories/       # Repository interfaces
│   ├── services/           # Domain services
│   └── events/             # Domain events
├── application/            # Application layer - use cases and DTOs
│   ├── dtos/               # Data Transfer Objects
│   └── use_cases/          # Application use cases
├── infrastructure/         # Infrastructure layer - external concerns
│   ├── persistence/        # Database implementations
│   └── graphql/            # GraphQL schema and resolvers
└── interfaces/             # Interface layer - API endpoints
    └── api/                # API implementation
```

## Features

- Rust 2024 edition
- Domain-Driven Design architecture
- GraphQL API using async-graphql and axum
- Hello operation example
- Error handling with thiserror
- Logging with tracing
- Async runtime with tokio

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Running the Application

1. Clone this repository
2. Build and run the application:

```bash
cargo run
```

3. Navigate to `http://localhost:8000/graphql` in your browser to access the GraphQL playground

### Example Query

```graphql
{
  hello(name: "World")
}
```

This will return:

```json
{
  "data": {
    "hello": {
      "message": "Hello, World!"
    }
  }
}
```

## Development

### Quality Checks

The project uses several quality checks that can be run with cargo commands:

```bash
# Run formatting
cargo fmt

# Check formatting (will not modify code)
cargo fmt -- --check

# Run clippy linter
cargo clippy

# Run clippy with warnings as errors
cargo clippy -- -D warnings

# Run tests
cargo test

# Run all checks in sequence
cargo fmt && cargo clippy -- -D warnings && cargo test
```

### Pre-commit Hooks

The project is configured with pre-commit hooks to ensure code quality. After cloning the repository:

```bash
# Install pre-commit hooks
pre-commit install
```

This will automatically check formatting, run clippy, and execute tests before each commit.

### Building for Production

```bash
cargo build --release
```

## License

MIT
