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
struct Args {
    /// Path to config file (default: .envcheck.yaml)
    #[arg(short, long)]
    config: Option<String>,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load config
    let config = if let Some(config_path) = args.config {
        Config::load(&config_path)?
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
    reporter.print();

    // Exit with appropriate code
    process::exit(reporter.exit_code());
}
