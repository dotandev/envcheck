use crate::config::NetworkCheck;
use crate::validators::{ValidationResult, Validator};
use anyhow::Result;
use reqwest::blocking::Client;
use std::time::Duration;

pub struct NetworkValidator {
    check: NetworkCheck,
    client: Client,
}

impl NetworkValidator {
    pub fn new(check: NetworkCheck) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| Client::new());
            
        Self { check, client }
    }
}

impl Validator for NetworkValidator {
    fn validate(&self) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();
        
        let url = &self.check.url;
        let expected_status = self.check.status_code.unwrap_or(200);
        
        match self.client.get(url).send() {
            Ok(response) => {
                let status = response.status().as_u16();
                
                if status == expected_status {
                    results.push(ValidationResult::success(format!(
                        "Successfully connected to {} (Status: {})", 
                        url, status
                    )));
                } else {
                    results.push(ValidationResult::error(
                        format!("Connected to {} but got status {}", url, status),
                        Some(format!("Expected status code {}", expected_status)),
                    ));
                }
            }
            Err(e) => {
                results.push(ValidationResult::error(
                    format!("Failed to connect to {}", url),
                    Some(format!("Error: {}", e)),
                ));
            }
        }

        Ok(results)
    }
}
