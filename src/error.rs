//! Error types for DTA operations.

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(all(feature = "serde", not(feature = "std")))]
use alloc::string::ToString;
#[cfg(feature = "std")]
use std::string::String;

#[cfg(feature = "serde")]
use core::fmt;

#[cfg(feature = "serde")]
use serde::{de, ser};

#[cfg(all(feature = "std", feature = "io"))]
use std::io;

/// Errors that can occur during DTA operations.
#[derive(Debug, thiserror::Error)]
pub enum DtaError {
    /// A required parameter was not provided.
    #[error("Missing required parameter: {0}")]
    MissingRequiredParameter(String),
    /// An invalid or unknown parameter was provided.
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),
    /// An I/O error occurred (requires `io` feature).
    #[cfg(all(feature = "std", feature = "io"))]
    #[error(transparent)]
    Io(#[from] io::Error),
    /// A parsing error occurred.
    #[error("Parse error: {0}")]
    ParseError(String),
    /// A serialization/deserialization error occurred (requires `serde` feature).
    #[cfg(feature = "serde")]
    #[error("Serde error: {0}")]
    Serde(String),
}

#[cfg(feature = "serde")]
impl ser::Error for DtaError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        DtaError::Serde(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl de::Error for DtaError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        DtaError::Serde(msg.to_string())
    }
}
