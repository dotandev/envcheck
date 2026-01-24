use crate::validators::{ValidationResult, Validator};
use anyhow::Result;
use std::net::TcpListener;

pub struct PortValidator {
    port: u16,
}

impl PortValidator {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

impl Validator for PortValidator {
    fn validate(&self) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();

        match TcpListener::bind(format!("127.0.0.1:{}", self.port)) {
            Ok(_) => {
                results.push(ValidationResult::success(format!(
                    "Port {} is available",
                    self.port
                )));
            }
            Err(_) => {
                results.push(ValidationResult::error(
                    format!("Port {} is already in use", self.port),
                    Some(format!(
                        "Free up port {} or change the port in your config",
                        self.port
                    )),
                ));
            }
        }

        Ok(results)
    }
}
