# Schema Validator

A flexible, type-safe schema validation library for Rust with a fluent API, built-in patterns, and powerful transformations.

## Features

- **Fluent API**: Chain validation methods for clear and concise code
- **Built-in Patterns**: Common validations like email, URL, date, etc.
- **Type Coercion**: Automatic conversion between compatible types
- **Transformations**: Transform and validate data in any order
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
    .min_length(3)
    .max_length(50);

// Email validation with transformation
let schema = s.string()
    .transform(|s| s.trim().to_lowercase())
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
    .field("name", s.string().min_length(2))
    .field("email", s.string().email())
    .field("age", s.number().optional());
```

## String Validation

### Built-in Patterns

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Email validation
let schema = s.string().email();
assert!(schema.validate(&"user@example.com".to_string()).is_ok());

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
let schema = s.string().username();
assert!(schema.validate(&"john_doe".to_string()).is_ok());

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
```

### Transformations

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Transform then validate
let schema = s.string()
    .transform(|s| s.trim().to_lowercase())
    .email();

// Validate then transform
let schema = s.string()
    .email()
    .transform(|s| s.to_uppercase());

// Multiple transforms
let schema = s.string()
    .transform(|s| s.trim().to_string())
    .email()
    .transform(|s| s.to_lowercase())
    .max_length(50);
```

## Number Validation

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Basic number validation
let schema = s.number();
assert!(schema.validate(&42.0).is_ok());

// With type coercion
let schema = s.coerce().number();
assert!(schema.validate(&"42".to_string()).is_ok());

// With transformation
let schema = s.number()
    .transform(|n| n > 0.0);  // Convert to boolean
```

## Object Validation

```rust
use schema_validator::{schema, Schema, Validate};
use serde_json::json;

#[derive(Debug, PartialEq, Clone, Validate)]
struct User {
    name: String,
    email: String,
    age: Option<f64>,
}

let s = schema();

// Define schema
let schema = s.object()
    .field("name", s.string().min_length(2))
    .field("email", s.string().email())
    .field("age", s.number().optional());

// Validate HashMap
let mut obj = HashMap::new();
obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
obj.insert("email".to_string(), Box::new("john@example.com".to_string()) as Box<dyn Any>);
obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);

let user: User = schema.validate_as(&obj).unwrap();

// Validate JSON
let json = json!({
    "name": "John",
    "email": "john@example.com",
    "age": 30
});

let user: User = schema.validate_as(&json).unwrap();
```

## Error Handling

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Custom error messages
let schema = s.string()
    .email()
    .max_length(50)
    .set_message("INVALID_EMAIL", "Invalid email format (max 50 chars)");

// Error handling
match schema.validate(&"not-an-email".to_string()) {
    Ok(email) => println!("Valid email: {}", email),
    Err(err) => {
        println!("Error code: {}", err.code);      // "INVALID_EMAIL"
        println!("Error message: {}", err.message); // "Invalid email format (max 50 chars)"
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.