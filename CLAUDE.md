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
