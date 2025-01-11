use std::fmt;
use std::collections::HashMap;

/// Configuration for custom error messages.
///
/// Used with the `set_message` method on schemas to customize error codes
/// and messages.
///
/// # Examples
///
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
/// let schema = s.string()
///     .set_message("CUSTOM_ERROR", "Invalid value");
///
/// let err = schema.validate(&42).unwrap_err();
/// assert_eq!(err.code, "CUSTOM_ERROR");
/// assert_eq!(err.message, "Invalid value");
/// ```
#[derive(Debug, Clone)]
pub struct ErrorConfig {
    /// The error code to use
    pub code: String,
    /// The error message to use
    pub message: String,
}

impl Default for ErrorConfig {
    fn default() -> Self {
        Self {
            code: String::new(),
            message: String::new(),
        }
    }
}

/// The type of validation error that occurred.
///
/// This enum represents the two main types of errors that can occur
/// during validation:
/// - Type errors: when a value is not of the expected type
/// - Coercion errors: when type coercion fails
#[derive(Debug, Clone)]
pub enum ErrorType {
    /// Error when a value is not of the expected type
    Type {
        /// The expected type
        expected: &'static str,
        /// The actual type received
        got: &'static str,
    },

    /// Error when type coercion fails
    Coercion {
        /// The type being converted from
        from: &'static str,
        /// The type being converted to
        to: &'static str,
    },

    /// Error when a required field is missing
    Missing {
        /// The name of the missing field
        field: String,
    },

    /// Error in object validation
    Object {
        /// Map of field names to their validation errors
        errors: HashMap<String, ValidationError>,
    },
}

/// An error that occurs during validation.
///
/// Contains:
/// - An error type (Type or Coercion)
/// - An error code (can be customized)
/// - An error message (can be customized)
///
/// # Examples
///
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
///
/// // Default error
/// let err = s.string().validate(&42).unwrap_err();
/// assert_eq!(err.code, "TYPE_ERROR");
///
/// // Custom error
/// let err = s.string()
///     .set_message("INVALID", "Value must be a string")
///     .validate(&42)
///     .unwrap_err();
/// assert_eq!(err.code, "INVALID");
/// assert_eq!(err.message, "Value must be a string");
/// ```
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The type of error that occurred
    pub error_type: ErrorType,
    /// The error code (default or custom)
    pub code: String,
    /// The error message (default or custom)
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ValidationError {}

impl ValidationError {
    /// Creates a new ValidationError.
    ///
    /// Uses custom error configuration if provided and valid,
    /// otherwise uses default error messages based on the error type.
    ///
    /// # Arguments
    ///
    /// * `error_type` - The type of error that occurred
    /// * `config` - Optional custom error configuration
    pub fn new(error_type: ErrorType, config: Option<ErrorConfig>) -> Self {
        let (code, message) = match (&error_type, &config) {
            (ErrorType::Type { expected, got }, Some(config)) if !config.code.is_empty() => (
                config.code.clone(),
                if config.message.is_empty() {
                    format!("Type error: expected {}, got {}", expected, got)
                } else {
                    config.message.clone()
                },
            ),
            (ErrorType::Coercion { from, to }, Some(config)) if !config.code.is_empty() => (
                config.code.clone(),
                if config.message.is_empty() {
                    format!("Coercion error: cannot convert {} to {}", from, to)
                } else {
                    config.message.clone()
                },
            ),
            (ErrorType::Missing { field }, Some(config)) if !config.code.is_empty() => (
                config.code.clone(),
                if config.message.is_empty() {
                    format!("Missing required field: {}", field)
                } else {
                    config.message.clone()
                },
            ),
            (ErrorType::Object { errors }, Some(config)) if !config.code.is_empty() => (
                config.code.clone(),
                if config.message.is_empty() {
                    format_object_errors(errors)
                } else {
                    config.message.clone()
                },
            ),
            (ErrorType::Type { expected, got }, _) => (
                "TYPE_ERROR".to_string(),
                format!("Type error: expected {}, got {}", expected, got),
            ),
            (ErrorType::Coercion { from, to }, _) => (
                "COERCION_ERROR".to_string(),
                format!("Coercion error: cannot convert {} to {}", from, to),
            ),
            (ErrorType::Missing { field }, _) => (
                "MISSING_FIELD".to_string(),
                format!("Missing required field: {}", field),
            ),
            (ErrorType::Object { errors }, _) => (
                "VALIDATION_ERROR".to_string(),
                format_object_errors(errors),
            ),
        };

        ValidationError {
            error_type,
            code,
            message,
        }
    }
}

fn format_object_errors(errors: &HashMap<String, ValidationError>) -> String {
    let mut messages = Vec::new();
    for (field, error) in errors {
        messages.push(format!("{}: {}", field, error.message));
    }
    messages.join(", ")
}

/// A Result type specialized for validation operations.
///
/// This type is used as the return type for all validation operations.
/// The `Ok` variant contains the validated and possibly transformed value,
/// while the `Err` variant contains a `ValidationError`.
pub type ValidationResult<T> = Result<T, ValidationError>;