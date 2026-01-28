use crate::config::FileCheck;
use crate::validators::{ValidationResult, Validator};
use anyhow::Result;
use std::path::Path;

pub struct FileValidator {
    check: FileCheck,
}

impl FileValidator {
    pub fn new(check: FileCheck) -> Self {
        Self { check }
    }
}

impl Validator for FileValidator {
    fn validate(&self) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();
        let path = Path::new(&self.check.path);

        if path.exists() {
            if self.check.is_directory {
                if path.is_dir() {
                    results.push(ValidationResult::success(
                        format!("Directory {} exists", self.check.path),
                    ));
                } else {
                    results.push(ValidationResult::error(
                        format!("{} exists but is not a directory", self.check.path),
                        Some(format!("Ensure {} is a directory", self.check.path)),
                    ));
                }
            } else {
                results.push(ValidationResult::success(
                    format!("{} exists", self.check.path),
                ));
            }
        } else {
            let item_type = if self.check.is_directory { "Directory" } else { "File" };
            if self.check.required {
                results.push(ValidationResult::error(
                    format!("{} {} does not exist", item_type, self.check.path),
                    Some(format!("Create {} {}", self.check.path, item_type.to_lowercase())),
                ));
            } else {
                results.push(ValidationResult::warning(
                    format!("{} {} does not exist (optional)", item_type, self.check.path),
                    None,
                ));
            }
        }

        Ok(results)
    }
}
