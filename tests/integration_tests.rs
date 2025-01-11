use schema_validator::{schema, Schema};

#[derive(Debug)]
struct User {
    username: String,
    age: f64,
    is_active: bool,
}

#[test]
fn test_strict_validation() {
    let s = schema();
    
    let user = User {
        username: "user@example.com".to_string(),
        age: 25.0,
        is_active: true,
    };

    // Valid cases - correct types
    let validated_username = s.string().validate(&user.username).unwrap();
    assert_eq!(validated_username, user.username);

    let validated_age = s.number().validate(&user.age).unwrap();
    assert_eq!(validated_age, user.age);

    let validated_active = s.boolean().validate(&user.is_active).unwrap();
    assert_eq!(validated_active, user.is_active);

    // Invalid cases - wrong types with custom error messages
    let string_schema = s.string()
        .set_message("INVALID_STRING", "Expected a string value");
    
    let err = string_schema.validate(&user.age).unwrap_err();
    assert_eq!(err.code, "INVALID_STRING");
    assert_eq!(err.message, "Expected a string value");
}

#[test]
fn test_coercion() {
    let s = schema();

    // Test string coercion
    let age: i64 = 25;
    let coerced_age = s.coerce().string().validate(&age).unwrap();
    assert_eq!(coerced_age, "25");

    // Test number coercion with custom error
    let invalid_str = "not a number".to_string();
    let number_schema = s.coerce().number()
        .set_message("INVALID_NUMBER", "Cannot convert string to number");
    
    let err = number_schema.validate(&invalid_str).unwrap_err();
    assert_eq!(err.code, "INVALID_NUMBER");
    assert_eq!(err.message, "Cannot convert string to number");

    // Test boolean coercion
    let truthy_str = "yes".to_string();
    let falsy_str = "false".to_string();
    
    let coerced_true = s.coerce().boolean().validate(&truthy_str).unwrap();
    assert_eq!(coerced_true, true);
    
    let coerced_false = s.coerce().boolean().validate(&falsy_str).unwrap();
    assert_eq!(coerced_false, false);
}

#[test]
fn test_error_messages() {
    let s = schema();

    // Custom error message
    let string_schema = s.string()
        .set_message("CUSTOM_ERROR", "Invalid value provided");
    let err = string_schema.validate(&42.0).unwrap_err();
    assert_eq!(err.code, "CUSTOM_ERROR");
    assert_eq!(err.message, "Invalid value provided");

    // Default error message for type error
    let default_schema = s.string();
    let err = default_schema.validate(&42.0).unwrap_err();
    assert_eq!(err.code, "TYPE_ERROR");
    assert!(err.message.contains("expected String"));

    // Default error message for coercion error
    let coerce_schema = s.coerce().number();
    let err = coerce_schema.validate(&"invalid".to_string()).unwrap_err();
    assert_eq!(err.code, "COERCION_ERROR");
    assert!(err.message.contains("cannot convert String to Number"));

    // Custom error message applies to both type and coercion errors
    let custom_schema = s.coerce().number()
        .set_message("INVALID", "Value is not valid");
    let err = custom_schema.validate(&"invalid".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID");
    assert_eq!(err.message, "Value is not valid");
}

#[test]
fn test_transform() {
    let s = schema();

    // String transformation
    let string_schema = s.string().transform(|s| s.trim().to_string());
    let result = string_schema.validate(&"  hello  ".to_string()).unwrap();
    assert_eq!(result, "hello");

    // Number transformation
    let number_schema = s.number().transform(|n| n.round());
    let result = number_schema.validate(&3.7).unwrap();
    assert_eq!(result, 4.0);

    // Boolean transformation
    let boolean_schema = s.boolean().transform(|b| !b);
    let result = boolean_schema.validate(&true).unwrap();
    assert_eq!(result, false);

    // Transformation with coercion
    let schema = s.coerce().string()
        .transform(|s| s.to_uppercase());
    let result = schema.validate(&42_i64).unwrap();
    assert_eq!(result, "42");

    // Multiple transformations
    let schema = s.string()
        .transform(|s| s.trim().to_string())
        .transform(|s| s.to_uppercase());
    let result = schema.validate(&"  hello  ".to_string()).unwrap();
    assert_eq!(result, "HELLO");
}