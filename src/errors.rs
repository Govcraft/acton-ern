use std::convert::Infallible;

/// Errors that can occur when working with Entity Resource Names (ERNs).
///
/// This enum provides specific error types for various failure scenarios
/// when creating, parsing, or manipulating ERNs.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ErnError {
    /// Error when parsing a component fails validation
    #[error("Failed to parse {0}: {1}")]
    ParseFailure(&'static str, String),
    
    /// Error when a part contains invalid characters (starts with ':' or contains '/')
    #[error("Part has invalid format (starts with ':' or contains '/')")]
    IllegalPartFormat,
    
    /// Error when an invalid prefix is used in the builder
    #[error("Builder Error - Invalid prefix: {0}")]
    InvalidPrefix(String),

    /// Error when an unexpected part is provided to the builder
    #[error("Builder Error - Unexpected part: {0}")]
    UnexpectedPart(String),

    /// Error when a part has an invalid format
    #[error("Builder Error - Part has invalid format")]
    InvalidPartFormat,

    /// Error when generating an ID fails
    #[error("Root Error - Generating an Id failed: {0}")]
    IdGenerationFailure(String),

    /// Error when a required part is missing
    #[error("Builder Error - Missing required part: {0}")]
    MissingPart(String),

    /// Error when an ERN string has an invalid format
    #[error("ERN has invalid format")]
    InvalidFormat,

    /// Error that should never occur (from Infallible conversions)
    #[error("Infallible error")]
    InfallibleError,
    
    /// Error from the underlying MagicTypeId library
    #[error("Entity Root Error: {0}")]
    EntityRootError(#[from] mti::prelude::MagicTypeIdError),
}

impl From<Infallible> for ErnError {
    fn from(_: Infallible) -> Self {
        ErnError::InfallibleError
    }
}

