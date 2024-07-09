use std::convert::Infallible;

// Merged ArnBuilderError and ArnParseError into ArnError
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EidError {
    #[error("Failed to parse {0}: {1}")]
    ParseFailure(&'static str, String),
    #[error("Part has invalid format (starts with ':' or contains '/')")]
    IllegalPartFormat,
    #[error("Builder Error - Invalid prefix: {0}")]
    InvalidPrefix(String),

    #[error("Builder Error - Unexpected part: {0}")]
    UnexpectedPart(String),

    #[error("Builder Error - Part has invalid format")]
    InvalidPartFormat,

    #[error("Root Error - Generating an Id failed: {0}")]
    IdGenerationFailure(String),

    #[error("Builder Error - Missing required part: {0}")]
    MissingPart(String),

    #[error("ARN has invalid format")]
    InvalidFormat,

    // Converted the Infallible implementation to ArnError
    #[error("Infallible error")]
    InfallibleError,
}

impl From<Infallible> for EidError {
    fn from(_: Infallible) -> Self {
        EidError::InfallibleError
    }
}
impl From<type_safe_id::Error> for EidError {
    fn from(e: type_safe_id::Error) -> Self {
        EidError::IdGenerationFailure(e.to_string())
    }
}
