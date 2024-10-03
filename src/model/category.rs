use std::fmt;

use derive_more::{AsRef, Into};

/// Represents a category in the ERN (Entity Resource Name) system, typically indicating the service.
#[derive(AsRef, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct Category(pub(crate) String);

impl Category {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn new(value: impl Into<String>) -> Self {
        Category(value.into())
    }
    pub fn into_owned(self) -> Category {
        Category(self.0.to_string())
    }
}

impl Default for Category {
    fn default() -> Self {
        Category("system".to_string())
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Category {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Category(s.to_string()))
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
