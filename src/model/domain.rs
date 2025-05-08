use std::fmt;

use derive_more::{AsRef, From, Into};

use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Domain(pub(crate) String);

impl Domain {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Domain {
        Domain(self.0)
    }
    /// Creates a new Domain with validation.
    ///
    /// # Arguments
    ///
    /// * `value` - The domain value to validate and create
    ///
    /// # Validation Rules
    ///
    /// * Domain cannot be empty
    /// * Domain must be between 1 and 63 characters
    /// * Domain can only contain alphanumeric characters, hyphens, and dots
    /// * Domain cannot start or end with a hyphen
    ///
    /// # Returns
    ///
    /// * `Ok(Domain)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    pub fn new(value: impl Into<String>) -> Result<Self, ErnError> {
        let val = value.into();

        // Check if empty
        if val.is_empty() {
            return Err(ErnError::ParseFailure(
                "Domain",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if val.len() > 63 {
            return Err(ErnError::ParseFailure(
                "Domain",
                format!(
                    "length exceeds maximum of 63 characters (got {})",
                    val.len()
                ),
            ));
        }

        // Check for valid characters
        let valid_chars = val
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '.');

        if !valid_chars {
            return Err(ErnError::ParseFailure(
                "Domain",
                "can only contain alphanumeric characters, hyphens, and dots".to_string(),
            ));
        }

        // Check if starts or ends with hyphen
        if val.starts_with('-') || val.ends_with('-') {
            return Err(ErnError::ParseFailure(
                "Domain",
                "cannot start or end with a hyphen".to_string(),
            ));
        }

        Ok(Domain(val))
    }
}

impl Default for Domain {
    fn default() -> Self {
        Domain("acton".to_string())
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Domain {
    type Err = ErnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Domain::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_creation() {
        let domain = Domain::new("test").unwrap();
        assert_eq!(domain.as_str(), "test");
    }

    #[test]
    fn test_domain_default() {
        let domain = Domain::default();
        assert_eq!(domain.as_str(), "acton");
    }

    #[test]
    fn test_domain_display() {
        let domain = Domain::new("example").unwrap();
        assert_eq!(format!("{}", domain), "example");
    }

    #[test]
    fn test_domain_from_str() {
        let domain: Domain = "test".parse().unwrap();
        assert_eq!(domain.as_str(), "test");
    }

    #[test]
    fn test_domain_equality() -> anyhow::Result<()> {
        let domain1 = Domain::new("test")?;
        let domain2 = Domain::new("test")?;
        let domain3 = Domain::new("other")?;
        assert_eq!(domain1, domain2);
        assert_ne!(domain1, domain3);
        Ok(())
    }

    #[test]
    fn test_domain_into_string() {
        let domain = Domain::new("test").unwrap();
        let string: String = domain.into();
        assert_eq!(string, "test");
    }

    #[test]
    fn test_domain_validation_empty() {
        let result = Domain::new("");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Domain");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty domain"),
        }
    }

    #[test]
    fn test_domain_validation_too_long() {
        let long_domain = "a".repeat(64);
        let result = Domain::new(long_domain);
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Domain");
                assert!(msg.contains("length exceeds maximum"));
            }
            _ => panic!("Expected ParseFailure error for too long domain"),
        }
    }

    #[test]
    fn test_domain_validation_invalid_chars() {
        let result = Domain::new("invalid_domain$");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Domain");
                assert!(msg.contains("can only contain"));
            }
            _ => panic!("Expected ParseFailure error for invalid characters"),
        }
    }

    #[test]
    fn test_domain_validation_hyphen_start_end() {
        let result1 = Domain::new("-invalid");
        let result2 = Domain::new("invalid-");

        assert!(result1.is_err());
        assert!(result2.is_err());

        match result1 {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Domain");
                assert!(msg.contains("cannot start or end with a hyphen"));
            }
            _ => panic!("Expected ParseFailure error for domain starting with hyphen"),
        }
    }

    #[test]
    fn test_domain_validation_valid_complex() {
        let result = Domain::new("valid-domain.name123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "valid-domain.name123");
    }
}
