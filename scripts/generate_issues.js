import fs from 'fs';

const issues = [
    {
        title: "Add support for Python version checking",
        body: "Improve the `ToolValidator` to correctly check Python versions using `python --version` or `python3 --version`. Currently, it has basic support but needs better parsing for various Python distributions.",
        labels: ["good first issue", "enhancement"]
    },
    {
        title: "Add support for Ruby version checking",
        body: "Add Ruby to the list of supported tools in `ToolValidator`. It should run `ruby --version` and parse the output.",
        labels: ["good first issue", "enhancement"]
    },
    {
        title: "Add support for Java version checking",
        body: "Implement Java version checking in `ToolValidator`. Keep in mind that `java -version` outputs to stderr on some systems.",
        labels: ["good first issue", "enhancement"]
    },
    {
        title: "Improve error message when config file is missing",
        body: "The current error message when `.envcheck.yaml` is missing is a bit technical. Let's make it more friendly and suggest running an 'init' command (future feature).",
        labels: ["good first issue", "ux"]
    },
    {
        title: "Implement regex pattern matching for environment variables",
        body: "The `EnvValidator` currently only checks for substrings. We should add support for full regex pattern matching as specified in the config.",
        labels: ["enhancement", "help wanted"]
    },
    {
        title: "Add support for checking directory existence",
        body: "The `FileValidator` should be expanded to check if a path is a directory, not just if it exists.",
        labels: ["enhancement"]
    },
    {
        title: "Add JSON output format for CI/CD integration",
        body: "Add a `--json` flag to output results in JSON format. This will make it easier to integrate `envcheck` into other tools and pipelines.",
        labels: ["feature", "help wanted"]
    },
    {
        title: "Implement 'envcheck init' command",
        body: "Create a new command to generate a default `.envcheck.yaml` file in the current directory. It could even try to auto-detect the project type (Node, Go, Rust).",
        labels: ["feature", "help wanted"]
    },
    {
        title: "Add support for checking file permissions",
        body: "Extend `FileValidator` to verify that a file has the correct permissions (e.g., executable, read-only).",
        labels: ["enhancement"]
    },
    {
        title: "Add more comprehensive integration tests",
        body: "We need better test coverage for end-to-end scenarios, especially across different platforms (mocking system calls where necessary).",
        labels: ["testing", "help wanted"]
    },
    {
        title: "Improve version parsing logic",
        body: "The current version parsing in `tool.rs` is very basic. We should use a more robust logic or a dedicated crate to parse various version string formats from different CLI tools.",
        labels: ["enhancement", "refactor"]
    },
    {
        title: "Add benchmarks for validation engine",
        body: "Let's add some benchmarks to ensure the tool remains lightning fast as we add more validators.",
        labels: ["performance"]
    },
    {
        title: "Create a website for envcheck",
        body: "We need a simple landing page or documentation site for `envcheck`. GitHub Pages would be a great choice.",
        labels: ["documentation", "help wanted"]
    },
    {
        title: "Add more example configurations",
        body: "Add more project templates to the `examples/` directory (e.g., Python, Rails, Django, PHP).",
        labels: ["documentation", "good first issue"]
    },
    {
        title: "Publish to crates.io and setup release workflows",
        body: "We should publish `envcheck` to crates.io for easier installation. This includes setting up automated release workflows to create GitHub Releases with attached binaries and publishing the crate automatically on tag push.",
        labels: ["enhancement", "help wanted"]
    }
];

async function createIssues() {
    const repo = process.argv[2];
    const token = process.env.GITHUB_TOKEN;

    if (!repo) {
        console.error("Error: Please provide a repository name (owner/repo)");
        process.exit(1);
    }

    if (!token) {
        console.error("Error: GITHUB_TOKEN environment variable is not set");
        process.exit(1);
    }

    console.log(`Creating ${issues.length} issues for ${repo}...`);

    for (const issue of issues) {
        try {
            const response = await fetch(`https://api.github.com/repos/${repo}/issues`, {
                method: 'POST',
                headers: {
                    'Authorization': `token ${token}`,
                    'Accept': 'application/vnd.github.v3+json',
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(issue)
            });

            if (response.ok) {
                const data = await response.json();
                console.log(`Created issue: ${issue.title} (#${data.number})`);
            } else {
                const error = await response.json();
                console.error(`Failed to create issue: ${issue.title}`);
                console.error(`Reason: ${error.message}`);
            }
        } catch (err) {
            console.error(`Error creating issue: ${issue.title}`);
            err && console.error(err);
        }

        await new Promise(resolve => setTimeout(resolve, 500));
    }

    console.log("\nDone!");
}

createIssues();
