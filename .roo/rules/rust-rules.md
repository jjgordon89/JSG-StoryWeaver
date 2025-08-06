# Rust Development Rules

## Code Quality & Linting

### Use rustfmt for Code Formatting
- Always format Rust code using rustfmt
- Run `cargo fmt` before committing code
- Configure rustfmt in rustfmt.toml:
```toml
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
```

### Use Clippy for Linting
- Run `cargo clippy` to catch common mistakes and improve code
- Configure Clippy to be strict:
```toml
# In Cargo.toml
[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
cargo = "warn"
```

### Enable Compiler Warnings
- Configure strict compiler warnings in Cargo.toml:
```toml
[lints.rust]
unsafe_code = "forbid"
unused_imports = "warn"
unused_variables = "warn"
dead_code = "warn"
```

## Static Application Security Testing (SAST)

### Use cargo-audit for Dependency Vulnerability Scanning
- Install and run `cargo audit` regularly
- Check for known security vulnerabilities in dependencies
- Configure in CI/CD pipeline

### Use cargo-deny for Dependency Management
- Configure cargo-deny to enforce security policies
- Create deny.toml configuration:
```toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
unsound = "warn"
notice = "warn"

[licenses]
unlicensed = "deny"
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
deny = ["GPL-2.0", "GPL-3.0"]

[bans]
multiple-versions = "warn"
wildcards = "allow"
```

### Use semgrep for Advanced SAST
- Use semgrep with Rust rules for security analysis
- Focus on unsafe code blocks and potential vulnerabilities
- Run `semgrep --config=rust .`

### Minimize Unsafe Code
- Avoid `unsafe` blocks unless absolutely necessary
- Document all unsafe code with safety comments
- Use safe alternatives when possible
- Consider using `#![forbid(unsafe_code)]` for safe-only crates

## Testing

### Use Built-in Testing Framework
- Write comprehensive unit tests using `#[cfg(test)]`
- Aim for >90% code coverage
- Use `cargo test` for running tests
- Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
        assert_eq!(expected, actual);
    }
}
```

### Use cargo-tarpaulin for Coverage
- Measure code coverage with `cargo tarpaulin`
- Generate HTML coverage reports
- Configure coverage thresholds in CI/CD

### Use proptest for Property-Based Testing
- Add proptest for property-based testing
- Test invariants and edge cases automatically
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(input in any::<i32>()) {
        // Property test implementation
    }
}
```

### Integration and Benchmark Tests
- Write integration tests in `tests/` directory
- Use `cargo bench` for performance benchmarks
- Use criterion.rs for detailed benchmarking

## Documentation

### Write Comprehensive Documentation
- Document all public APIs with `///` comments
- Include examples in documentation comments
- Use `cargo doc` to generate documentation
- Example documentation:
```rust
/// Calculates the factorial of a number.
///
/// # Arguments
///
/// * `n` - A positive integer
///
/// # Returns
///
/// The factorial of `n`
///
/// # Examples
///
/// ```
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// ```
pub fn factorial(n: u32) -> u32 {
    // Implementation
}
```

### Maintain README.md
- Include installation instructions
- Provide usage examples
- Document feature flags
- Include performance characteristics

### Use doc tests
- Include executable examples in documentation
- Run `cargo test` to verify doc examples
- Ensure examples stay up-to-date

## Architecture Decision Records (ADRs)

### Document Architectural Decisions
- Create ADRs for significant design choices
- Store ADRs in `docs/adr/` directory
- Document decisions about:
  - Error handling strategies
  - Async vs sync approaches
  - Memory management patterns
  - API design choices

### ADR Template for Rust
```markdown
# ADR-001: [Title]

## Status
[Proposed | Accepted | Deprecated | Superseded]

## Context
[Describe the Rust-specific context and problem]

## Decision
[Describe the decision and Rust-specific rationale]

## Consequences
[Describe impact on performance, safety, and maintainability]
```

## Project Structure

### Follow Standard Rust Project Layout
```
project/
├── src/
│   ├── lib.rs or main.rs
│   ├── modules/
│   └── bin/
├── tests/
│   └── integration_test.rs
├── benches/
│   └── benchmark.rs
├── examples/
│   └── example.rs
├── docs/
│   └── adr/
├── Cargo.toml
├── Cargo.lock
└── README.md
```

### Configure Cargo.toml Properly
- Define clear package metadata
- Use semantic versioning
- Configure feature flags appropriately
- Set edition to latest stable

## Error Handling

### Use Result<T, E> for Recoverable Errors
- Return `Result` types for operations that can fail
- Use `?` operator for error propagation
- Create custom error types when appropriate
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {message}")]
    Parse { message: String },
}
```

### Use panic! Only for Unrecoverable Errors
- Reserve `panic!` for programming errors
- Use `expect()` with descriptive messages
- Consider `unwrap()` only in examples and tests

### Implement Proper Error Context
- Use `anyhow` for application errors
- Use `thiserror` for library errors
- Provide meaningful error messages

## Performance

### Profile Before Optimizing
- Use `cargo flamegraph` for profiling
- Use `perf` on Linux for detailed analysis
- Benchmark with `criterion.rs`

### Memory Management Best Practices
- Prefer borrowing over cloning
- Use `Cow<T>` for conditional ownership
- Avoid unnecessary allocations
- Use `Box<T>` for large stack objects

### Async Programming
- Use `tokio` for async runtime
- Prefer `async/await` over manual futures
- Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared state
- Consider `rayon` for CPU-bound parallelism

## Security Best Practices

### Input Validation
- Validate all external inputs
- Use type system for validation when possible
- Sanitize data before processing

### Dependency Management
- Regularly update dependencies
- Use `cargo audit` in CI/CD
- Pin versions for reproducible builds
- Minimize dependency count

### Safe Concurrency
- Use Rust's ownership system for thread safety
- Prefer message passing over shared memory
- Use `Arc` and `Mutex` appropriately
- Avoid data races with type system

## Build and Release

### Configure Release Builds
- Optimize for release builds in Cargo.toml:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

### Use Feature Flags
- Define optional features in Cargo.toml
- Use conditional compilation with `#[cfg(feature = "...")]`
- Document feature flags in README

### Cross-compilation
- Support multiple targets when appropriate
- Test on different platforms
- Use GitHub Actions for multi-platform builds

## Workspace Management

### Use Cargo Workspaces for Multi-crate Projects
- Configure workspace in root Cargo.toml:
```toml
[workspace]
members = ["crate1", "crate2"]
resolver = "2"
```

### Shared Dependencies
- Define common dependencies in workspace Cargo.toml
- Use consistent versions across workspace
- Share development tools configuration
