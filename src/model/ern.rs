use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::Add;

use crate::{Account, Category, Domain, EntityRoot, ErnComponent, Part, Parts};
use crate::errors::ErnError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an Entity Resource Name (ERN), which uniquely identifies resources in distributed systems.
///
/// An ERN follows the URN format and has the structure:
/// `ern:domain:category:account:root/path/to/resource`
///
/// Each component serves a specific purpose:
/// - `domain`: Classifies the resource (e.g., internal, external, custom domains)
/// - `category`: Specifies the service or category within the system
/// - `account`: Identifies the owner or account responsible for the resource
/// - `root`: A unique identifier for the root of the resource hierarchy
/// - `parts`: Optional path-like structure showing the resource's position within the hierarchy
///
/// ERNs can be k-sortable when using `UnixTime` or `Timestamp` ID types, enabling
/// efficient ordering and range queries.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ern {
    pub domain: Domain,
    pub category: Category,
    pub account: Account,
    pub root: EntityRoot,
    pub parts: Parts,
}

impl Ord for Ern {
    fn cmp(&self, other: &Self) -> Ordering {
        self.root.name().cmp(other.root.name())
    }
}

impl PartialOrd for Ern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Ern {
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

impl Add for Ern {
    type Output = Ern;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_parts = self.parts.0;
        new_parts.extend(rhs.parts.0);
        Ern {
            domain: self.domain,
            category: self.category,
            account: self.account,
            root: self.root,
            parts: Parts(new_parts),
        }
    }
}

impl Ern {
    /// Creates a new ERN with the specified components.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain component
    /// * `category` - The category component
    /// * `account` - The account component
    /// * `root` - The root component
    /// * `parts` - The path parts component
    ///
    /// # Example
    ///
    /// ```
    /// # use acton_ern::prelude::*;
    /// # fn example() -> Result<(), ErnError> {
    /// let ern = Ern::new(
    ///     Domain::new("my-app")?,
    ///     Category::new("users")?,
    ///     Account::new("tenant123")?,
    ///     EntityRoot::new("profile".to_string())?,
    ///     Parts::new(vec![Part::new("settings")?]),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        domain: Domain,
        category: Category,
        account: Account,
        root: EntityRoot,
        parts: Parts,
    ) -> Self {
        Ern {
            domain,
            category,
            account,
            root,
            parts,
        }
    }

        /// Creates a new ERN with the given root and default values for other components.
        ///
        /// This is a convenient way to create an ERN when you only care about the root component.
        ///
        /// # Arguments
        ///
        /// * `root` - The string value for the root component
        ///
        /// # Returns
        ///
        /// * `Ok(Ern)` - The created ERN with default values for domain, category, account, and parts
        /// * `Err(ErnError)` - If the root value is invalid
        ///
        /// # Example
        ///
        /// ```
        /// # use acton_ern::prelude::*;
        /// # fn example() -> Result<(), ErnError> {
        /// let ern = Ern::with_root("profile")?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn with_root(root: impl Into<String>) -> Result<Self, ErnError> {
            let root = EntityRoot::new(root.into())?;
            Ok(Ern {
                root,
                ..Default::default()
            })
        }

        /// Creates a new ERN based on an existing ERN but with a different root.
        ///
        /// This method preserves all other components (domain, category, account, parts)
        /// but replaces the root with a new value.
        ///
        /// # Arguments
        ///
        /// * `new_root` - The string value for the new root component
        ///
        /// # Returns
        ///
        /// * `Ok(Ern)` - A new ERN with the updated root
        /// * `Err(ErnError)` - If the new root value is invalid
        ///
        /// # Example
        ///
        /// ```
        /// # use acton_ern::prelude::*;
        /// # fn example() -> Result<(), ErnError> {
        /// let ern1 = Ern::with_root("profile")?;
        /// let ern2 = ern1.with_new_root("settings")?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn with_new_root(&self, new_root: impl Into<String>) -> Result<Self, ErnError> {
            let new_root = EntityRoot::new(new_root.into())?;
            Ok(Ern {
                domain: self.domain.clone(),
                category: self.category.clone(),
                account: self.account.clone(),
                root: new_root,
                parts: self.parts.clone(),
            })
        }

        pub fn with_domain(domain: impl Into<String>) -> Result<Self, ErnError> {
            let domain = Domain::new(domain)?;
            Ok(Ern {
                domain,
                category: Category::default(),
                account: Account::default(),
                root: EntityRoot::default(),
                parts: Parts::default(),
            })
        }

        pub fn with_category(category: impl Into<String>) -> Result<Self, ErnError> {
            let category = Category::new(category)?;
            Ok(Ern {
                domain: Domain::default(),
                category,
                account: Account::default(),
                root: EntityRoot::default(),
                parts: Parts::default(),
            })
        }

        pub fn with_account(account: impl Into<String>) -> Result<Self, ErnError> {
            let account = Account::new(account)?;
            Ok(Ern {
                domain: Domain::default(),
                category: Category::default(),
                account,
                root: EntityRoot::default(),
                parts: Parts::default(),
            })
        }

        /// Adds a new part to the ERN's path.
        ///
        /// This method creates a new ERN with the same domain, category, account, and root,
        /// but with an additional part appended to the path.
        ///
        /// # Arguments
        ///
        /// * `part` - The string value for the new part
        ///
        /// # Returns
        ///
        /// * `Ok(Ern)` - A new ERN with the added part
        /// * `Err(ErnError)` - If the part value is invalid or adding it would exceed the maximum of 10 parts
        ///
        /// # Example
        ///
        /// ```
        /// # use acton_ern::prelude::*;
        /// # fn example() -> Result<(), ErnError> {
        /// let ern1 = Ern::with_root("profile")?;
        /// let ern2 = ern1.add_part("settings")?;
        /// let ern3 = ern2.add_part("appearance")?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn add_part(&self, part: impl Into<String>) -> Result<Self, ErnError> {
            let new_part = Part::new(part)?;
            let mut new_parts = self.parts.clone();
            
            // Check if adding another part would exceed the maximum
            if new_parts.0.len() >= 10 {
                return Err(ErnError::ParseFailure(
                    "Parts",
                    "cannot exceed maximum of 10 parts".to_string(),
                ));
            }
            
            new_parts.0.push(new_part);
            Ok(Ern {
                domain: self.domain.clone(),
                category: self.category.clone(),
                account: self.account.clone(),
                root: self.root.clone(),
                parts: new_parts,
            })
        }

        pub fn with_parts(
            &self,
            parts: impl IntoIterator<Item = impl Into<String>>,
        ) -> Result<Self, ErnError> {
            let new_parts: Result<Vec<Part>, _> = parts.into_iter().map(Part::new).collect();
            Ok(Ern {
                domain: self.domain.clone(),
                category: self.category.clone(),
                account: self.account.clone(),
                root: self.root.clone(),
                parts: Parts(new_parts?),
            })
        }

        /// Checks if this ERN is a child of another ERN.
        ///
        /// An ERN is considered a child of another ERN if:
        /// 1. They have the same domain, category, account, and root
        /// 2. The child's parts start with all of the parent's parts
        /// 3. The child has more parts than the parent
        ///
        /// # Arguments
        ///
        /// * `other` - The potential parent ERN
        ///
        /// # Returns
        ///
        /// * `true` - If this ERN is a child of the other ERN
        /// * `false` - Otherwise
        ///
        /// # Example
        ///
        /// ```
        /// # use acton_ern::prelude::*;
        /// # fn example() -> Result<(), ErnError> {
        /// let parent = Ern::with_root("profile")?.add_part("settings")?;
        /// let child = parent.add_part("appearance")?;
        ///
        /// assert!(child.is_child_of(&parent));
        /// assert!(!parent.is_child_of(&child));
        /// # Ok(())
        /// # }
        /// ```
        pub fn is_child_of(&self, other: &Ern) -> bool {
            self.domain == other.domain
                && self.category == other.category
                && self.account == other.account
                && self.root == other.root
                && other.parts.0.len() < self.parts.0.len()
                && self.parts.0.starts_with(&other.parts.0)
        }

        /// Returns the parent ERN of this ERN, if it exists.
        ///
        /// The parent ERN has the same domain, category, account, and root,
        /// but with one fewer part in the path. If this ERN has no parts,
        /// it has no parent and this method returns `None`.
        ///
        /// # Returns
        ///
        /// * `Some(Ern)` - The parent ERN
        /// * `None` - If this ERN has no parts (and thus no parent)
        ///
        /// # Example
        ///
        /// ```
        /// # use acton_ern::prelude::*;
        /// # fn example() -> Result<(), ErnError> {
        /// let ern1 = Ern::with_root("profile")?;
        /// let ern2 = ern1.add_part("settings")?;
        /// let ern3 = ern2.add_part("appearance")?;
        ///
        /// assert_eq!(ern3.parent().unwrap().to_string(), ern2.to_string());
        /// assert_eq!(ern2.parent().unwrap().to_string(), ern1.to_string());
        /// assert!(ern1.parent().is_none());
        /// # Ok(())
        /// # }
        /// ```
        pub fn parent(&self) -> Option<Self> {
            if self.parts.0.is_empty() {
                None
            } else {
                Some(Ern {
                    domain: self.domain.clone(),
                    category: self.category.clone(),
                    account: self.account.clone(),
                    root: self.root.clone(),
                    parts: Parts(self.parts.0[..self.parts.0.len() - 1].to_vec()),
                })
            }
        }
}

impl Default for Ern {
    /// Provides a default ERN using the default values of all its components.
    ///
    /// This is primarily used internally and for testing. For creating ERNs in
    /// application code, prefer using `ErnBuilder` or the `with_root` method.
    fn default() -> Self {
        Ern {
            domain: Domain::default(),
            category: Category::default(),
            account: Account::default(),
            root: EntityRoot::default(),
            parts: Parts::new(Vec::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::thread::sleep;
    use std::time::Duration;

    use crate::Part;

    use super::*;

    #[test]
    fn test_ern_timestamp_ordering() {
        let ern1: Ern = Ern::with_root("root_a").unwrap();
        sleep(Duration::from_millis(10));
        let ern2: Ern = Ern::with_root("root_b").unwrap();
        sleep(Duration::from_millis(10));
        let ern3: Ern = Ern::with_root("root_c").unwrap();

        // Test ascending order
        assert!(ern1 < ern2);
        assert!(ern2 < ern3);
        assert!(ern1 < ern3);

        // Test descending order
        assert!(ern3 > ern2);
        assert!(ern2 > ern1);
        assert!(ern3 > ern1);

        // Test equality
        let ern1_clone = ern1.clone();
        assert_eq!(ern1, ern1_clone);

        // Test sorting
        let mut erns = vec![ern3.clone(), ern1.clone(), ern2.clone()];
        erns.sort();
        assert_eq!(erns, vec![ern1, ern2, ern3]);
    }

    #[test]
    fn test_ern_unixtime_ordering() {
        let ern1: Ern = Ern::with_root("root_a").unwrap();
        sleep(Duration::from_millis(10));
        let ern2: Ern = Ern::with_root("root_b").unwrap();
        sleep(Duration::from_millis(10));
        let ern3: Ern = Ern::with_root("root_c").unwrap();

        // Test ascending order
        assert!(ern1 < ern2);
        assert!(ern2 < ern3);
        assert!(ern1 < ern3);

        // Test descending order
        assert!(ern3 > ern2);
        assert!(ern2 > ern1);
        assert!(ern3 > ern1);

        // Test equality
        let ern1_clone = ern1.clone();
        assert_eq!(ern1, ern1_clone);

        // Test sorting
        let mut erns = vec![ern3.clone(), ern1.clone(), ern2.clone()];
        erns.sort();
        assert_eq!(erns, vec![ern1, ern2, ern3]);
    }

    #[test]
    fn test_ern_timestamp_unixtime_consistency() {
        let ern_timestamp1: Ern = Ern::with_root("root_a").unwrap();
        let ern_unixtime1: Ern = Ern::with_root("root_a").unwrap();

        sleep(Duration::from_millis(10));

        let ern_timestamp2: Ern = Ern::with_root("root_b").unwrap();
        let ern_unixtime2: Ern = Ern::with_root("root_b").unwrap();

        // Ensure that the ordering is consistent between Timestamp and UnixTime
        assert_eq!(
            ern_timestamp1 < ern_timestamp2,
            ern_unixtime1 < ern_unixtime2
        );
    }
    #[test]
    fn test_ern_with_root() {
        let ern: Ern = Ern::with_root("custom_root").unwrap();
        assert!(ern.root.as_str().starts_with("custom_root"));
        assert_eq!(ern.domain, Domain::default());
        assert_eq!(ern.category, Category::default());
        assert_eq!(ern.account, Account::default());
        assert_eq!(ern.parts, Parts::default());
    }

    #[test]
    fn test_ern_with_new_root() {
        let original_ern: Ern = Ern::default();
        let new_ern: Ern = original_ern.with_new_root("new_root").unwrap();
        assert!(new_ern.root.as_str().starts_with("new_root"));
        assert_eq!(new_ern.domain, original_ern.domain);
        assert_eq!(new_ern.category, original_ern.category);
        assert_eq!(new_ern.account, original_ern.account);
        assert_eq!(new_ern.parts, original_ern.parts);
    }

    #[test]
    fn test_add_erns() -> anyhow::Result<()> {
        let parent_root = EntityRoot::from_str("root_a")?;
        let parent: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            parent_root.clone(),
            Parts(vec![
                Part::from_str("department_a").unwrap(),
                Part::from_str("team1").unwrap(),
            ]),
        );

        let child: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            EntityRoot::from_str("root_b").unwrap(),
            Parts(vec![Part::from_str("role_x").unwrap()]),
        );

        let combined: Ern = parent + child;

        assert_eq!(combined.domain, Domain::from_str("acton-internal").unwrap());
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
    fn test_add_erns_empty_child() {
        let parent: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            EntityRoot::from_str("rootp").unwrap(),
            Parts(vec![Part::from_str("department_a").unwrap()]),
        );

        let child: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            EntityRoot::from_str("rootc").unwrap(),
            Parts(vec![]),
        );

        let combined = parent + child;

        assert_eq!(
            combined.parts,
            Parts(vec![Part::from_str("department_a").unwrap()])
        );
    }

    #[test]
    fn test_add_erns_empty_parent() {
        let parent: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            EntityRoot::from_str("rootp").unwrap(),
            Parts(vec![]),
        );
        let child: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            EntityRoot::from_str("rootc").unwrap(),
            Parts(vec![Part::from_str("role_x").unwrap()]),
        );
        let combined = parent + child;
        assert_eq!(
            combined.parts,
            Parts(vec![Part::from_str("role_x").unwrap()])
        );
    }

    #[test]
    fn test_add_erns_display() {
        let parent: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            EntityRoot::from_str("rootp").unwrap(),
            Parts(vec![Part::from_str("department_a").unwrap()]),
        );

        let child: Ern = Ern::new(
            Domain::from_str("acton-internal").unwrap(),
            Category::from_str("hr").unwrap(),
            Account::from_str("company123").unwrap(),
            EntityRoot::from_str("rootc").unwrap(),
            Parts(vec![Part::from_str("team1").unwrap()]),
        );

        let combined = parent + child;

        assert!(combined
            .to_string()
            .starts_with("ern:acton-internal:hr:company123:rootp"));
    }
    #[test]
    fn test_ern_custom() -> anyhow::Result<()> {
        let ern: Ern = Ern::new(
            Domain::new("custom")?,
            Category::new("service")?,
            Account::new("account123")?,
            EntityRoot::new("root".to_string())?,
            Parts::new(vec![Part::new("resource")?]),
        );
        assert!(ern
            .to_string()
            .starts_with("ern:custom:service:account123:root"));
        Ok(())
    }

    #[test]
    fn test_ern_append_invalid_part() -> anyhow::Result<()> {
        let invalid_part = Part::new(":invalid");

        assert!(invalid_part.is_err());
        Ok(())
    }
}
