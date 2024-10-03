use std::fmt;

use derive_more::{AsRef, From, Into};

/// Represents an account identifier in the ERN (Entity Resource Name) system.
#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct Account(pub(crate) String);

impl Account {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn new(value: impl Into<String>) -> Self {
        Account(value.into())
    }
    pub fn into_owned(self) -> Account {
        Account(self.0.to_string())
    }
}

impl Default for Account {
    fn default() -> Self {
        Account("account".to_string())
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Account {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Account(s.to_string()))
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
    fn test_account_creation() {
        let account = Account::new("test123");
        assert_eq!(account.as_str(), "test123");
    }

    #[test]
    fn test_account_default() {
        let account = Account::default();
        assert_eq!(account.as_str(), "account");
    }

    #[test]
    fn test_account_display() {
        let account = Account::new("example456");
        assert_eq!(format!("{}", account), "example456");
    }

    #[test]
    fn test_account_from_str() {
        let account: Account = "test789".parse().unwrap();
        assert_eq!(account.as_str(), "test789");
    }

    #[test]
    fn test_account_equality() {
        let account1 = Account::new("test123");
        let account2 = Account::new("test123");
        let account3 = Account::new("other456");
        assert_eq!(account1, account2);
        assert_ne!(account1, account3);
    }

    #[test]
    fn test_account_into_string() {
        let account = Account::new("test123");
        let string: String = account.into();
        assert_eq!(string, "test123");
    }
}
