use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProconError {
    #[error("Project '{0}' already exists")]
    ProjectExists(String),
    
    #[error("Project directory not found")]
    ProjectNotFound,
    
    #[error("Template '{0}' not found")]
    TemplateNotFound(String),
    
    #[error("Failed to create project: {0}")]
    ProjectCreationFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
    
    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, ProconError>;