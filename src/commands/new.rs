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
        // For built-in templates, load from the templates directory
        if template_name == "default" {
            // Try to find the template in the project root
            let mut template_path = std::env::current_dir()?;
            template_path.push("templates");
            template_path.push(template_name);

            // If not found in current dir, try cargo manifest dir
            if !template_path.exists() {
                if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
                    template_path = PathBuf::from(manifest_dir);
                    template_path.push("templates");
                    template_path.push(template_name);
                }
            }

            Template::load_from_path(&template_path)
        } else {
            let loader = TemplateLoader::new();
            let _template_path = loader.find_template(template_name)?;
            // For now, only support default template
            Err(ProconError::TemplateNotFound(template_name.to_string()))
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
