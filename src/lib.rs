//! A flexible, type-safe schema validation library with support for type coercion and transformations.
//!
//! # Examples
//!
//! Basic validation:
//! ```
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Validate types
//! let valid_string = s.string().validate(&"hello".to_string()).unwrap();
//! let valid_number = s.number().validate(&42.0).unwrap();
//! let valid_bool = s.boolean().validate(&true).unwrap();
//! ```
//!
//! Type coercion:
//! ```
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Convert between types
//! let string_from_num = s.coerce().string().validate(&42_i64).unwrap(); // "42"
//! let num_from_string = s.coerce().number().validate(&"42".to_string()).unwrap(); // 42.0
//! let bool_from_num = s.coerce().boolean().validate(&1_i64).unwrap(); // true
//! ```
//!
//! Value transformations:
//! ```
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Transform values
//! let uppercase = s.string()
//!     .transform(|s| s.to_uppercase())
//!     .validate(&"hello".to_string())
//!     .unwrap(); // "HELLO"
//!
//! // Transform with type changes
//! let length = s.string()
//!     .transform(|s| s.len() as f64)
//!     .validate(&"hello".to_string())
//!     .unwrap(); // 5.0
//! ```

pub mod error;
pub mod schema;

pub use error::{ValidationError, ValidationResult};
pub use schema::Schema;
use schema::string::StringSchema;
use schema::number::NumberSchema;
use schema::boolean::BooleanSchema;

/// The main entry point for creating schemas.
///
/// # Examples
///
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
///
/// // Create different schema types
/// let string_schema = s.string();
/// let number_schema = s.number();
/// let boolean_schema = s.boolean();
///
/// // Enable type coercion
/// let coerce_schema = s.coerce().string();
/// ```
#[derive(Debug)]
pub struct SchemaBuilder {
    coerce: bool,
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self { coerce: false }
    }
}

impl SchemaBuilder {
    /// Creates a new SchemaBuilder with type coercion disabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a string validation schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    ///
    /// // Basic string validation
    /// let result = s.string().validate(&"hello".to_string()).unwrap();
    /// assert_eq!(result, "hello");
    ///
    /// // String validation with transformation
    /// let result = s.string()
    ///     .transform(|s| s.to_uppercase())
    ///     .validate(&"hello".to_string())
    ///     .unwrap();
    /// assert_eq!(result, "HELLO");
    /// ```
    pub fn string(&self) -> StringSchema {
        StringSchema::new(self.coerce)
    }

    /// Creates a number validation schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    ///
    /// // Basic number validation
    /// let result = s.number().validate(&42.0).unwrap();
    /// assert_eq!(result, 42.0);
    ///
    /// // Number validation with coercion
    /// let result = s.coerce().number()
    ///     .validate(&"42".to_string())
    ///     .unwrap();
    /// assert_eq!(result, 42.0);
    /// ```
    pub fn number(&self) -> NumberSchema {
        NumberSchema::new(self.coerce)
    }

    /// Creates a boolean validation schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    ///
    /// // Basic boolean validation
    /// let result = s.boolean().validate(&true).unwrap();
    /// assert_eq!(result, true);
    ///
    /// // Boolean validation with coercion
    /// let result = s.coerce().boolean()
    ///     .validate(&1_i64)
    ///     .unwrap();
    /// assert_eq!(result, true);
    /// ```
    pub fn boolean(&self) -> BooleanSchema {
        BooleanSchema::new(self.coerce)
    }

    /// Enables type coercion for the schema.
    ///
    /// When type coercion is enabled, the schema will attempt to convert values
    /// to the target type before validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    ///
    /// // Convert number to string
    /// let result = s.coerce().string()
    ///     .validate(&42_i64)
    ///     .unwrap();
    /// assert_eq!(result, "42");
    ///
    /// // Convert string to number
    /// let result = s.coerce().number()
    ///     .validate(&"42".to_string())
    ///     .unwrap();
    /// assert_eq!(result, 42.0);
    /// ```
    pub fn coerce(&self) -> CoerceBuilder {
        CoerceBuilder {
            builder: SchemaBuilder { coerce: true },
        }
    }
}

/// A builder for schemas with type coercion enabled.
#[derive(Debug)]
pub struct CoerceBuilder {
    builder: SchemaBuilder,
}

impl CoerceBuilder {
    /// Creates a string validation schema with type coercion enabled.
    pub fn string(&self) -> StringSchema {
        self.builder.string()
    }

    /// Creates a number validation schema with type coercion enabled.
    pub fn number(&self) -> NumberSchema {
        self.builder.number()
    }

    /// Creates a boolean validation schema with type coercion enabled.
    pub fn boolean(&self) -> BooleanSchema {
        self.builder.boolean()
    }
}

/// Creates a new schema builder.
///
/// This is the main entry point for creating validation schemas.
///
/// # Examples
///
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
///
/// // Create different types of schemas
/// let string_schema = s.string();
/// let number_schema = s.number();
/// let boolean_schema = s.boolean();
///
/// // Enable type coercion
/// let coerce_schema = s.coerce().string();
/// ```
pub fn schema() -> SchemaBuilder {
    SchemaBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_validation() {
        let s = schema();

        // Valid cases
        assert!(s.string().validate(&"hello".to_string()).is_ok());
        assert!(s.boolean().validate(&true).is_ok());
        assert!(s.number().validate(&50.0).is_ok());

        // Invalid cases
        let num: i64 = 42;
        assert!(s.string().validate(&num).is_err());
    }

    #[test]
    fn test_coercion() {
        let s = schema();

        // String coercion
        let num: i64 = 42;
        let coerced_str = s.coerce().string().validate(&num).unwrap();
        assert_eq!(coerced_str, "42");

        // Number coercion
        let str_num = "123".to_string();
        let coerced_num = s.coerce().number().validate(&str_num).unwrap();
        assert_eq!(coerced_num, 123.0);

        // Boolean coercion
        let num_one: i64 = 1;
        let coerced_bool = s.coerce().boolean().validate(&num_one).unwrap();
        assert_eq!(coerced_bool, true);

        let num_zero: i64 = 0;
        let coerced_bool = s.coerce().boolean().validate(&num_zero).unwrap();
        assert_eq!(coerced_bool, false);
    }
}
