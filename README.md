# envcheck

[![Crates.io](https://img.shields.io/crates/v/envcheck.svg)](https://crates.io/crates/envcheck)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**envcheck** is a lightweight CLI tool that validates your development environment against project requirements. Say goodbye to "works on my machine" problems!

## Features

- **Fast** - Written in Rust, single binary, no dependencies
- **Simple** - Just add a `.envcheck.yaml` file to your project
- **Comprehensive** - Check tools, versions, env vars, ports, and files
- **Clean Output** - Clear, colored output with actionable suggestions
- **Cross-platform** - Works on macOS, Linux, and Windows
- **Community-driven** - Easy to extend with new validators

## Quick Start

### Installation

```bash
# Using cargo
cargo install envcheck

# Or download from releases
# https://github.com/dotandev/envcheck/releases
```

### Usage

1. Initialize a new configuration or use an existing one:

```bash
# Generate a default configuration
$ envcheck init
```

2. Run `envcheck`:

```bash
$ envcheck

Running environment checks...

✓ node 18.17.0 found
✓ docker found
✗ DATABASE_URL is not set
  Set DATABASE_URL environment variable
✓ Port 3000 is available
✓ .env exists

--- 1 issue(s) found. Fix them to continue.
```

3. Export results to JSON for CI/CD:

```bash
$ envcheck --json
```

## Configuration

### Tools

Check if tools are installed and verify versions using proper semver comparison:

```yaml
tools:
  - name: node
    version: ">=18.0.0"  # Supports semver ranges
    required: true
  - name: docker
    required: false      # Optional tools won't fail the check
```

Supported tools include `node`, `npm`, `go`, `rust`, `cargo`, `python`, `docker`, `git`, `java`, `ruby`, and more.

### Environment Variables

Validate that required environment variables are set and optionally match a regex:

```yaml
env_vars:
  - name: DATABASE_URL
    required: true
  - name: NODE_ENV
    pattern: "^(development|test|production)$"
    required: true
```

### Ports

Check if ports are available:

```yaml
ports:
  - 3000
  - 5432
```

### Files & Directories

Verify that required files or directories exist and have correct permissions:

```yaml
files:
  - path: .env
    required: true
    permissions: 0o600 # Verify octal permissions (Unix)
  - path: storage/logs
    is_directory: true
    required: true
```

## Contributing

We love contributions! This project is designed to be community-driven. See our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

### Good First Issues

Looking to contribute? Check out issues labeled [`good first issue`](https://github.com/dotandev/envcheck/labels/good%20first%20issue) - these are perfect for newcomers!

## Examples

See the [`examples/`](examples/) directory for sample configurations:

- [Node.js project](examples/.envcheck.yaml)
- [Django project](examples/django-project.yaml)
- [Rails project](examples/rails-project.yaml)
- [Go project](examples/go-project.yaml)
- [Rust project](examples/rust-project.yaml)

## Development

```bash
# Clone the repo
git clone https://github.com/dotandev/envcheck.git
cd envcheck

# Build
cargo build

# Run tests
cargo test

# Run locally
cargo run -- --help
```

## License

MIT License - see [LICENSE](LICENSE) for details

## Inspiration

Inspired by tools like [Redis](https://redis.io) - built by the community, for the community.

---

**Made by the envcheck community**
