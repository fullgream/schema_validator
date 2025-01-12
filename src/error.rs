/// Configuration for error messages.
///
/// # Examples
///
/// ```
/// use schema_validator::error::ErrorConfig;
///
/// let config = ErrorConfig {
///     code: "INVALID_EMAIL".to_string(),
///     message: "Invalid email format".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ErrorConfig {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Type { expected: &'static str, got: &'static str },
    Pattern { pattern: String, got: String },
    MinLength { min: usize, got: usize },
    MaxLength { max: usize, got: usize },
    UnknownField { field: String },
    MissingField { field: String },
    Literal { expected: String, got: String },
    Coercion { from: &'static str, to: &'static str },
    Missing { field: String },
    Object { errors: Vec<(String, ValidationError)> },
}

/// A validation error with a code and message.
///
/// # Examples
///
/// ```
/// use schema_validator::error::{ValidationError, ErrorType};
///
/// let err = ValidationError::new(
///     ErrorType::Type {
///         expected: "String",
///         got: "Integer",
///     },
///     None,
/// );
///
/// assert_eq!(err.code, "TYPE_ERROR");
/// assert_eq!(err.message, "Type error: expected String, got Integer");
/// ```
#[derive(Debug, Clone)]
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
            match &error_type {
                ErrorType::Type { expected, got } => ValidationError {
                    code: "TYPE_ERROR".to_string(),
                    message: format!("Type error: expected {}, got {}", expected, got),
                    error_type,
                },
                ErrorType::Pattern { pattern, got } => ValidationError {
                    code: "PATTERN_ERROR".to_string(),
                    message: format!("Pattern error: '{}' does not match pattern '{}'", got, pattern),
                    error_type,
                },
                ErrorType::MinLength { min, got } => ValidationError {
                    code: "MIN_LENGTH_ERROR".to_string(),
                    message: format!("Length error: expected at least {} characters, got {}", min, got),
                    error_type,
                },
                ErrorType::MaxLength { max, got } => ValidationError {
                    code: "MAX_LENGTH_ERROR".to_string(),
                    message: format!("Length error: expected at most {} characters, got {}", max, got),
                    error_type,
                },
                ErrorType::UnknownField { field } => ValidationError {
                    code: "UNKNOWN_FIELD".to_string(),
                    message: format!("Unknown field: '{}'", field),
                    error_type,
                },
                ErrorType::MissingField { field } => ValidationError {
                    code: "MISSING_FIELD".to_string(),
                    message: format!("Missing required field: '{}'", field),
                    error_type,
                },
                ErrorType::Literal { expected, got } => ValidationError {
                    code: "LITERAL_ERROR".to_string(),
                    message: format!("Literal error: expected {}, got {}", expected, got),
                    error_type,
                },
                ErrorType::Coercion { from, to } => ValidationError {
                    code: "COERCION_ERROR".to_string(),
                    message: format!("Coercion error: cannot convert {} to {}", from, to),
                    error_type,
                },
                ErrorType::Missing { field } => ValidationError {
                    code: "MISSING_FIELD".to_string(),
                    message: format!("Missing required field: '{}'", field),
                    error_type,
                },
                ErrorType::Object { errors } => ValidationError {
                    code: "OBJECT_ERROR".to_string(),
                    message: format!("Object validation failed: {:?}", errors),
                    error_type,
                },
            }
        }
    }
}

pub type ValidationResult<T> = Result<T, ValidationError>;