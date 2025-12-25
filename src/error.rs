use std::collections::HashMap;
use thiserror::Error;

/// Error types for the MailBreeze SDK
#[derive(Error, Debug)]
pub enum Error {
    /// Authentication failed (401)
    #[error("Authentication failed: {message}")]
    Authentication {
        message: String,
        code: Option<String>,
    },

    /// Bad request (400)
    #[error("Bad request: {message}")]
    BadRequest {
        message: String,
        code: Option<String>,
    },

    /// Resource not found (404)
    #[error("Not found: {message}")]
    NotFound {
        message: String,
        code: Option<String>,
    },

    /// Validation error (422)
    #[error("Validation failed: {message}")]
    Validation {
        message: String,
        errors: HashMap<String, Vec<String>>,
        code: Option<String>,
    },

    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        retry_after: Option<u64>,
        code: Option<String>,
    },

    /// Server error (5xx)
    #[error("Server error: {message}")]
    Server {
        message: String,
        status_code: u16,
        code: Option<String>,
    },

    /// HTTP client error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON parsing error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid header value
    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    /// Request timeout
    #[error("Request timeout")]
    Timeout,
}

impl Error {
    /// Returns the error code if available
    pub fn code(&self) -> Option<&str> {
        match self {
            Error::Authentication { code, .. } => code.as_deref(),
            Error::BadRequest { code, .. } => code.as_deref(),
            Error::NotFound { code, .. } => code.as_deref(),
            Error::Validation { code, .. } => code.as_deref(),
            Error::RateLimit { code, .. } => code.as_deref(),
            Error::Server { code, .. } => code.as_deref(),
            _ => None,
        }
    }

    /// Returns the HTTP status code if applicable
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Authentication { .. } => Some(401),
            Error::BadRequest { .. } => Some(400),
            Error::NotFound { .. } => Some(404),
            Error::Validation { .. } => Some(422),
            Error::RateLimit { .. } => Some(429),
            Error::Server { status_code, .. } => Some(*status_code),
            _ => None,
        }
    }

    /// Returns retry-after seconds for rate limit errors
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Error::RateLimit { retry_after, .. } => *retry_after,
            _ => None,
        }
    }

    /// Returns validation errors if this is a validation error
    pub fn validation_errors(&self) -> Option<&HashMap<String, Vec<String>>> {
        match self {
            Error::Validation { errors, .. } => Some(errors),
            _ => None,
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Server { status_code, .. } => {
                matches!(status_code, 500 | 502 | 503 | 504)
            }
            Error::Timeout => true,
            Error::Http(e) => e.is_connect() || e.is_timeout(),
            _ => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
