use schema_validator::{schema, Schema};

#[test]
fn test_string_coercion() {
    let s = schema();

    // Number to string
    let num: i64 = 42;
    let result = s.coerce().string().validate(&num).unwrap();
    assert_eq!(result, "42");

    // Boolean to string
    let result = s.coerce().string().validate(&true).unwrap();
    assert_eq!(result, "true");
}

#[test]
fn test_number_coercion() {
    let s = schema();

    // String to number
    let num_str = "42".to_string();
    let result = s.coerce().number().validate(&num_str).unwrap();
    assert_eq!(result, 42.0);

    // Boolean to number
    let result = s.coerce().number().validate(&true).unwrap();
    assert_eq!(result, 1.0);
    let result = s.coerce().number().validate(&false).unwrap();
    assert_eq!(result, 0.0);

    // Invalid string to number
    let invalid_str = "not a number".to_string();
    assert!(s.coerce().number().validate(&invalid_str).is_err());
}

#[test]
fn test_boolean_coercion() {
    let s = schema();

    // String to boolean
    let truthy = "yes".to_string();
    let falsy = "false".to_string();
    let result = s.coerce().boolean().validate(&truthy).unwrap();
    assert_eq!(result, true);
    let result = s.coerce().boolean().validate(&falsy).unwrap();
    assert_eq!(result, false);

    // Number to boolean
    let result = s.coerce().boolean().validate(&1_i64).unwrap();
    assert_eq!(result, true);
    let result = s.coerce().boolean().validate(&0_i64).unwrap();
    assert_eq!(result, false);
}