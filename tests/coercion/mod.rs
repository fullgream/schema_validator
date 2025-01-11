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
    let result = s.coerce().string().validate(&false).unwrap();
    assert_eq!(result, "false");
}

#[test]
fn test_number_coercion() {
    let s = schema();

    // String to number
    let num_str = "42".to_string();
    let result = s.coerce().number().validate(&num_str).unwrap();
    assert_eq!(result, 42.0);

    let float_str = "42.5".to_string();
    let result = s.coerce().number().validate(&float_str).unwrap();
    assert_eq!(result, 42.5);

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

    // Truthy values
    let schema = s.coerce().boolean();

    // Any non-empty string is truthy
    assert!(schema.validate(&"tuna".to_string()).unwrap());
    assert!(schema.validate(&"true".to_string()).unwrap());
    assert!(schema.validate(&"false".to_string()).unwrap());

    // Non-zero numbers are truthy
    assert!(schema.validate(&1_i64).unwrap());
    assert!(schema.validate(&-1_i64).unwrap());
    assert!(schema.validate(&1.0).unwrap());
    assert!(schema.validate(&-1.0).unwrap());

    // Non-empty arrays are truthy
    assert!(schema.validate(&vec![true]).unwrap());
    assert!(schema.validate(&vec![1_i64]).unwrap());
    assert!(schema.validate(&vec!["hello".to_string()]).unwrap());

    // Falsy values
    // Empty string
    assert!(!schema.validate(&"".to_string()).unwrap());

    // Zero
    assert!(!schema.validate(&0_i64).unwrap());
    assert!(!schema.validate(&0.0).unwrap());

    // Empty arrays
    assert!(!schema.validate(&Vec::<bool>::new()).unwrap());
    assert!(!schema.validate(&Vec::<i64>::new()).unwrap());
    assert!(!schema.validate(&Vec::<String>::new()).unwrap());

    // None (null in JS)
    let none_value: Option<bool> = None;
    assert!(!schema.validate(&none_value).unwrap());
}