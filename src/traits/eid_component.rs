use crate::{Account, Category, Domain, Part, Parts, Root};
use std::borrow::Cow;

/// Represents a component of a Ein (Akton Resource Name) that ensures type safety and ordering.
pub trait ArnComponent {
    /// Returns the prefix string that should appear before this component in a Ein.
    fn prefix() -> &'static str;
    /// The type of the next Ein component in the sequence.
    type NextState;
    /// Returns the string representation of this component.
    fn as_cow(&self) -> Cow<'static, str>;
}

macro_rules! impl_arn_component {
    ($type:ty, $prefix:expr, $next:ty) => {
        impl ArnComponent for $type {
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

impl_arn_component!(Domain, "arn:", Category);
impl_arn_component!(Category, "", Account);
impl_arn_component!(Account, "", Root);
impl_arn_component!(Root, "", Part);
impl_arn_component!(Part, "", Parts);

/// Implementation for the `Parts` component of a Ein.
impl ArnComponent for Parts {
    fn prefix() -> &'static str {
        ":"
    }
    type NextState = Parts;
    fn as_cow(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
