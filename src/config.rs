use crate::error::{ProconError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub template: TemplateConfig,
    pub project: ProjectConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub default: String,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub cpp_standard: String,
    pub cmake_minimum_version: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            template: TemplateConfig {
                default: "default".to_string(),
                path: dirs::config_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("procon_rs")
                    .join("templates"),
            },
            project: ProjectConfig {
                cpp_standard: "17".to_string(),
                cmake_minimum_version: "3.16".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // For now, just return default config
        // In a real implementation, this would load from config file
        Ok(Config::default())
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "template.default" => Some(self.template.default.clone()),
            "template.path" => Some(self.template.path.display().to_string()),
            "project.cpp_standard" => Some(self.project.cpp_standard.clone()),
            "project.cmake_minimum_version" => Some(self.project.cmake_minimum_version.clone()),
            _ => None,
        }
    }
    
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "template.default" => self.template.default = value.to_string(),
            "template.path" => self.template.path = PathBuf::from(value),
            "project.cpp_standard" => self.project.cpp_standard = value.to_string(),
            "project.cmake_minimum_version" => self.project.cmake_minimum_version = value.to_string(),
            _ => return Err(ProconError::ConfigError(format!("Unknown configuration key: {}", key))),
        }
        Ok(())
    }
}