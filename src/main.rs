use clap::Parser;
use anyhow::Result;
use std::process;

mod config;
mod validators;
mod reporter;

use config::Config;
use reporter::Reporter;

#[derive(Parser, Debug)]
#[command(name = "envcheck")]
#[command(author = "envcheck contributors")]
#[command(version = "0.1.0")]
#[command(about = "Validate your development environment", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to config file (default: .envcheck.yaml)
    #[arg(short, long)]
    config: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output in JSON format
    #[arg(long)]
    json: bool,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Initialize a new .envcheck.yaml file
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            handle_init()?;
        }
        None => {
            run_validation(&cli)?;
        }
    }

    Ok(())
}

fn handle_init() -> Result<()> {
    let config_path = ".envcheck.yaml";
    if std::path::Path::new(config_path).exists() {
        anyhow::bail!(".envcheck.yaml already exists in the current directory");
    }

    let default_config = r#"version: "1"
tools:
  - name: node
    version: ">=18.0.0"
  - name: git
env_vars:
  - name: NODE_ENV
    required: false
ports:
  - 3000
files:
  - path: .env
    required: false
"#;

    std::fs::write(config_path, default_config)?;
    println!("Successfully initialized .envcheck.yaml");
    Ok(())
}

fn run_validation(args: &Cli) -> Result<()> {
    // Load config
    let config = if let Some(config_path) = &args.config {
        Config::load(config_path)?
    } else {
        Config::find_config()?
    };

    if args.verbose {
        println!("Loaded config: {:?}", config);
    }

    // Run validations
    let results = validators::run_all_validations(&config)?;

    // Report results
    let reporter = Reporter::new(results);
    reporter.print(args.json);

    // Exit with appropriate code
    process::exit(reporter.exit_code());
}
