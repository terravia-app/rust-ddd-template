# Contributing Guidelines

Thank you for your interest in contributing to our Rust DDD template with GraphQL. This document provides guidelines and instructions for contributing to this project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Environment](#development-environment)
4. [Coding Standards](#coding-standards)
5. [Commit Guidelines](#commit-guidelines)
6. [Pull Request Process](#pull-request-process)
7. [Development Workflow](#development-workflow)
8. [Testing Guidelines](#testing-guidelines)

## Code of Conduct

By participating in this project, you agree to uphold our Code of Conduct:

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git
- Pre-commit (for quality checks)

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-ddd-template.git
   cd rust-ddd-template
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/rust-ddd-template.git
   ```
4. Install pre-commit hooks:
   ```bash
   pre-commit install
   ```

## Development Environment

### Recommended Tools

- **Editor**: VS Code with rust-analyzer extension or any Rust-friendly IDE
- **Terminal**: Any terminal with git support
- **Debug Tools**: LLDB or GDB for debugging

### Useful Commands

- Build the project: `cargo build`
- Run tests: `cargo test`
- Run the application: `cargo run`
- Format code: `cargo fmt`
- Run linter: `cargo clippy`

## Coding Standards

We follow Rust's official style guidelines and additional project-specific standards outlined in our [ARCHITECTURE.md](ARCHITECTURE.md) document.

### Key Points

1. **Follow DDD Principles**:
   - Keep domain logic in the domain layer
   - Use proper abstractions for repositories
   - Maintain clear layer boundaries

2. **Code Formatting**:
   - All code must be formatted with `cargo fmt`
   - No warnings from `cargo clippy`

3. **Documentation**:
   - Document all public APIs
   - Include examples for complex functionality
   - Keep documentation up-to-date with code changes

4. **Error Handling**:
   - Use appropriate error types
   - Provide meaningful error messages
   - Handle errors at the appropriate layer

5. **Testing**:
   - Write tests for all business logic
   - Aim for high test coverage
   - Test both success and failure cases

## Commit Guidelines

We follow conventional commit messages to make the commit history more readable and to automate versioning.

### Commit Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code changes that neither fix bugs nor add features
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Changes to build process or auxiliary tools

### Example

```
feat(domain): add user entity with validation

- Add User entity with name and email fields
- Implement validation for email format
- Add tests for all validation rules

Closes #123
```

## Pull Request Process

1. **Create a Branch**:
   - Create a branch from `main` for your changes
   - Use a descriptive name for your branch, e.g., `feat/user-authentication`

2. **Make Changes**:
   - Follow coding standards
   - Write tests for your changes
   - Ensure all tests pass
   - Update documentation as needed

3. **Submit Pull Request**:
   - Create a pull request to the `main` branch
   - Fill out the PR template
   - Reference any related issues

4. **Review Process**:
   - Maintainers will review your PR
   - Address any requested changes
   - PR must pass all CI checks
   - PR requires approval from at least one maintainer

5. **After Merge**:
   - Delete your branch
   - Update your local repository

## Development Workflow

We follow a feature-branch workflow:

1. **Issue Creation**:
   - All work should be tied to an issue
   - Issues should clearly describe the problem or feature

2. **Feature Development**:
   - Create a feature branch from `main`
   - Implement the feature following our coding standards
   - Write tests for the feature

3. **Code Review**:
   - Submit a PR for review
   - Address feedback
   - Get approval

4. **Integration**:
   - Merge the PR into `main`
   - CI/CD pipeline will build and test the changes

## Testing Guidelines

### Unit Tests

- Test each component in isolation
- Mock dependencies when necessary
- Focus on testing behavior, not implementation details

### Integration Tests

- Test interactions between components
- Verify system behavior as a whole
- Test realistic user scenarios

### Test Organization

- Place tests in the same file as the code they test using `#[cfg(test)]` modules
- For complex tests, create separate files in a `tests` directory
- Use descriptive test names that explain what is being tested

### Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email_should_pass_validation() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email.value(), "user@example.com");
    }

    #[test]
    fn test_invalid_email_should_fail_validation() {
        let result = Email::new("invalid-email".to_string());
        assert!(matches!(result, Err(EmailError::InvalidFormat)));
    }
}
```

## Conclusion

Thank you for taking the time to read our contributing guidelines. We appreciate your contributions and look forward to collaborating with you!
