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

1. Create a `.envcheck.yaml` file in your project root:

```yaml
version: "1"

tools:
  - name: node
    version: ">=18.0.0"
    required: true
  - name: docker
    required: true

env_vars:
  - name: DATABASE_URL
    required: true

ports:
  - 3000

files:
  - path: .env
    required: true
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

## Configuration

### Tools

Check if tools are installed and optionally verify versions:

```yaml
tools:
  - name: node
    version: ">=18.0.0"  # Supports >=, <=, >, <, =
    required: true
  - name: docker
    required: false      # Optional tools won't fail the check
```

Supported tools out of the box:
- `node`, `npm`, `go`, `rust`, `cargo`, `python`, `docker`, `git`, `java`, `ruby`

### Environment Variables

Validate that required environment variables are set:

```yaml
env_vars:
  - name: DATABASE_URL
    required: true
  - name: DEBUG
    required: false
```

### Ports

Check if ports are available:

```yaml
ports:
  - 3000
  - 5432
  - 8080
```

### Files

Verify that required files exist:

```yaml
files:
  - path: .env
    required: true
  - path: config/database.yml
    required: false
```

## Contributing

We love contributions! This project is designed to be community-driven. Here are some ways you can help:

- **Report bugs** - Open an issue if you find a bug
- **Suggest features** - Have an idea? We'd love to hear it
- **Improve docs** - Help make our documentation better
- **Add validators** - Add support for new tools, languages, or checks
- **Write tests** - Help us improve test coverage

Check out our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

### Good First Issues

Looking to contribute? Check out issues labeled [`good first issue`](https://github.com/dotandev/envcheck/labels/good%20first%20issue) - these are perfect for newcomers!

## Examples

See the [`examples/`](examples/) directory for sample configurations:

- [Node.js project](examples/.envcheck.yaml)
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
