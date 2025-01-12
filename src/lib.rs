//! A flexible, type-safe schema validation library with support for type coercion, transformations,
//! optional fields, and object validation.
//!
//! # Features
//!
//! - **Type Validation**: Basic validation for strings, numbers, and booleans
//! - **Optional Fields**: Support for optional values with proper type checking
//! - **Type Coercion**: Automatic conversion between compatible types
//! - **Object Validation**: Validate complex objects with multiple fields
//! - **Custom Transformations**: Transform validated data into custom types
//! - **Error Handling**: Detailed error messages with customizable codes
//!
//! # Basic Usage
//!
//! ```rust
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Basic type validation
//! let valid_string = s.string().validate(&"hello".to_string()).unwrap();
//! let valid_number = s.number().validate(&42.0).unwrap();
//! let valid_bool = s.boolean().validate(&true).unwrap();
//!
//! // Optional fields
//! let optional_string = s.string().optional().validate(&Some("hello".to_string())).unwrap(); // Some("hello")
//! let optional_none = s.number().optional().validate(&None::<f64>).unwrap(); // None
//!
//! // Type coercion
//! let string_from_num = s.coerce().string().validate(&42_i64).unwrap(); // "42"
//! let num_from_string = s.coerce().number().validate(&"42".to_string()).unwrap(); // 42.0
//! let bool_from_num = s.coerce().boolean().validate(&1_i64).unwrap(); // true
//! ```
//!
//! # Object Validation
//!
//! ```rust
//! use schema_validator::{schema, Schema, ValidateAs, Validate};
//! use std::collections::HashMap;
//! use std::any::Any;
//!
//! #[derive(Debug, PartialEq, Clone, Validate)]
//! struct User {
//!     name: String,
//!     age: Option<f64>,
//!     is_active: bool,
//! }
//!
//! let s = schema();
//!
//! // Define schema with optional field
//! let schema = s.object()
//!     .field("name", s.string())
//!     .field("age", s.number().optional())
//!     .field("is_active", s.boolean());
//!
//! // Create object
//! let mut obj = HashMap::new();
//! obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
//! obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);
//! obj.insert("is_active".to_string(), Box::new(true) as Box<dyn Any>);
//!
//! // Validate and convert to User struct
//! let user: User = schema.validate_as(&obj).unwrap();
//! assert_eq!(user.name, "John");
//! assert_eq!(user.age, Some(30.0));
//! assert_eq!(user.is_active, true);
//! ```
//!
//! # Custom Transformations
//!
//! ```rust
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Transform optional string to optional length
//! let schema = s.string()
//!     .optional()
//!     .transform(|opt_str| opt_str.map(|s| s.len()));
//!
//! assert_eq!(schema.validate(&Some("hello".to_string())).unwrap(), Some(5));
//! assert_eq!(schema.validate(&None::<String>).unwrap(), None);
//!
//! // Transform optional number to optional boolean
//! let schema = s.number()
//!     .optional()
//!     .transform(|opt_num| opt_num.map(|n| n > 0.0));
//!
//! assert_eq!(schema.validate(&Some(42.0)).unwrap(), Some(true));
//! assert_eq!(schema.validate(&Some(-1.0)).unwrap(), Some(false));
//! assert_eq!(schema.validate(&None::<f64>).unwrap(), None);
//! ```

pub mod error;
pub mod schema;

pub use schema_validator_derive::Validate;

pub use error::{ValidationError, ValidationResult};
pub use schema::Schema;
pub use schema::mapping::{FromFields, ValidateAs};
use schema::string::StringSchema;
use schema::number::NumberSchema;
use schema::boolean::BooleanSchema;
use schema::object::ObjectSchema;

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

    /// Creates an object validation schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema, ValidateAs, Validate};
    /// use std::collections::HashMap;
    /// use std::any::Any;
    ///
    /// #[derive(Debug, PartialEq, Clone, Validate)]
    /// struct User {
    ///     name: String,
    ///     age: Option<f64>,
    ///     is_active: bool,
    /// }
    ///
    /// let s = schema();
    ///
    /// // Define schema with optional field
    /// let schema = s.object()
    ///     .field("name", s.string())
    ///     .field("age", s.number().optional())
    ///     .field("is_active", s.boolean());
    ///
    /// // Create object
    /// let mut obj = HashMap::new();
    /// obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    /// obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);
    /// obj.insert("is_active".to_string(), Box::new(true) as Box<dyn Any>);
    ///
    /// // Validate and convert to User struct
    /// let user: User = schema.validate_as(&obj).unwrap();
    /// assert_eq!(user.name, "John");
    /// assert_eq!(user.age, Some(30.0));
    /// assert_eq!(user.is_active, true);
    /// ```
    pub fn object(&self) -> ObjectSchema {
        ObjectSchema::new()
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

    /// Creates an object validation schema with type coercion enabled.
    pub fn object(&self) -> ObjectSchema {
        self.builder.object()
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