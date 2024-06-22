use derive_more::{AsRef, From, Into};
use std::borrow::Cow;
use std::fmt;
/// Represents a category in the Arn system, typically indicating the service.

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash)]
pub struct Category<'a>(pub(crate) Cow<'a, str>);

impl<'a> Category<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Category(value.into())
    }
    pub fn into_owned(self) -> Category<'static> {
        Category(Cow::Owned(self.0.into_owned()))
    }
}

impl<'a> Default for Category<'a> {
    fn default() -> Self {
        Category(Cow::Borrowed("system"))
    }
}

impl<'a> fmt::Display for Category<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> std::str::FromStr for Category<'a> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Category(Cow::Owned(s.to_owned())))
    }
}
impl<'a> From<Category<'a>> for String {
    fn from(category: Category<'a>) -> Self {
        category.0.into_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_creation() {
        let category = Category::new("test");
        assert_eq!(category.as_str(), "test");
    }

    #[test]
    fn test_category_default() {
        let category = Category::default();
        assert_eq!(category.as_str(), "system");
    }

    #[test]
    fn test_category_display() {
        let category = Category::new("example");
        assert_eq!(format!("{}", category), "example");
    }

    #[test]
    fn test_category_from_str() {
        let category: Category = "test".parse().unwrap();
        assert_eq!(category.as_str(), "test");
    }

    #[test]
    fn test_category_equality() {
        let category1 = Category::new("test");
        let category2 = Category::new("test");
        let category3 = Category::new("other");
        assert_eq!(category1, category2);
        assert_ne!(category1, category3);
    }

    #[test]
    fn test_category_into_string() {
        let category = Category::new("test");
        let string: String = category.into();
        assert_eq!(string, "test");
    }
}
