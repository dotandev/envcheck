#!/bin/bash

# Usage: ./generate_issues.sh [repo_owner/repo_name]
# Example: ./generate_issues.sh dotandev/envcheck

REPO=$1

if [ -z "$REPO" ]; then
  echo "Error: Please provide a repository name (owner/repo)"
  exit 1
fi

if ! command -v gh &> /dev/null; then
  echo "Error: GitHub CLI (gh) is not installed."
  exit 1
fi

create_issue() {
  local title=$1
  local body=$2
  local labels=$3
  
  echo "Creating issue: $title"
  gh issue create --repo "$REPO" --title "$title" --body "$body" --label "$labels"
}

# 1. Good First Issues (Beginner Friendly)
create_issue "Add support for Python version checking" \
"Improve the `ToolValidator` to correctly check Python versions using `python --version` or `python3 --version`.
Currently, it has basic support but needs better parsing for various Python distributions." \
"good first issue,enhancement"

create_issue "Add support for Ruby version checking" \
"Add Ruby to the list of supported tools in `ToolValidator`. It should run `ruby --version` and parse the output." \
"good first issue,enhancement"

create_issue "Add support for Java version checking" \
"Implement Java version checking in `ToolValidator`. Keep in mind that `java -version` outputs to stderr on some systems." \
"good first issue,enhancement"

create_issue "Improve error message when config file is missing" \
"The current error message when `.envcheck.yaml` is missing is a bit technical. Let's make it more friendly and suggest running an 'init' command (future feature)." \
"good first issue,ux"

# 2. Enhancements & Features
create_issue "Implement regex pattern matching for environment variables" \
"The `EnvValidator` currently only checks for substrings. We should add support for full regex pattern matching as specified in the config." \
"enhancement,help wanted"

create_issue "Add support for checking directory existence" \
"The `FileValidator` should be expanded to check if a path is a directory, not just if it exists." \
"enhancement"

create_issue "Add JSON output format for CI/CD integration" \
"Add a `--json` flag to output results in JSON format. This will make it easier to integrate `envcheck` into other tools and pipelines." \
"feature,help wanted"

create_issue "Implement 'envcheck init' command" \
"Create a new command to generate a default `.envcheck.yaml` file in the current directory. It could even try to auto-detect the project type (Node, Go, Rust)." \
"feature,help wanted"

create_issue "Add support for checking file permissions" \
"Extend `FileValidator` to verify that a file has the correct permissions (e.g., executable, read-only)." \
"enhancement"

# 3. Infrastructure & Cleanup
create_issue "Add more comprehensive integration tests" \
"We need better test coverage for end-to-end scenarios, especially across different platforms (mocking system calls where necessary)." \
"testing,help wanted"

create_issue "Improve version parsing logic" \
"The current version parsing in `tool.rs` is very basic. We should use a more robust logic or a dedicated crate to parse various version string formats from different CLI tools." \
"enhancement,refactor"

create_issue "Add benchmarks for validation engine" \
"Let's add some benchmarks to ensure the tool remains lightning fast as we add more validators." \
"performance"

# 4. Documentation
create_issue "Create a website for envcheck" \
"We need a simple landing page or documentation site for `envcheck`. GitHub Pages would be a great choice." \
"documentation,help wanted"

create_issue "Add more example configurations" \
"Add more project templates to the `examples/` directory (e.g., Python, Rails, Django, PHP)." \
"documentation,good first issue"

echo "Check successfully completed! 14 issues suggested for $REPO."
