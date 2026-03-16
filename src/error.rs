use std::fmt;

/// Errors that can occur during serialization or deserialization.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Input validation failed (e.g., paint_wear out of range, custom_name too long).
    ValidationError(String),
    /// Failed to parse the input data (e.g., invalid hex, too short, malformed proto).
    ParseError(String),
    /// The payload exceeds the maximum allowed size.
    PayloadTooLarge,
    /// The payload is too small to be valid.
    PayloadTooSmall,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ValidationError(msg) => write!(f, "validation error: {}", msg),
            Error::ParseError(msg) => write!(f, "parse error: {}", msg),
            Error::PayloadTooLarge => write!(f, "payload too large (max 4096 hex chars)"),
            Error::PayloadTooSmall => write!(f, "payload too small (min 6 bytes)"),
        }
    }
}

impl std::error::Error for Error {}
