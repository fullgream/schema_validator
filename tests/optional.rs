use schema_validator::{schema, Schema, ValidateAs, Validate};
use std::collections::HashMap;
use std::any::Any;

#[test]
fn test_optional_string() {
    let s = schema();
    let schema = s.string().optional();

    // Valid values
    assert!(schema.validate(&Some("hello".to_string())).is_ok());
    assert!(schema.validate(&None::<String>).is_ok());
    assert!(schema.validate(&"hello".to_string()).is_ok());

    // Invalid values
    assert!(schema.validate(&42_i64).is_err());
    assert!(schema.validate(&true).is_err());
}

#[test]
fn test_optional_number() {
    let s = schema();
    let schema = s.number().optional();

    // Valid values
    assert!(schema.validate(&Some(42.0)).is_ok());
    assert!(schema.validate(&None::<f64>).is_ok());
    assert!(schema.validate(&42.0).is_ok());

    // Invalid values
    assert!(schema.validate(&"42".to_string()).is_err());
    assert!(schema.validate(&true).is_err());
}

#[test]
fn test_optional_boolean() {
    let s = schema();
    let schema = s.boolean().optional();

    // Valid values
    assert!(schema.validate(&Some(true)).is_ok());
    assert!(schema.validate(&None::<bool>).is_ok());
    assert!(schema.validate(&false).is_ok());

    // Invalid values
    assert!(schema.validate(&"true".to_string()).is_err());
    assert!(schema.validate(&42_i64).is_err());
}

#[test]
fn test_optional_with_coercion() {
    let s = schema();

    // String coercion
    let schema = s.coerce().string().optional();
    assert_eq!(schema.validate(&42_i64).unwrap(), Some("42".to_string()));
    assert_eq!(schema.validate(&None::<String>).unwrap(), None);

    // Number coercion
    let schema = s.coerce().number().optional();
    assert_eq!(schema.validate(&"42".to_string()).unwrap(), Some(42.0));
    assert_eq!(schema.validate(&None::<f64>).unwrap(), None);

    // Boolean coercion
    let schema = s.coerce().boolean().optional();
    assert_eq!(schema.validate(&1_i64).unwrap(), Some(true));
    assert_eq!(schema.validate(&None::<bool>).unwrap(), None);
}

#[derive(Debug, PartialEq, Clone, Validate)]
struct User {
    name: String,
    age: Option<f64>,
    is_active: bool,
}

#[test]
fn test_optional_object_field() {
    let s = schema();

    // Define schema with optional field
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number().optional())
        .field("is_active", s.boolean());

    // Object with all fields
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn Any>);

    let user: User = schema.validate_as(&obj).unwrap();
    assert_eq!(user, User {
        name: "John".to_string(),
        age: Some(30.0),
        is_active: true,
    });

    // Object with missing optional field
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(None::<f64>) as Box<dyn Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn Any>);

    let user: User = schema.validate_as(&obj).unwrap();
    assert_eq!(user, User {
        name: "John".to_string(),
        age: None,
        is_active: true,
    });

    // Missing required field should fail
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(None::<f64>) as Box<dyn Any>);

    let result: Result<User, _> = schema.validate_as(&obj);
    assert!(result.is_err());
}

#[test]
fn test_optional_with_transform() {
    let s = schema();

    // Transform optional string to optional length
    let schema = s.string()
        .optional()
        .transform(|opt_str| opt_str.map(|s| s.len()));

    assert_eq!(schema.validate(&Some("hello".to_string())).unwrap(), Some(5));
    assert_eq!(schema.validate(&None::<String>).unwrap(), None);

    // Transform optional number to optional boolean
    let schema = s.number()
        .optional()
        .transform(|opt_num| opt_num.map(|n| n > 0.0));

    assert_eq!(schema.validate(&Some(42.0)).unwrap(), Some(true));
    assert_eq!(schema.validate(&Some(-1.0)).unwrap(), Some(false));
    assert_eq!(schema.validate(&None::<f64>).unwrap(), None);
}