use schema_validator::{schema, Schema};

#[test]
fn test_string_pattern() {
    let s = schema();

    // Custom pattern
    let schema = s.string()
        .pattern(r"^\d{4}-\d{2}-\d{2}$")
        .set_message("INVALID_DATE", "Invalid date format, expected YYYY-MM-DD");

    // Valid date
    assert!(schema.validate(&"2024-01-15".to_string()).is_ok());

    // Invalid date
    let err = schema.validate(&"2024/01/15".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_DATE");
    assert_eq!(err.message, "Invalid date format, expected YYYY-MM-DD");
}

#[test]
fn test_string_validation_methods() {
    let s = schema();

    // Email validation
    let schema = s.string().email();
    assert!(schema.validate(&"user@example.com".to_string()).is_ok());
    assert!(schema.validate(&"test.user+label@example.co.uk".to_string()).is_ok());
    let schema = s.string().transform(|s| s.trim().to_lowercase().to_string()).email();
    assert_eq!(schema.validate(&"User@example.com ".to_string()).unwrap(), "user@example.com");

    let err = schema.validate(&"not-an-email".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_EMAIL");
    assert_eq!(err.message, "Invalid email format");
    // URL validation
    let schema = s.string().url();
    assert!(schema.validate(&"https://example.com".to_string()).is_ok());
    assert!(schema.validate(&"http://sub.example.com/path?query=1".to_string()).is_ok());
    let err = schema.validate(&"not-a-url".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_URL");
    assert_eq!(err.message, "Invalid URL format");

    // Date validation
    let schema = s.string().date();
    assert!(schema.validate(&"2024-01-15".to_string()).is_ok());
    let err = schema.validate(&"2024/01/15".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_DATE");
    assert_eq!(err.message, "Invalid date format, expected YYYY-MM-DD");

    // Time validation
    let schema = s.string().time();
    assert!(schema.validate(&"13:45:30".to_string()).is_ok());
    let err = schema.validate(&"25:00:00".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_TIME");
    assert_eq!(err.message, "Invalid time format, expected HH:MM:SS");

    // UUID validation
    let schema = s.string().uuid();
    assert!(schema.validate(&"123e4567-e89b-42d3-a456-556642440000".to_string()).is_ok());
    let err = schema.validate(&"not-a-uuid".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_UUID");
    assert_eq!(err.message, "Invalid UUID format");

    // IPv4 validation
    let schema = s.string().ipv4();
    assert!(schema.validate(&"192.168.1.1".to_string()).is_ok());
    let err = schema.validate(&"256.256.256.256".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_IPV4");
    assert_eq!(err.message, "Invalid IPv4 address format");

    // Phone validation
    let schema = s.string().phone();
    assert!(schema.validate(&"+1234567890".to_string()).is_ok());
    assert!(schema.validate(&"1234567890".to_string()).is_ok());
    let err = schema.validate(&"not-a-phone".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_PHONE");
    assert_eq!(err.message, "Invalid phone number format");

    // Username validation
    let schema = s.string().username();
    assert!(schema.validate(&"john_doe".to_string()).is_ok());
    assert!(schema.validate(&"user123".to_string()).is_ok());
    let err = schema.validate(&"a".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_USERNAME");
    assert!(err.message.contains("3-16 chars"));

    // Password validation
    let schema = s.string().password();
    assert!(schema.validate(&"Password123".to_string()).is_ok());
    let err = schema.validate(&"weak".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_PASSWORD");
    assert!(err.message.contains("min 8 chars"));
}

#[test]
fn test_string_length() {
    let s = schema();

    // Length constraints
    let schema = s.string()
        .min_length(3)
        .max_length(10)
        .set_message("INVALID_LENGTH", "String must be between 3 and 10 characters");

    // Valid length
    assert!(schema.validate(&"hello".to_string()).is_ok());

    // Too short
    let err = schema.validate(&"hi".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_LENGTH");
    assert!(err.message.contains("between 3 and 10 characters"));

    // Too long
    let err = schema.validate(&"hello world!".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_LENGTH");
    assert!(err.message.contains("between 3 and 10 characters"));
}

#[test]
fn test_string_combined() {
    let s = schema();

    // Combined validation
    let schema = s.string()
        .email()
        .max_length(50)
        .set_message("INVALID_EMAIL", "Invalid email format (max 50 chars)");

    // Valid email
    assert!(schema.validate(&"user@example.com".to_string()).is_ok());

    // Too long
    let err = schema.validate(&format!("{}@example.com", "a".repeat(100)).to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_EMAIL");
    assert!(err.message.contains("max 50 chars"));

    // Invalid format
    let err = schema.validate(&"not-an-email".to_string()).unwrap_err();
    assert_eq!(err.code, "INVALID_EMAIL");
    assert!(err.message.contains("max 50 chars"));
}

#[test]
fn test_string_coercion() {
    let s = schema();

    // Pattern with coercion
    let schema = s.coerce().string().ipv4();

    // Number coerced to string (but invalid IP)
    let err = schema.validate(&42_i64).unwrap_err();
    assert_eq!(err.code, "INVALID_IPV4");
    assert!(err.message.contains("IPv4"));

    // Boolean coerced to string (but invalid IP)
    let err = schema.validate(&true).unwrap_err();
    assert_eq!(err.code, "INVALID_IPV4");
    assert!(err.message.contains("IPv4"));
}