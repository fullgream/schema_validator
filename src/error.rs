use std::collections::HashMap;
use thiserror::Error;

/// The result type for validation operations.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Configuration for error messages.
///
/// # Examples
///
/// ```
/// use schema_validator::error::ErrorConfig;
///
/// let config = ErrorConfig {
///     code: "INVALID_USER".to_string(),
///     message: "Invalid user data".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ErrorConfig {
    pub code: String,
    pub message: String,
}

/// The type of validation error.
#[derive(Debug, Error)]
pub enum ErrorType {
    /// Type mismatch error
    #[error("Type error: expected {expected}, got {got}")]
    Type {
        expected: &'static str,
        got: &'static str,
    },

    /// Missing field error
    #[error("Missing required field: {field}")]
    Missing {
        field: String,
    },

    /// Object validation error
    #[error("Object validation failed: {}", format_errors(.errors))]
    Object {
        errors: HashMap<String, ValidationError>,
    },

    /// String minimum length error
    #[error("String too short: expected at least {min} characters, got {got}")]
    MinLength {
        min: usize,
        got: usize,
    },

    /// String maximum length error
    #[error("String too long: expected at most {max} characters, got {got}")]
    MaxLength {
        max: usize,
        got: usize,
    },

    /// String pattern mismatch error
    #[error("String does not match pattern: expected {pattern}, got {got}")]
    Pattern {
        pattern: String,
        got: String,
    },

    /// Type coercion error
    #[error("Coercion error: cannot convert {from} to {to}")]
    Coercion {
        from: &'static str,
        to: &'static str,
    },
}

/// A validation error with an optional custom error message.
///
/// # Examples
///
/// ```
/// use schema_validator::error::{ValidationError, ErrorType, ErrorConfig};
///
/// let error = ValidationError::new(
///     ErrorType::Type {
///         expected: "String",
///         got: "Integer",
///     },
///     Some(ErrorConfig {
///         code: "INVALID_TYPE".to_string(),
///         message: "Invalid type".to_string(),
///     }),
/// );
///
/// assert_eq!(error.code, "INVALID_TYPE");
/// assert_eq!(error.message, "Invalid type");
/// ```
#[derive(Debug)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub error_type: ErrorType,
}

impl ValidationError {
    pub fn new(error_type: ErrorType, config: Option<ErrorConfig>) -> Self {
        if let Some(config) = config {
            ValidationError {
                code: config.code,
                message: config.message,
                error_type,
            }
        } else {
            let (code, message) = match &error_type {
                ErrorType::Type { .. } => ("TYPE_ERROR", error_type.to_string()),
                ErrorType::Coercion { .. } => ("COERCION_ERROR", error_type.to_string()),
                _ => ("VALIDATION_ERROR", error_type.to_string()),
            };
            ValidationError {
                code: code.to_string(),
                message,
                error_type,
            }
        }
    }
}

fn format_errors(errors: &HashMap<String, ValidationError>) -> String {
    let mut messages = Vec::new();
    for (field, error) in errors {
        messages.push(format!("{}: {}", field, error.message));
    }
    messages.join(", ")
}