use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "procon_rs")]
#[command(about = "A CLI tool for creating C++ competitive programming projects")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: String,
        
        /// Template to use
        #[arg(short, long, default_value = "default")]
        template: String,
        
        /// Directory to create the project in
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    
    /// Initialize existing directory
    Init {
        /// Force overwrite existing files
        #[arg(long)]
        force: bool,
    },
    
    /// Manage configuration
    Config {
        /// Configuration key
        key: String,
        
        /// Configuration value (if not provided, shows current value)
        value: Option<String>,
    },
}