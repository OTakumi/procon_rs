use crate::error::{ProconError, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// Embedded template content
const DEFAULT_MAIN_CPP: &str = include_str!("../templates/default/main.cpp");
const DEFAULT_CMAKE: &str = include_str!("../templates/default/CMakeLists.txt");

#[derive(Debug, Clone)]
pub struct Template {
    pub files: HashMap<String, String>,
}

pub struct TemplateLoader;

impl TemplateLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn find_template(&self, name: &str) -> Result<PathBuf> {
        // First, check user config directory
        if let Some(config_dir) = dirs::config_dir() {
            let user_template_path = config_dir
                .join("procon_rs")
                .join("templates")
                .join(name);
            
            if user_template_path.exists() {
                return Ok(user_template_path);
            }
        }
        
        // For builtin templates, we need to return an error since they don't exist
        // The caller should handle the fallback
        Err(ProconError::TemplateNotFound(name.to_string()))
    }
}

impl Template {
    /// Creates a template from embedded content strings.
    /// 
    /// This method allows creating templates from compile-time embedded strings,
    /// enabling built-in templates to be included in the binary without requiring
    /// external template files. This ensures the application works out-of-the-box
    /// even when user-specific template directories don't exist.
    /// 
    /// # Arguments
    /// 
    /// * `_name` - The template name (currently unused but kept for future extensibility)
    /// * `main_cpp_content` - Content for the main.cpp file
    /// * `cmake_content` - Content for the CMakeLists.txt file
    /// 
    /// # Returns
    /// 
    /// * `Template` - A template instance with the embedded content
    /// 
    /// # Examples
    /// 
    /// ```
    /// use procon_rs::template::Template;
    /// 
    /// let main_cpp = "#include <iostream>\nint main() { return 0; }";
    /// let cmake = "project({{PROJECT_NAME}})";
    /// let template = Template::from_embedded_content("default", main_cpp, cmake);
    /// ```
    pub fn from_embedded_content(_name: &str, main_cpp_content: &str, cmake_content: &str) -> Self {
        let mut files = HashMap::new();
        files.insert("main.cpp".to_string(), main_cpp_content.to_string());
        files.insert("CMakeLists.txt".to_string(), cmake_content.to_string());
        
        Self { files }
    }

    /// Creates a template from built-in embedded templates.
    /// 
    /// This method provides access to templates that are compiled into the binary,
    /// ensuring the application can create projects even when external template
    /// files are not available. Built-in templates serve as fallbacks when user
    /// templates are not found.
    /// 
    /// # Arguments
    /// 
    /// * `template_name` - The name of the built-in template to load
    /// 
    /// # Returns
    /// 
    /// * `Ok(Template)` - Successfully loaded built-in template
    /// * `Err(ProconError)` - Template name is not a recognized built-in template
    /// 
    /// # Supported Templates
    /// 
    /// * `"default"` - Basic C++ competitive programming template
    /// 
    /// # Examples
    /// 
    /// ```
    /// use procon_rs::template::Template;
    /// 
    /// let template = Template::from_builtin("default").unwrap();
    /// ```
    pub fn from_builtin(template_name: &str) -> Result<Self> {
        match template_name {
            "default" => Ok(Self::from_embedded_content("default", DEFAULT_MAIN_CPP, DEFAULT_CMAKE)),
            _ => Err(ProconError::TemplateNotFound(template_name.to_string()))
        }
    }

    /// Loads a template from the specified directory path with dynamic file detection.
    /// 
    /// This method implements a comprehensive template loading system that:
    /// 1. Validates that required files (main.cpp, CMakeLists.txt) are present
    /// 2. Dynamically discovers and loads all additional files in the template directory
    /// 3. Recursively processes subdirectories to maintain project structure
    /// 4. Preserves relative paths for proper project hierarchy recreation
    /// 
    /// The dynamic detection allows templates to include any additional files without
    /// requiring explicit configuration, making the template system flexible and extensible.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The filesystem path to the template directory
    /// 
    /// # Returns
    /// 
    /// * `Ok(Template)` - Successfully loaded template with all discovered files
    /// * `Err(ProconError)` - Template loading failed due to missing required files or IO errors
    /// 
    /// # Errors
    /// 
    /// * `TemplateNotFound` - Required files (main.cpp, CMakeLists.txt) are missing
    /// * `Io` - Filesystem errors during directory traversal or file reading
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::path::Path;
    /// use procon_rs::template::Template;
    /// 
    /// let template = Template::load_from_path(Path::new("templates/default")).unwrap();
    /// // Template now contains all files from the directory, including subdirectories
    /// ```
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let mut files = HashMap::new();

        // Validate and load required files first
        let required_files = ["main.cpp", "CMakeLists.txt"];
        for &file_name in &required_files {
            let file_path = path.join(file_name);
            if !file_path.exists() {
                return Err(ProconError::TemplateNotFound(
                    format!("{} not found in template", file_name),
                ));
            }
            files.insert(file_name.to_string(), fs::read_to_string(&file_path)?);
        }

        // Dynamically discover and load all other files in the template directory
        Self::load_directory_recursively(path, "", &mut files)?;

        Ok(Self { files })
    }

    /// Recursively loads all files from a directory and its subdirectories.
    /// 
    /// This private helper method implements the core dynamic file detection logic:
    /// - Traverses the directory tree recursively
    /// - Maintains relative path structure using path prefixes
    /// - Skips required files that are already loaded to avoid duplication
    /// - Handles both files and subdirectories appropriately
    /// - Preserves the original directory hierarchy for accurate project recreation
    /// 
    /// The recursive approach ensures that complex template structures with nested
    /// directories (like lib/, include/, src/, tests/) are fully captured while
    /// maintaining their relative relationships.
    /// 
    /// # Arguments
    /// 
    /// * `dir` - The directory to scan for files
    /// * `prefix` - The relative path prefix for files in this directory (empty for root)
    /// * `files` - Mutable reference to the HashMap where discovered files are stored
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - Directory successfully processed
    /// * `Err(ProconError)` - IO error during directory traversal or file reading
    /// 
    /// # Path Handling
    /// 
    /// The method constructs relative paths by combining the prefix with the file name:
    /// - Root level files: "filename.ext"
    /// - Nested files: "subdir/filename.ext" 
    /// - Deeply nested: "dir1/dir2/filename.ext"
    /// 
    /// This ensures that when the template is later copied to a destination,
    /// the directory structure is accurately recreated.
    fn load_directory_recursively(
        dir: &Path,
        prefix: &str,
        files: &mut HashMap<String, String>,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry
                .file_name()
                .to_string_lossy()
                .into_owned();

            // Construct the relative path for this file/directory
            let relative_path = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", prefix, name)
            };

            if path.is_dir() {
                // Recursively process subdirectories to maintain hierarchy
                Self::load_directory_recursively(&path, &relative_path, files)?;
            } else if path.is_file() {
                // Skip required files that are already loaded to prevent duplication
                let required_files = ["main.cpp", "CMakeLists.txt"];
                if prefix.is_empty() && required_files.contains(&name.as_str()) {
                    continue;
                }

                // Load file content and store with relative path as key
                if let Ok(content) = fs::read_to_string(&path) {
                    files.insert(relative_path, content);
                }
                // Note: We silently skip files that cannot be read (e.g., binary files)
                // This allows templates to include various file types without breaking
            }
        }
        Ok(())
    }

    pub fn apply_variables(&self, project_name: &str) -> Self {
        let mut processed_files = HashMap::new();

        for (filename, content) in &self.files {
            let processed_content = content.replace("{{PROJECT_NAME}}", project_name);
            processed_files.insert(filename.clone(), processed_content);
        }

        Self {
            files: processed_files,
        }
    }

    /// Copies all template files to the specified destination directory with full directory structure.
    /// 
    /// This method recreates the complete template structure in the destination:
    /// 1. Creates the destination directory if it doesn't exist
    /// 2. Processes all template files, including those in subdirectories
    /// 3. Automatically creates necessary subdirectories to maintain hierarchy
    /// 4. Writes file contents to their appropriate locations
    /// 
    /// The method handles complex directory structures by parsing the relative paths
    /// stored in the template files HashMap and creating intermediate directories
    /// as needed. This ensures that templates with nested structures (lib/, include/,
    /// src/, tests/) are correctly recreated in the destination.
    /// 
    /// # Arguments
    /// 
    /// * `dest_dir` - The destination directory where the template should be instantiated
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - Template successfully copied to destination
    /// * `Err(ProconError)` - IO error during directory creation or file writing
    /// 
    /// # Errors
    /// 
    /// * `Io` - Filesystem errors such as permission issues, disk space, or invalid paths
    /// 
    /// # Directory Structure Handling
    /// 
    /// The method automatically creates subdirectories based on file paths:
    /// - "main.cpp" → `dest_dir/main.cpp`
    /// - "lib/utils.hpp" → `dest_dir/lib/utils.hpp` (creates `lib/` directory)
    /// - "src/helpers/math.cpp" → `dest_dir/src/helpers/math.cpp` (creates `src/helpers/`)
    /// 
    /// # Examples
    /// 
    /// ```no_run
    /// use std::path::Path;
    /// use procon_rs::template::Template;
    /// 
    /// let template = Template::load_from_path(Path::new("templates/advanced")).unwrap();
    /// template.copy_to(Path::new("my_project")).unwrap();
    /// // Creates my_project/ with full directory structure from template
    /// ```
    pub fn copy_to(&self, dest_dir: &Path) -> Result<()> {
        // Ensure the destination directory exists
        fs::create_dir_all(dest_dir)?;

        for (relative_path, content) in &self.files {
            let dest_file = dest_dir.join(relative_path);
            
            // Create parent directories if the file is in a subdirectory
            if let Some(parent_dir) = dest_file.parent() {
                fs::create_dir_all(parent_dir)?;
            }
            
            // Write the file content to the destination
            fs::write(&dest_file, content)?;
        }

        Ok(())
    }
}

