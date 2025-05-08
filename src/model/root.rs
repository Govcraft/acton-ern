use std::fmt;
use std::hash::Hash;

use derive_more::{AsRef, From, Into};
use mti::prelude::*;

use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents the root component in an Entity Resource Name (ERN).
///
/// The root component is a unique identifier for the base resource in the ERN hierarchy.
/// It uses the `mti` crate's `MagicTypeId` with UUID v7 algorithm to generate
/// time-ordered, unique identifiers that enable k-sortability.
///
/// When using `EntityRoot`, each call to create a new root with the same name will
/// generate a different ID, as it incorporates the current timestamp. This makes
/// `EntityRoot` suitable for resources that should be ordered by creation time.
///
/// For content-addressable, deterministic IDs, use `SHA1Name` instead.
#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, Default, PartialOrd)]
pub struct EntityRoot {
    /// The unique identifier for this root entity, generated using the `mti` crate's
    /// `MagicTypeId` type.
    name: MagicTypeId,
}

impl EntityRoot {
    /// Returns a reference to the underlying `MagicTypeId`.
    ///
    /// This is useful when you need to access the raw identifier for
    /// comparison or sorting operations.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let root1 = EntityRoot::new("resource1".to_string())?;
    /// let root2 = EntityRoot::new("resource2".to_string())?;
    ///
    /// // Compare roots by their MagicTypeId
    /// let comparison = root1.name().cmp(root2.name());
    /// # Ok(())
    /// # }
    /// ```
    pub fn name(&self) -> &MagicTypeId {
        &self.name
    }

    /// Returns the string representation of this root's identifier.
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let root = EntityRoot::new("profile".to_string())?;
    /// let id_str = root.as_str();
    ///
    /// // The string will contain the original name followed by a timestamp-based suffix
    /// assert!(id_str.starts_with("profile_"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Creates a new `EntityRoot` with the given value.
    ///
    /// This method generates a time-ordered, unique identifier using the UUID v7 algorithm.
    /// Each call to this method with the same input value will generate a different ID,
    /// as it incorporates the current timestamp. This makes `EntityRoot` suitable for
    /// resources that should be ordered by creation time.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to use as the base for the entity root ID
    ///
    /// # Validation Rules
    ///
    /// * Value cannot be empty
    /// * Value must be between 1 and 255 characters
    ///
    /// # Returns
    ///
    /// * `Ok(EntityRoot)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let root = EntityRoot::new("profile".to_string())?;
    ///
    /// // The ID will contain the original name followed by a timestamp-based suffix
    /// assert!(root.to_string().starts_with("profile_"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(value: String) -> Result<Self, ErnError> {
        // Check if empty
        if value.is_empty() {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if value.len() > 255 {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                format!(
                    "length exceeds maximum of 255 characters (got {})",
                    value.len()
                ),
            ));
        }

        Ok(EntityRoot {
            name: value.create_type_id::<V7>(),
        })
    }
}

impl fmt::Display for EntityRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = &self.name;
        write!(f, "{id}")
    }
}

/// Implementation of `FromStr` for `EntityRoot` to create an entity root from a string.
impl std::str::FromStr for EntityRoot {
    type Err = ErnError;

    /// Creates an `EntityRoot` from a string.
    ///
    /// This method generates a time-ordered, unique identifier using the UUID v7 algorithm.
    /// Each call to this method with the same input string will generate a different ID,
    /// as it incorporates the current timestamp.
    ///
    /// # Arguments
    ///
    /// * `s` - The string value to use as the base for the entity root ID
    ///
    /// # Returns
    ///
    /// * `Ok(EntityRoot)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if empty
        if s.is_empty() {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                "cannot be empty".to_string(),
            ));
        }

        // Check length
        if s.len() > 255 {
            return Err(ErnError::ParseFailure(
                "EntityRoot",
                format!("length exceeds maximum of 255 characters (got {})", s.len()),
            ));
        }

        Ok(EntityRoot {
            name: s.create_type_id::<V7>(),
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for EntityRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the MagicTypeId as a string
        serializer.serialize_str(self.name.as_ref())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EntityRoot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize as a string, then create a new EntityRoot
        let s = String::deserialize(deserializer)?;
        EntityRoot::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_entity_root_creation() -> anyhow::Result<()> {
        let root = EntityRoot::new("test-entity".to_string())?;
        assert!(!root.to_string().is_empty());
        Ok(())
    }

    #[test]
    fn test_entity_root_uniqueness() -> anyhow::Result<()> {
        // EntityRoot should generate different IDs for the same input (non-deterministic)
        let root1 = EntityRoot::new("same-content".to_string())?;
        let root2 = EntityRoot::new("same-content".to_string())?;

        // The string representations should be different
        assert_ne!(root1.to_string(), root2.to_string());
        Ok(())
    }

    #[test]
    fn test_entity_root_from_str() -> anyhow::Result<()> {
        let root = EntityRoot::from_str("test-entity")?;
        assert!(!root.to_string().is_empty());
        Ok(())
    }

    #[test]
    fn test_entity_root_validation_empty() {
        let result = EntityRoot::new("".to_string());
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "EntityRoot");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty EntityRoot"),
        }
    }

    #[test]
    fn test_entity_root_validation_too_long() {
        let long_value = "a".repeat(256);
        let result = EntityRoot::new(long_value);
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "EntityRoot");
                assert!(msg.contains("length exceeds maximum"));
            }
            _ => panic!("Expected ParseFailure error for too long EntityRoot"),
        }
    }

    #[test]
    fn test_entity_root_from_str_validation() {
        let result = EntityRoot::from_str("");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "EntityRoot");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty EntityRoot from_str"),
        }
    }
}
