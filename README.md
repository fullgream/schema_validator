# Schema Validator

A flexible, type-safe schema validation library for Rust with a fluent API, built-in patterns, and powerful transformations.

## Features

- **Fluent API**: Chain validation methods and transformations for clear and concise code
- **Built-in Patterns**: Common validations like email, URL, date, etc.
- **String Transformations**: Built-in methods for trim, lowercase, uppercase
- **Type Coercion**: Automatic conversion between compatible types
- **Object Validation**: Validate complex objects with multiple fields
- **Error Handling**: Detailed error messages with customizable codes
- **JSON Support**: Direct validation of JSON values
- **Derive Macro**: Automatically implement validation traits

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
schema_validator = "0.1.0"
```

## Quick Start

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Basic string validation
let schema = s.string()
    .trim()
    .to_lowercase()
    .min_length(3)
    .max_length(50);

// Email validation with transformations
let schema = s.string()
    .trim()
    .to_lowercase()
    .email()
    .max_length(50);

// Object validation
#[derive(Debug, PartialEq, Clone, Validate)]
struct User {
    name: String,
    email: String,
    age: Option<f64>,
}

let schema = s.object()
    .field("name", s.string().trim().min_length(2))
    .field("email", s.string().trim().to_lowercase().email())
    .field("age", s.number().optional());
```

## String Validation

### Built-in Transformations

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Trim whitespace
let schema = s.string().trim();
assert_eq!(schema.validate(&" hello ".to_string()).unwrap(), "hello");

// Convert to lowercase
let schema = s.string().to_lowercase();
assert_eq!(schema.validate(&"Hello".to_string()).unwrap(), "hello");

// Convert to uppercase
let schema = s.string().to_uppercase();
assert_eq!(schema.validate(&"hello".to_string()).unwrap(), "HELLO");

// Chain transformations
let schema = s.string()
    .trim()
    .to_lowercase();
assert_eq!(schema.validate(&" Hello ".to_string()).unwrap(), "hello");

// Transform then validate
let schema = s.string()
    .trim()
    .to_lowercase()
    .email();
assert!(schema.validate(&" User@Example.Com ".to_string()).is_ok());

// Validate then transform
let schema = s.string()
    .email()
    .to_uppercase();
assert_eq!(
    schema.validate(&"user@example.com".to_string()).unwrap(),
    "USER@EXAMPLE.COM"
);

// Custom transformations
let schema = s.string()
    .trim()
    .to_uppercase()
    .transform(|s| s.replace("HELLO", "HI"))
    .to_lowercase();
assert_eq!(
    schema.validate(&" hello world ".to_string()).unwrap(),
    "hi world"
);
```

### Built-in Patterns

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Email validation
let schema = s.string()
    .trim()
    .to_lowercase()
    .email();
assert!(schema.validate(&" User@Example.Com ".to_string()).is_ok());

// URL validation
let schema = s.string().url();
assert!(schema.validate(&"https://example.com".to_string()).is_ok());

// Date validation (YYYY-MM-DD)
let schema = s.string().date();
assert!(schema.validate(&"2024-01-15".to_string()).is_ok());

// Time validation (HH:MM:SS)
let schema = s.string().time();
assert!(schema.validate(&"13:45:30".to_string()).is_ok());

// UUID validation
let schema = s.string().uuid();
assert!(schema.validate(&"123e4567-e89b-42d3-a456-556642440000".to_string()).is_ok());

// IPv4 validation
let schema = s.string().ipv4();
assert!(schema.validate(&"192.168.1.1".to_string()).is_ok());

// Phone validation
let schema = s.string().phone();
assert!(schema.validate(&"+1234567890".to_string()).is_ok());

// Username validation
let schema = s.string()
    .trim()
    .to_lowercase()
    .username();
assert!(schema.validate(&" John_Doe ".to_string()).is_ok());

// Password validation
let schema = s.string().password();
assert!(schema.validate(&"Password123".to_string()).is_ok());
```

### Custom Patterns

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Custom regex pattern
let schema = s.string()
    .pattern(r"^\d{4}-\d{2}-\d{2}$")
    .set_message("INVALID_DATE", "Invalid date format, expected YYYY-MM-DD");

assert!(schema.validate(&"2024-01-15".to_string()).is_ok());
assert!(schema.validate(&"not-a-date".to_string()).is_err());

// Custom pattern with transformations
let schema = s.string()
    .trim()
    .to_lowercase()
    .pattern(r"^[a-z0-9]+$")
    .set_message("INVALID_FORMAT", "Only lowercase letters and numbers allowed");

assert!(schema.validate(&" Hello123 ".to_string()).is_err());
assert!(schema.validate(&" hello123 ".to_string()).is_ok());
```

## Object Validation

```rust
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

let s = schema();

// Define schema with transformations
let schema = s.object()
    .field("name", s.string().trim().min_length(2))
    .field("email", s.string().trim().to_lowercase().email())
    .field("age", s.number().optional());

// Validate HashMap
let mut obj = HashMap::new();
obj.insert("name".to_string(), Box::new(" John ".to_string()) as Box<dyn Any>);
obj.insert("email".to_string(), Box::new(" User@Example.Com ".to_string()) as Box<dyn Any>);
obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);

let user: User = schema.validate_as(&obj).unwrap();
assert_eq!(user.name, "John");
assert_eq!(user.email, "user@example.com");
assert_eq!(user.age, Some(30.0));

// Validate JSON
let json = json!({
    "name": " John ",
    "email": " User@Example.Com ",
    "age": 30
});

let user: User = schema.validate_as(&json).unwrap();
assert_eq!(user.name, "John");
assert_eq!(user.email, "user@example.com");
assert_eq!(user.age, Some(30.0));
```

## Error Handling

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Custom error messages
let schema = s.string()
    .trim()
    .to_lowercase()
    .email()
    .max_length(50)
    .set_message("INVALID_EMAIL", "Invalid email format (max 50 chars)");

// Error handling
match schema.validate(&" User@Example.Com ".to_string()) {
    Ok(email) => println!("Valid email: {}", email), // "user@example.com"
    Err(err) => {
        println!("Error code: {}", err.code);      // "INVALID_EMAIL"
        println!("Error message: {}", err.message); // "Invalid email format (max 50 chars)"
    }
}

// Pattern validation error
let schema = s.string()
    .trim()
    .to_lowercase()
    .pattern(r"^[a-z0-9]+$")
    .set_message("INVALID_FORMAT", "Only lowercase letters and numbers allowed");

let err = schema.validate(&" Hello123 ".to_string()).unwrap_err();
assert_eq!(err.code, "INVALID_FORMAT");
assert_eq!(err.message, "Only lowercase letters and numbers allowed");
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.