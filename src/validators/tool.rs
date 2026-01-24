use crate::config::ToolCheck;
use crate::validators::{ValidationResult, Validator};
use anyhow::Result;
use std::process::Command;

pub struct ToolValidator {
    check: ToolCheck,
}

impl ToolValidator {
    pub fn new(check: ToolCheck) -> Self {
        Self { check }
    }

    fn get_version_command<'a>(&self, tool: &'a str) -> Option<(&'a str, Vec<&'static str>)> {
        match tool {
            "node" => Some(("node", vec!["--version"])),
            "npm" => Some(("npm", vec!["--version"])),
            "go" => Some(("go", vec!["version"])),
            "rust" | "rustc" => Some(("rustc", vec!["--version"])),
            "cargo" => Some(("cargo", vec!["--version"])),
            "python" | "python3" => Some(("python3", vec!["--version"])),
            "docker" => Some(("docker", vec!["--version"])),
            "git" => Some(("git", vec!["--version"])),
            "java" => Some(("java", vec!["--version"])),
            "ruby" => Some(("ruby", vec!["--version"])),
            _ => Some((tool, vec!["--version"])),
        }
    }

    fn parse_version(&self, output: &str, _tool: &str) -> Option<String> {
        let output = output.trim();
        
        // Simple version extraction - find first occurrence of X.Y.Z or X.Y pattern
        for word in output.split_whitespace() {
            let cleaned = word.trim_start_matches('v').trim_start_matches('V');
            
            // Check if it looks like a version number
            let parts: Vec<&str> = cleaned.split('.').collect();
            if parts.len() >= 2 && parts.iter().all(|p| p.chars().all(|c| c.is_numeric())) {
                return Some(cleaned.to_string());
            }
        }

        // Fallback: just return the first line
        output.lines().next().map(|s| s.to_string())
    }

    fn check_version_requirement(&self, version: &str, requirement: &str) -> bool {
        // Simple version comparison
        // For now, just check if version contains the requirement
        // TODO: Use proper semver comparison
        if requirement.starts_with(">=") {
            let req_ver = requirement.trim_start_matches(">=").trim();
            return version >= req_ver;
        }
        if requirement.starts_with("<=") {
            let req_ver = requirement.trim_start_matches("<=").trim();
            return version <= req_ver;
        }
        if requirement.starts_with('>') {
            let req_ver = requirement.trim_start_matches('>').trim();
            return version > req_ver;
        }
        if requirement.starts_with('<') {
            let req_ver = requirement.trim_start_matches('<').trim();
            return version < req_ver;
        }
        if requirement.starts_with('=') {
            let req_ver = requirement.trim_start_matches('=').trim();
            return version == req_ver;
        }
        
        // Default: exact match
        version.contains(requirement)
    }
}

impl Validator for ToolValidator {
    fn validate(&self) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();

        // Check if tool exists
        if which::which(&self.check.name).is_err() {
            if self.check.required {
                results.push(ValidationResult::error(
                    format!("{} not found", self.check.name),
                    Some(format!("Install {} to continue", self.check.name)),
                ));
            } else {
                results.push(ValidationResult::warning(
                    format!("{} not found (optional)", self.check.name),
                    None,
                ));
            }
            return Ok(results);
        }

        // If version check is required
        if let Some(version_req) = &self.check.version {
            if let Some((cmd, args)) = self.get_version_command(&self.check.name) {
                match Command::new(cmd).args(&args).output() {
                    Ok(output) => {
                        let version_output = String::from_utf8_lossy(&output.stdout);
                        if let Some(version) = self.parse_version(&version_output, &self.check.name) {
                            if self.check_version_requirement(&version, version_req) {
                                results.push(ValidationResult::success(
                                    format!("{} {} found", self.check.name, version),
                                ));
                            } else {
                                results.push(ValidationResult::error(
                                    format!("{} version {} does not meet requirement {}", 
                                        self.check.name, version, version_req),
                                    Some(format!("Update {} to version {}", self.check.name, version_req)),
                                ));
                            }
                        } else {
                            results.push(ValidationResult::warning(
                                format!("{} found but version could not be determined", self.check.name),
                                None,
                            ));
                        }
                    }
                    Err(_) => {
                        results.push(ValidationResult::warning(
                            format!("{} found but version check failed", self.check.name),
                            None,
                        ));
                    }
                }
            }
        } else {
            results.push(ValidationResult::success(
                format!("{} found", self.check.name),
            ));
        }

        Ok(results)
    }
}
