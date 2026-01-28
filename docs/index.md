# envcheck

**A lightweight, fast CLI tool for validating developer environments.**

`envcheck` helps you and your team ensure that everyone has the right tools, environment variables, and file structures to work on a project.

## Key Features

- **Tool Checks**: Verify that required binaries (Node, Go, Rust, etc.) are installed and meet version requirements.
- **Environment Variables**: Check if essential env vars are set and match specific patterns.
- **Port Checks**: Ensure that required ports are available and not in use.
- **File & Directory Checks**: Verify existence and permissions of critical files and folders.
- **JSON Output**: Easy integration into CI/CD pipelines.
- **Initialization**: Quickly bootstrap your project with `envcheck init`.

## Getting Started

### Installation

```bash
cargo install envcheck
```

### Usage

1. Initialize a configuration file:
   ```bash
   envcheck init
   ```
2. Customize the generated `.envcheck.yaml`.
3. Run the checks:
   ```bash
   envcheck
   ```

## Configuration Example

```yaml
version: "1"

tools:
  - name: node
    version: ">=18.0.0"
    required: true

env_vars:
  - name: DATABASE_URL
    required: true

ports:
  - 5432

files:
  - path: .env
    required: true
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/dotandev/envcheck/blob/main/CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/dotandev/envcheck/blob/main/LICENSE) file for details.
