pub mod error;
pub mod schema;

pub use error::{ValidationError, ValidationResult};
pub use schema::Schema;
use schema::string::StringSchema;
use schema::number::NumberSchema;
use schema::boolean::BooleanSchema;

#[derive(Debug)]
pub struct SchemaBuilder {
    coerce: bool,
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self { coerce: false }
    }
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn string(&self) -> StringSchema {
        StringSchema::new(self.coerce)
    }

    pub fn number(&self) -> NumberSchema {
        NumberSchema::new(self.coerce)
    }

    pub fn boolean(&self) -> BooleanSchema {
        BooleanSchema::new(self.coerce)
    }
}

#[derive(Debug)]
pub struct CoerceBuilder {
    builder: SchemaBuilder,
}

impl CoerceBuilder {
    pub fn string(&self) -> StringSchema {
        self.builder.string()
    }

    pub fn number(&self) -> NumberSchema {
        self.builder.number()
    }

    pub fn boolean(&self) -> BooleanSchema {
        self.builder.boolean()
    }
}

impl SchemaBuilder {
    pub fn coerce(&self) -> CoerceBuilder {
        CoerceBuilder {
            builder: SchemaBuilder { coerce: true },
        }
    }
}

pub fn schema() -> SchemaBuilder {
    SchemaBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_validation() {
        let s = schema();

        // Valid cases
        assert!(s.string().validate(&"hello".to_string()).is_ok());
        assert!(s.boolean().validate(&true).is_ok());
        assert!(s.number().validate(&50.0).is_ok());

        // Invalid cases
        let num: i64 = 42;
        assert!(s.string().validate(&num).is_err());
    }

    #[test]
    fn test_coercion() {
        let s = schema();

        // String coercion
        let num: i64 = 42;
        let coerced_str = s.coerce().string().validate(&num).unwrap();
        assert_eq!(coerced_str, "42");

        // Number coercion
        let str_num = "123".to_string();
        let coerced_num = s.coerce().number().validate(&str_num).unwrap();
        assert_eq!(coerced_num, 123.0);

        // Boolean coercion
        let num_one: i64 = 1;
        let coerced_bool = s.coerce().boolean().validate(&num_one).unwrap();
        assert_eq!(coerced_bool, true);

        let num_zero: i64 = 0;
        let coerced_bool = s.coerce().boolean().validate(&num_zero).unwrap();
        assert_eq!(coerced_bool, false);
    }
}
