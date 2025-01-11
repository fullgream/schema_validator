use std::any::Any;
use std::marker::PhantomData;
use crate::error::{ValidationError, ValidationResult, ErrorType};
use crate::schema::{Schema, clone};

/// A schema that makes another schema optional.
///
/// # Examples
///
/// ```
/// use schema_validator::{schema, Schema};
///
/// let s = schema();
/// let schema = s.string().optional();
///
/// // Valid values
/// assert!(schema.validate(&Some("hello".to_string())).is_ok());
/// assert!(schema.validate(&None::<String>).is_ok());
///
/// // Invalid values still fail
/// assert!(schema.validate(&42_i64).is_err());
/// ```
pub struct OptionalSchema<S> {
    schema: S,
    _phantom: PhantomData<S>,
}

impl<S> OptionalSchema<S> {
    pub fn new(schema: S) -> Self {
        OptionalSchema {
            schema,
            _phantom: PhantomData,
        }
    }

    pub fn transform<F, T>(self, f: F) -> TransformedOptionalSchema<S, T>
    where
        F: Fn(Option<S::Output>) -> T + 'static,
        T: 'static + clone::CloneAny,
        S: Schema,
    {
        TransformedOptionalSchema {
            schema: self,
            transform: Box::new(f),
            _phantom: PhantomData,
        }
    }
}

pub struct TransformedOptionalSchema<S: Schema, T> {
    schema: OptionalSchema<S>,
    transform: Box<dyn Fn(Option<S::Output>) -> T>,
    _phantom: PhantomData<T>,
}

impl<S: Schema, T: clone::CloneAny + 'static> Schema for TransformedOptionalSchema<S, T> where S::Output: Clone {
    type Output = T;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let opt = self.schema.validate(value)?;
        Ok((self.transform)(opt))
    }
}

impl<S: Schema> Schema for OptionalSchema<S> where S::Output: Clone {
    type Output = Option<S::Output>;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        if let Some(none) = value.downcast_ref::<Option<()>>() {
            if none.is_none() {
                Ok(None)
            } else {
                Err(ValidationError::new(
                    ErrorType::Type {
                        expected: "Option",
                        got: "Unknown",
                    },
                    None,
                ))
            }
        } else if let Some(option) = value.downcast_ref::<Option<S::Output>>() {
            Ok(option.clone())
        } else if let Some(option) = value.downcast_ref::<Option<Box<dyn Any>>>() {
            match option {
                None => Ok(None),
                Some(boxed) => {
                    let val = self.schema.validate(boxed.as_ref())?;
                    Ok(Some(val))
                }
            }
        } else {
            // Try to validate the value directly
            match self.schema.validate(value) {
                Ok(val) => Ok(Some(val)),
                Err(err) => Err(err),
            }
        }
    }
}