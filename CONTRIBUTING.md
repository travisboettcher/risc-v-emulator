# Contributing to RISC-V Emulator

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:

- Clear, descriptive title
- Exact steps to reproduce
- Expected vs actual behavior
- Code samples (minimal reproducible example)
- Environment details (OS, Rust version)
- Stack traces or error messages

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- Use a clear, descriptive title
- Provide detailed description of the proposed functionality
- Explain why this enhancement would be useful
- List any similar features in other emulators

### Pull Requests

1. **Fork and clone** the repository
2. **Create a branch** from `master`:
   - `feature/add-rv64i-support`
   - `fix/memory-alignment-bug`
   - `docs/improve-cpu-documentation`

3. **Make your changes**:
   - Follow the Rust style guide
   - Write tests for new functionality
   - Update documentation
   - Add rustdoc comments for public APIs

4. **Ensure CI passes**:
   ```bash
   cargo test --all-features
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   cargo doc --no-deps
   ```

5. **Commit with clear messages**:
   ```
   Add support for RV64I extension

   Implement 64-bit integer instructions including ADDW, SUBW,
   SLLW, SRLW, SRAW. Update CPU to support both 32-bit and
   64-bit modes.

   Closes #123
   ```

6. **Submit pull request**

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Setup

```bash
# Clone your fork
git clone https://github.com/your-username/risc-v-emulator
cd risc-v-emulator

# Add upstream remote
git remote add upstream https://github.com/original/risc-v-emulator
```

### Building

```bash
# Build library
cargo build

# Build with all features
cargo build --all-features
```

### Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Documentation

```bash
# Build and open docs
cargo doc --no-deps --open

# Check doc warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Code Style

### Rust Style Guide

Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/). Key points:

- Use `cargo fmt` for formatting
- Max line length: 100 characters
- Use descriptive variable names
- Prefer explicit types in public APIs
- Document public items with rustdoc comments

### Documentation Style

```rust
/// Executes a single instruction.
///
/// # Errors
///
/// Returns an error if the instruction is invalid or
/// memory access is out of bounds.
///
/// # Examples
///
/// ```
/// use risc_v_emulator::processor::Processor;
///
/// let mut cpu = Processor::new();
/// cpu.execute_instructions();
/// ```
pub fn execute_instructions(&mut self) {
    // ...
}
```

### Testing

- Write unit tests for new functionality
- Write integration tests for user-facing features
- Use descriptive test names: `test_memory_out_of_bounds_error`
- Test error cases, not just happy paths

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `perf`: Performance improvements

## Review Process

1. **Automated checks**: CI must pass
2. **Code review**: At least one maintainer approval required
3. **Testing**: New code must have tests
4. **Documentation**: Public APIs must be documented

## Questions?

- Open an issue for questions
- Check existing documentation

## License

By contributing, you agree that your contributions will be licensed under
the same license as the project (MIT OR Apache-2.0).
