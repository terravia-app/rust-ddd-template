# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Building and Running

- Build the project: `cargo build`
- Build for production: `cargo build --release`
- Run the application: `cargo run`
- Access GraphQL playground: http://localhost:8000/graphql

### Testing

- Run all tests: `cargo test`
- Run specific test: `cargo test test_name`
- Run tests in a specific module: `cargo test module_name`
- Check test coverage: `cargo tarpaulin --ignore-tests`

### Quality Checks

- Format code: `cargo fmt`
- Check formatting without changes: `cargo fmt -- --check`
- Run linter: `cargo clippy -- -D warnings`
- Run all quality checks: `cargo fmt && cargo clippy -- -D warnings && cargo test`

## Development Workflow

This project uses an optimized workflow to balance code quality with development efficiency:

### Pre-commit Hooks (Fast)

Only lightweight checks run on every commit:
- File formatting (end-of-file, whitespace)
- Code formatting (`cargo fmt`)
- Clippy on changed files only

Install hooks:
```bash
pre-commit install
pre-commit install --hook-type pre-push
```

### Pre-push Hooks (Comprehensive)

More comprehensive checks run before pushing to remote:
- Full workspace clippy
- All tests
- Code coverage check

These hooks won't slow down your commit process but ensure code quality before sharing.

### GitHub Actions (CI/CD)

The CI pipeline runs in GitHub Actions with the following jobs:
- Check: Format and clippy
- Test: Run all tests
- Coverage: Ensure 80% minimum coverage and upload to Codecov

This setup offloads intensive checks from local development to CI.

## Architecture Overview

This project follows Domain-Driven Design (DDD) principles with a clean architecture approach:

### Layered Architecture

1. **Domain Layer** (`src/domain/`):
   - Core business logic and rules
   - Value objects (immutable, validated)
   - Domain services (core operations)
   - Domain errors

2. **Application Layer** (`src/application/`):
   - Use cases (orchestrate domain objects)
   - DTOs (data transfer objects)
   - Application-specific errors

3. **Infrastructure Layer** (`src/infrastructure/`):
   - External integrations
   - GraphQL schema definitions
   - Persistence implementations

4. **Interface Layer** (`src/interfaces/`):
   - API endpoints
   - GraphQL handlers

### Key Design Principles

- **Clean Architecture**: Each layer depends only on inner layers
- **Dependency Inversion**: High-level modules don't depend on low-level modules
- **Explicit Dependencies**: Dependencies passed as parameters
- **Error Handling**: Structured approach with proper error types
- **Immutability**: Preference for immutable data structures

## GraphQL Implementation

The application exposes a GraphQL API using async-graphql and axum:

- Schema defined in `src/infrastructure/graphql/schema.rs`
- API endpoints in `src/interfaces/api/graphql.rs`
- Example query:
  ```graphql
  {
    hello(name: "World") {
      message
    }
  }
  ```

## Error Handling Strategy

The application uses a structured approach to errors:

- Domain errors: `src/domain/value_objects/name.rs` (NameError)
- Application errors: `src/application/use_cases/greeting_use_case.rs` (GreetingUseCaseError)
- Errors properly propagated through layers
- thiserror for defining error types

## Testing Approach

- Tests are co-located with the code they test using `#[cfg(test)]` modules
- Unit tests focus on domain and application logic
- Each layer has its own tests
- Minimum code coverage requirement: 80%

## Learnings and Best Practices

> This section captures insights gained while working with this codebase. Feel free to expand it as you discover more patterns and solutions.

### Development Workflow Optimization

- **Progressive Validation**: The best workflow uses fast checks for commits, thorough checks for pushes, and complete validation in CI
- **Performance**: For large codebases, running clippy only on changed files can reduce commit times from minutes to seconds
- **Pre-commit Configuration**: Use `stages: [pre-push]` for push hooks, not the deprecated `stages: [push]`

### Rust-Specific Patterns

- **Imports**: Prefer fully qualified paths (e.g., `tracing_subscriber::fmt::init()`) over imports for single-use items
- **Collection Checks**: Use `!collection.is_empty()` rather than `collection.len() > 0` for better readability
- **Test Assertions**: Avoid `assert!(true)` statements; instead use comments to explain what would cause failure
- **Tracing Setup**: Simple applications can use `tracing_subscriber::fmt::init()` without complex configuration

### CI/CD Practices

- **Job Isolation**: Separate CI into distinct jobs (check, test, coverage) for clearer error reporting
- **Dependency Caching**: Cache Cargo artifacts to reduce build times
- **Optional Integrations**: Make third-party services like Codecov conditional to avoid CI failures
- **Git Practices**: Always specify remote and branch names explicitly when pushing (`git push origin main`)

### GraphQL Best Practices

- **Error Handling**: Map domain-specific errors to GraphQL errors with clear messages
- **Testing**: Test both successful queries and error cases
- **Schema Design**: Start with minimal schema and expand based on actual needs
