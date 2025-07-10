#[cfg(test)]
mod template_tests {
    use procon_rs::template::{Template, TemplateLoader};
    use std::fs;
    use tempfile::TempDir;

    /// Tests that TemplateLoader can find built-in templates by name.
    /// 
    /// This verifies that the template system can locate default templates
    /// that ship with the application, ensuring users can create projects
    /// using standard templates without additional setup.
    #[test]
    fn test_template_loader_find_builtin_template() {
        // Arrange: Create a template loader instance
        let loader = TemplateLoader::new();

        // Act: Attempt to find the default built-in template
        let template_path = loader.find_template("default");

        // Assert: Verify the template can be found
        assert!(template_path.is_ok());
    }

    /// Tests that TemplateLoader returns an error for non-existent templates.
    /// 
    /// This ensures that attempts to use invalid template names fail gracefully
    /// with descriptive error messages, helping users identify typos or missing
    /// custom templates.
    #[test]
    fn test_template_loader_find_nonexistent_template() {
        // Arrange: Create a template loader instance
        let loader = TemplateLoader::new();

        // Act: Attempt to find a template that doesn't exist
        let result = loader.find_template("nonexistent");

        // Assert: Verify the operation returns an error
        assert!(result.is_err());

        // Assert: Verify the error message is descriptive
        if let Err(e) = result {
            assert!(e.to_string().contains("Template 'nonexistent' not found"));
        }
    }

    /// Tests that Template can load required files from a directory path.
    /// 
    /// This verifies that the template loading system can read template files
    /// from the filesystem and correctly populate the template structure with
    /// all necessary files for C++ project generation.
    #[test]
    fn test_template_load_from_path() {
        // Arrange: Create a temporary directory with template files
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("test_template");
        fs::create_dir_all(&template_dir).unwrap();

        let main_cpp = template_dir.join("main.cpp");
        fs::write(&main_cpp, "#include <iostream>\nint main() { return 0; }").unwrap();

        let cmake_file = template_dir.join("CMakeLists.txt");
        fs::write(&cmake_file, "cmake_minimum_required(VERSION {{CMAKE_VERSION}})").unwrap();

        // Act: Load the template from the directory
        let template = Template::load_from_path(&template_dir).unwrap();

        // Assert: Verify all required files are loaded
        assert!(template.files.contains_key("main.cpp"));
        assert!(template.files.contains_key("CMakeLists.txt"));
    }

    /// Tests that Template::apply_variables() correctly substitutes template variables.
    /// 
    /// This verifies that the template variable substitution system works correctly,
    /// replacing placeholder variables like {{PROJECT_NAME}} with actual values
    /// to generate personalized project files.
    #[test]
    fn test_template_variable_substitution() {
        // Arrange: Create a template with variable placeholders
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("test_template");
        fs::create_dir_all(&template_dir).unwrap();

        let main_cpp = template_dir.join("main.cpp");
        fs::write(&main_cpp, "// Project: {{PROJECT_NAME}}\nint main() { return 0; }").unwrap();

        let cmake_file = template_dir.join("CMakeLists.txt");
        fs::write(&cmake_file, "project({{PROJECT_NAME}})").unwrap();

        let template = Template::load_from_path(&template_dir).unwrap();

        // Act: Apply variable substitution to the template
        let processed = template.apply_variables("test_project");

        // Assert: Verify the variables were correctly substituted
        assert!(processed.files["main.cpp"].contains("// Project: test_project"));
    }

    /// Tests that Template::load_from_path() validates required files are present.
    /// 
    /// This ensures that incomplete templates (missing required files like
    /// CMakeLists.txt) are rejected with clear error messages, preventing
    /// users from creating broken project structures.
    #[test]
    fn test_template_required_files() {
        // Arrange: Create an incomplete template directory (missing CMakeLists.txt)
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("incomplete_template");
        fs::create_dir_all(&template_dir).unwrap();

        let main_cpp = template_dir.join("main.cpp");
        fs::write(&main_cpp, "int main() { return 0; }").unwrap();
        // Note: Intentionally not creating CMakeLists.txt

        // Act: Attempt to load the incomplete template
        let result = Template::load_from_path(&template_dir);

        // Assert: Verify the operation fails with appropriate error
        assert!(result.is_err());

        // Assert: Verify the error mentions the missing file
        if let Err(e) = result {
            assert!(e.to_string().contains("CMakeLists.txt"));
        }
    }

    /// Tests that Template::copy_to() creates all template files in the destination directory.
    /// 
    /// This verifies the complete template instantiation process: loading a template,
    /// applying variable substitution, and copying the processed files to a new
    /// project directory with correct content.
    #[test]
    fn test_template_copy_to_destination() {
        // Arrange: Create a source template with variable placeholders
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("template");
        fs::create_dir_all(&template_dir).unwrap();

        let main_cpp = template_dir.join("main.cpp");
        fs::write(&main_cpp, "// {{PROJECT_NAME}}\nint main() { return 0; }").unwrap();

        let cmake_file = template_dir.join("CMakeLists.txt");
        fs::write(&cmake_file, "project({{PROJECT_NAME}})").unwrap();

        // Arrange: Load and process the template
        let template = Template::load_from_path(&template_dir).unwrap();
        let processed = template.apply_variables("my_project");

        // Act: Copy the processed template to a destination directory
        let dest_dir = temp_dir.path().join("output");
        processed.copy_to(&dest_dir).unwrap();

        // Assert: Verify all files were created in the destination
        let dest_main = dest_dir.join("main.cpp");
        let dest_cmake = dest_dir.join("CMakeLists.txt");
        assert!(dest_main.exists());
        assert!(dest_cmake.exists());

        // Assert: Verify the file contents have processed variables
        let main_content = fs::read_to_string(&dest_main).unwrap();
        let cmake_content = fs::read_to_string(&dest_cmake).unwrap();
        assert!(main_content.contains("// my_project"));
        assert!(cmake_content.contains("project(my_project)"));
    }

    /// Tests that Template::load_from_path() automatically detects and loads additional files.
    /// 
    /// This verifies the dynamic file detection capability, ensuring that templates
    /// can include additional files beyond the required main.cpp and CMakeLists.txt,
    /// such as README.md, configuration files, and additional source files.
    #[test]
    fn test_template_dynamic_file_detection() {
        // Arrange: Create a template with additional files
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("rich_template");
        fs::create_dir_all(&template_dir).unwrap();

        // Arrange: Create required files
        let main_cpp = template_dir.join("main.cpp");
        fs::write(&main_cpp, "#include <iostream>\nint main() { return 0; }").unwrap();

        let cmake_file = template_dir.join("CMakeLists.txt");
        fs::write(&cmake_file, "project({{PROJECT_NAME}})").unwrap();

        // Arrange: Create additional files that should be auto-detected
        let readme_file = template_dir.join("README.md");
        fs::write(&readme_file, "# {{PROJECT_NAME}}\n\nProject description").unwrap();

        let config_file = template_dir.join("config.json");
        fs::write(&config_file, r#"{"project": "{{PROJECT_NAME}}"}"#).unwrap();

        let makefile = template_dir.join("Makefile");
        fs::write(&makefile, "all:\n\techo Building {{PROJECT_NAME}}").unwrap();

        // Act: Load the template with dynamic file detection
        let template = Template::load_from_path(&template_dir).unwrap();

        // Assert: Verify all files were detected and loaded
        assert!(template.files.contains_key("main.cpp"));
        assert!(template.files.contains_key("CMakeLists.txt"));
        assert!(template.files.contains_key("README.md"));
        assert!(template.files.contains_key("config.json"));
        assert!(template.files.contains_key("Makefile"));

        // Assert: Verify file contents are correct
        assert!(template.files["README.md"].contains("{{PROJECT_NAME}}"));
        assert!(template.files["config.json"].contains("{{PROJECT_NAME}}"));
        assert!(template.files["Makefile"].contains("{{PROJECT_NAME}}"));
    }

    /// Tests that Template can handle subdirectories and nested file structures.
    /// 
    /// This verifies that the template system can recursively process subdirectories,
    /// maintaining the directory structure and relative paths when creating projects
    /// with complex hierarchies like lib/, include/, src/, tests/.
    #[test]
    fn test_template_subdirectory_handling() {
        // Arrange: Create a template with subdirectories
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("complex_template");
        fs::create_dir_all(&template_dir).unwrap();

        // Arrange: Create required files
        let main_cpp = template_dir.join("main.cpp");
        fs::write(&main_cpp, "#include \"lib/utils.hpp\"\nint main() { return 0; }").unwrap();

        let cmake_file = template_dir.join("CMakeLists.txt");
        fs::write(&cmake_file, "project({{PROJECT_NAME}})").unwrap();

        // Arrange: Create subdirectories with files
        let lib_dir = template_dir.join("lib");
        fs::create_dir_all(&lib_dir).unwrap();
        let utils_hpp = lib_dir.join("utils.hpp");
        fs::write(&utils_hpp, "// {{PROJECT_NAME}} utilities\n#pragma once").unwrap();

        let include_dir = template_dir.join("include");
        fs::create_dir_all(&include_dir).unwrap();
        let header_h = include_dir.join("header.h");
        fs::write(&header_h, "// Header for {{PROJECT_NAME}}").unwrap();

        let tests_dir = template_dir.join("tests");
        fs::create_dir_all(&tests_dir).unwrap();
        let test_cpp = tests_dir.join("test_main.cpp");
        fs::write(&test_cpp, "// Tests for {{PROJECT_NAME}}").unwrap();

        // Act: Load the template with subdirectory handling
        let template = Template::load_from_path(&template_dir).unwrap();

        // Assert: Verify all files including subdirectory files were loaded
        assert!(template.files.contains_key("main.cpp"));
        assert!(template.files.contains_key("CMakeLists.txt"));
        assert!(template.files.contains_key("lib/utils.hpp"));
        assert!(template.files.contains_key("include/header.h"));
        assert!(template.files.contains_key("tests/test_main.cpp"));

        // Assert: Verify subdirectory file contents
        assert!(template.files["lib/utils.hpp"].contains("{{PROJECT_NAME}}"));
        assert!(template.files["include/header.h"].contains("{{PROJECT_NAME}}"));
        assert!(template.files["tests/test_main.cpp"].contains("{{PROJECT_NAME}}"));
    }

    /// Tests that Template::copy_to() correctly creates subdirectories in the destination.
    /// 
    /// This verifies that when copying templates with subdirectories, the target
    /// directory structure is properly created and all files are placed in their
    /// correct relative paths.
    #[test]
    fn test_template_copy_with_subdirectories() {
        // Arrange: Create a template with subdirectories
        let temp_dir = TempDir::new().unwrap();
        let template_dir = temp_dir.path().join("template");
        fs::create_dir_all(&template_dir).unwrap();

        // Arrange: Create required files
        let main_cpp = template_dir.join("main.cpp");
        fs::write(&main_cpp, "int main() { return 0; }").unwrap();

        let cmake_file = template_dir.join("CMakeLists.txt");
        fs::write(&cmake_file, "project({{PROJECT_NAME}})").unwrap();

        // Arrange: Create subdirectory with files
        let src_dir = template_dir.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        let helper_cpp = src_dir.join("helper.cpp");
        fs::write(&helper_cpp, "// Helper for {{PROJECT_NAME}}").unwrap();

        let template = Template::load_from_path(&template_dir).unwrap();
        let processed = template.apply_variables("test_project");

        // Act: Copy the template to destination
        let dest_dir = temp_dir.path().join("output");
        processed.copy_to(&dest_dir).unwrap();

        // Assert: Verify directory structure was created
        assert!(dest_dir.join("main.cpp").exists());
        assert!(dest_dir.join("CMakeLists.txt").exists());
        assert!(dest_dir.join("src").is_dir());
        assert!(dest_dir.join("src/helper.cpp").exists());

        // Assert: Verify file content was processed
        let helper_content = fs::read_to_string(dest_dir.join("src/helper.cpp")).unwrap();
        assert!(helper_content.contains("// Helper for test_project"));
    }
}