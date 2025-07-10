#[cfg(test)]
mod error_tests {
    use procon_rs::error::ProconError;

    /// Tests that ProjectExists error displays a user-friendly message with the project name.
    ///
    /// This ensures that when a user tries to create a project that already exists,
    /// they receive a clear and specific error message indicating which project name
    /// conflicts with an existing project.
    #[test]
    fn test_project_exists_error_display() {
        // Arrange: Create a ProjectExists error with a specific project name
        let project_name = "test_project";
        let error = ProconError::ProjectExists(project_name.to_string());

        // Act: Convert the error to its string representation
        let error_message = error.to_string();

        // Assert: Verify the error message contains the expected format and project name
        assert_eq!(error_message, "Project 'test_project' already exists");
    }

    /// Tests that ProjectNotFound error displays a clear message when no project directory is found.
    ///
    /// This error occurs when operations are attempted on a non-existent project directory,
    /// helping users understand that the expected project structure is missing.
    #[test]
    fn test_project_not_found_error_display() {
        // Arrange: Create a ProjectNotFound error
        let error = ProconError::ProjectNotFound;

        // Act: Convert the error to its string representation
        let error_message = error.to_string();

        // Assert: Verify the error message is clear and descriptive
        assert_eq!(error_message, "Project directory not found");
    }

    /// Tests that TemplateNotFound error displays the specific template name that couldn't be found.
    ///
    /// This helps users identify which template name they specified that doesn't exist,
    /// enabling them to correct the template name or create the missing template.
    #[test]
    fn test_template_not_found_error_display() {
        // Arrange: Create a TemplateNotFound error with a specific template name
        let template_name = "custom";
        let error = ProconError::TemplateNotFound(template_name.to_string());

        // Act: Convert the error to its string representation
        let error_message = error.to_string();

        // Assert: Verify the error message includes the template name
        assert_eq!(error_message, "Template 'custom' not found");
    }

    /// Tests that ProjectCreationFailed error includes the underlying failure reason.
    ///
    /// This error provides context about why project creation failed, such as
    /// permission issues, disk space problems, or other filesystem errors.
    #[test]
    fn test_project_creation_failed_error_display() {
        // Arrange: Create a ProjectCreationFailed error with a specific reason
        let failure_reason = "permission denied";
        let error = ProconError::ProjectCreationFailed(failure_reason.to_string());

        // Act: Convert the error to its string representation
        let error_message = error.to_string();

        // Assert: Verify the error message includes both prefix and reason
        assert_eq!(error_message, "Failed to create project: permission denied");
    }

    /// Tests that ConfigError displays configuration-specific error messages.
    ///
    /// Configuration errors help users understand issues with their settings,
    /// such as invalid configuration keys or malformed configuration values.
    #[test]
    fn test_config_error_display() {
        // Arrange: Create a ConfigError with a specific configuration issue
        let config_issue = "invalid key";
        let error = ProconError::ConfigError(config_issue.to_string());

        // Act: Convert the error to its string representation
        let error_message = error.to_string();

        // Assert: Verify the error message includes configuration context
        assert_eq!(error_message, "Configuration error: invalid key");
    }

    /// Tests that std::io::Error can be automatically converted to ProconError.
    ///
    /// This ensures that filesystem operations that return io::Error can be
    /// seamlessly integrated into our error handling system using the ? operator.
    #[test]
    fn test_io_error_conversion() {
        // Arrange: Create a standard IO error
        use std::io;
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");

        // Act: Convert the IO error to ProconError using the From trait
        let error: ProconError = io_error.into();

        // Assert: Verify the error was converted to the Io variant
        assert!(matches!(error, ProconError::Io(_)));
    }

    /// Tests that ProconError implements Send and Sync traits for thread safety.
    ///
    /// This is crucial for error handling in multi-threaded environments and
    /// ensures that errors can be safely passed between threads.
    #[test]
    fn test_error_is_send_sync() {
        // Arrange & Act & Assert: Compile-time check for Send + Sync traits
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ProconError>();
    }
}

