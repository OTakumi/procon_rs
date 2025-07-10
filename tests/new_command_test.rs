#[cfg(test)]
mod new_command_tests {
    use procon_rs::commands::new::{NewCommand, NewCommandArgs};
    use std::fs;
    use tempfile::TempDir;

    /// Tests that NewCommand creates a project with all required files using the default template.
    /// 
    /// This verifies the end-to-end project creation workflow: accepting command arguments,
    /// loading the default template, and generating a complete C++ project structure
    /// with main.cpp, CMakeLists.txt, and .gitignore files.
    #[test]
    fn test_new_command_create_project_with_default_template() {
        // Arrange: Set up a temporary directory and project parameters
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test_project";
        let project_path = temp_dir.path().join(project_name);

        let args = NewCommandArgs {
            name: project_name.to_string(),
            template: "default".to_string(),
            path: Some(temp_dir.path().to_path_buf()),
        };

        // Act: Execute the new command to create the project
        let result = NewCommand::execute(args);

        // Assert: Verify the project was created successfully
        assert!(result.is_ok());
        assert!(project_path.exists());

        // Assert: Verify all required files are present
        assert!(project_path.join("main.cpp").exists());
        assert!(project_path.join("CMakeLists.txt").exists());
        assert!(project_path.join(".gitignore").exists());
    }

    /// Tests that NewCommand correctly substitutes project name variables in generated files.
    /// 
    /// This ensures that template variables like {{PROJECT_NAME}} are properly replaced
    /// with the actual project name throughout all generated files, creating personalized
    /// project content.
    #[test]
    fn test_new_command_variable_replacement() {
        // Arrange: Set up a temporary directory with a descriptive project name
        let temp_dir = TempDir::new().unwrap();
        let project_name = "my_awesome_project";
        let project_path = temp_dir.path().join(project_name);

        let args = NewCommandArgs {
            name: project_name.to_string(),
            template: "default".to_string(),
            path: Some(temp_dir.path().to_path_buf()),
        };

        // Act: Create the project with variable substitution
        let result = NewCommand::execute(args);
        assert!(result.is_ok());

        // Act: Read the generated file contents
        let main_content = fs::read_to_string(project_path.join("main.cpp")).unwrap();
        let cmake_content = fs::read_to_string(project_path.join("CMakeLists.txt")).unwrap();

        // Assert: Verify project name was substituted in main.cpp
        assert!(main_content.contains("my_awesome_project"));

        // Assert: Verify project name was substituted in CMakeLists.txt
        assert!(cmake_content.contains("project(my_awesome_project)"));
        assert!(cmake_content.contains("add_executable(my_awesome_project"));
    }

    /// Tests that NewCommand returns an error when attempting to create a project that already exists.
    /// 
    /// This prevents accidental overwriting of existing projects and provides clear
    /// error feedback when users try to create projects with conflicting names,
    /// protecting existing work from being destroyed.
    #[test]
    fn test_new_command_project_already_exists() {
        // Arrange: Create a temporary directory with an existing project directory
        let temp_dir = TempDir::new().unwrap();
        let project_name = "existing_project";
        let project_path = temp_dir.path().join(project_name);
        fs::create_dir_all(&project_path).unwrap();

        let args = NewCommandArgs {
            name: project_name.to_string(),
            template: "default".to_string(),
            path: Some(temp_dir.path().to_path_buf()),
        };

        // Act: Attempt to create a project with an existing name
        let result = NewCommand::execute(args);

        // Assert: Verify the operation fails with an appropriate error
        assert!(result.is_err());

        // Assert: Verify the error message indicates the conflict
        if let Err(e) = result {
            assert!(e.to_string().contains("already exists"));
        }
    }

    /// Tests that NewCommand returns an error for non-existent templates.
    /// 
    /// This ensures that users receive clear feedback when they specify invalid
    /// template names, helping them identify typos or understand available template
    /// options rather than failing silently.
    #[test]
    fn test_new_command_nonexistent_template() {
        // Arrange: Set up parameters with a non-existent template name
        let temp_dir = TempDir::new().unwrap();
        let project_name = "test_project";

        let args = NewCommandArgs {
            name: project_name.to_string(),
            template: "nonexistent".to_string(),
            path: Some(temp_dir.path().to_path_buf()),
        };

        // Act: Attempt to create a project with an invalid template
        let result = NewCommand::execute(args);

        // Assert: Verify the operation fails with template-specific error
        assert!(result.is_err());

        // Assert: Verify the error message identifies the missing template
        if let Err(e) = result {
            assert!(e.to_string().contains("Template 'nonexistent' not found"));
        }
    }

    /// Tests that NewCommand can create projects in the current directory when no path is specified.
    /// 
    /// This verifies the default behavior when users don't specify a target directory,
    /// ensuring projects are created in the current working directory as expected
    /// by typical command-line tool conventions.
    #[test]
    fn test_new_command_create_in_current_directory() {
        // Arrange: Set up a temporary directory as the current working directory
        let temp_dir = TempDir::new().unwrap();
        let project_name = "current_dir_project";
        let original_dir = std::env::current_dir().unwrap();

        // Arrange: Change to the temporary directory
        std::env::set_current_dir(&temp_dir).unwrap();

        let args = NewCommandArgs {
            name: project_name.to_string(),
            template: "default".to_string(),
            path: None, // Should use current directory
        };

        // Act: Create the project in the current directory
        let result = NewCommand::execute(args);

        // Cleanup: Restore original working directory
        std::env::set_current_dir(original_dir).unwrap();

        // Assert: Verify the project was created successfully
        assert!(result.is_ok());

        // Assert: Verify the project exists in the expected location
        let project_path = temp_dir.path().join(project_name);
        assert!(project_path.exists());
        assert!(project_path.join("main.cpp").exists());
    }

    /// Tests that NewCommand correctly substitutes CMake configuration variables.
    /// 
    /// This verifies that configuration-specific variables like {{CMAKE_VERSION}}
    /// and {{CPP_STANDARD}} are replaced with values from the user's configuration,
    /// ensuring generated projects use appropriate build settings.
    #[test]
    fn test_new_command_cmake_variables_replaced() {
        // Arrange: Set up a project for testing CMake variable substitution
        let temp_dir = TempDir::new().unwrap();
        let project_name = "cmake_test";
        let project_path = temp_dir.path().join(project_name);

        let args = NewCommandArgs {
            name: project_name.to_string(),
            template: "default".to_string(),
            path: Some(temp_dir.path().to_path_buf()),
        };

        // Act: Create the project with CMake variable substitution
        let result = NewCommand::execute(args);
        assert!(result.is_ok());

        // Act: Read the generated CMakeLists.txt content
        let cmake_content = fs::read_to_string(project_path.join("CMakeLists.txt")).unwrap();

        // Assert: Verify template variables were replaced (not present)
        assert!(!cmake_content.contains("{{CMAKE_VERSION}}"));
        assert!(!cmake_content.contains("{{CPP_STANDARD}}"));

        // Assert: Verify actual values are present
        assert!(cmake_content.contains("VERSION"));
        assert!(cmake_content.contains("17")); // Default C++ standard from config
    }
}