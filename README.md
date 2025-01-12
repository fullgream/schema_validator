# Schema Validator

A flexible, type-safe schema validation library for Rust with support for type coercion, transformations, optional fields, and object validation.

## Features

- **Type Validation**: Basic validation for strings, numbers, and booleans
- **Optional Fields**: Support for optional values with proper type checking
- **Type Coercion**: Automatic conversion between compatible types
- **Object Validation**: Validate complex objects with multiple fields
- **Custom Transformations**: Transform validated data into custom types
- **Error Handling**: Detailed error messages with customizable codes
- **Derive Macro**: Automatically implement validation traits for your structs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
schema_validator = "0.1.0"
```

## Basic Usage

```rust
use schema_validator::{schema, Schema};

let s = schema();

// Basic type validation
let valid_string = s.string().validate(&"hello".to_string()).unwrap();
let valid_number = s.number().validate(&42.0).unwrap();
let valid_bool = s.boolean().validate(&true).unwrap();

// Optional fields
let optional_string = s.string().optional().validate(&Some("hello".to_string())).unwrap(); // Some("hello")
let optional_none = s.number().optional().validate(&None::<f64>).unwrap(); // None

// Type coercion
let string_from_num = s.coerce().string().validate(&42_i64).unwrap(); // "42"
let num_from_string = s.coerce().number().validate(&"42".to_string()).unwrap(); // 42.0
let bool_from_num = s.coerce().boolean().validate(&1_i64).unwrap(); // true
```

## Object Validation

```rust
use schema_validator::{schema, Schema, ValidateAs, Validate};
use std::collections::HashMap;
use std::any::Any;

// Just derive Validate and you're good to go!
#[derive(Debug, PartialEq, Clone, Validate)]
struct User {
    name: String,
    age: Option<f64>,
    is_active: bool,
}

let s = schema();

// Define schema with optional field
let schema = s.object()
    .field("name", s.string())
    .field("age", s.number().optional())
    .field("is_active", s.boolean());

// Create object
let mut obj = HashMap::new();
obj.insert("name".to_string(), Box::new("John".to_string()) as Box<dyn Any>);
obj.insert("age".to_string(), Box::new(Some(30.0)) as Box<dyn Any>);
obj.insert("is_active".to_string(), Box::new(true) as Box<dyn Any>);

// Validate and convert to User struct
let user: User = schema.validate_as(&obj).unwrap();
assert_eq!(user.name, "John");
assert_eq!(user.age, Some(30.0));
assert_eq!(user.is_active, true);
```

## Custom Error Messages

```rust
use schema_validator::{schema, Schema, ValidateAs, Validate};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Validate)]
struct Point {
    x: f64,
    y: f64,
}

let s = schema();

// Define schema with custom error message
let schema = s.object()
    .field("x", s.number())
    .field("y", s.number())
    .set_message("INVALID_POINT", "Invalid point coordinates");

// Invalid object
let mut obj = HashMap::new();
obj.insert("x".to_string(), Box::new(10.0) as Box<dyn std::any::Any>);
// Missing y field

let err = schema.validate_as::<Point>(&obj).unwrap_err();
assert_eq!(err.code, "INVALID_POINT");
assert_eq!(err.message, "Invalid point coordinates");
```

## Type Coercion Rules

The library supports automatic type coercion between compatible types:

### String Coercion
From | To String | Example
--- | --- | ---
Number (integer) | String representation | `42` → `"42"`
Number (float) | String representation | `3.14` → `"3.14"`
Boolean | "true" or "false" | `true` → `"true"`
Optional | Optional string | `Some(42)` → `Some("42")`

### Number Coercion
From | To Number | Example
--- | --- | ---
String | Parsed number or error | `"42"` → `42.0`
Boolean | 1.0 for true, 0.0 for false | `true` → `1.0`
Optional | Optional number | `Some("42")` → `Some(42.0)`

### Boolean Coercion
From | To Boolean | Example
--- | --- | ---
String | `false` for empty, "false", "0"; `true` otherwise | `"true"` → `true`
Number | `false` for 0, `true` for any other number | `1` → `true`
Optional | Optional boolean | `Some("true")` → `Some(true)`

### Optional Value Coercion
- Optional values are coerced recursively
- `None` values remain `None`
- `Some(value)` is coerced according to the target type rules

## Custom Transformations

```rust
use schema_validator::{schema, Schema};

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
```

## Error Handling

The library provides detailed error messages with error codes:

```rust
use schema_validator::{schema, Schema, ValidateAs, Validate};
use std::collections::HashMap;

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

// Invalid object
let mut obj = HashMap::new();
obj.insert("name".to_string(), Box::new(42.0) as Box<dyn std::any::Any>); // Wrong type
obj.insert("age".to_string(), Box::new("not a number") as Box<dyn std::any::Any>); // Wrong type

let err = schema.validate_as::<User>(&obj).unwrap_err();
assert_eq!(err.code, "INVALID_USER");
assert_eq!(err.message, "Invalid user data");
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.