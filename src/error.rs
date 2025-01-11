use std::fmt;

#[derive(Debug, Clone)]
pub struct ErrorConfig {
    pub code: String,
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

#[derive(Debug, Clone)]
pub enum ErrorType {
    Type { expected: &'static str, got: &'static str },
    Coercion { from: &'static str, to: &'static str },
}

#[derive(Debug)]
pub struct ValidationError {
    pub error_type: ErrorType,
    pub code: String,
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ValidationError {}

impl ValidationError {
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
            (ErrorType::Type { expected, got }, _) => (
                "TYPE_ERROR".to_string(),
                format!("Type error: expected {}, got {}", expected, got),
            ),
            (ErrorType::Coercion { from, to }, _) => (
                "COERCION_ERROR".to_string(),
                format!("Coercion error: cannot convert {} to {}", from, to),
            ),
        };

        ValidationError {
            error_type,
            code,
            message,
        }
    }
}

pub type ValidationResult<T> = Result<T, ValidationError>;