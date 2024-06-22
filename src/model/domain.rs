use crate::errors::ArnError;
use derive_more::{AsRef, From, Into};
use std::borrow::Cow;
use std::fmt;

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone)]
pub struct Domain<'a>(pub(crate) Cow<'a, str>);

impl<'a> Domain<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Domain<'static> {
        Domain(Cow::Owned(self.0.into_owned()))
    }
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Domain(value.into())
    }
}

impl<'a> Default for Domain<'a> {
    fn default() -> Self {
        Domain(Cow::Borrowed("akton"))
    }
}

impl<'a> fmt::Display for Domain<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> std::str::FromStr for Domain<'a> {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ArnError::ParseFailure("Domain", "cannot be empty".to_string()))
        } else {
            Ok(Domain(Cow::Owned(s.to_owned())))
        }
    }
}
impl<'a> From<Domain<'a>> for String {
    fn from(domain: Domain<'a>) -> Self {
        domain.0.into_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_creation() {
        let domain = Domain::new("test");
        assert_eq!(domain.as_str(), "test");
    }

    #[test]
    fn test_domain_default() {
        let domain = Domain::default();
        assert_eq!(domain.as_str(), "akton");
    }

    #[test]
    fn test_domain_display() {
        let domain = Domain::new("example");
        assert_eq!(format!("{}", domain), "example");
    }

    #[test]
    fn test_domain_from_str() {
        let domain: Domain = "test".parse().unwrap();
        assert_eq!(domain.as_str(), "test");
    }

    #[test]
    fn test_domain_equality() {
        let domain1 = Domain::new("test");
        let domain2 = Domain::new("test");
        let domain3 = Domain::new("other");
        assert_eq!(domain1, domain2);
        assert_ne!(domain1, domain3);
    }

    #[test]
    fn test_domain_into_string() {
        let domain = Domain::new("test");
        let string: String = domain.into();
        assert_eq!(string, "test");
    }
}
