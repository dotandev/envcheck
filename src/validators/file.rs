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
            results.push(ValidationResult::success(
                format!("{} exists", self.check.path),
            ));
        } else {
            if self.check.required {
                results.push(ValidationResult::error(
                    format!("{} does not exist", self.check.path),
                    Some(format!("Create {} file", self.check.path)),
                ));
            } else {
                results.push(ValidationResult::warning(
                    format!("{} does not exist (optional)", self.check.path),
                    None,
                ));
            }
        }

        Ok(results)
    }
}
