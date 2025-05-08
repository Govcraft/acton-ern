use std::fmt;

use derive_more::{AsRef, From, Into};
use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an account identifier in the ERN (Entity Resource Name) system.
#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Account(pub(crate) String);

impl Account {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// Creates a new Account with validation.
    ///
    /// # Arguments
    ///
    /// * `value` - The account value to validate and create
    ///
    /// # Validation Rules
    ///
    /// * Account cannot be empty
    /// * Account must be between 1 and 63 characters
    /// * Account can only contain alphanumeric characters, hyphens, and underscores
    /// * Account cannot start or end with a hyphen or underscore
    ///
    /// # Returns
    ///
    /// * `Ok(Account)` - If validation passes
    /// * `Err(ErnError)` - If validation fails
    pub fn new(value: impl Into<String>) -> Result<Self, ErnError> {
        let val = value.into();
        
        // Check if empty
        if val.is_empty() {
            return Err(ErnError::ParseFailure("Account", "cannot be empty".to_string()));
        }
        
        // Check length
        if val.len() > 63 {
            return Err(ErnError::ParseFailure(
                "Account",
                format!("length exceeds maximum of 63 characters (got {})", val.len())
            ));
        }
        
        // Check for valid characters
        let valid_chars = val.chars().all(|c| {
            c.is_alphanumeric() || c == '-' || c == '_'
        });
        
        if !valid_chars {
            return Err(ErnError::ParseFailure(
                "Account",
                "can only contain alphanumeric characters, hyphens, and underscores".to_string()
            ));
        }
        
        // Check if starts or ends with hyphen or underscore
        if val.starts_with(['-', '_'].as_ref()) || val.ends_with(['-', '_'].as_ref()) {
            return Err(ErnError::ParseFailure(
                "Account",
                "cannot start or end with a hyphen or underscore".to_string()
            ));
        }
        
        Ok(Account(val))
    }
    pub fn into_owned(self) -> Account {
        Account(self.0.to_string())
    }
}

impl Default for Account {
    fn default() -> Self {
        Account("component".to_string())
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Account {
    type Err = ErnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Account::new(s)
    }
}
//
// impl From<Account> for String {
//     fn from(domain: Account) -> Self {
//         domain.0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_creation() -> anyhow::Result<()> {
        let account = Account::new("test123")?;
        assert_eq!(account.as_str(), "test123");
        Ok(())
    }

    #[test]
    fn test_account_default() {
        let account = Account::default();
        assert_eq!(account.as_str(), "component");
    }

    #[test]
    fn test_account_display() -> anyhow::Result<()> {
        let account = Account::new("example456")?;
        assert_eq!(format!("{}", account), "example456");
        Ok(())
    }

    #[test]
    fn test_account_from_str() {
        let account: Account = "test789".parse().unwrap();
        assert_eq!(account.as_str(), "test789");
    }

    #[test]
    fn test_account_equality() -> anyhow::Result<()> {
        let account1 = Account::new("test123")?;
        let account2 = Account::new("test123")?;
        let account3 = Account::new("other456")?;
        assert_eq!(account1, account2);
        assert_ne!(account1, account3);
        Ok(())
    }

    #[test]
    fn test_account_into_string() -> anyhow::Result<()> {
        let account = Account::new("test123")?;
        let string: String = account.into();
        assert_eq!(string, "test123");
        Ok(())
    }
    
    #[test]
    fn test_account_validation_empty() {
        let result = Account::new("");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Account");
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected ParseFailure error for empty account"),
        }
    }
    
    #[test]
    fn test_account_validation_too_long() {
        let long_account = "a".repeat(64);
        let result = Account::new(long_account);
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Account");
                assert!(msg.contains("length exceeds maximum"));
            }
            _ => panic!("Expected ParseFailure error for too long account"),
        }
    }
    
    #[test]
    fn test_account_validation_invalid_chars() {
        let result = Account::new("invalid.account$");
        assert!(result.is_err());
        match result {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Account");
                assert!(msg.contains("can only contain"));
            }
            _ => panic!("Expected ParseFailure error for invalid characters"),
        }
    }
    
    #[test]
    fn test_account_validation_hyphen_underscore_start_end() {
        let result1 = Account::new("-invalid");
        let result2 = Account::new("invalid-");
        let result3 = Account::new("_invalid");
        let result4 = Account::new("invalid_");
        
        assert!(result1.is_err());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());
        
        match result1 {
            Err(ErnError::ParseFailure(component, msg)) => {
                assert_eq!(component, "Account");
                assert!(msg.contains("cannot start or end with a hyphen or underscore"));
            }
            _ => panic!("Expected ParseFailure error for account starting with hyphen"),
        }
    }
    
    #[test]
    fn test_account_validation_valid_complex() -> anyhow::Result<()> {
        let result = Account::new("valid-account_123")?;
        assert_eq!(result.as_str(), "valid-account_123");
        Ok(())
    }
}
