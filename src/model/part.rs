use std::borrow::Cow;
use std::fmt;

use derive_more::{AsRef, Into};

use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(AsRef, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Part(pub(crate) String);

impl Part {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Part {
        Part(self.0.to_string())
    }

    /// Creates a new Part with validation.
    ///
    /// # Arguments
    ///
    /// * `value` - The part value to validate and create
    ///
    /// # Validation Rules
    ///
    /// * Part cannot be empty
    /// * Part must be between 1 and 63 characters
    /// * Part cannot contain ':' or '/' characters (reserved for ERN syntax)
    /// * Part can only contain alphanumeric characters, hyphens, underscores, and dots
    ///
    /// # Returns
    ///
    /// * `Ok(Part)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    pub fn new(value: impl Into<String>) -> Result<Part, ErnError> {
        let value = value.into();

        // Check for reserved characters
        if value.contains(':') || value.contains('/') {
            return Err(ErnError::InvalidPartFormat);
        }

        // Check if empty
        if value.is_empty() {
            return Err(ErnError::ParseFailure(
                "Part",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if value.len() > 63 {
            return Err(ErnError::ParseFailure(
                "Part",
                format!(
                    "length exceeds maximum of 63 characters (got {})",
                    value.len()
                ),
            ));
        }

        // Check for valid characters
        let valid_chars = value
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.');

        if !valid_chars {
            return Err(ErnError::ParseFailure(
                "Part",
                "can only contain alphanumeric characters, hyphens, underscores, and dots"
                    .to_string(),
            ));
        }

        Ok(Part(value))
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Part {
    type Err = ErnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Part::new(Cow::Owned(s.to_owned()))
    }
}

// impl From<Part> for String {
//     fn from(part: Part) -> Self {
//         part.0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_creation() -> anyhow::Result<()> {
        let part = Part::new("segment")?;
        assert_eq!(part.as_str(), "segment");
        Ok(())
    }

    #[test]
    fn test_part_display() -> anyhow::Result<()> {
        let part = Part::new("example")?;
        assert_eq!(format!("{}", part), "example");
        Ok(())
    }

    #[test]
    fn test_part_from_str() {
        let part: Part = "test".parse().unwrap();
        assert_eq!(part.as_str(), "test");
    }

    #[test]
    fn test_part_equality() -> anyhow::Result<()> {
        let part1 = Part::new("segment1")?;
        let part2 = Part::new("segment1")?;
        let part3 = Part::new("segment2")?;
        assert_eq!(part1, part2);
        assert_ne!(part1, part3);
        Ok(())
    }

    #[test]
    fn test_part_into_string() -> anyhow::Result<()> {
        let part = Part::new("segment")?;
        let string: String = part.into();
        assert_eq!(string, "segment");
        Ok(())
    }
    #[test]
    fn test_part_validation_too_long() {
        let long_part = "a".repeat(64);
        let result = Part::new(long_part);
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Part");
                assert!(msg.contains("length exceeds maximum"));
            }
            _ => panic!("Expected ParseFailure error for too long part"),
        }
    }

    #[test]
    fn test_part_validation_invalid_chars() {
        let result = Part::new("invalid*part");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Part");
                assert!(msg.contains("can only contain"));
            }
            _ => panic!("Expected ParseFailure error for invalid characters"),
        }
    }

    #[test]
    fn test_part_validation_reserved_chars() {
        let result1 = Part::new("invalid:part");
        let result2 = Part::new("invalid/part");

        assert!(result1.is_err());
        assert!(result2.is_err());

        match result1 {
            Err(ErnError::InvalidPartFormat) => {}
            _ => panic!("Expected InvalidPartFormat error for part with ':'"),
        }

        match result2 {
            Err(ErnError::InvalidPartFormat) => {}
            _ => panic!("Expected InvalidPartFormat error for part with '/'"),
        }
    }

    #[test]
    fn test_part_validation_valid_complex() -> anyhow::Result<()> {
        let result = Part::new("valid-part_123.test")?;
        assert_eq!(result.as_str(), "valid-part_123.test");
        Ok(())
    }
}
