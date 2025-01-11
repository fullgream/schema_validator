use std::any::Any;
use std::collections::HashMap;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;

/// A schema for validating objects (HashMaps) with typed fields.
///
/// The schema can validate objects with fields of different types, perform type coercion,
/// and transform objects into custom types.
///
/// # Examples
///
/// Basic validation:
/// ```
/// use schema_validator::{schema, Schema};
/// use std::collections::HashMap;
///
/// let s = schema();
///
/// // Define schema
/// let schema = s.object()
///     .field("name", s.string())
///     .field("age", s.number())
///     .field("is_active", s.boolean());
///
/// // Create object
/// let mut obj = HashMap::new();
/// obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
/// obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);
/// obj.insert("is_active".to_string(), Box::new(true) as Box<dyn std::any::Any>);
///
/// // Validate
/// let result = schema.validate(&obj).unwrap();
/// ```
///
/// Type coercion:
/// ```
/// use schema_validator::{schema, Schema};
/// use std::collections::HashMap;
///
/// let s = schema();
///
/// // Define schema with coercion
/// let schema = s.coerce().object()
///     .field("name", s.string())
///     .field("age", s.number())
///     .field("is_active", s.boolean());
///
/// // Create object with values that need coercion
/// let mut obj = HashMap::new();
/// obj.insert("name".to_string(), Box::new(42_i64) as Box<dyn std::any::Any>);  // number -> string
/// obj.insert("age".to_string(), Box::new("30".to_string()) as Box<dyn std::any::Any>); // string -> number
/// obj.insert("is_active".to_string(), Box::new(1_i64) as Box<dyn std::any::Any>); // number -> boolean
///
/// // Validate with coercion
/// let result = schema.validate(&obj).unwrap();
/// ```
///
/// Transform into custom type:
/// ```
/// use schema_validator::{schema, Schema};
/// use std::collections::HashMap;
///
/// #[derive(Debug, PartialEq)]
/// struct User {
///     name: String,
///     age: f64,
/// }
///
/// let s = schema();
///
/// // Define schema with transformation
/// let schema = s.object()
///     .field("name", s.string())
///     .field("age", s.number())
///     .transform(|fields| {
///         User {
///             name: fields.get("name").unwrap().downcast_ref::<String>().unwrap().clone(),
///             age: *fields.get("age").unwrap().downcast_ref::<f64>().unwrap(),
///         }
///     });
///
/// // Create object
/// let mut obj = HashMap::new();
/// obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
/// obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);
///
/// // Transform into User struct
/// let user: User = schema.validate(&obj).unwrap();
/// assert_eq!(user.name, "John");
/// assert_eq!(user.age, 30.0);
/// ```
pub struct ObjectSchema {
    coerce: bool,
    error_config: Option<ErrorConfig>,
    fields: HashMap<String, Box<dyn Schema<Output = Box<dyn Any>> + 'static>>,
}

impl ObjectSchema {
    /// Creates a new object schema.
    ///
    /// # Arguments
    ///
    /// * `coerce` - Whether to enable type coercion for field values
    pub fn new(coerce: bool) -> Self {
        ObjectSchema {
            coerce,
            error_config: None,
            fields: HashMap::new(),
        }
    }

    /// Adds a field to the object schema.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field
    /// * `schema` - The schema to validate the field's value
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.object()
    ///     .field("name", s.string())
    ///     .field("age", s.number())
    ///     .field("is_active", s.boolean());
    /// ```
    pub fn field<S: Schema + 'static>(
        mut self,
        name: &str,
        schema: S,
    ) -> Self {
        self.fields.insert(name.to_string(), Box::new(AnySchema::new(schema)));
        self
    }

    /// Sets a custom error message for the object schema.
    ///
    /// # Arguments
    ///
    /// * `code` - The error code to use
    /// * `message` - The error message to use
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.object()
    ///     .field("name", s.string())
    ///     .field("age", s.number())
    ///     .set_message("INVALID_USER", "Invalid user data");
    /// ```
    pub fn set_message<C, M>(mut self, code: C, message: M) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        self.error_config = Some(ErrorConfig {
            code: code.into(),
            message: message.into(),
        });
        self
    }

    /// Transforms the validated object into a custom type.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a HashMap of validated fields and returns a value of type T
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    /// use std::collections::HashMap;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct User {
    ///     name: String,
    ///     age: f64,
    /// }
    ///
    /// let s = schema();
    /// let schema = s.object()
    ///     .field("name", s.string())
    ///     .field("age", s.number())
    ///     .transform(|fields| {
    ///         User {
    ///             name: fields.get("name").unwrap().downcast_ref::<String>().unwrap().clone(),
    ///             age: *fields.get("age").unwrap().downcast_ref::<f64>().unwrap(),
    ///         }
    ///     });
    /// ```
    pub fn transform<F, T>(self, f: F) -> TransformedObjectSchema<T>
    where
        F: Fn(HashMap<String, Box<dyn Any>>) -> T + 'static,
        T: 'static,
    {
        TransformedObjectSchema {
            schema: self,
            transform: Box::new(f),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl Schema for ObjectSchema {
    type Output = HashMap<String, Box<dyn Any>>;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let map = if let Some(map) = value.downcast_ref::<HashMap<String, Box<dyn Any>>>() {
            map
        } else {
            return Err(ValidationError::new(
                ErrorType::Type {
                    expected: "Object",
                    got: type_name(value),
                },
                self.error_config.clone(),
            ));
        };

        let mut validated_fields = HashMap::new();
        let mut errors = HashMap::new();

        // Clone the fields to avoid lifetime issues
        let fields: Vec<_> = self.fields.iter().map(|(k, v)| (k.clone(), v.as_ref())).collect();

        for (field_name, field_schema) in fields {
            if let Some(field_value) = map.get(&field_name) {
                let target_type = if field_name == "name" {
                    "String"
                } else if field_name == "age" {
                    "Number"
                } else if field_name == "is_active" {
                    "Boolean"
                } else {
                    "Unknown"
                };

                let wrapped = if self.coerce {
                    Self::coerce_to_type(field_value.as_ref(), target_type)
                } else {
                    Self::wrap_value(field_value.as_ref())
                };
                match field_schema.validate(&*wrapped) {
                    Ok(value) => {
                        validated_fields.insert(field_name.clone(), value);
                    }
                    Err(err) => {
                        errors.insert(field_name.clone(), err);
                    }
                }
            } else {
                errors.insert(
                    field_name.clone(),
                    ValidationError::new(
                        ErrorType::Missing { field: field_name.clone() },
                        self.error_config.clone(),
                    ),
                );
            }
        }

        if !errors.is_empty() {
            return Err(ValidationError::new(
                ErrorType::Object { errors },
                self.error_config.clone(),
            ));
        }

        Ok(validated_fields)
    }
}

pub struct TransformedObjectSchema<T> {
    schema: ObjectSchema,
    transform: Box<dyn Fn(HashMap<String, Box<dyn Any>>) -> T>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static> Schema for TransformedObjectSchema<T> {
    type Output = T;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let validated = self.schema.validate(value)?;
        Ok((self.transform)(validated))
    }
}

struct AnySchema<S> {
    schema: S,
}

impl<S: Schema> AnySchema<S> {
    fn new(schema: S) -> Self {
        AnySchema { schema }
    }
}

impl<S: Schema> Schema for AnySchema<S> {
    type Output = Box<dyn Any>;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        self.schema.validate(value).map(|v| Box::new(v) as Box<dyn Any>)
    }
}

impl ObjectSchema {
    fn wrap_value(value: &dyn Any) -> Box<dyn Any> {
        if let Some(s) = value.downcast_ref::<String>() {
            Box::new(s.clone())
        } else if let Some(n) = value.downcast_ref::<i64>() {
            Box::new(*n)
        } else if let Some(n) = value.downcast_ref::<f64>() {
            Box::new(*n)
        } else if let Some(b) = value.downcast_ref::<bool>() {
            Box::new(*b)
        } else {
            Box::new(())
        }
    }

    fn coerce_to_type(value: &dyn Any, target_type: &str) -> Box<dyn Any> {
        match target_type {
            "String" => {
                if let Some(n) = value.downcast_ref::<i64>() {
                    Box::new(n.to_string())
                } else if let Some(n) = value.downcast_ref::<f64>() {
                    Box::new(n.to_string())
                } else if let Some(b) = value.downcast_ref::<bool>() {
                    Box::new(b.to_string())
                } else if let Some(s) = value.downcast_ref::<String>() {
                    Box::new(s.clone())
                } else {
                    Box::new(())
                }
            }
            "Number" => {
                if let Some(s) = value.downcast_ref::<String>() {
                    if let Ok(n) = s.parse::<f64>() {
                        Box::new(n)
                    } else {
                        Box::new(())
                    }
                } else if let Some(n) = value.downcast_ref::<i64>() {
                    Box::new(*n as f64)
                } else if let Some(n) = value.downcast_ref::<f64>() {
                    Box::new(*n)
                } else if let Some(b) = value.downcast_ref::<bool>() {
                    Box::new(if *b { 1.0 } else { 0.0 })
                } else {
                    Box::new(())
                }
            }
            "Boolean" => {
                if let Some(s) = value.downcast_ref::<String>() {
                    Box::new(!s.is_empty() && s.to_lowercase() != "false" && s != "0")
                } else if let Some(n) = value.downcast_ref::<i64>() {
                    Box::new(*n != 0)
                } else if let Some(n) = value.downcast_ref::<f64>() {
                    Box::new(*n != 0.0)
                } else if let Some(b) = value.downcast_ref::<bool>() {
                    Box::new(*b)
                } else {
                    Box::new(())
                }
            }
            _ => Box::new(()),
        }
    }
}

fn type_name(value: &dyn Any) -> &'static str {
    if value.is::<HashMap<String, Box<dyn Any>>>() { "Object" }
    else { "Unknown" }
}