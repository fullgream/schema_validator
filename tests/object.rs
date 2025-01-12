use schema_validator::{schema, Schema, Validate, ValidateAs};
use std::collections::HashMap;
use std::any::Any;
use serde_json::json;

#[derive(Debug, PartialEq, Clone, Validate)]
struct User {
    name: String,
    email: String,
    age: Option<f64>,
}

#[test]
fn test_object_validation() {
    let s = schema();

    // Define schema
    let schema = s.object()
        .field("name", s.string().min_length(2))
        .field("email", s.string().email())
        .field("age", s.number().optional());

    // Create object
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("email".to_string(), Box::new("john@example.com".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);

    // Valid object
    let user: User = schema.validate_as(&obj).unwrap();
    assert_eq!(user.name, "John");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.age, Some(30.0));

    // Invalid name (too short)
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("J".to_string()) as Box<dyn Any>);
    obj.insert("email".to_string(), Box::new("john@example.com".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);

    let err = schema.validate_as::<User>(&obj).unwrap_err();
    assert_eq!(err.code, "OBJECT_ERROR");
    assert!(err.message.contains("name"));
    assert!(err.message.contains("MIN_LENGTH_ERROR"));

    // Invalid email
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("email".to_string(), Box::new("not-an-email".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);

    let err = schema.validate_as::<User>(&obj).unwrap_err();
    assert_eq!(err.code, "OBJECT_ERROR");
    assert!(err.message.contains("email"));
    assert!(err.message.contains("pattern"));
}

#[test]
fn test_object_custom_errors() {
    let s = schema();

    // Define schema with custom error messages
    let schema = s.object()
        .field("name", s.string().min_length(2).set_message("INVALID_NAME", "Name must be at least 2 characters"))
        .field("email", s.string().email().set_message("INVALID_EMAIL", "Invalid email format"))
        .field("age", s.number().optional());

    // Invalid name
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("J".to_string()) as Box<dyn Any>);
    obj.insert("email".to_string(), Box::new("john@example.com".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);

    let err = schema.validate_as::<User>(&obj).unwrap_err();
    assert_eq!(err.code, "OBJECT_ERROR");
    assert!(err.message.contains("INVALID_NAME"));
    assert!(err.message.contains("Name must be at least 2 characters"));

    // Invalid email
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
    obj.insert("email".to_string(), Box::new("not-an-email".to_string()) as Box<dyn Any>);
    obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);

    let err = schema.validate_as::<User>(&obj).unwrap_err();
    assert_eq!(err.code, "OBJECT_ERROR");
    assert!(err.message.contains("INVALID_EMAIL"));
    assert!(err.message.contains("Invalid email format"));
}

#[test]
fn test_unknown_json() {
    let s = schema();

    // Define schema
    let schema = s.object()
        .field("name", s.string().min_length(2))
        .field("email", s.string().email())
        .field("age", s.number().optional());

    // Valid JSON
    let json = json!({
        "name": "John",
        "email": "john@example.com",
        "age": 30
    });

    let user: User = schema.validate_as(&json).unwrap();
    assert_eq!(user.name, "John");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.age, Some(30.0));

    // Invalid JSON (wrong types)
    let json = json!({
        "name": 42,
        "email": true,
        "age": "not a number"
    });

    let err = schema.validate_as::<User>(&json).unwrap_err();
    assert_eq!(err.code, "OBJECT_ERROR");
    assert!(err.message.contains("name"));
    assert!(err.message.contains("email"));
    assert!(err.message.contains("age"));
}

#[test]
fn test_unknown_json_with_coercion() {
    let s = schema();

    // Define schema with coercion
    let schema = s.object()
        .field("name", s.string().min_length(2))
        .field("email", s.string().email())
        .field("age", s.coerce().number().optional());

    // JSON with numbers and booleans
    let json = json!({
        "name": "John",
        "email": "john@example.com",
        "age": "30"  // String that can be coerced to number
    });

    let user: User = schema.validate_as(&json).unwrap();
    assert_eq!(user.name, "John");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.age, Some(30.0));

    // Invalid JSON (cannot be coerced)
    let json = json!({
        "name": [],  // Array cannot be coerced to string
        "email": {},  // Object cannot be coerced to string
        "age": "not a number"  // Cannot be coerced to number
    });

    let err = schema.validate_as::<User>(&json).unwrap_err();
    assert_eq!(err.code, "TYPE_ERROR");
    assert!(err.message.contains("expected String"));
    assert!(err.message.contains("got Array"));
}