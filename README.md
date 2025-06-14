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
- pre-commit (for development workflow)

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
  hello(name: "World") {
    message
  }
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

### Optimized Development Workflow

This project uses an optimized workflow to balance code quality with development efficiency:

#### 1. Fast Pre-commit Hooks

Only lightweight checks run on every commit:
- File formatting (end-of-file, whitespace)
- Code formatting (`cargo fmt`)
- Clippy on changed files only

This ensures quick commits while maintaining basic code quality.

#### 2. Comprehensive Pre-push Hooks

More thorough checks run before pushing to remote:
- Full workspace clippy
- All tests
- Code coverage check

These hooks won't slow down your commit process but ensure code quality before sharing.

#### 3. GitHub Actions CI

The CI pipeline runs in GitHub Actions with:
- Check: Format and clippy
- Test: Run all tests
- Coverage: Ensure 80% minimum coverage

#### Setup

```bash
# Install pre-commit
pip install pre-commit

# Install both pre-commit and pre-push hooks
pre-commit install
pre-commit install --hook-type pre-push
```

### Quality Checks

Individual quality checks can be run manually:

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

# Check coverage
cargo tarpaulin --ignore-tests
```

### Building for Production

```bash
cargo build --release
```

## License

MIT
