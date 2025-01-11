use schema_validator::{schema, Schema};

#[test]
fn test_default_errors() {
    let s = schema();

    // Type error
    let err = s.string().validate(&42.0).unwrap_err();
    assert_eq!(err.code, "TYPE_ERROR");
    assert!(err.message.contains("expected String"));

    // Coercion error
    let err = s.coerce().number()
        .validate(&"invalid".to_string())
        .unwrap_err();
    assert_eq!(err.code, "COERCION_ERROR");
    assert!(err.message.contains("cannot convert String to Number"));
}

#[test]
fn test_custom_error_messages() {
    let s = schema();

    // Custom type error
    let schema = s.string()
        .set_message("INVALID_TYPE", "Must be a string value");
    let err = schema.validate(&42.0).unwrap_err();
    assert_eq!(err.code, "INVALID_TYPE");
    assert_eq!(err.message, "Must be a string value");

    // Custom coercion error
    let schema = s.coerce().number()
        .set_message("INVALID_NUMBER", "Cannot convert to number");
    let err = schema.validate(&"invalid".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_NUMBER");
    assert_eq!(err.message, "Cannot convert to number");
}

#[test]
fn test_error_with_transform() {
    let s = schema();

    // Error happens before transform
    let schema = s.string()
        .transform(|s| s.to_uppercase())
        .set_message("INVALID", "Invalid value");
    
    let err = schema.validate(&42.0).unwrap_err();
    assert_eq!(err.code, "INVALID");
    assert_eq!(err.message, "Invalid value");
}