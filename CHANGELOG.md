# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-28

### Added
- Core validation engine for development environments.
- **Tool Validators**: Check for installed binaries (Node, Go, Rust, Python, etc.) with semver version requirements.
- **Environment Variable Validators**: Check if required env vars are set, with optional regex pattern matching.
- **Port Validators**: Verify that specific ports are available.
- **File & Directory Validators**: Check for existence of files or directories with permission verification.
- **JSON Output**: Added `--json` flag for machine-readable output in CI/CD.
- **Initialization Command**: `envcheck init` to quickly bootstrap a project configuration.
- Comprehensive integration test suite.
- Performance benchmarks using `criterion`.
- Project templates for Django, Rails, Go, and Rust in `examples/`.
- Automated release workflow for GitHub and crates.io.

### Fixed
- Fixed Java version command to use `-version`.
- Improved error messages for missing configuration files.
- Refactored internal tool mapping to improve maintainability.
