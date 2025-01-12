use std::any::Any;
use std::fmt::Debug;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;
use crate::schema::clone::CloneAny;

pub struct LiteralSchema<T: 'static + Clone + PartialEq + Debug + CloneAny> {
    value: T,
    error_config: Option<ErrorConfig>,
}

impl<T: 'static + Clone + PartialEq + Debug + CloneAny> LiteralSchema<T> {
    pub(crate) fn new(value: T) -> Self {
        LiteralSchema {
            value,
            error_config: None,
        }
    }

    /// Sets a custom error message for the literal schema.
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
    /// let schema = s.literal("tuna")
    ///     .set_message("INVALID_VALUE", "Value must be 'tuna'");
    ///
    /// let err = schema.validate(&"salmon".to_string()).unwrap_err();
    /// assert_eq!(err.code, "INVALID_VALUE");
    /// assert_eq!(err.message, "Value must be 'tuna'");
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
}

impl<T: 'static + Clone + PartialEq + Debug + CloneAny> Schema for LiteralSchema<T> {
    type Output = T;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let value = if let Some(v) = value.downcast_ref::<T>() {
            v.clone()
        } else {
            return Err(ValidationError::new(
                ErrorType::Literal {
                    expected: format!("{:?}", self.value),
                    got: format!("{:?}", value),
                },
                self.error_config.clone(),
            ));
        };

        if value == self.value {
            Ok(value)
        } else {
            Err(ValidationError::new(
                ErrorType::Literal {
                    expected: format!("{:?}", self.value),
                    got: format!("{:?}", value),
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