use std::any::Any;
use std::collections::HashMap;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;
use crate::schema::mapping::{FromFields, ValidateAs};
use crate::schema::clone::CloneAny;
use serde_json::Value;

/// A schema for validating objects (HashMaps) with typed fields.
///
/// The schema can validate objects with fields of different types and transform
/// them into custom types.
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
/// Transform into custom type:
/// ```
/// use schema_validator::{schema, Schema};
/// use std::collections::HashMap;
/// use std::any::Any;
///
/// // Define a struct that implements Clone and Any
/// #[derive(Debug, PartialEq, Clone)]
/// struct User {
///     name: String,
///     age: f64,
/// }
///
/// // Implement CloneAny manually for the example
/// impl schema_validator::schema::clone::CloneAny for User {
///     fn clone_any(&self) -> Box<dyn Any> {
///         Box::new(self.clone())
///     }
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
    error_config: Option<ErrorConfig>,
    fields: HashMap<String, Box<dyn Schema<Output = Box<dyn Any>> + 'static>>,
}

impl ObjectSchema {
    /// Creates a new object schema.
    pub fn new() -> Self {
        ObjectSchema {
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
    /// use std::any::Any;
    ///
    /// // Define a struct that implements Clone and Any
    /// #[derive(Debug, PartialEq, Clone)]
    /// struct User {
    ///     name: String,
    ///     age: f64,
    /// }
    ///
    /// // Implement CloneAny manually for the example
    /// impl schema_validator::schema::clone::CloneAny for User {
    ///     fn clone_any(&self) -> Box<dyn Any> {
    ///         Box::new(self.clone())
    ///     }
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
        T: 'static + CloneAny,
    {
        TransformedObjectSchema {
            schema: self,
            transform: Box::new(f),
            _phantom: std::marker::PhantomData,
        }
    }

    fn validate_json(&self, json: &Value) -> ValidationResult<HashMap<String, Box<dyn Any>>> {
        match json {
            Value::Object(obj) => {
                let mut fields = HashMap::new();
                for (field_name, field_value) in obj {
                    match field_value {
                        Value::String(s) => {
                            fields.insert(field_name.clone(), Box::new(s.clone()) as Box<dyn Any>);
                        }
                        Value::Number(n) => {
                            if let Some(f) = n.as_f64() {
                                fields.insert(field_name.clone(), Box::new(f) as Box<dyn Any>);
                            }
                        }
                        Value::Bool(b) => {
                            fields.insert(field_name.clone(), Box::new(*b) as Box<dyn Any>);
                        }
                        Value::Null => {
                            fields.insert(field_name.clone(), Box::new(None::<()>) as Box<dyn Any>);
                        }
                        _ => {
                            return Err(ValidationError::new(
                                ErrorType::Type {
                                    expected: "String, Number, Boolean or Null",
                                    got: "Array or Object",
                                },
                                self.error_config.clone(),
                            ));
                        }
                    }
                }
                Ok(fields)
            }
            _ => Err(ValidationError::new(
                ErrorType::Type {
                    expected: "Object",
                    got: "Non-object JSON value",
                },
                self.error_config.clone(),
            )),
        }
    }
}

impl Schema for ObjectSchema {
    type Output = HashMap<String, Box<dyn Any>>;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let raw_fields: HashMap<String, Box<dyn Any>> = if let Some(map) = value.downcast_ref::<HashMap<String, Box<dyn Any>>>() {
            map.iter().map(|(k, v)| (k.clone(), Self::wrap_value(v.as_ref()))).collect()
        } else if let Some(json) = value.downcast_ref::<Value>() {
            self.validate_json(json)?
        } else {
            return Err(ValidationError::new(
                ErrorType::Type {
                    expected: "Object or JSON object",
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
            if let Some(field_value) = raw_fields.get(&field_name) {
                let wrapped = Self::wrap_value(field_value.as_ref());

                let wrapped_val = if let Some(opt) = wrapped.downcast_ref::<Option<Box<dyn Any>>>() {
                    match opt {
                        None => None,
                        Some(val) => Some(val.as_ref()),
                    }
                } else if let Some(opt) = wrapped.downcast_ref::<Option<()>>() {
                    if opt.is_none() {
                        None
                    } else {
                        Some(wrapped.as_ref())
                    }
                } else {
                    Some(wrapped.as_ref())
                };

                if let Err(err) = match wrapped_val {
                    None => field_schema.validate(&None::<()>),
                    Some(val) => field_schema.validate(val),
                }.and_then(|value| {
                    validated_fields.insert(field_name.clone(), value);
                    Ok(())
                }) {
                    errors.insert(field_name.clone(), err);
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
                ErrorType::Object { errors: errors.into_iter().collect() },
                self.error_config.clone(),
            ));
        }

        Ok(validated_fields)
    }
}

pub struct TransformedObjectSchema<T: 'static + CloneAny> {
    schema: ObjectSchema,
    transform: Box<dyn Fn(HashMap<String, Box<dyn Any>>) -> T>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static + CloneAny> Schema for TransformedObjectSchema<T> {
    type Output = T;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let fields = self.schema.validate(value)?;
        Ok((self.transform)(fields))
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
        } else if let Some(opt) = value.downcast_ref::<Option<f64>>() {
            Box::new(opt.clone())
        } else if let Some(opt) = value.downcast_ref::<Option<String>>() {
            Box::new(opt.clone())
        } else if let Some(opt) = value.downcast_ref::<Option<bool>>() {
            Box::new(opt.clone())
        } else if let Some(opt) = value.downcast_ref::<Option<Box<dyn Any>>>() {
            match opt {
                None => Box::new(None::<()>),
                Some(val) => Box::new(Some(Self::wrap_value(val.as_ref()))),
            }
        } else if let Some(opt) = value.downcast_ref::<Option<()>>() {
            Box::new(opt.clone())
        } else {
            Box::new(())
        }
    }
}

impl ValidateAs for ObjectSchema {
    fn validate_as<T: FromFields>(&self, value: &dyn Any) -> ValidationResult<T> {
        let fields = self.validate(value)?;
        T::from_fields(&fields).ok_or_else(|| ValidationError::new(
            ErrorType::Type {
                expected: "Object with required fields",
                got: "Object with missing or invalid fields",
            },
            self.error_config.clone(),
        ))
    }
}

fn type_name(value: &dyn Any) -> &'static str {
    if value.is::<HashMap<String, Box<dyn Any>>>() { "Object" }
    else if value.is::<Value>() { "JSON value" }
    else { "Unknown" }
}