# Schema Validator

A flexible, type-safe schema validation library for Rust with support for type coercion, transformations, and object validation.

## Features

- Type validation for strings, numbers, booleans, and objects
- Type coercion (e.g., number to string, string to number)
- Value transformations with type changes
- Object validation with nested fields
- Custom error messages
- Fluent API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
schema_validator = "0.1.0"
```

## Quick Start

```rust
use schema_validator::{schema, Schema};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = schema();

    // Basic type validation
    let valid_string = s.string().validate(&"hello".to_string())?;
    let valid_number = s.number().validate(&42.0)?;
    let valid_bool = s.boolean().validate(&true)?;

    // Type coercion
    let string_from_number = s.coerce().string().validate(&42)?; // "42"
    let number_from_string = s.coerce().number().validate(&"42".to_string())?; // 42.0
    let bool_from_number = s.coerce().boolean().validate(&1)?; // true

    // Object validation
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .field("is_active", s.boolean());

    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn std::any::Any>);

    let result = schema.validate(&obj)?;
    
    Ok(())
}
```

## Type Coercion Rules

### To String
- Numbers (both integer and float) -> their string representation
- Booleans -> "true" or "false"

### To Number
- Strings that contain valid numbers -> the corresponding number
- Booleans -> 1.0 for true, 0.0 for false

### To Boolean
- Numbers -> false for 0, true for any other number
- Strings -> false for "", true for any non-empty string

## Object Validation

The library supports validation of objects with nested fields:

```rust
use schema_validator::{schema, Schema};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = schema();

    // Define schema
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .field("is_active", s.boolean());

    // Create object
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);
    obj.insert("is_active".to_string(), Box::new(true) as Box<dyn std::any::Any>);

    // Validate
    let result = schema.validate(&obj)?;

    // Object with type coercion
    let schema = s.coerce().object()
        .field("name", s.string())
        .field("age", s.number())
        .field("is_active", s.boolean());

    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new(42) as Box<dyn std::any::Any>);  // number -> string
    obj.insert("age".to_string(), Box::new("30") as Box<dyn std::any::Any>); // string -> number
    obj.insert("is_active".to_string(), Box::new(1) as Box<dyn std::any::Any>); // number -> boolean

    let result = schema.validate(&obj)?;

    Ok(())
}
```

### Object Transformations

Objects can be transformed into Rust structs:

```rust
use schema_validator::{schema, Schema};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = schema();

    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .transform(|fields| {
            User {
                name: fields.get("name").unwrap().downcast_ref::<String>().unwrap().clone(),
                age: *fields.get("age").unwrap().downcast_ref::<f64>().unwrap(),
            }
        });

    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);
    obj.insert("age".to_string(), Box::new(30.0) as Box<dyn std::any::Any>);

    let user: User = schema.validate(&obj)?;
    assert_eq!(user.name, "John");
    assert_eq!(user.age, 30.0);

    Ok(())
}
```

## Error Handling

The library provides detailed error messages with error codes:

```rust
use schema_validator::{schema, Schema};

fn main() {
    let s = schema();
    
    // Type error
    let result = s.string().validate(&42);
    assert_eq!(result.unwrap_err().code, "TYPE_ERROR");

    // Missing field error
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number());

    let mut obj = std::collections::HashMap::new();
    obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn std::any::Any>);

    let err = schema.validate(&obj).unwrap_err();
    assert_eq!(err.code, "VALIDATION_ERROR");
    assert!(err.message.contains("Missing required field: age"));

    // Custom error
    let schema = s.object()
        .field("name", s.string())
        .field("age", s.number())
        .set_message("INVALID_USER", "Invalid user data");

    let err = schema.validate(&obj).unwrap_err();
    assert_eq!(err.code, "INVALID_USER");
    assert_eq!(err.message, "Invalid user data");
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.