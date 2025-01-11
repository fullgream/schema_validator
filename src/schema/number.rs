use std::any::Any;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;

type TransformFn = Box<dyn Fn(f64) -> f64>;

pub struct NumberSchema {
    coerce: bool,
    error_config: Option<ErrorConfig>,
    transforms: Vec<TransformFn>,
}

impl std::fmt::Debug for NumberSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NumberSchema")
            .field("coerce", &self.coerce)
            .field("error_config", &self.error_config)
            .field("transforms_count", &self.transforms.len())
            .finish()
    }
}

impl NumberSchema {
    pub fn new(coerce: bool) -> Self {
        NumberSchema {
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
        F: Fn(f64) -> f64 + 'static,
    {
        self.transforms.push(Box::new(f));
        self
    }

    fn apply_transforms(&self, mut value: f64) -> f64 {
        for transform in &self.transforms {
            value = transform(value);
        }
        value
    }
}

impl Schema for NumberSchema {
    type Output = f64;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let result = if let Some(n) = value.downcast_ref::<f64>() {
            Ok(*n)
        } else if self.coerce {
            // Try to coerce different types to number
            if let Some(n) = value.downcast_ref::<i64>() {
                Ok(*n as f64)
            } else if let Some(s) = value.downcast_ref::<String>() {
                s.parse::<f64>().map_err(|_| ValidationError::new(
                    ErrorType::Coercion {
                        from: "String",
                        to: "Number",
                    },
                    self.error_config.clone(),
                ))
            } else if let Some(b) = value.downcast_ref::<bool>() {
                Ok(if *b { 1.0 } else { 0.0 })
            } else {
                Err(ValidationError::new(
                    ErrorType::Coercion {
                        from: type_name(value),
                        to: "Number",
                    },
                    self.error_config.clone(),
                ))
            }
        } else {
            Err(ValidationError::new(
                ErrorType::Type {
                    expected: "Number",
                    got: type_name(value),
                },
                self.error_config.clone(),
            ))
        };

        result.map(|n| self.apply_transforms(n))
    }
}

fn type_name(_value: &dyn Any) -> &'static str {
    std::any::type_name::<f64>()
}