use crate::config::Config;
use anyhow::Result;
use serde::Serialize;

pub mod tool;
pub mod env;
pub mod port;
pub mod file;
pub mod network;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub message: String,
    pub suggestion: Option<String>,
}

impl ValidationResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            status: ValidationStatus::Success,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn warning(message: impl Into<String>, suggestion: Option<String>) -> Self {
        Self {
            status: ValidationStatus::Warning,
            message: message.into(),
            suggestion,
        }
    }

    pub fn error(message: impl Into<String>, suggestion: Option<String>) -> Self {
        Self {
            status: ValidationStatus::Error,
            message: message.into(),
            suggestion,
        }
    }
}

pub trait Validator {
    fn validate(&self) -> Result<Vec<ValidationResult>>;
}

pub fn run_all_validations(config: &Config) -> Result<Vec<ValidationResult>> {
    let mut results = Vec::new();

    // Validate tools
    for tool_check in &config.tools {
        let validator = tool::ToolValidator::new(tool_check.clone());
        results.extend(validator.validate()?);
    }

    // Validate environment variables
    for env_check in &config.env_vars {
        let validator = env::EnvValidator::new(env_check.clone());
        results.extend(validator.validate()?);
    }

    // Validate ports
    for &port in &config.ports {
        let validator = port::PortValidator::new(port);
        results.extend(validator.validate()?);
    }

    // Validate files
    for file_check in &config.files {
        let validator = file::FileValidator::new(file_check.clone());
        results.extend(validator.validate()?);
    }

    // Validate network
    for network_check in &config.network {
        let validator = network::NetworkValidator::new(network_check.clone());
        results.extend(validator.validate()?);
    }

    Ok(results)
}
