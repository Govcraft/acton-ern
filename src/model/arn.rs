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
            display = format!("{}/{}", display, self.parts);
        }
        write!(f, "{}", display)
    }
}
use std::ops::Add;

impl<'a> Add for Arn<'a> {
    type Output = Arn<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_parts = self.parts.0;
        new_parts.extend(rhs.parts.0);
        Arn {
            domain: self.domain,
            category: self.category,
            account: self.account,
            root: self.root,
            parts: Parts(new_parts),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Part;
    use std::str::FromStr;

    #[test]
    fn test_add_arns() -> anyhow::Result<()> {
        let parent_root = Root::from_str("root_a")?;
        let parent = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            parent_root.clone(),
            Parts(vec![
                Part::from_str("department_a").unwrap(),
                Part::from_str("team1").unwrap(),
            ]),
        );

        let child = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("root_b").unwrap(),
            Parts(vec![Part::from_str("role_x").unwrap()]),
        );

        let combined = parent + child;

        assert_eq!(combined.domain, Domain::from_str("akton-internal").unwrap());
        assert_eq!(combined.category, Category::from_str("hr").unwrap());
        assert_eq!(combined.account, Account::from_str("company123").unwrap());
        assert_eq!(combined.root, parent_root);
        assert_eq!(
            combined.parts,
            Parts(vec![
                Part::from_str("department_a").unwrap(),
                Part::from_str("team1").unwrap(),
                Part::from_str("role_x").unwrap(),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_add_arns_empty_child() {
        let parent = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootp").unwrap(),
            Parts(vec![Part::from_str("department_a").unwrap()]),
        );

        let child = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootc").unwrap(),
            Parts(vec![]),
        );

        let combined = parent + child;

        assert_eq!(
            combined.parts,
            Parts(vec![Part::from_str("department_a").unwrap()])
        );
    }

    #[test]
    fn test_add_arns_empty_parent() {
        let parent = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootp").unwrap(),
            Parts(vec![]),
        );
        let child = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootc").unwrap(),
            Parts(vec![Part::from_str("role_x").unwrap()]),
        );
        let combined = parent + child;
        assert_eq!(
            combined.parts,
            Parts(vec![Part::from_str("role_x").unwrap()])
        );
    }

    #[test]
    fn test_add_arns_display() {
        let parent = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootp").unwrap(),
            Parts(vec![Part::from_str("department_a").unwrap()]),
        );

        let child = Arn::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootc").unwrap(),
            Parts(vec![Part::from_str("team1").unwrap()]),
        );

        let combined = parent + child;

        assert!(combined
            .to_string()
            .starts_with("arn:akton-internal:hr:company123:rootp"));
    }
    #[test]
    fn test_arn_custom() -> anyhow::Result<()> {
        let arn = Arn::new(
            Domain::new("custom"),
            Category::new("service"),
            Account::new("account123"),
            Root::new("root")?,
            Parts::new(vec![Part::new("resource")?]),
        );
        assert!(arn
            .to_string()
            .starts_with("arn:custom:service:account123:root"));
        Ok(())
    }

    #[test]
    fn test_arn_append_part() -> anyhow::Result<()> {
        let mut arn = Arn::default();
        let arn_clone = arn.clone();
        arn.append_part(Part::new("subresource")?)?;
        let expected = format!("{}/subresource", arn_clone);
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
