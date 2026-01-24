use crate::validators::{ValidationResult, ValidationStatus};
use colored::*;

pub struct Reporter {
    results: Vec<ValidationResult>,
}

impl Reporter {
    pub fn new(results: Vec<ValidationResult>) -> Self {
        Self { results }
    }

    pub fn print(&self) {
        println!();
        println!("{}", "Running environment checks...".bold());
        println!();

        let mut error_count = 0;
        let mut warning_count = 0;
        for result in &self.results {
            match result.status {
                ValidationStatus::Success => {
                    println!("{} {}", "✓".green().bold(), result.message);
                }
                ValidationStatus::Warning => {
                    warning_count += 1;
                    println!("{} {}", "⚠".yellow().bold(), result.message.yellow());
                    if let Some(suggestion) = &result.suggestion {
                        println!("  {}", suggestion.dimmed());
                    }
                }
                ValidationStatus::Error => {
                    error_count += 1;
                    println!("{} {}", "✗".red().bold(), result.message.red());
                    if let Some(suggestion) = &result.suggestion {
                        println!("  {}", suggestion.dimmed());
                    }
                }
            }
        }

        println!();
        
        if error_count > 0 {
            println!(
                "{} {} issue(s) found. Fix them to continue.",
                "❌".red().bold(),
                error_count
            );
        } else if warning_count > 0 {
            println!(
                "{} {} warning(s) found, but you can proceed.",
                "⚠".yellow().bold(),
                warning_count
            );
        } else {
            println!("{} All checks passed!", "✓".green().bold());
        }
        
        println!();
    }

    pub fn has_errors(&self) -> bool {
        self.results.iter().any(|r| matches!(r.status, ValidationStatus::Error))
    }

    pub fn exit_code(&self) -> i32 {
        if self.has_errors() {
            1
        } else {
            0
        }
    }
}
