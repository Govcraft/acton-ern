use crate::errors::ArnError;
use crate::{Account, ArnComponent, Category, Domain, Part, Parts, Root};
use std::fmt;
use std::fmt::{Display, Formatter};

/// Represents an Akton Resource Name (Arn), which uniquely identifies resources within the Akton framework.
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Arn<'a> {
    pub domain: Domain<'a>,
    pub category: Category<'a>,
    pub account: Account<'a>,
    pub root: Root<'a>,
    pub parts: Parts<'a>,
}

impl Display for Arn<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut display = format!(
            "{}{}:{}:{}:{}",
            Domain::prefix(),
            self.domain,
            self.category,
            self.account,
            self.root
        );
        if !self.parts.0.is_empty() {
            display = format!("{}:{}", display, self.parts);
        }

        write!(f, "{}", display)
    }
}

impl<'a> Arn<'a> {
    /// Creates a new Arn with the given components.
    pub fn new(
        domain: Domain<'a>,
        category: Category<'a>,
        account: Account<'a>,
        root: Root<'a>,
        parts: Parts<'a>,
    ) -> Self {
        Arn {
            domain,
            category,
            account,
            root,
            parts,
        }
    }

    /// Appends a new part to the Arn.
    ///
    /// # Arguments
    ///
    /// * `part` - A string slice representing the part to be added.
    ///
    /// # Panics
    ///
    /// This function will panic if `part` starts with ':' or contains '/'.
    pub fn append_part(&mut self, part: Part<'static>) -> Result<(), ArnError> {
        self.parts = self.parts.clone().add_part(part);
        Ok(())
    }

    // /// Produces the full Arn string.
    // pub fn to_string(&self) -> String {
    //     format!("{}{}:{}:{}:{}",
    //             Domain::prefix(),
    //             self.domain,
    //             self.category,
    //             self.account,
    //             self.parts
    //     )
    // }
}

impl<'a> Default for Arn<'a> {
    /// Provides a default value for Arn using the defaults of all its components.
    fn default() -> Self {
        Arn {
            domain: Domain::default(),
            category: Category::default(),
            account: Account::default(),
            root: Root::default(),
            parts: Parts::new(Vec::default()),
        }
    }
}

// impl<'a> fmt::Display for Arn<'a> {
//     /// Formats the Arn as a string.
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Part;

    #[test]
    fn test_arn_custom() -> anyhow::Result<()> {
        let arn = Arn::new(
            Domain::new("custom"),
            Category::new("service"),
            Account::new("account123"),
            Root::new("root")?,
            Parts::new(vec![Part::new("resource")?]),
        );
        assert_eq!(
            arn.to_string(),
            "arn:custom:service:account123:root:resource"
        );
        Ok(())
    }

    #[test]
    fn test_arn_append_part() -> anyhow::Result<()> {
        let mut arn = Arn::default();
        let arn_clone = arn.clone();
        arn.append_part(Part::new("subresource")?)?;
        let expected = format!("{}:subresource", arn_clone);
        assert_eq!(arn.to_string(), expected);
        Ok(())
    }

    #[test]
    fn test_arn_append_invalid_part() -> anyhow::Result<()> {
        let invalid_part = Part::new(":invalid");

        assert!(invalid_part.is_err());
        Ok(())
    }
}
