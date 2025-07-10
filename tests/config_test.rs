#[cfg(test)]
mod config_tests {
    use procon_rs::config::{Config, ProjectConfig, TemplateConfig};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Tests that Config::default() creates a configuration with expected default values.
    ///
    /// This ensures that new users get sensible defaults for template selection,
    /// C++ standard version, CMake requirements, and template storage location.
    /// These defaults should work out-of-the-box for most competitive programming scenarios.
    #[test]
    fn test_config_default_values() {
        // Arrange & Act: Create a default configuration
        let config = Config::default();

        // Assert: Verify all default values match expected standards
        assert_eq!(config.template.default, "default");
        assert_eq!(config.project.cpp_standard, "17");
        assert_eq!(config.project.cmake_minimum_version, "3.16");

        // Verify template path is in the user's config directory
        let path_str = config.template.path.to_string_lossy();
        assert!(path_str.contains("procon_rs"));
        assert!(path_str.contains("templates"));
    }

    /// Tests that Config::get() returns correct values for all supported configuration keys.
    ///
    /// This verifies the dot-notation key access system works correctly and returns
    /// the expected string representations of configuration values. It also tests
    /// that unknown keys return None rather than panicking.
    #[test]
    fn test_config_get_values() {
        // Arrange: Create a default configuration
        let config = Config::default();

        // Act & Assert: Verify all known keys return expected values
        assert_eq!(config.get("template.default"), Some("default".to_string()));
        assert_eq!(config.get("project.cpp_standard"), Some("17".to_string()));
        assert_eq!(
            config.get("project.cmake_minimum_version"),
            Some("3.16".to_string())
        );
        assert!(config.get("template.path").is_some());

        // Assert: Verify unknown keys return None
        assert_eq!(config.get("unknown.key"), None);
    }

    /// Tests that Config::set() correctly updates configuration values for all supported keys.
    ///
    /// This verifies that the configuration system can be customized by users,
    /// allowing them to change template preferences, C++ standards, and paths
    /// according to their specific needs and development environment.
    #[test]
    fn test_config_set_values() {
        // Arrange: Create a mutable default configuration
        let mut config = Config::default();

        // Act: Update template default setting
        config.set("template.default", "custom").unwrap();
        // Assert: Verify the change was applied
        assert_eq!(config.template.default, "custom");

        // Act: Update template path setting
        config.set("template.path", "/custom/path").unwrap();
        // Assert: Verify the path was updated
        assert_eq!(config.template.path, PathBuf::from("/custom/path"));

        // Act: Update C++ standard setting
        config.set("project.cpp_standard", "20").unwrap();
        // Assert: Verify the C++ standard was updated
        assert_eq!(config.project.cpp_standard, "20");

        // Act: Update CMake minimum version setting
        config.set("project.cmake_minimum_version", "3.20").unwrap();
        // Assert: Verify the CMake version was updated
        assert_eq!(config.project.cmake_minimum_version, "3.20");
    }

    /// Tests that Config::set() returns an error for unknown configuration keys.
    ///
    /// This ensures that typos in configuration keys are caught early and
    /// provides clear feedback to users about invalid configuration attempts,
    /// helping prevent silent configuration failures.
    #[test]
    fn test_config_set_unknown_key_returns_error() {
        // Arrange: Create a mutable configuration
        let mut config = Config::default();

        // Act: Attempt to set an unknown configuration key
        let result = config.set("unknown.key", "value");

        // Assert: Verify that the operation returns an error
        assert!(result.is_err());

        // Assert: Verify the error message is descriptive
        if let Err(e) = result {
            assert!(e.to_string().contains("Unknown configuration key"));
        }
    }

    /// Tests that configuration can be serialized to TOML and loaded back correctly.
    ///
    /// This verifies the persistence mechanism works properly, ensuring that
    /// user configuration changes are preserved between application runs.
    /// This is crucial for maintaining user preferences and custom settings.
    #[test]
    fn test_config_save_and_load() {
        // Arrange: Create a temporary file for testing
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("config.toml");

        // Arrange: Create a custom configuration to save
        let mut config = Config::default();
        config.template.default = "custom".to_string();
        config.project.cpp_standard = "20".to_string();

        // Act: Serialize and save the configuration
        let toml_content = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_file, toml_content).unwrap();

        // Act: Load the configuration from file
        let loaded_content = fs::read_to_string(&config_file).unwrap();
        let loaded_config: Config = toml::from_str(&loaded_content).unwrap();

        // Assert: Verify the loaded configuration matches the saved one
        assert_eq!(loaded_config.template.default, "custom");
        assert_eq!(loaded_config.project.cpp_standard, "20");
    }

    /// Tests that Config can be correctly serialized to and deserialized from TOML format.
    ///
    /// This verifies the complete round-trip serialization process works correctly,
    /// ensuring that all configuration data types (strings, paths) are properly
    /// handled by the TOML serialization system.
    #[test]
    fn test_config_serialization() {
        // Arrange: Create a configuration with specific test values
        let config = Config {
            template: TemplateConfig {
                default: "advanced".to_string(),
                path: PathBuf::from("/home/user/templates"),
            },
            project: ProjectConfig {
                cpp_standard: "23".to_string(),
                cmake_minimum_version: "3.25".to_string(),
            },
        };

        // Act: Serialize the configuration to TOML
        let serialized = toml::to_string(&config).unwrap();

        // Assert: Verify all values appear in the serialized format
        assert!(serialized.contains("advanced"));
        assert!(serialized.contains("/home/user/templates"));
        assert!(serialized.contains("23"));
        assert!(serialized.contains("3.25"));

        // Act: Deserialize the TOML back to a Config struct
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        // Assert: Verify all fields match the original configuration
        assert_eq!(deserialized.template.default, config.template.default);
        assert_eq!(deserialized.template.path, config.template.path);
        assert_eq!(
            deserialized.project.cpp_standard,
            config.project.cpp_standard
        );
        assert_eq!(
            deserialized.project.cmake_minimum_version,
            config.project.cmake_minimum_version
        );
    }
}
