# Architecture and Development Guidelines

This document outlines the architectural principles, code organization, and development guidelines for this Rust DDD template with GraphQL.

## Table of Contents

1. [Domain-Driven Design Principles](#domain-driven-design-principles)
2. [Project Structure](#project-structure)
3. [Architectural Layers](#architectural-layers)
4. [Coding Standards](#coding-standards)
5. [Error Handling](#error-handling)
6. [Testing Strategy](#testing-strategy)
7. [GraphQL API Design](#graphql-api-design)
8. [Development Workflow](#development-workflow)

## Domain-Driven Design Principles

This project follows Domain-Driven Design (DDD) principles to create a clear separation of concerns and ensure that the business logic is properly encapsulated.

### Key DDD Concepts Implemented

- **Entities**: Domain objects that have an identity and are tracked through time
- **Value Objects**: Immutable objects that have no identity and are defined by their attributes
- **Repositories**: Abstractions for data persistence and retrieval
- **Domain Services**: Logic that doesn't naturally fit into entities or value objects
- **Application Services/Use Cases**: Orchestration of domain objects to fulfill business operations
- **Bounded Contexts**: Clear boundaries between different parts of the domain model

### DDD Benefits

- Focus on the core domain and domain logic
- Collaboration between technical and domain experts
- Iterative and continuous model refinement
- Clear organization of business logic

## Project Structure

```
src/
├── domain/                 # Domain layer - core business logic
│   ├── entities/           # Business entities with identity
│   ├── value_objects/      # Immutable objects without identity
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

## Architectural Layers

### 1. Domain Layer

The domain layer is the heart of the application. It contains:

- **Value Objects**: Immutable objects that are defined by their attributes, not by identity
  - Must validate their inputs
  - Should be immutable
  - Must encapsulate domain rules relevant to their properties

- **Entities**: Objects with identity that are tracked through time
  - Must have a unique identifier
  - May contain domain logic relevant to their lifecycle

- **Domain Services**: Logic that doesn't naturally fit into entities or value objects
  - Operate on multiple entities or value objects
  - Named using domain terminology
  - Stateless operations

- **Repositories (Interfaces)**: Abstractions for persistence
  - Defined as traits
  - Operations defined in domain terms

- **Domain Events**: Notifications about important occurrences within the domain
  - Named in past tense
  - Immutable

### 2. Application Layer

The application layer orchestrates the domain objects to perform specific use cases:

- **DTOs (Data Transfer Objects)**: Simple data structures for input/output
  - Should not contain business logic
  - Used to transfer data between layers

- **Use Cases**: Application-specific logic
  - Each use case represents one business operation
  - Orchestrates domain objects
  - Translates between DTOs and domain objects
  - Handles application-level errors

### 3. Infrastructure Layer

The infrastructure layer provides implementations for external concerns:

- **Persistence**: Database access and repository implementations
  - Implements domain repository traits
  - Handles database-specific concerns

- **GraphQL**: Schema and resolver implementations
  - Translates between GraphQL types and application DTOs
  - Defines the GraphQL API schema

### 4. Interfaces Layer

The interfaces layer handles external communications:

- **API**: HTTP and GraphQL endpoints
  - Routing configuration
  - Request/response handling
  - Authentication/authorization (when applicable)

## Coding Standards

### General Principles

1. **Single Responsibility Principle**: Each module, struct, and function should have only one reason to change
2. **Dependency Inversion**: High-level modules should not depend on low-level modules
3. **Explicit Dependencies**: Dependencies should be explicitly passed as parameters
4. **Error Handling**: Use proper error types and propagation
5. **Immutability**: Prefer immutable data when possible

### Rust-Specific Guidelines

1. **Naming Conventions**:
   - Use `snake_case` for variables, functions, and modules
   - Use `CamelCase` for types, traits, and enums
   - Use `SCREAMING_SNAKE_CASE` for constants

2. **Code Organization**:
   - Group related items in modules
   - Use clear and descriptive names
   - Keep functions small and focused

3. **Documentation**:
   - Document public APIs with rustdoc comments
   - Include examples where appropriate

4. **Error Handling**:
   - Use `thiserror` for defining error types
   - Make error messages user-friendly
   - Return `Result` types for fallible operations

5. **Testing**:
   - Write unit tests for all business logic
   - Use test modules with the `#[cfg(test)]` attribute
   - Consider property-based testing for complex algorithms

## Error Handling

This project uses a structured approach to error handling:

1. **Domain Errors**: Defined in the domain layer
   - Represent business rule violations
   - Named using domain terminology

2. **Application Errors**: Defined in the application layer
   - May wrap domain errors
   - Represent application-specific failures

3. **Infrastructure Errors**: Defined in the infrastructure layer
   - Represent technical failures
   - Should be mapped to application or domain errors when crossing layer boundaries

4. **Error Propagation**:
   - Use the `?` operator for error propagation
   - Use `thiserror` for error type definitions
   - Include context when converting between error types

Example:
```rust
#[derive(Debug, Error)]
pub enum NameError {
    #[error("Name cannot be empty")]
    Empty,
    #[error("Name is too long (maximum 100 characters)")]
    TooLong,
}
```

## Testing Strategy

### Unit Tests

- **Domain Layer**: Test all business rules and logic
- **Application Layer**: Test use case orchestration
- **Infrastructure Layer**: Test external integrations (with mocks)

### Integration Tests

- Test the interaction between layers
- Test API endpoints with realistic data

### Test Coverage

- **Minimum Coverage Requirement**: 80% code coverage
- Domain and application layers should aim for 100% coverage
- Infrastructure layer should have critical paths covered
- Use `cargo-tarpaulin` to measure and verify coverage:
  ```bash
  cargo tarpaulin --ignore-tests
  ```
- Pre-commit hooks enforce minimum coverage levels
- Current coverage: 86.54% (as of last update)

### Test Organization

- Place unit tests in the same file as the code they test using `#[cfg(test)]` modules
- Use test fixtures to set up common test data

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = Name::new("John".to_string()).unwrap();
        assert_eq!(name.value(), "John");
    }
}
```

## GraphQL API Design

### Schema Design Principles

1. **Type System**: Leverage GraphQL's strong type system
   - Define clear input and output types
   - Use enums for fixed sets of values

2. **Query Design**:
   - Follow resource-oriented design
   - Use descriptive field names
   - Provide optional fields for flexibility

3. **Mutation Design**:
   - Use clear, action-oriented names
   - Return the affected resource
   - Include error information in the response

4. **Error Handling**:
   - Return error messages in the response
   - Use standard error formats

### Schema Organization

- Group related types and fields together
- Use a modular approach to schema definition

## Development Workflow

### 1. Feature Development

1. Start with domain models and business rules
2. Implement application use cases
3. Create infrastructure implementations
4. Add interface endpoints
5. Write tests at each step

### 2. Quality Assurance

All code must pass the following checks before being committed:

1. **Formatting**:
   ```bash
   cargo fmt
   ```

2. **Linting**:
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Testing**:
   ```bash
   cargo test
   ```

4. **Coverage**:
   ```bash
   cargo tarpaulin --ignore-tests
   ```
   - Must maintain at least 80% code coverage
   - New code should aim for 100% coverage

### 3. Pre-commit Hooks

The repository includes pre-commit hooks to ensure code quality:

- Automatic code formatting check
- Clippy lints with warnings treated as errors
- Running tests
- Verifying test coverage meets minimum requirements (80%)

To install the hooks:
```bash
pre-commit install
```

### 4. Continuous Integration

Future CI/CD pipelines should include:

- Building and testing the application
- Running all quality checks
- Verifying test coverage
- Deployment to staging/production environments

## Conclusion

By following these guidelines, we aim to create a maintainable, robust, and well-structured application that properly encapsulates business logic and provides a clean, reliable API.
