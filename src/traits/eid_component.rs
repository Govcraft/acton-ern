use std::borrow::Cow;

use crate::{Account, Category, Domain, IdType, Part, Parts, Root};

/// Represents a component of a Ein (Akton Resource Name) that ensures type safety and ordering.
pub trait EidComponent {
    /// Returns the prefix string that should appear before this component in a Ein.
    fn prefix() -> &'static str;
    /// The type of the next Ein component in the sequence.
    type NextState;
    /// Returns the string representation of this component.
    fn as_cow(&self) -> Cow<'static, str>;
}

macro_rules! impl_eid_component {
    ($type:ty, $prefix:expr, $next:ty) => {
        impl EidComponent for $type {
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
impl<T: IdType + Clone + PartialEq> EidComponent for Root<T> {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = Part;
    fn as_cow(&self) -> Cow<'static, str> {
        self.name.clone()
    }
}
impl EidComponent for Account {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = Root;
    fn as_cow(&self) -> Cow<'static, str> {
        self.0.clone()
    }
}

impl_eid_component!(Domain, "eid:", Category);
impl_eid_component!(Category, "", Account);
impl_eid_component!(Part, "", Parts);

/// Implementation for the `Parts` component of a Ein.
impl EidComponent for Parts {
    fn prefix() -> &'static str {
        ":"
    }
    type NextState = Parts;
    fn as_cow(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
