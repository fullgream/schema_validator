use schema_validator::{schema, Schema, schema::clone::CloneAny};
use std::any::Any;
use std::collections::HashMap;

#[test]
fn test_object_validation() {
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

    let result = schema.validate(&obj);
    assert!(result.is_ok());

    // Missing field
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);

    let err = schema.validate(&obj).unwrap_err();
    assert_eq!(err.code, "VALIDATION_ERROR");
    assert!(err.message.contains("is_active"));
    assert!(err.message.contains("Missing required field"));

    // Wrong type
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new(42.0) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn std::any::Any>);

    let err = schema.validate(&obj).unwrap_err();
    assert_eq!(err.code, "VALIDATION_ERROR");
    assert!(err.message.contains("name"));
    assert!(err.message.contains("Type error"));
}

#[test]
fn test_object_transform() {
    #[derive(Debug, PartialEq, Clone)]
    struct User {
        name: String,
        age: f64,
    }

    impl CloneAny for User {
        fn clone_any(&self) -> Box<dyn Any> {
            Box::new(self.clone())
        }
    }

    let s = schema();

    // Define schema with transformation
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .transform(|fields| {
            User {
                name: fields.get("name").unwrap().downcast_ref::<String>().unwrap().clone(),
                age: *fields.get("age").unwrap().downcast_ref::<f64>().unwrap(),
            }
        });

    // Test transformation
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);

    let result: User = schema.validate(&obj).unwrap();
    assert_eq!(result.name, "John");
    assert_eq!(result.age, 30.0);
}

#[test]
fn test_object_custom_errors() {
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

    let err = schema.validate(&obj).unwrap_err();
    assert_eq!(err.code, "INVALID_USER");
    assert_eq!(err.message, "Invalid user data");
}