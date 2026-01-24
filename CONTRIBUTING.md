# Contributing to envcheck

First off, thank you for considering contributing to envcheck! It's people like you that make envcheck such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide your `.envcheck.yaml` configuration**
- **Include your OS and version**
- **Paste the full error message**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- **Use a clear and descriptive title**
- **Provide a detailed description of the suggested enhancement**
- **Explain why this enhancement would be useful**
- **List some examples of how it would be used**

### Your First Code Contribution

Unsure where to begin? Look for issues labeled:

- `good first issue` - Simple issues perfect for newcomers
- `help wanted` - Issues where we need community help

### Pull Requests

1. **Fork the repo** and create your branch from `main`
2. **Make your changes** - follow the coding style below
3. **Add tests** if you've added code that should be tested
4. **Ensure the test suite passes** (`cargo test`)
5. **Run `cargo fmt`** to format your code
6. **Run `cargo clippy`** to catch common mistakes
7. **Write a good commit message**
8. **Submit your pull request**

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/envcheck.git
cd envcheck

# Build the project
cargo build

# Run tests
cargo test

# Run the tool locally
cargo run -- --help

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Project Structure

```
envcheck/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── config.rs            # Config parsing
│   ├── validators/          # All validators
│   │   ├── mod.rs          # Validator trait
│   │   ├── tool.rs         # Tool checking
│   │   ├── env.rs          # Env var checking
│   │   ├── port.rs         # Port checking
│   │   └── file.rs         # File checking
│   └── reporter.rs          # Output formatting
└── tests/                   # Integration tests
```

## Adding a New Validator

Want to add a new type of check? Here's how:

1. **Create a new file** in `src/validators/` (e.g., `network.rs`)

2. **Implement the `Validator` trait**:

```rust
use crate::validators::{ValidationResult, Validator};
use anyhow::Result;

pub struct NetworkValidator {
    host: String,
}

impl NetworkValidator {
    pub fn new(host: String) -> Self {
        Self { host }
    }
}

impl Validator for NetworkValidator {
    fn validate(&self) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();
        
        // Your validation logic here
        
        Ok(results)
    }
}
```

3. **Add to `mod.rs`** and integrate into `run_all_validations`

4. **Update the config schema** in `config.rs`

5. **Add tests** for your validator

6. **Update documentation** in README.md

## Coding Style

- Follow Rust conventions and idioms
- Use `cargo fmt` for formatting
- Use `cargo clippy` to catch common mistakes
- Write clear, descriptive variable names
- Add comments for complex logic
- Keep functions small and focused

## Testing

- Write unit tests for new functionality
- Add integration tests for end-to-end scenarios
- Ensure all tests pass before submitting PR
- Aim for high test coverage

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters
- Reference issues and pull requests liberally

Examples:
- `Add support for Python version checking`
- `Fix port validation on Windows`
- `Update README with new examples`

## Questions?

Feel free to open an issue with your question or reach out to the maintainers.

## Recognition

Contributors will be recognized in our README and release notes. Thank you for making envcheck better!
