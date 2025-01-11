use std::any::Any;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;

pub struct StringSchema {
    coerce: bool,
    error_config: Option<ErrorConfig>,
    transform: Option<Box<dyn Fn(String) -> Box<dyn Any>>>,
}

impl std::fmt::Debug for StringSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringSchema")
            .field("coerce", &self.coerce)
            .field("error_config", &self.error_config)
            .field("has_transform", &self.transform.is_some())
            .finish()
    }
}

impl StringSchema {
    pub fn new(coerce: bool) -> Self {
        StringSchema {
            coerce,
            error_config: None,
            transform: None,
        }
    }

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

    pub fn transform<F, T>(mut self, f: F) -> TransformedSchema<T>
    where
        F: Fn(String) -> T + 'static,
        T: 'static,
    {
        TransformedSchema {
            schema: self,
            transform: Box::new(move |s| Box::new(f(s))),
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct TransformedSchema<T> {
    schema: StringSchema,
    transform: Box<dyn Fn(String) -> Box<dyn Any>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static> TransformedSchema<T> {
    pub fn transform<F, U>(self, f: F) -> TransformedSchema<U>
    where
        F: Fn(T) -> U + 'static,
        U: 'static,
    {
        let old_transform = self.transform;
        TransformedSchema {
            schema: self.schema,
            transform: Box::new(move |s| {
                let result = old_transform(s);
                let value = result.downcast::<T>().unwrap();
                Box::new(f(*value))
            }),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn set_message<C, M>(mut self, code: C, message: M) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        self.schema.error_config = Some(ErrorConfig {
            code: code.into(),
            message: message.into(),
        });
        self
    }
}

impl<T: 'static> Schema for TransformedSchema<T> {
    type Output = T;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let string_result = if let Some(s) = value.downcast_ref::<String>() {
            Ok(s.clone())
        } else if self.schema.coerce {
            // Try to coerce different types to string
            if let Some(n) = value.downcast_ref::<i64>() {
                Ok(n.to_string())
            } else if let Some(n) = value.downcast_ref::<f64>() {
                Ok(n.to_string())
            } else if let Some(b) = value.downcast_ref::<bool>() {
                Ok(b.to_string())
            } else {
                Err(ValidationError::new(
                    ErrorType::Coercion {
                        from: type_name(value),
                        to: "String",
                    },
                    self.schema.error_config.clone(),
                ))
            }
        } else {
            Err(ValidationError::new(
                ErrorType::Type {
                    expected: "String",
                    got: type_name(value),
                },
                self.schema.error_config.clone(),
            ))
        };

        string_result.map(|s| {
            let result = (self.transform)(s);
            *result.downcast::<T>().unwrap()
        })
    }
}

impl Schema for StringSchema {
    type Output = String;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        if let Some(s) = value.downcast_ref::<String>() {
            Ok(s.clone())
        } else if self.coerce {
            // Try to coerce different types to string
            if let Some(n) = value.downcast_ref::<i64>() {
                Ok(n.to_string())
            } else if let Some(n) = value.downcast_ref::<f64>() {
                Ok(n.to_string())
            } else if let Some(b) = value.downcast_ref::<bool>() {
                Ok(b.to_string())
            } else {
                Err(ValidationError::new(
                    ErrorType::Coercion {
                        from: type_name(value),
                        to: "String",
                    },
                    self.error_config.clone(),
                ))
            }
        } else {
            Err(ValidationError::new(
                ErrorType::Type {
                    expected: "String",
                    got: type_name(value),
                },
                self.error_config.clone(),
            ))
        }
    }
}

fn type_name(value: &dyn Any) -> &'static str {
    if value.is::<String>() { "String" }
    else if value.is::<i64>() { "Integer" }
    else if value.is::<f64>() { "Float" }
    else if value.is::<bool>() { "Boolean" }
    else { "Unknown" }
}