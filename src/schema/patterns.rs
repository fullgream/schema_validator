use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Email pattern (simplified but practical)
    pub static ref EMAIL: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    /// URL pattern (simplified but practical)
    pub static ref URL: Regex = Regex::new(r"^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}[a-zA-Z0-9./_?=&-]*$").unwrap();

    /// Date pattern in YYYY-MM-DD format
    pub static ref DATE: Regex = Regex::new(r"^\d{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12]\d|3[01])$").unwrap();

    /// Time pattern in HH:MM:SS format
    pub static ref TIME: Regex = Regex::new(r"^(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d$").unwrap();

    /// UUID pattern (version 4)
    pub static ref UUID: Regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();

    /// IPv4 address pattern
    pub static ref IPV4: Regex = Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();

    /// Phone number pattern (basic international format)
    pub static ref PHONE: Regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();

    /// Username pattern (alphanumeric with underscore and dash, 3-16 chars)
    pub static ref USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_-]{3,16}$").unwrap();

    /// Strong password pattern (min 8 chars, at least one uppercase, one lowercase, one number)
    pub static ref STRONG_PASSWORD: Regex = Regex::new(r"^[A-Z][a-zA-Z0-9\W_]{7,}$").unwrap();
}

/// Common pattern types for string validation
#[derive(Debug, Clone, Copy)]
pub enum Pattern {
    /// Email address pattern
    Email,
    /// URL pattern
    Url,
    /// Date pattern (YYYY-MM-DD)
    Date,
    /// Time pattern (HH:MM:SS)
    Time,
    /// UUID pattern (version 4)
    Uuid,
    /// IPv4 address pattern
    Ipv4,
    /// Phone number pattern
    Phone,
    /// Username pattern
    Username,
    /// Strong password pattern
    StrongPassword,
}

impl Pattern {
    /// Get the regex pattern for this pattern type
    pub fn regex(&self) -> &'static Regex {
        match self {
            Pattern::Email => &EMAIL,
            Pattern::Url => &URL,
            Pattern::Date => &DATE,
            Pattern::Time => &TIME,
            Pattern::Uuid => &UUID,
            Pattern::Ipv4 => &IPV4,
            Pattern::Phone => &PHONE,
            Pattern::Username => &USERNAME,
            Pattern::StrongPassword => &STRONG_PASSWORD,
        }
    }

    /// Get a human-readable description of the pattern
    pub fn description(&self) -> &'static str {
        match self {
            Pattern::Email => "valid email address",
            Pattern::Url => "valid URL starting with http:// or https://",
            Pattern::Date => "date in YYYY-MM-DD format",
            Pattern::Time => "time in HH:MM:SS format",
            Pattern::Uuid => "UUID version 4",
            Pattern::Ipv4 => "IPv4 address",
            Pattern::Phone => "phone number in international format",
            Pattern::Username => "username (3-16 characters, alphanumeric with underscore and dash)",
            Pattern::StrongPassword => "strong password (min 8 chars, at least one uppercase, one lowercase, one number)",
        }
    }

    /// Try to find a predefined pattern that matches the given regex string
    pub fn from_regex(pattern: &str) -> Option<Pattern> {
        let patterns = [
            (Pattern::Email, &*EMAIL),
            (Pattern::Url, &*URL),
            (Pattern::Date, &*DATE),
            (Pattern::Time, &*TIME),
            (Pattern::Uuid, &*UUID),
            (Pattern::Ipv4, &*IPV4),
            (Pattern::Phone, &*PHONE),
            (Pattern::Username, &*USERNAME),
            (Pattern::StrongPassword, &*STRONG_PASSWORD),
        ];

        patterns.iter()
            .find(|(_, regex)| regex.as_str() == pattern)
            .map(|(pattern, _)| *pattern)
    }
}