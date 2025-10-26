//! Validation utilities for API requests
//!
//! This module provides validation functions for user inputs,
//! particularly for cassette names and other API parameters.

use once_cell::sync::Lazy;
use regex::Regex;

/// Regex pattern for valid cassette names
/// Allows alphanumeric characters, hyphens, and underscores (1-100 chars)
static CASSETTE_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_-]{1,100}$").unwrap());

/// Reserved cassette names that cannot be used
const RESERVED_NAMES: &[&str] = &[
    "api",
    "vocab",
    "templates",
    "context",
    "swagger",
    "openapi",
    "health",
    "metrics",
    "admin",
];

/// Validates a cassette name
///
/// # Rules
/// - Must be 1-100 characters long
/// - Can only contain: alphanumeric, hyphens (-), underscores (_)
/// - Cannot be a reserved name
/// - Cannot start or end with hyphen or underscore
///
/// # Examples
/// ```
/// use magneto_serge::api::validation::is_valid_cassette_name;
///
/// assert!(is_valid_cassette_name("my-api-test"));
/// assert!(is_valid_cassette_name("user_service_v2"));
/// assert!(is_valid_cassette_name("Test123"));
///
/// assert!(!is_valid_cassette_name(""));  // Too short
/// assert!(!is_valid_cassette_name("my cassette"));  // Spaces not allowed
/// assert!(!is_valid_cassette_name("api"));  // Reserved name
/// assert!(!is_valid_cassette_name("-test"));  // Starts with hyphen
/// ```
pub fn is_valid_cassette_name(name: &str) -> bool {
    // Check length
    if name.is_empty() || name.len() > 100 {
        return false;
    }

    // Check regex pattern
    if !CASSETTE_NAME_REGEX.is_match(name) {
        return false;
    }

    // Check reserved names
    if RESERVED_NAMES.contains(&name.to_lowercase().as_str()) {
        return false;
    }

    // Cannot start or end with hyphen or underscore
    let first = name.chars().next().unwrap();
    let last = name.chars().last().unwrap();
    if first == '-' || first == '_' || last == '-' || last == '_' {
        return false;
    }

    true
}

/// Validates a cassette name and returns a detailed error message
///
/// # Returns
/// - `Ok(())` if valid
/// - `Err(String)` with error message if invalid
///
/// # Examples
/// ```
/// use magneto_serge::api::validation::validate_cassette_name;
///
/// assert!(validate_cassette_name("my-test").is_ok());
/// assert!(validate_cassette_name("").is_err());
/// assert!(validate_cassette_name("api").is_err());
/// ```
pub fn validate_cassette_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Cassette name cannot be empty".to_string());
    }

    if name.len() > 100 {
        return Err(format!(
            "Cassette name too long ({} chars). Maximum is 100 characters",
            name.len()
        ));
    }

    if !CASSETTE_NAME_REGEX.is_match(name) {
        return Err(
            "Invalid cassette name. Use only alphanumeric characters, hyphens, and underscores"
                .to_string(),
        );
    }

    if RESERVED_NAMES.contains(&name.to_lowercase().as_str()) {
        return Err(format!(
            "Cassette name '{}' is reserved and cannot be used",
            name
        ));
    }

    let first = name.chars().next().unwrap();
    let last = name.chars().last().unwrap();
    if first == '-' || first == '_' {
        return Err("Cassette name cannot start with hyphen or underscore".to_string());
    }
    if last == '-' || last == '_' {
        return Err("Cassette name cannot end with hyphen or underscore".to_string());
    }

    Ok(())
}

/// Sanitizes a cassette name by replacing invalid characters
///
/// Useful for auto-generating cassette names from URLs or other sources.
///
/// # Examples
/// ```
/// use magneto_serge::api::validation::sanitize_cassette_name;
///
/// assert_eq!(sanitize_cassette_name("My Test Cassette!"), "My-Test-Cassette");
/// assert_eq!(sanitize_cassette_name("user@service.com"), "user-service-com");
/// ```
pub fn sanitize_cassette_name(name: &str) -> String {
    let mut sanitized = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '.' || c == '@' {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>();

    // Remove leading/trailing hyphens and underscores
    sanitized = sanitized.trim_matches(|c| c == '-' || c == '_').to_string();

    // Truncate to 100 chars
    if sanitized.len() > 100 {
        sanitized.truncate(100);
        sanitized = sanitized.trim_matches(|c| c == '-' || c == '_').to_string();
    }

    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cassette_names() {
        assert!(is_valid_cassette_name("my-api-test"));
        assert!(is_valid_cassette_name("user_service"));
        assert!(is_valid_cassette_name("Test123"));
        assert!(is_valid_cassette_name("a"));
        assert!(is_valid_cassette_name("test-with-many-hyphens"));
        assert!(is_valid_cassette_name("test_with_underscores"));
        assert!(is_valid_cassette_name("MixedCase123"));
    }

    #[test]
    fn test_invalid_cassette_names() {
        assert!(!is_valid_cassette_name(""));
        assert!(!is_valid_cassette_name("my cassette")); // Space
        assert!(!is_valid_cassette_name("test.cassette")); // Dot
        assert!(!is_valid_cassette_name("test@cassette")); // At sign
        assert!(!is_valid_cassette_name("-test")); // Starts with hyphen
        assert!(!is_valid_cassette_name("_test")); // Starts with underscore
        assert!(!is_valid_cassette_name("test-")); // Ends with hyphen
        assert!(!is_valid_cassette_name("test_")); // Ends with underscore
        assert!(!is_valid_cassette_name(&"a".repeat(101))); // Too long
    }

    #[test]
    fn test_reserved_names() {
        assert!(!is_valid_cassette_name("api"));
        assert!(!is_valid_cassette_name("API")); // Case insensitive
        assert!(!is_valid_cassette_name("vocab"));
        assert!(!is_valid_cassette_name("templates"));
        assert!(!is_valid_cassette_name("swagger"));
        assert!(!is_valid_cassette_name("openapi"));
    }

    #[test]
    fn test_validate_cassette_name_errors() {
        assert!(validate_cassette_name("valid-name").is_ok());

        let err = validate_cassette_name("").unwrap_err();
        assert!(err.contains("cannot be empty"));

        let err = validate_cassette_name(&"a".repeat(101)).unwrap_err();
        assert!(err.contains("too long"));

        let err = validate_cassette_name("invalid name").unwrap_err();
        assert!(err.contains("alphanumeric"));

        let err = validate_cassette_name("api").unwrap_err();
        assert!(err.contains("reserved"));

        let err = validate_cassette_name("-test").unwrap_err();
        assert!(err.contains("cannot start"));
    }

    #[test]
    fn test_sanitize_cassette_name() {
        assert_eq!(sanitize_cassette_name("My Test"), "My-Test");
        assert_eq!(
            sanitize_cassette_name("user@service.com"),
            "user-service-com"
        );
        assert_eq!(
            sanitize_cassette_name("test with spaces!"),
            "test-with-spaces_"
        );
        assert_eq!(sanitize_cassette_name("  leading  "), "leading");
        assert_eq!(sanitize_cassette_name("---test---"), "test");

        // Test truncation
        let long_name = "a".repeat(150);
        let sanitized = sanitize_cassette_name(&long_name);
        assert!(sanitized.len() <= 100);
    }
}
