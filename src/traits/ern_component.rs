use std::borrow::Cow;
use std::hash::Hash;

use crate::{Account, Category, Domain, IdType, Part, Parts, Root};

/// Represents a component of a ERN (Entity Resource Name) (Acton Resource Name) that ensures type safety and ordering.
pub trait ErnComponent {
    /// Returns the prefix string that should appear before this component in a ERN (Entity Resource Name).
    fn prefix() -> &'static str;
    /// The type of the next ERN (Entity Resource Name) component in the sequence.
    type NextState;
    /// Returns the string representation of this component.
    fn as_cow(&self) -> Cow<'static, str>;
}

macro_rules! impl_ern_component {
    ($type:ty, $prefix:expr, $next:ty) => {
        impl ErnComponent for $type {
            fn prefix() -> &'static str {
                $prefix
            }
            type NextState = $next;
            fn as_cow(&self) -> Cow<'static, str> {
                self.0.clone()
            }
        }
    };
}
impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> ErnComponent for Root<T> {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = Part;
    fn as_cow(&self) -> Cow<'static, str> {
        self.name.clone()
    }
}

impl ErnComponent for Account {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = Root;
    fn as_cow(&self) -> Cow<'static, str> {
        self.0.clone()
    }
}

impl_ern_component!(Domain, "ern:", Category);
impl_ern_component!(Category, "", Account);
impl_ern_component!(Part, "", Parts);

/// Implementation for the `Parts` component of a ERN (Entity Resource Name).
impl ErnComponent for Parts {
    fn prefix() -> &'static str {
        ":"
    }
    type NextState = Parts;
    fn as_cow(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
