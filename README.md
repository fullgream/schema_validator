# Schema Validator

A flexible, type-safe schema validation library for Rust with support for type coercion and transformations.

## Features

- Type validation for strings, numbers, and booleans
- Type coercion (e.g., number to string, string to number)
- Value transformations with type changes
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

    // Value transformations
    let uppercase = s.string()
        .transform(|s| s.to_uppercase())
        .validate(&"hello".to_string())?; // "HELLO"

    // Transformations with type changes
    let string_length = s.string()
        .transform(|s| s.len() as f64)
        .validate(&"hello".to_string())?; // 5.0

    // Multiple transformations
    let processed = s.string()
        .transform(|s| s.trim().to_string())
        .transform(|s| s.to_uppercase())
        .validate(&"  hello  ".to_string())?; // "HELLO"

    // Custom error messages
    let schema = s.string()
        .set_message("INVALID_TYPE", "Value must be a string");
    let result = schema.validate(&42);
    assert!(result.is_err());
    
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
- Strings -> false for "", "false", "0", true for any other string

## Error Handling

The library provides detailed error messages with error codes:

```rust
use schema_validator::{schema, Schema};

fn main() {
    let s = schema();
    
    // Type error
    let result = s.string().validate(&42);
    assert_eq!(result.unwrap_err().code, "TYPE_ERROR");

    // Coercion error
    let result = s.coerce().number().validate(&"not a number".to_string());
    assert_eq!(result.unwrap_err().code, "COERCION_ERROR");

    // Custom error
    let result = s.string()
        .set_message("CUSTOM_ERROR", "Invalid value")
        .validate(&42);
    assert_eq!(result.unwrap_err().code, "CUSTOM_ERROR");
}
```

## Advanced Usage

### Transforming Values with Type Changes

```rust
use schema_validator::{schema, Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = schema();

    // String -> Number
    let schema = s.string()
        .transform(|s| s.len() as f64);
    let length = schema.validate(&"hello".to_string())?; // 5.0

    // String -> Boolean
    let schema = s.string()
        .transform(|s| s.contains("yes"));
    let has_yes = schema.validate(&"yes please".to_string())?; // true

    // Coercion + Transformation
    let schema = s.coerce().string()
        .transform(|s| s.parse::<i32>().unwrap_or(0));
    let number = schema.validate(&true)?; // 0

    Ok(())
}
```

### Complex Validation Pipeline

```rust
use schema_validator::{schema, Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = schema();

    let schema = s.coerce().string()
        .transform(|s| s.trim().to_string())
        .transform(|s| s.to_uppercase())
        .transform(|s| s.contains("HELLO"));

    // These will all work:
    assert!(schema.validate(&"  hello world  ".to_string())?.is_true());
    assert!(schema.validate(&42)?.is_false()); // Coerced to "42"
    assert!(schema.validate(&true)?.is_false()); // Coerced to "true"

    Ok(())
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.