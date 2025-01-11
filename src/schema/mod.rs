pub mod string;
pub mod boolean;
pub mod number;

use crate::error::ValidationResult;
use std::any::Any;

/// A trait for schema validation.
///
/// This trait is implemented by all schema types and provides a common interface
/// for validation and transformation.
///
/// # Examples
///
/// Basic validation:
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
/// let schema = s.string();
///
/// let result = schema.validate(&"hello".to_string()).unwrap();
/// assert_eq!(result, "hello");
/// ```
///
/// Validation with transformation:
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
/// let schema = s.string()
///     .transform(|s| s.to_uppercase());
///
/// let result = schema.validate(&"hello".to_string()).unwrap();
/// assert_eq!(result, "HELLO");
/// ```
///
/// Type coercion:
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
/// let schema = s.coerce().string();
///
/// let result = schema.validate(&42_i64).unwrap();
/// assert_eq!(result, "42");
/// ```
pub trait Schema {
    /// The type that this schema outputs after validation and transformation.
    type Output;

    /// Validates and optionally transforms a value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to validate. Can be any type that implements `Any`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Output)` if validation succeeds, or `Err(ValidationError)` if it fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    ///
    /// // Basic validation
    /// let schema = s.string();
    /// assert!(schema.validate(&"hello".to_string()).is_ok());
    /// assert!(schema.validate(&42_i64).is_err());
    ///
    /// // Validation with coercion
    /// let schema = s.coerce().string();
    /// let result = schema.validate(&42_i64).unwrap();
    /// assert_eq!(result, "42");
    /// ```
    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output>;
}