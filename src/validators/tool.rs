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
            "python2" => Some(("python2", vec!["--version"])),
            "docker" => Some(("docker", vec!["--version"])),
            "git" => Some(("git", vec!["--version"])),
            "java" => Some(("java", vec!["-version"])),
            "ruby" => Some(("ruby", vec!["--version"])),
            _ => Some((tool, vec!["--version"])),
        }
    }

    fn parse_version(&self, output: &str, _tool: &str) -> Option<String> {
        let output = output.trim();
        
        // Look for the first thing that looks like a version in the output
        // Handle common formats like "node v14.15.0", "openjdk version \"25.0.1\"", "ruby 2.6.10p210"
        
        // Clean the output by removing common prefixes and quotes
        let cleaned_output = output.replace('"', " ");
        
        for line in cleaned_output.lines() {
            for word in line.split_whitespace() {
                // Find the first position of a digit
                if let Some(digit_pos) = word.find(|c: char| c.is_ascii_digit()) {
                    let potential_version = &word[digit_pos..];
                    
                    // Extract only the leading numeric and dot parts
                    let mut version_part = String::new();
                    let mut dot_count = 0;
                    
                    for c in potential_version.chars() {
                        if c.is_ascii_digit() {
                            version_part.push(c);
                        } else if c == '.' {
                            version_part.push(c);
                            dot_count += 1;
                        } else {
                            // Stop at first non-numeric/non-dot character
                            break;
                        }
                    }
                    
                    // We need at least X.Y format
                    if dot_count >= 1 && version_part.len() >= 3 {
                        return Some(version_part);
                    }
                }
            }
        }

        // Fallback: just return the first line if nothing else works
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

        // Determine binaries to check
        let binaries = match self.check.name.as_str() {
            "python" => vec!["python3", "python"],
            name => vec![name],
        };

        let bin_found = binaries.iter().any(|b| which::which(b).is_ok());

        if !bin_found {
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
            let tool_names = match self.check.name.as_str() {
                "python" => vec!["python3", "python"],
                name => vec![name],
            };

            let mut last_error = None;
            let mut detected_version = None;

            for tool_name in tool_names {
                if let Some((cmd, args)) = self.get_version_command(tool_name) {
                    // Check if binary exists before running
                    if which::which(cmd).is_err() {
                        continue;
                    }

                    match Command::new(cmd).args(&args).output() {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            let version_output = if stdout.is_empty() { stderr } else { stdout };
                            
                            if let Some(version) = self.parse_version(&version_output, tool_name) {
                                detected_version = Some((version, tool_name.to_string()));
                                break;
                            }
                        }
                        Err(e) => {
                            last_error = Some(e.to_string());
                        }
                    }
                }
            }

            if let Some((version, tool_name)) = detected_version {
                if self.check_version_requirement(&version, version_req) {
                    results.push(ValidationResult::success(
                        format!("{} ({}) {} found", self.check.name, tool_name, version),
                    ));
                } else {
                    results.push(ValidationResult::error(
                        format!("{} ({}) version {} does not meet requirement {}", 
                            self.check.name, tool_name, version, version_req),
                        Some(format!("Update {} to version {}", self.check.name, version_req)),
                    ));
                }
            } else {
                let err_msg = last_error.unwrap_or_else(|| "Version could not be determined".to_string());
                results.push(ValidationResult::warning(
                    format!("{} found but {}", self.check.name, err_msg),
                    None,
                ));
            }
        } else {
            results.push(ValidationResult::success(
                format!("{} found", self.check.name),
            ));
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ToolCheck;

    #[test]
    fn test_parse_version() {
        let check = ToolCheck {
            name: "test".to_string(),
            version: None,
            required: true,
        };
        let validator = ToolValidator::new(check);

        // Node.js
        assert_eq!(validator.parse_version("v14.15.0", "node"), Some("14.15.0".to_string()));
        
        // Go
        assert_eq!(validator.parse_version("go version go1.16.3 darwin/amd64", "go"), Some("1.16.3".to_string()));
        
        // Python
        assert_eq!(validator.parse_version("Python 3.9.1", "python"), Some("3.9.1".to_string()));
        
        // OpenJDK
        let openjdk_output = r#"openjdk version "25.0.1" 2025-10-21 LTS
OpenJDK Runtime Environment Temurin-25.0.1+8 (build 25.0.1+8-LTS)
OpenJDK 64-Bit Server VM Temurin-25.0.1+8 (build 25.0.1+8-LTS, mixed mode, sharing)"#;
        assert_eq!(validator.parse_version(openjdk_output, "java"), Some("25.0.1".to_string()));
        
        // Ruby
        assert_eq!(validator.parse_version("ruby 2.6.10p210 (2022-04-12 revision 67958) [universal.arm64e-darwin25]", "ruby"), Some("2.6.10".to_string()));

        // Rust
        assert_eq!(validator.parse_version("rustc 1.51.0 (2fd73fabe 2021-03-23)", "rust"), Some("1.51.0".to_string()));
    }
}
