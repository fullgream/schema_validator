//! A flexible, type-safe schema validation library with a fluent API, built-in patterns, and powerful transformations.
//!
//! # Features
//!
//! - **Fluent API**: Chain validation methods for clear and concise code
//! - **Built-in Patterns**: Common validations like email, URL, date, etc.
//! - **Type Coercion**: Automatic conversion between compatible types
//! - **Transformations**: Transform and validate data in any order
//! - **Object Validation**: Validate complex objects with multiple fields
//! - **Error Handling**: Detailed error messages with customizable codes
//! - **JSON Support**: Direct validation of JSON values
//! - **Derive Macro**: Automatically implement validation traits
//!
//! # Quick Start
//!
//! ```rust
//! use schema_validator::{schema, Schema, Validate};
//!
//! let s = schema();
//!
//! // Basic string validation
//! let schema = s.string()
//!     .min_length(3)
//!     .max_length(50);
//!
//! // Email validation with transformation
//! let schema = s.string()
//!     .transform(|s| s.trim().to_lowercase())
//!     .email()
//!     .max_length(50);
//!
//! // Object validation
//! #[derive(Debug, PartialEq, Clone, Validate)]
//! struct User {
//!     name: String,
//!     email: String,
//!     age: Option<f64>,
//! }
//!
//! let schema = s.object()
//!     .field("name", s.string().min_length(2))
//!     .field("email", s.string().email())
//!     .field("age", s.number().optional());
//! ```
//!
//! # String Validation
//!
//! ## Built-in Patterns
//!
//! ```rust
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Email validation
//! let schema = s.string().email();
//! assert!(schema.validate(&"user@example.com".to_string()).is_ok());
//!
//! // URL validation
//! let schema = s.string().url();
//! assert!(schema.validate(&"https://example.com".to_string()).is_ok());
//!
//! // Date validation (YYYY-MM-DD)
//! let schema = s.string().date();
//! assert!(schema.validate(&"2024-01-15".to_string()).is_ok());
//!
//! // Time validation (HH:MM:SS)
//! let schema = s.string().time();
//! assert!(schema.validate(&"13:45:30".to_string()).is_ok());
//!
//! // UUID validation
//! let schema = s.string().uuid();
//! assert!(schema.validate(&"123e4567-e89b-42d3-a456-556642440000".to_string()).is_ok());
//!
//! // IPv4 validation
//! let schema = s.string().ipv4();
//! assert!(schema.validate(&"192.168.1.1".to_string()).is_ok());
//!
//! // Phone validation
//! let schema = s.string().phone();
//! assert!(schema.validate(&"+1234567890".to_string()).is_ok());
//!
//! // Username validation
//! let schema = s.string().username();
//! assert!(schema.validate(&"john_doe".to_string()).is_ok());
//!
//! // Password validation
//! let schema = s.string().password();
//! assert!(schema.validate(&"Password123".to_string()).is_ok());
//! ```
//!
//! ## Transformations
//!
//! ```rust
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Transform then validate
//! let schema = s.string()
//!     .transform(|s| s.trim().to_lowercase())
//!     .email();
//!
//! // Validate then transform
//! let schema = s.string()
//!     .email()
//!     .transform(|s| s.to_uppercase());
//!
//! // Multiple transforms
//! let schema = s.string()
//!     .transform(|s| s.trim().to_string())
//!     .email()
//!     .transform(|s| s.to_lowercase())
//!     .max_length(50);
//! ```
//!
//! # Object Validation
//!
//! ```rust
//! use schema_validator::{schema, Schema, Validate, ValidateAs};
//! use std::collections::HashMap;
//! use std::any::Any;
//! use serde_json::json;
//!
//! #[derive(Debug, PartialEq, Clone, Validate)]
//! struct User {
//!     name: String,
//!     email: String,
//!     age: Option<f64>,
//! }
//!
//! let s = schema();
//!
//! // Define schema
//! let schema = s.object()
//!     .field("name", s.string().min_length(2))
//!     .field("email", s.string().email())
//!     .field("age", s.number().optional());
//!
//! // Validate HashMap
//! let mut obj = HashMap::new();
//! obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
//! obj.insert("email".to_string(), Box::new("john@example.com".to_string()) as Box<dyn Any>);
//! obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);
//!
//! let user: User = schema.validate_as(&obj).unwrap();
//!
//! // Validate JSON
//! let json = json!({
//!     "name": "John",
//!     "email": "john@example.com",
//!     "age": 30
//! });
//!
//! let user: User = schema.validate_as(&json).unwrap();
//! ```
//!
//! # Error Handling
//!
//! ```rust
//! use schema_validator::{schema, Schema};
//!
//! let s = schema();
//!
//! // Custom error messages
//! let schema = s.string()
//!     .email()
//!     .max_length(50)
//!     .set_message("INVALID_EMAIL", "Invalid email format (max 50 chars)");
//!
//! // Error handling
//! match schema.validate(&"not-an-email".to_string()) {
//!     Ok(email) => println!("Valid email: {}", email),
//!     Err(err) => {
//!         println!("Error code: {}", err.code);      // "INVALID_EMAIL"
//!         println!("Error message: {}", err.message); // "Invalid email format (max 50 chars)"
//!     }
//! }
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
    /// let schema = s.string()
    ///     .min_length(3)
    ///     .max_length(50);
    ///
    /// // Email validation with transformation
    /// let schema = s.string()
    ///     .transform(|s| s.trim().to_lowercase())
    ///     .email();
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
    /// let schema = s.number();
    /// assert!(schema.validate(&42.0).is_ok());
    ///
    /// // With type coercion
    /// let schema = s.coerce().number();
    /// assert!(schema.validate(&"42".to_string()).is_ok());
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
    /// let schema = s.boolean();
    /// assert!(schema.validate(&true).is_ok());
    ///
    /// // With type coercion
    /// let schema = s.coerce().boolean();
    /// assert!(schema.validate(&1_i64).is_ok());
    /// ```
    pub fn boolean(&self) -> BooleanSchema {
        BooleanSchema::new(self.coerce)
    }

    /// Creates an object validation schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema, Validate, ValidateAs};
    /// use std::collections::HashMap;
    /// use std::any::Any;
    ///
    /// #[derive(Debug, PartialEq, Clone, Validate)]
    /// struct User {
    ///     name: String,
    ///     email: String,
    ///     age: Option<f64>,
    /// }
    ///
    /// let s = schema();
    ///
    /// // Define schema
    /// let schema = s.object()
    ///     .field("name", s.string().min_length(2))
    ///     .field("email", s.string().email())
    ///     .field("age", s.number().optional());
    ///
    /// // Create object
    /// let mut obj = HashMap::new();
    /// obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    /// obj.insert("email".to_string(), Box::new("john@example.com".to_string()) as Box<dyn Any>);
    /// obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);
    ///
    /// // Validate and convert to User struct
    /// let user: User = schema.validate_as(&obj).unwrap();
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
    /// let schema = s.coerce().string();
    /// assert!(schema.validate(&42_i64).is_ok());
    ///
    /// // Convert string to number
    /// let schema = s.coerce().number();
    /// assert!(schema.validate(&"42".to_string()).is_ok());
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