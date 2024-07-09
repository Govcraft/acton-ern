use std::borrow::Cow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Add;

use crate::{Account, Category, Domain, EidComponent, IdType, Part, Parts, Root};
use crate::errors::EidError;

/// Represents an Akton Resource Name (Ein), which uniquely identifies resources within the Akton framework.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Ein<T: IdType + Clone + PartialEq> {
    pub domain: Domain,
    pub category: Category,
    pub account: Account,
    pub root: Root<T>,
    pub parts: Parts,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IdType + Clone + PartialEq> Display for Ein<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl<T: IdType + Clone + PartialEq> Add for Ein<T> {
    type Output = Ein<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_parts = self.parts.0;
        new_parts.extend(rhs.parts.0);
        Ein {
            domain: self.domain,
            category: self.category,
            account: self.account,
            root: self.root,
            parts: Parts(new_parts),
            _marker: Default::default(),
        }
    }
}
impl<T: IdType + Clone + PartialEq> Ein<T> {
    /// Creates a new Ein with the given components.
    pub fn new(
        domain: Domain,
        category: Category,
        account: Account,
        root: Root<T>,
        parts: Parts,
    ) -> Self {
        Ein {
            domain,
            category,
            account,
            root,
            parts,
            _marker: Default::default(),
        }
    }

    /// Creates a new Ein with the given root and default values for other fields
    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Result<Self, EidError> {
        let root = Root::new(root)?;
        Ok(Ein {
            root,
            ..Default::default()
        })
    }

    /// Creates a new Ein based on an existing Ein but with a new root
    pub fn with_new_root(&self, new_root: impl Into<Cow<'static, str>>) -> Result<Self, EidError> {
        let new_root = Root::new(new_root)?;
        Ok(Ein {
            domain: self.domain.clone(),
            category: self.category.clone(),
            account: self.account.clone(),
            root: new_root,
            parts: self.parts.clone(),
            _marker: Default::default(),
        })
    }

    pub fn with_domain(domain: impl Into<Cow<'static, str>>) -> Result<Self, EidError> {
        let domain = Domain::new(domain)?;
        Ok(Ein {
            domain,
            category: Category::default(),
            account: Account::default(),
            root: Root::default(),
            parts: Parts::default(),
            _marker: Default::default(),
        })
    }

    pub fn with_category(category: impl Into<Cow<'static, str>>) -> Result<Self, EidError> {
        let category = Category::new(category);
        Ok(Ein {
            domain: Domain::default(),
            category,
            account: Account::default(),
            root: Root::default(),
            parts: Parts::default(),
            _marker: Default::default(),
        })
    }

    pub fn with_account(account: impl Into<Cow<'static, str>>) -> Result<Self, EidError> {
        let account = Account::new(account);
        Ok(Ein {
            domain: Domain::default(),
            category: Category::default(),
            account,
            root: Root::default(),
            parts: Parts::default(),
            _marker: Default::default(),
        })
    }

    pub fn add_part(&self, part: impl Into<Cow<'static, str>>) -> Result<Self, EidError> {
        let mut new_parts = self.parts.clone();
        new_parts.0.push(Part::new(part)?);
        Ok(Ein {
            domain: self.domain.clone(),
            category: self.category.clone(),
            account: self.account.clone(),
            root: self.root.clone(),
            parts: new_parts,
            _marker: Default::default(),
        })
    }

    pub fn with_parts(
        &self,
        parts: impl IntoIterator<Item = impl Into<Cow<'static, str>>>,
    ) -> Result<Self, EidError> {
        let new_parts: Result<Vec<Part>, _> = parts.into_iter().map(Part::new).collect();
        Ok(Ein {
            domain: self.domain.clone(),
            category: self.category.clone(),
            account: self.account.clone(),
            root: self.root.clone(),
            parts: Parts(new_parts?),
            _marker: Default::default(),
        })
    }

    pub fn is_child_of(&self, other: &Ein<T>) -> bool {
        self.domain == other.domain
            && self.category == other.category
            && self.account == other.account
            && self.root == other.root
            && other.parts.0.len() < self.parts.0.len()
            && self.parts.0.starts_with(&other.parts.0)
    }

    pub fn parent(&self) -> Option<Self> {
        if self.parts.0.is_empty() {
            None
        } else {
            Some(Ein {
                domain: self.domain.clone(),
                category: self.category.clone(),
                account: self.account.clone(),
                root: self.root.clone(),
                parts: Parts(self.parts.0[..self.parts.0.len() - 1].to_vec()),
                _marker: Default::default(),
            })
        }
    }
}

impl<T: IdType + Clone + PartialEq> Default for Ein<T> {
    /// Provides a default value for Ein using the defaults of all its components.
    fn default() -> Self {
        Ein {
            domain: Domain::default(),
            category: Category::default(),
            account: Account::default(),
            root: Root::default(),
            parts: Parts::new(Vec::default()),
            _marker: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Part, UnixTime};

    use super::*;

    #[test]
    fn test_arn_with_root() {
        let arn: Ein<UnixTime> = Ein::with_root("custom_root").unwrap();
        assert!(arn.root.as_str().starts_with("custom_root"));
        assert_eq!(arn.domain, Domain::default());
        assert_eq!(arn.category, Category::default());
        assert_eq!(arn.account, Account::default());
        assert_eq!(arn.parts, Parts::default());
    }

    #[test]
    fn test_arn_with_new_root() {
        let original_arn: Ein<UnixTime> = Ein::default();
        let new_arn: Ein<UnixTime> = original_arn.with_new_root("new_root").unwrap();
        assert!(new_arn.root.as_str().starts_with("new_root"));
        assert_eq!(new_arn.domain, original_arn.domain);
        assert_eq!(new_arn.category, original_arn.category);
        assert_eq!(new_arn.account, original_arn.account);
        assert_eq!(new_arn.parts, original_arn.parts);
    }

    #[test]
    fn test_add_arns() -> anyhow::Result<()> {
        let parent_root: Root<UnixTime> = Root::from_str("root_a")?;
        let parent: Ein<UnixTime> = Ein::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            parent_root.clone(),
            Parts(vec![
                Part::from_str("department_a").unwrap(),
                Part::from_str("team1").unwrap(),
            ]),
        );

        let child: Ein<UnixTime> = Ein::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("root_b").unwrap(),
            Parts(vec![Part::from_str("role_x").unwrap()]),
        );

        let combined: Ein<UnixTime> = parent + child;

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
        let parent: Ein<UnixTime> = Ein::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootp").unwrap(),
            Parts(vec![Part::from_str("department_a").unwrap()]),
        );

        let child: Ein<UnixTime> = Ein::new(
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
        let parent: Ein<UnixTime> = Ein::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootp").unwrap(),
            Parts(vec![]),
        );
        let child: Ein<UnixTime> = Ein::new(
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
        let parent: Ein<UnixTime> = Ein::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootp").unwrap(),
            Parts(vec![Part::from_str("department_a").unwrap()]),
        );

        let child: Ein<UnixTime> = Ein::new(
            Domain::from_str("akton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            Root::from_str("rootc").unwrap(),
            Parts(vec![Part::from_str("team1").unwrap()]),
        );

        let combined = parent + child;

        assert!(combined
            .to_string()
            .starts_with("eid:akton-internal:hr:company123:rootp"));
    }
    #[test]
    fn test_arn_custom() -> anyhow::Result<()> {
        let arn: Ein<UnixTime> = Ein::new(
            Domain::new("custom")?,
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
    fn test_arn_append_invalid_part() -> anyhow::Result<()> {
        let invalid_part = Part::new(":invalid");

        assert!(invalid_part.is_err());
        Ok(())
    }
}
