use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub version: String,
    #[serde(default)]
    pub tools: Vec<ToolCheck>,
    #[serde(default)]
    pub env_vars: Vec<EnvVarCheck>,
    #[serde(default)]
    pub ports: Vec<u16>,
    #[serde(default)]
    pub files: Vec<FileCheck>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolCheck {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default = "default_true")]
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EnvVarCheck {
    pub name: String,
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default)]
    pub pattern: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileCheck {
    pub path: String,
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default = "default_false")]
    pub is_directory: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read config file")?;
        
        let config: Config = serde_yaml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }

    pub fn find_config() -> Result<Self> {
        let config_names = [".envcheck.yaml", ".envcheck.yml", "envcheck.yaml", "envcheck.yml"];
        
        for name in &config_names {
            if Path::new(name).exists() {
                return Self::load(name);
            }
        }
        
        anyhow::bail!("No config file found. Looking for: {}", config_names.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let yaml = r#"
version: "1"
tools:
  - name: node
    version: ">=18.0.0"
    required: true
env_vars:
  - name: DATABASE_URL
    required: true
ports:
  - 3000
files:
  - path: .env
    required: true
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.version, "1");
        assert_eq!(config.tools.len(), 1);
        assert_eq!(config.tools[0].name, "node");
    }
}
