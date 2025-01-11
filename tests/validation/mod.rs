use schema_validator::{schema, Schema};
use crate::common::TestUser;

#[test]
fn test_string_validation() {
    let s = schema();
    let user = TestUser::new("user@example.com", 25.0, true);

    let result = s.string().validate(&user.username).unwrap();
    assert_eq!(result, user.username);

    // Invalid type
    assert!(s.string().validate(&user.age).is_err());
}

#[test]
fn test_number_validation() {
    let s = schema();
    let user = TestUser::new("user@example.com", 25.0, true);

    let result = s.number().validate(&user.age).unwrap();
    assert_eq!(result, user.age);

    // Invalid type
    assert!(s.number().validate(&user.username).is_err());
}

#[test]
fn test_boolean_validation() {
    let s = schema();
    let user = TestUser::new("user@example.com", 25.0, true);

    let result = s.boolean().validate(&user.is_active).unwrap();
    assert_eq!(result, user.is_active);

    // Invalid type
    assert!(s.boolean().validate(&user.age).is_err());
}