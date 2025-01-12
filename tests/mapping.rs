use schema_validator::{schema, Schema, ValidateAs, Validate};
use std::collections::HashMap;
use std::any::Any;

#[derive(Debug, PartialEq, Clone, Validate)]
struct User {
    name: String,
    age: f64,
    is_active: bool,
}

#[test]
fn test_validate_as() {
    let s = schema();

    // Define schema
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .field("is_active", s.boolean());

    // Valid object
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn Any>);

    let user: User = schema.validate_as(&obj).unwrap();
    assert_eq!(user, User {
        name: "John".to_string(),
        age: 30.0,
        is_active: true,
    });

    // Missing field
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn Any>);

    let result = schema.validate(&obj);
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("Missing required field"));

    let result: Result<User, _> = schema.validate_as(&obj);
    assert!(result.is_err());

    // Wrong type
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new(42.0) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn Any>);

    let result: Result<User, _> = schema.validate_as(&obj);
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("Type error"));
}

#[derive(Debug, PartialEq, Clone, Validate)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn test_validate_as_custom_error() {
    let s = schema();

    // Define schema with custom error
    let schema = s.object()
        .field("x", s.number())
        .field("y", s.number())
        .set_message("INVALID_POINT", "Invalid point coordinates");

    // Missing field
    let mut obj = HashMap::new();
    obj.insert("x".to_string(), Box::new(10.0) as Box<dyn Any>);

    let result: Result<Point, _> = schema.validate_as(&obj);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, "INVALID_POINT");
    assert_eq!(err.message, "Invalid point coordinates");
}