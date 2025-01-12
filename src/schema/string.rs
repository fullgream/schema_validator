use std::any::Any;
use crate::error::{ValidationError, ValidationResult, ErrorType, ErrorConfig};
use crate::schema::Schema;
use crate::schema::clone::CloneAny;
use crate::schema::patterns;
use regex::Regex;

pub struct TransformedSchema<T: 'static + CloneAny> {
    schema: StringSchema,
    transform: Box<dyn Fn(String) -> T>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static + CloneAny + Clone> TransformedSchema<T> {
    /// Sets a custom error message for the schema.
    ///
    /// # Arguments
    ///
    /// * `code` - The error code to use
    /// * `message` - The error message to use
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .transform(|s| s.len())
    ///     .set_message("INVALID_LENGTH", "Invalid string length");
    /// ```
    pub fn set_message<C, M>(mut self, code: C, message: M) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        self.schema = self.schema.set_message(code, message);
        self
    }

    /// Transforms the validated value into another type.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a value of type T and returns a value of type U
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .transform(|s| s.trim().to_string())  // First transform
    ///     .transform(|s| s.to_uppercase());     // Second transform
    ///
    /// let result = schema.validate(&" hello ".to_string()).unwrap();
    /// assert_eq!(result, "HELLO");
    /// ```
    pub fn transform<F, U>(self, f: F) -> TransformedSchema<U>
    where
        F: Fn(T) -> U + 'static,
        U: 'static + CloneAny,
    {
        let old_transform = self.transform;
        TransformedSchema {
            schema: self.schema,
            transform: Box::new(move |s| f((old_transform)(s))),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Trims whitespace from both ends of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .to_lowercase()
    ///     .trim();
    ///
    /// let result = schema.validate(&" Hello ".to_string()).unwrap();
    /// assert_eq!(result, "hello");
    /// ```
    pub fn trim(self) -> TransformedSchema<String>
    where
        T: Into<String>,
    {
        self.transform(|s| s.into().trim().to_string())
    }

    /// Converts the string to lowercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .trim()
    ///     .to_lowercase();
    ///
    /// let result = schema.validate(&" Hello ".to_string()).unwrap();
    /// assert_eq!(result, "hello");
    /// ```
    pub fn to_lowercase(self) -> TransformedSchema<String>
    where
        T: Into<String>,
    {
        self.transform(|s| s.into().to_lowercase())
    }

    /// Converts the string to uppercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .trim()
    ///     .to_uppercase();
    ///
    /// let result = schema.validate(&" hello ".to_string()).unwrap();
    /// assert_eq!(result, "HELLO");
    /// ```
    pub fn to_uppercase(self) -> TransformedSchema<String>
    where
        T: Into<String>,
    {
        self.transform(|s| s.into().to_uppercase())
    }

    /// Validates that the string is a valid email address.
    pub fn email(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.email();
        self
    }

    /// Validates that the string is a valid URL.
    pub fn url(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.url();
        self
    }

    /// Validates that the string is a valid date in YYYY-MM-DD format.
    pub fn date(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.date();
        self
    }

    /// Validates that the string is a valid time in HH:MM:SS format.
    pub fn time(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.time();
        self
    }

    /// Validates that the string is a valid UUID (version 4).
    pub fn uuid(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.uuid();
        self
    }

    /// Validates that the string is a valid IPv4 address.
    pub fn ipv4(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.ipv4();
        self
    }

    /// Validates that the string is a valid phone number in international format.
    pub fn phone(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.phone();
        self
    }

    /// Validates that the string is a valid username.
    pub fn username(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.username();
        self
    }

    /// Validates that the string is a strong password.
    pub fn password(mut self) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.password();
        self
    }

    /// Sets a regular expression pattern that the string must match.
    pub fn pattern<P: AsRef<str>>(mut self, pattern: P) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.pattern(pattern);
        self
    }

    /// Sets the minimum length for the string.
    pub fn min_length(mut self, length: usize) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.min_length(length);
        self
    }

    /// Sets the maximum length for the string.
    pub fn max_length(mut self, length: usize) -> Self
    where
        T: Into<String>,
    {
        self.schema = self.schema.max_length(length);
        self
    }
}

impl<T: 'static + CloneAny + Clone> Schema for TransformedSchema<T> {
    type Output = T;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let string = if let Some(s) = value.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = self.schema.coerce_to_string(value) {
            s
        } else {
            return Err(ValidationError::new(
                ErrorType::Type {
                    expected: "String",
                    got: type_name(value),
                },
                self.schema.error_config.clone(),
            ));
        };

        let transformed = (self.transform)(string);
        if let Some(pattern) = &self.schema.pattern {
            // Only validate pattern if T can be converted to String
            if let Some(string) = transformed_to_string(&transformed) {
                if !pattern.is_match(&string) {
                    return Err(ValidationError::new(
                        ErrorType::Pattern {
                            pattern: pattern.as_str().to_string(),
                            got: string,
                        },
                        self.schema.error_config.clone(),
                    ));
                }
            }
        }

        Ok(transformed)
    }
}

pub struct StringSchema {
    coerce: bool,
    error_config: Option<ErrorConfig>,
    pattern: Option<Regex>,
    min_length: Option<usize>,
    max_length: Option<usize>,
}

impl StringSchema {
    pub(crate) fn new(coerce: bool) -> Self {
        StringSchema {
            coerce,
            error_config: None,
            pattern: None,
            min_length: None,
            max_length: None,
        }
    }

    /// Sets a custom error message for the string schema.
    ///
    /// # Arguments
    ///
    /// * `code` - The error code to use
    /// * `message` - The error message to use
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .set_message("INVALID_STRING", "Invalid string value");
    /// ```
    pub fn set_message<C, M>(mut self, code: C, message: M) -> Self
    where
        C: Into<String>,
        M: Into<String>,
    {
        self.error_config = Some(ErrorConfig {
            code: code.into(),
            message: message.into(),
        });
        self
    }

    /// Sets a regular expression pattern that the string must match.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regular expression pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .pattern(r"^\d{4}-\d{2}-\d{2}$")  // YYYY-MM-DD format
    ///     .set_message("INVALID_DATE", "Invalid date format, expected YYYY-MM-DD");
    ///
    /// assert!(schema.validate(&"2024-01-15".to_string()).is_ok());
    /// assert!(schema.validate(&"not-a-date".to_string()).is_err());
    /// ```
    pub fn pattern<P: AsRef<str>>(mut self, pattern: P) -> Self {
        self.pattern = Some(Regex::new(pattern.as_ref()).unwrap());
        self
    }

    /// Validates that the string is a valid email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().email();
    ///
    /// assert!(schema.validate(&"user@example.com".to_string()).is_ok());
    /// assert!(schema.validate(&"not-an-email".to_string()).is_err());
    /// ```
    pub fn email(mut self) -> Self {
        self.pattern = Some(patterns::EMAIL.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_EMAIL".to_string(),
            message: "Invalid email format".to_string(),
        });
        self
    }

    /// Validates that the string is a valid URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().url();
    ///
    /// assert!(schema.validate(&"https://example.com".to_string()).is_ok());
    /// assert!(schema.validate(&"not-a-url".to_string()).is_err());
    /// ```
    pub fn url(mut self) -> Self {
        self.pattern = Some(patterns::URL.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_URL".to_string(),
            message: "Invalid URL format".to_string(),
        });
        self
    }

    /// Validates that the string is a valid date in YYYY-MM-DD format.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().date();
    ///
    /// assert!(schema.validate(&"2024-01-15".to_string()).is_ok());
    /// assert!(schema.validate(&"2024/01/15".to_string()).is_err());
    /// ```
    pub fn date(mut self) -> Self {
        self.pattern = Some(patterns::DATE.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_DATE".to_string(),
            message: "Invalid date format, expected YYYY-MM-DD".to_string(),
        });
        self
    }

    /// Validates that the string is a valid time in HH:MM:SS format.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().time();
    ///
    /// assert!(schema.validate(&"13:45:30".to_string()).is_ok());
    /// assert!(schema.validate(&"25:00:00".to_string()).is_err());
    /// ```
    pub fn time(mut self) -> Self {
        self.pattern = Some(patterns::TIME.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_TIME".to_string(),
            message: "Invalid time format, expected HH:MM:SS".to_string(),
        });
        self
    }

    /// Validates that the string is a valid UUID (version 4).
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().uuid();
    ///
    /// assert!(schema.validate(&"123e4567-e89b-42d3-a456-556642440000".to_string()).is_ok());
    /// assert!(schema.validate(&"not-a-uuid".to_string()).is_err());
    /// ```
    pub fn uuid(mut self) -> Self {
        self.pattern = Some(patterns::UUID.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_UUID".to_string(),
            message: "Invalid UUID format".to_string(),
        });
        self
    }

    /// Validates that the string is a valid IPv4 address.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().ipv4();
    ///
    /// assert!(schema.validate(&"192.168.1.1".to_string()).is_ok());
    /// assert!(schema.validate(&"256.256.256.256".to_string()).is_err());
    /// ```
    pub fn ipv4(mut self) -> Self {
        self.pattern = Some(patterns::IPV4.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_IPV4".to_string(),
            message: "Invalid IPv4 address format".to_string(),
        });
        self
    }

    /// Validates that the string is a valid phone number in international format.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().phone();
    ///
    /// assert!(schema.validate(&"+1234567890".to_string()).is_ok());
    /// assert!(schema.validate(&"not-a-phone".to_string()).is_err());
    /// ```
    pub fn phone(mut self) -> Self {
        self.pattern = Some(patterns::PHONE.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_PHONE".to_string(),
            message: "Invalid phone number format".to_string(),
        });
        self
    }

    /// Validates that the string is a valid username (3-16 chars, alphanumeric with underscore and dash).
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().username();
    ///
    /// assert!(schema.validate(&"john_doe".to_string()).is_ok());
    /// assert!(schema.validate(&"a".to_string()).is_err());
    /// ```
    pub fn username(mut self) -> Self {
        self.pattern = Some(patterns::USERNAME.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_USERNAME".to_string(),
            message: "Invalid username format (3-16 chars, alphanumeric with underscore and dash)".to_string(),
        });
        self
    }

    /// Validates that the string is a strong password.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().password();
    ///
    /// assert!(schema.validate(&"Password123".to_string()).is_ok());
    /// assert!(schema.validate(&"weak".to_string()).is_err());
    /// ```
    pub fn password(mut self) -> Self {
        self.pattern = Some(patterns::STRONG_PASSWORD.clone());
        self.error_config = Some(ErrorConfig {
            code: "INVALID_PASSWORD".to_string(),
            message: "Invalid password format (min 8 chars, at least one uppercase, one lowercase, one number)".to_string(),
        });
        self
    }

    /// Sets the minimum length for the string.
    ///
    /// # Arguments
    ///
    /// * `length` - The minimum length
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .min_length(3)
    ///     .set_message("TOO_SHORT", "String must be at least 3 characters long");
    ///
    /// assert!(schema.validate(&"hello".to_string()).is_ok());
    /// assert!(schema.validate(&"hi".to_string()).is_err());
    /// ```
    pub fn min_length(mut self, length: usize) -> Self {
        self.min_length = Some(length);
        self
    }

    /// Sets the maximum length for the string.
    ///
    /// # Arguments
    ///
    /// * `length` - The maximum length
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .max_length(10)
    ///     .set_message("TOO_LONG", "String must not exceed 10 characters");
    ///
    /// assert!(schema.validate(&"hello".to_string()).is_ok());
    /// assert!(schema.validate(&"hello world!".to_string()).is_err());
    /// ```
    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    /// Transforms the validated string into a custom type.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes a String and returns a value of type T
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string()
    ///     .transform(|s| s.len());  // Convert string to its length
    ///
    /// let length = schema.validate(&"hello".to_string()).unwrap();
    /// assert_eq!(length, 5);
    /// ```
    pub fn transform<F, T>(self, f: F) -> TransformedSchema<T>
    where
        F: Fn(String) -> T + 'static,
        T: 'static + CloneAny,
    {
        TransformedSchema {
            schema: self,
            transform: Box::new(f),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Trims whitespace from both ends of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().trim();
    ///
    /// let result = schema.validate(&" hello ".to_string()).unwrap();
    /// assert_eq!(result, "hello");
    /// ```
    pub fn trim(self) -> TransformedSchema<String> {
        self.transform(|s| s.trim().to_string())
    }

    /// Converts the string to lowercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().to_lowercase();
    ///
    /// let result = schema.validate(&"Hello".to_string()).unwrap();
    /// assert_eq!(result, "hello");
    /// ```
    pub fn to_lowercase(self) -> TransformedSchema<String> {
        self.transform(|s| s.to_lowercase())
    }

    /// Converts the string to uppercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_validator::{schema, Schema};
    ///
    /// let s = schema();
    /// let schema = s.string().to_uppercase();
    ///
    /// let result = schema.validate(&"hello".to_string()).unwrap();
    /// assert_eq!(result, "HELLO");
    /// ```
    pub fn to_uppercase(self) -> TransformedSchema<String> {
        self.transform(|s| s.to_uppercase())
    }

    fn coerce_to_string(&self, value: &dyn Any) -> Option<String> {
        if !self.coerce {
            return None;
        }

        if let Some(n) = value.downcast_ref::<i64>() {
            Some(n.to_string())
        } else if let Some(n) = value.downcast_ref::<f64>() {
            Some(format!("{:.0}", n))  // Format without decimal point
        } else if let Some(b) = value.downcast_ref::<bool>() {
            Some(b.to_string())
        } else {
            None
        }
    }
}

impl Schema for StringSchema {
    type Output = String;

    fn validate(&self, value: &dyn Any) -> ValidationResult<Self::Output> {
        let string = if let Some(s) = value.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = self.coerce_to_string(value) {
            s
        } else {
            return Err(ValidationError::new(
                ErrorType::Type {
                    expected: "String",
                    got: type_name(value),
                },
                self.error_config.clone(),
            ));
        };

        // Check minimum length
        if let Some(min_length) = self.min_length {
            if string.len() < min_length {
                return Err(ValidationError::new(
                    ErrorType::MinLength {
                        min: min_length,
                        got: string.len(),
                    },
                    self.error_config.clone(),
                ));
            }
        }

        // Check maximum length
        if let Some(max_length) = self.max_length {
            if string.len() > max_length {
                return Err(ValidationError::new(
                    ErrorType::MaxLength {
                        max: max_length,
                        got: string.len(),
                    },
                    self.error_config.clone(),
                ));
            }
        }

        // Check pattern
        if let Some(pattern) = &self.pattern {
            if !pattern.is_match(&string) {
                return Err(ValidationError::new(
                    ErrorType::Pattern {
                        pattern: pattern.as_str().to_string(),
                        got: string,
                    },
                    self.error_config.clone(),
                ));
            }
        }

        Ok(string)
    }
}

fn transformed_to_string<T: Clone + 'static>(value: &T) -> Option<String> {
    if let Some(s) = (value as &dyn Any).downcast_ref::<String>() {
        Some(s.clone())
    } else if let Some(n) = (value as &dyn Any).downcast_ref::<i64>() {
        Some(n.to_string())
    } else if let Some(n) = (value as &dyn Any).downcast_ref::<f64>() {
        Some(n.to_string())
    } else if let Some(b) = (value as &dyn Any).downcast_ref::<bool>() {
        Some(b.to_string())
    } else {
        None
    }
}

fn type_name(value: &dyn Any) -> &'static str {
    if value.is::<String>() { "String" }
    else if value.is::<i64>() { "Integer" }
    else if value.is::<f64>() { "Float" }
    else if value.is::<bool>() { "Boolean" }
    else { "Unknown" }
}