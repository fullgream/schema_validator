use std::any::Any;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;

type TransformFn = Box<dyn Fn(String) -> String>;

pub struct StringSchema {
    coerce: bool,
    error_config: Option<ErrorConfig>,
    transforms: Vec<TransformFn>,
}

impl std::fmt::Debug for StringSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringSchema")
            .field("coerce", &self.coerce)
            .field("error_config", &self.error_config)
            .field("transforms_count", &self.transforms.len())
            .finish()
    }
}

impl StringSchema {
    pub fn new(coerce: bool) -> Self {
        StringSchema {
            coerce,
            error_config: None,
            transforms: Vec::new(),
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

    pub fn transform<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String + 'static,
    {
        self.transforms.push(Box::new(f));
        self
    }

    fn apply_transforms(&self, mut value: String) -> String {
        for transform in &self.transforms {
            value = transform(value);
        }
        value
    }
}

impl Schema for StringSchema {
    type Output = String;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let result = if let Some(s) = value.downcast_ref::<String>() {
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
        };

        result.map(|s| self.apply_transforms(s))
    }
}

fn type_name(_value: &dyn Any) -> &'static str {
    std::any::type_name::<String>()
}