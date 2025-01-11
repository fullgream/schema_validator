pub mod string;
pub mod boolean;
pub mod number;

use crate::error::ValidationResult;
use std::any::Any;

pub trait Schema {
    type Output;
    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output>;
}