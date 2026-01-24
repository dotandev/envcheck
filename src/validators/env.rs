use crate::config::EnvVarCheck;
use crate::validators::{ValidationResult, Validator};
use anyhow::Result;
use std::env;

pub struct EnvValidator {
    check: EnvVarCheck,
}

impl EnvValidator {
    pub fn new(check: EnvVarCheck) -> Self {
        Self { check }
    }
}

impl Validator for EnvValidator {
    fn validate(&self) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();

        match env::var(&self.check.name) {
            Ok(value) => {
                if let Some(pattern) = &self.check.pattern {
                    // TODO: Add regex pattern matching
                    if value.contains(pattern) {
                        results.push(ValidationResult::success(
                            format!("{} is set and matches pattern", self.check.name),
                        ));
                    } else {
                        results.push(ValidationResult::error(
                            format!("{} is set but does not match pattern", self.check.name),
                            Some(format!("Ensure {} matches pattern: {}", self.check.name, pattern)),
                        ));
                    }
                } else {
                    results.push(ValidationResult::success(
                        format!("{} is set", self.check.name),
                    ));
                }
            }
            Err(_) => {
                if self.check.required {
                    results.push(ValidationResult::error(
                        format!("{} is not set", self.check.name),
                        Some(format!("Set {} environment variable", self.check.name)),
                    ));
                } else {
                    results.push(ValidationResult::warning(
                        format!("{} is not set (optional)", self.check.name),
                        None,
                    ));
                }
            }
        }

        Ok(results)
    }
}
