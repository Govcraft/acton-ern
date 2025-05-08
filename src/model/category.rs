use std::fmt;

use crate::errors::ErnError;
use derive_more::{AsRef, Into};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a category in the ERN (Entity Resource Name) system, typically indicating the service.
#[derive(AsRef, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Category(pub(crate) String);

impl Category {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// Creates a new Category with validation.
    ///
    /// # Arguments
    ///
    /// * `value` - The category value to validate and create
    ///
    /// # Validation Rules
    ///
    /// * Category cannot be empty
    /// * Category must be between 1 and 63 characters
    /// * Category can only contain alphanumeric characters and hyphens
    /// * Category cannot start or end with a hyphen
    ///
    /// # Returns
    ///
    /// * `Ok(Category)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    pub fn new(value: impl Into<String>) -> Result<Self, ErnError> {
        let val = value.into();

        // Check if empty
        if val.is_empty() {
            return Err(ErnError::ParseFailure(
                "Category",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if val.len() > 63 {
            return Err(ErnError::ParseFailure(
                "Category",
                format!(
                    "length exceeds maximum of 63 characters (got {})",
                    val.len()
                ),
            ));
        }

        // Check for valid characters
        let valid_chars = val.chars().all(|c| c.is_alphanumeric() || c == '-');

        if !valid_chars {
            return Err(ErnError::ParseFailure(
                "Category",
                "can only contain alphanumeric characters and hyphens".to_string(),
            ));
        }

        // Check if starts or ends with hyphen
        if val.starts_with('-') || val.ends_with('-') {
            return Err(ErnError::ParseFailure(
                "Category",
                "cannot start or end with a hyphen".to_string(),
            ));
        }

        Ok(Category(val))
    }
    pub fn into_owned(self) -> Category {
        Category(self.0.to_string())
    }
}

impl Default for Category {
    fn default() -> Self {
        Category("reactive".to_string())
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Category {
    type Err = ErnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Category::new(s)
    }
}
//
// impl From<Category> for String {
//     fn from(category: Category) -> Self {
//         category.0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_creation() -> anyhow::Result<()> {
        let category = Category::new("test")?;
        assert_eq!(category.as_str(), "test");
        Ok(())
    }

    #[test]
    fn test_category_default() {
        let category = Category::default();
        assert_eq!(category.as_str(), "reactive");
    }

    #[test]
    fn test_category_display() -> anyhow::Result<()> {
        let category = Category::new("example")?;
        assert_eq!(format!("{}", category), "example");
        Ok(())
    }

    #[test]
    fn test_category_from_str() {
        let category: Category = "test".parse().unwrap();
        assert_eq!(category.as_str(), "test");
    }

    #[test]
    fn test_category_equality() -> anyhow::Result<()> {
        let category1 = Category::new("test")?;
        let category2 = Category::new("test")?;
        let category3 = Category::new("other")?;
        assert_eq!(category1, category2);
        assert_ne!(category1, category3);
        Ok(())
    }

    #[test]
    fn test_category_into_string() -> anyhow::Result<()> {
        let category = Category::new("test")?;
        let string: String = category.into();
        assert_eq!(string, "test");
        Ok(())
    }

    #[test]
    fn test_category_validation_empty() {
        let result = Category::new("");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Category");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty category"),
        }
    }

    #[test]
    fn test_category_validation_too_long() {
        let long_category = "a".repeat(64);
        let result = Category::new(long_category);
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Category");
                assert!(msg.contains("length exceeds maximum"));
            }
            _ => panic!("Expected ParseFailure error for too long category"),
        }
    }

    #[test]
    fn test_category_validation_invalid_chars() {
        let result = Category::new("invalid_category$");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Category");
                assert!(msg.contains("can only contain"));
            }
            _ => panic!("Expected ParseFailure error for invalid characters"),
        }
    }

    #[test]
    fn test_category_validation_hyphen_start_end() {
        let result1 = Category::new("-invalid");
        let result2 = Category::new("invalid-");

        assert!(result1.is_err());
        assert!(result2.is_err());

        match result1 {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Category");
                assert!(msg.contains("cannot start or end with a hyphen"));
            }
            _ => panic!("Expected ParseFailure error for category starting with hyphen"),
        }
    }

    #[test]
    fn test_category_validation_valid_complex() -> anyhow::Result<()> {
        let result = Category::new("valid-category123")?;
        assert_eq!(result.as_str(), "valid-category123");
        Ok(())
    }
}
