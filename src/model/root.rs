use derive_more::{AsRef, From, Into};
use std::borrow::Cow;
use std::fmt;
use type_safe_id::{StaticType, TypeSafeId};
use crate::errors::ArnError;

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone)]
pub struct Root<'a>(pub(crate) Cow<'a, str>);

impl<'a> Root<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Root<'static> {
        Root(Cow::Owned(self.0.into_owned()))
    }
    pub fn new(value: impl Into<Cow<'a, str>>) ->  Result<Self, ArnError> {
        let value = value.into();
        if value.is_empty() {
            return Err(ArnError::ParseFailure(
                "Root",
                "cannot be empty".to_string(),
            ));
        }
        Ok(Root(value))
    }
}


impl<'a> Default for Root<'a> {
    fn default() -> Self {
        Root(Cow::Owned(
            RootId::new_with_type(Root::new(AKTON).unwrap()).to_string(),
        ))
    }
}
impl<'a> fmt::Display for Root<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
const AKTON: &str = "akton";
type RootId = TypeSafeId<Root<'static>>;
impl StaticType for Root<'static> {
    const TYPE: &'static str = AKTON;
}

impl<'a> std::str::FromStr for Root<'a> {
    type Err = ArnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ArnError::ParseFailure("Root", "cannot be empty".to_string()))
        } else {
            Ok(Root(Cow::Owned(s.to_owned())))
        }
    }
}
impl<'a> From<Root<'a>> for String {
    fn from(root: Root<'a>) -> Self {
        root.0.into_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_creation() {
        let root = Root::new("test").unwrap();
        assert_eq!(root.as_str(), "test");
    }

    #[test]
    fn test_root_default() {
        let root = Root::default();
        assert!(root.as_str().starts_with("akton"));
    }

    #[test]
    fn test_root_display() {
        let root = Root::new("example").unwrap();
        assert_eq!(format!("{}", root), "example");
    }

    #[test]
    fn test_root_from_str() {
        let root: Root = "test".parse().unwrap();
        assert_eq!(root.as_str(), "test");
    }

    #[test]
    fn test_root_equality() {
        let root1 = Root::new("test");
        let root2 = Root::new("test");
        let root3 = Root::new("other");
        assert_eq!(root1, root2);
        assert_ne!(root1, root3);
    }

    #[test]
    fn test_root_into_string() {
        let root = Root::new("test").unwrap();
        let string: String = root.into();
        assert_eq!(string, "test");
    }
}
