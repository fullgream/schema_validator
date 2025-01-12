use schema_validator::{schema, Schema};

#[test]
fn test_string_literal() {
    let s = schema();

    // Basic validation
    let schema = s.literal("tuna".to_string());
    assert!(schema.validate(&"tuna".to_string()).is_ok());
    assert!(schema.validate(&"salmon".to_string()).is_err());

    // Custom error message
    let schema = s.literal("tuna".to_string())
        .set_message("INVALID_FISH", "Only tuna is allowed");

    let err = schema.validate(&"salmon".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_FISH");
    assert_eq!(err.message, "Only tuna is allowed");

    // Type error
    let err = schema.validate(&42_i64).unwrap_err();
    assert_eq!(err.code, "INVALID_FISH");
    assert!(err.message.contains("Only tuna is allowed"));
}

#[test]
fn test_number_literal() {
    let s = schema();

    // Basic validation
    let schema = s.literal(42_i64);
    assert!(schema.validate(&42_i64).is_ok());
    assert!(schema.validate(&43_i64).is_err());

    // Custom error message
    let schema = s.literal(42_i64)
        .set_message("INVALID_NUMBER", "Value must be 42");

    let err = schema.validate(&43_i64).unwrap_err();
    assert_eq!(err.code, "INVALID_NUMBER");
    assert_eq!(err.message, "Value must be 42");

    // Type error
    let err = schema.validate(&"42".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_NUMBER");
    assert!(err.message.contains("Value must be 42"));
}

#[test]
fn test_boolean_literal() {
    let s = schema();

    // Basic validation
    let schema = s.literal(true);
    assert!(schema.validate(&true).is_ok());
    assert!(schema.validate(&false).is_err());

    // Custom error message
    let schema = s.literal(true)
        .set_message("INVALID_BOOL", "Value must be true");

    let err = schema.validate(&false).unwrap_err();
    assert_eq!(err.code, "INVALID_BOOL");
    assert_eq!(err.message, "Value must be true");

    // Type error
    let err = schema.validate(&"true".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_BOOL");
    assert!(err.message.contains("Value must be true"));
}

#[test]
fn test_literal_error_messages() {
    let s = schema();

    // Default error messages
    let schema = s.literal("tuna".to_string());
    let err = schema.validate(&"salmon".to_string()).unwrap_err();
    assert_eq!(err.code, "LITERAL_ERROR");
    assert!(err.message.contains("expected \"tuna\""));
    assert!(err.message.contains("got \"salmon\""));

    let schema = s.literal(42_i64);
    let err = schema.validate(&43_i64).unwrap_err();
    assert_eq!(err.code, "LITERAL_ERROR");
    assert!(err.message.contains("expected 42"));
    assert!(err.message.contains("got 43"));

    let schema = s.literal(true);
    let err = schema.validate(&false).unwrap_err();
    assert_eq!(err.code, "LITERAL_ERROR");
    assert!(err.message.contains("expected true"));
    assert!(err.message.contains("got false"));
}