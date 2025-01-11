use schema_validator::{schema, Schema};

#[test]
fn test_string_transform() {
    let s = schema();

    // Single transformation
    let schema = s.string().transform(|s| s.trim().to_string());
    let result = schema.validate(&"  hello  ".to_string()).unwrap();
    assert_eq!(result, "hello");

    // Multiple transformations
    let schema = s.string()
        .transform(|s| s.trim().to_string())
        .transform(|s| s.to_uppercase());
    let result = schema.validate(&"  hello  ".to_string()).unwrap();
    assert_eq!(result, "HELLO");

    // Transform with coercion
    let schema = s.coerce().string()
        .transform(|s| s.to_uppercase());
    let result = schema.validate(&42_i64).unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_number_transform() {
    let s = schema();

    // Round numbers
    let schema = s.number().transform(|n| n.round());
    let result = schema.validate(&3.7).unwrap();
    assert_eq!(result, 4.0);

    // Multiple transformations
    let schema = s.number()
        .transform(|n| n * 2.0)
        .transform(|n| n.round());
    let result = schema.validate(&3.7).unwrap();
    assert_eq!(result, 7.0); // 3.7 * 2 = 7.4, rounded to 7.0
}

#[test]
fn test_boolean_transform() {
    let s = schema();

    // Invert boolean
    let schema = s.boolean().transform(|b| !b);
    let result = schema.validate(&true).unwrap();
    assert_eq!(result, false);

    // Multiple transformations
    let schema = s.boolean()
        .transform(|b| !b)
        .transform(|b| !b);
    let result = schema.validate(&true).unwrap();
    assert_eq!(result, true);
}