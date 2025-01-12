use std::any::Any;
use crate::error::ValidationResult;
use crate::schema::clone::CloneAny;

pub mod string;
pub mod number;
pub mod boolean;
pub mod object;
pub mod optional;
pub mod clone;
pub mod mapping;
pub mod patterns;
pub mod literal;

/// A schema for validating values.
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
/// assert!(schema.validate(&"hello".to_string()).is_ok());
/// assert!(schema.validate(&42).is_err());
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
///
/// Optional values:
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
/// let schema = s.string().optional();
///
/// assert!(schema.validate(&Some("hello".to_string())).is_ok());
/// assert!(schema.validate(&None::<String>).is_ok());
/// ```
pub trait Schema {
    /// The type of value produced by this schema.
    type Output: CloneAny + 'static;

    /// Validates a value against this schema.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to validate
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The validated value
    /// * `Err(ValidationError)` - The validation error
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string();
    ///
    /// let result = schema.validate(&"hello".to_string());
    /// assert!(result.is_ok());
    /// ```
    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output>;

    /// Makes this schema optional.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().optional();
    ///
    /// assert!(schema.validate(&Some("hello".to_string())).is_ok());
    /// assert!(schema.validate(&None::<String>).is_ok());
    /// ```
    fn optional(self) -> optional::OptionalSchema<Self>
    where
        Self: Sized,
    {
        optional::OptionalSchema::new(self)
    }
}