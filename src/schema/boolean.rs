use std::any::Any;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;

type TransformFn = Box<dyn Fn(bool) -> bool>;

pub struct BooleanSchema {
    coerce: bool,
    error_config: Option<ErrorConfig>,
    transforms: Vec<TransformFn>,
}

impl std::fmt::Debug for BooleanSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BooleanSchema")
            .field("coerce", &self.coerce)
            .field("error_config", &self.error_config)
            .field("transforms_count", &self.transforms.len())
            .finish()
    }
}

impl BooleanSchema {
    pub fn new(coerce: bool) -> Self {
        BooleanSchema {
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
        F: Fn(bool) -> bool + 'static,
    {
        self.transforms.push(Box::new(f));
        self
    }

    fn apply_transforms(&self, mut value: bool) -> bool {
        for transform in &self.transforms {
            value = transform(value);
        }
        value
    }
}

impl Schema for BooleanSchema {
    type Output = bool;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let result = if let Some(b) = value.downcast_ref::<bool>() {
            Ok(*b)
        } else if self.coerce {
            // Try to coerce different types to boolean
            if let Some(n) = value.downcast_ref::<i64>() {
                Ok(*n != 0)
            } else if let Some(n) = value.downcast_ref::<f64>() {
                Ok(*n != 0.0)
            } else if let Some(s) = value.downcast_ref::<String>() {
                Ok(!s.is_empty())
            } else if let Some(opt) = value.downcast_ref::<Option<bool>>() {
                Ok(opt.is_some())
            } else if let Some(vec) = value.downcast_ref::<Vec<bool>>() {
                Ok(!vec.is_empty())
            } else if let Some(vec) = value.downcast_ref::<Vec<i64>>() {
                Ok(!vec.is_empty())
            } else if let Some(vec) = value.downcast_ref::<Vec<String>>() {
                Ok(!vec.is_empty())
            } else {
                Err(ValidationError::new(
                    ErrorType::Coercion {
                        from: type_name(value),
                        to: "Boolean",
                    },
                    self.error_config.clone(),
                ))
            }
        } else {
            Err(ValidationError::new(
                ErrorType::Type {
                    expected: "Boolean",
                    got: type_name(value),
                },
                self.error_config.clone(),
            ))
        };

        result.map(|b| self.apply_transforms(b))
    }
}

fn type_name(_value: &dyn Any) -> &'static str {
    std::any::type_name::<bool>()
}