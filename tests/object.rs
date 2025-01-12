use schema_validator::{schema, ValidateAs, Validate};
use std::collections::HashMap;

#[test]
fn test_object_validation() {
    #[derive(Debug, PartialEq, Clone, Validate)]
    struct User {
        name: String,
        age: f64,
        is_active: bool,
    }

    let s = schema();

    // Define schema
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .field("is_active", s.boolean());

    // Valid object
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn std::any::Any>);

    let user: User = schema.validate_as(&obj).unwrap();
    assert_eq!(user.name, "John");
    assert_eq!(user.age, 30.0);
    assert_eq!(user.is_active, true);

    // Missing field
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);

    let err = schema.validate_as::<User>(&obj).unwrap_err();
    assert_eq!(err.code, "VALIDATION_ERROR");
    assert!(err.message.contains("is_active"));
    assert!(err.message.contains("Missing required field"));

    // Wrong type
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new(42.0) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn std::any::Any>);

    let err = schema.validate_as::<User>(&obj).unwrap_err();
    assert_eq!(err.code, "VALIDATION_ERROR");
    assert!(err.message.contains("name"));
    assert!(err.message.contains("Type error"));
}

#[test]
fn test_object_custom_errors() {
    #[derive(Debug, PartialEq, Clone, Validate)]
    struct User {
        name: String,
        age: f64,
    }

    let s = schema();

    // Define schema with custom error
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .set_message("INVALID_USER", "Invalid user data");

    // Test custom error
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new(42.0) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new("not a number") as Box<dyn std::any::Any>);

    let err = schema.validate_as::<User>(&obj).unwrap_err();
    assert_eq!(err.code, "INVALID_USER");
    assert_eq!(err.message, "Invalid user data");
}