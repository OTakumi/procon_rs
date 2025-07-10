use crate::config::Config;
use crate::error::{ProconError, Result};
use crate::template::{Template, TemplateLoader};
use std::fs;
use std::path::PathBuf;

pub struct NewCommandArgs {
    pub name: String,
    pub template: String,
    pub path: Option<PathBuf>,
}

pub struct NewCommand;

impl NewCommand {
    pub fn execute(args: NewCommandArgs) -> Result<()> {
        let config = Config::load().unwrap_or_default();

        // Determine project path
        let project_path = match args.path {
            Some(base_path) => base_path.join(&args.name),
            None => std::env::current_dir()?.join(&args.name),
        };

        // Check if project already exists
        if project_path.exists() {
            return Err(ProconError::ProjectExists(args.name));
        }

        // Load template
        let template = Self::load_template(&args.template, &config)?;

        // Process template with variables
        let processed_template = Self::process_template_variables(template, &args.name, &config);

        // Create project directory and copy files
        fs::create_dir_all(&project_path)?;
        processed_template.copy_to(&project_path)?;

        Ok(())
    }

    fn load_template(template_name: &str, _config: &Config) -> Result<Template> {
        let loader = TemplateLoader::new();
        
        // Try to find user template first
        match loader.find_template(template_name) {
            Ok(template_path) => {
                Template::load_from_path(&template_path)
            }
            Err(_) => {
                // Template not found in user directory, try builtin templates
                
                // Check if this is a builtin template
                let builtin_templates = vec!["default", "advanced"];
                if builtin_templates.contains(&template_name) {
                    // Try cargo manifest dir for development
                    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
                        let dev_template_path = PathBuf::from(manifest_dir)
                            .join("templates")
                            .join(template_name);
                        
                        if dev_template_path.exists() {
                            return Template::load_from_path(&dev_template_path);
                        }
                    }
                    
                    // If CARGO_MANIFEST_DIR doesn't work, suggest user to create the template
                    return Err(ProconError::TemplateNotFoundWithHint(template_name.to_string()));
                }
                
                Err(ProconError::TemplateNotFound(template_name.to_string()))
            }
        }
    }

    fn process_template_variables(
        template: Template,
        project_name: &str,
        config: &Config,
    ) -> Template {
        let mut files = std::collections::HashMap::new();

        for (filename, content) in template.files {
            let processed_content = content
                .replace("{{PROJECT_NAME}}", project_name)
                .replace("{{CMAKE_VERSION}}", &config.project.cmake_minimum_version)
                .replace("{{CPP_STANDARD}}", &config.project.cpp_standard);
            files.insert(filename, processed_content);
        }

        Template { files }
    }
}
