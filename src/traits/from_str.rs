use crate::{Category, Company, Domain, Part, Parts};

/// Trait for parsing strings into domain, category, or company components.
pub trait FromStr {
    /// Parses a string into a component of a Arn.
    fn from_str(s: &str) -> Result<Self, &'static str> where Self: Sized;
}

impl FromStr for Domain {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Domain(s.to_string()))
        } else {
            Err("Invalid domain")
        }
    }
}

impl FromStr for Category {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Category(s.to_string()))
        } else {
            Err("Invalid category")
        }
    }
}

impl FromStr for Company {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Company(s.to_string()))
        } else {
            Err("Invalid company")
        }
    }
}

/// Implementation of `ArnComponent` for `Parts`.
impl FromStr for Parts {
    /// Parses a string into a collection of `Part` instances, representing multiple Arn parts.
    fn from_str(s: &str) -> Result<Self, &'static str> {
        let parts = s.split('/')
            .map(|part| Part::new(part)) // Create a `Part` from each split string
            .collect(); // Collect into Vec<Part>
        Ok(Parts(parts))
    }
}