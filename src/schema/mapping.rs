use std::any::Any;
use std::collections::HashMap;
use crate::error::ValidationResult;

/// A trait for types that can be created from validated object fields.
///
/// # Examples
///
/// ```
/// use schema_validator::{schema, Schema, FromFields, ValidateAs, ValidationResult};
/// use std::collections::HashMap;
/// use std::any::Any;
///
/// #[derive(Debug, PartialEq)]
/// struct User {
///     name: String,
///     age: f64,
/// }
///
/// impl FromFields for User {
///     fn from_fields(fields: &HashMap<String, Box<dyn Any>>) -> Option<Self> {
///         Some(User {
///             name: fields.get("name")?.downcast_ref::<String>()?.clone(),
///             age: *fields.get("age")?.downcast_ref::<f64>()?,
///         })
///     }
/// }
///
/// let s = schema();
///
/// // Define schema
/// let schema = s.object()
///     .field("name", s.string())
///     .field("age", s.number());
///
/// // Create object
/// let mut obj = HashMap::new();
/// obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
/// obj.insert("age".to_string(), Box::new(30.0) as Box<dyn Any>);
///
/// // Validate and convert to User
/// let user: ValidationResult<User> = schema.validate_as(&obj);
/// assert_eq!(user.unwrap(), User { name: "John".to_string(), age: 30.0 });
/// ```
pub trait FromFields: Sized {
    /// Creates an instance of Self from validated object fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A HashMap containing the validated field values
    ///
    /// # Returns
    ///
    /// Returns `Some(Self)` if all required fields are present and have the correct types,
    /// or `None` if any field is missing or has an incorrect type.
    fn from_fields(fields: &HashMap<String, Box<dyn Any>>) -> Option<Self>;
}

/// Extension trait for Schema to add validation with direct struct mapping.
pub trait ValidateAs {
    /// Validates the value and attempts to convert it to type T.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to convert to, must implement FromFields
    ///
    /// # Arguments
    ///
    /// * `value` - The value to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` if validation and conversion succeed, or `Err` if either fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema, FromFields, ValidateAs, ValidationResult};
    /// use std::collections::HashMap;
    /// use std::any::Any;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct User {
    ///     name: String,
    ///     age: f64,
    /// }
    ///
    /// impl FromFields for User {
    ///     fn from_fields(fields: &HashMap<String, Box<dyn Any>>) -> Option<Self> {
    ///         Some(User {
    ///             name: fields.get("name")?.downcast_ref::<String>()?.clone(),
    ///             age: *fields.get("age")?.downcast_ref::<f64>()?,
    ///         })
    ///     }
    /// }
    ///
    /// let s = schema();
    /// let schema = s.object()
    ///     .field("name", s.string())
    ///     .field("age", s.number());
    ///
    /// let mut obj = HashMap::new();
    /// obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    /// obj.insert("age".to_string(), Box::new(30.0) as Box<dyn Any>);
    ///
    /// let user: ValidationResult<User> = schema.validate_as(&obj);
    /// assert_eq!(user.unwrap(), User { name: "John".to_string(), age: 30.0 });
    /// ```
    fn validate_as<T: FromFields>(&self, value: &dyn Any) -> ValidationResult<T>;
}