use crate::errors::ArnError;
use derive_more::{AsRef, From, Into};
use std::borrow::Cow;
use std::fmt;
use type_safe_id::{DynamicType, TypeSafeId};

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone)]
pub struct Root<'a>(pub(crate) Cow<'a, str>);

impl<'a> Root<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Root<'static> {
        Root(Cow::Owned(self.0.into_owned()))
    }

    pub fn new(value: impl Into<Cow<'a, str>>) -> Result<Self, ArnError> {
        let value = value.into();
        let value = if value.is_empty() {
            let val = AKTON;
            TypeSafeId::from_type_and_uuid(DynamicType::new(val)?, uuid::Uuid::now_v7())
                .to_string()
        } else {
            TypeSafeId::from_type_and_uuid(DynamicType::new(&value)?, uuid::Uuid::now_v7())
                .to_string()
        };
        Ok(Root(Cow::from(value)))
    }
}

impl<'a> Default for Root<'a> {
    fn default() -> Self {
        Root::new("").expect("Couldn't create default Akton ARN")
        // Root(Cow::Borrowed(AKTON))
    }
}

impl<'a> fmt::Display for Root<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = &self.0;
        write!(f, "{id}")
    }
}
const AKTON: &str = "akton";

impl<'a> std::str::FromStr for Root<'a> {
    type Err = ArnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Root::new(s.to_string())
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
        assert!(root.as_str().starts_with("test"));
    }

    #[test]
    fn test_root_default() {
        let root = Root::default();
        assert!(root.as_str().starts_with("akton"));
    }

    #[test]
    fn test_root_display() {
        let root = Root::new("example").unwrap();
        assert!(format!("{}", root).starts_with("example"));
    }

    #[test]
    fn test_root_from_str() {
        let root: Root = "test".parse().unwrap();
        assert!(root.as_str().starts_with("test"));
    }

    #[test]
    fn test_root_equality() {
        let root1 = Root::new("test");
        let root2 = Root::new("test");
        let root3 = Root::new("other");
        assert_ne!(root1, root2);
        assert_ne!(root1, root3);
    }

    #[test]
    fn test_root_into_string() {
        let root = Root::new("test").unwrap();
        let string: String = root.into();
        assert!(string.starts_with("test"));
    }
}
