use crate::{Account, Category, Domain, Part, Parts, Root};
use std::borrow::Cow;

/// Represents a component of a Arn (Akton Resource Name) that ensures type safety and ordering.
pub trait ArnComponent<'a> {
    /// Returns the prefix string that should appear before this component in a Arn.
    fn prefix() -> &'static str;
    /// The type of the next Arn component in the sequence.
    type NextState;
    /// Returns the string representation of this component.
    fn as_cow(&self) -> Cow<'a, str>;
}

macro_rules! impl_arn_component {
    ($type:ty, $prefix:expr, $next:ty) => {
        impl<'a> ArnComponent<'a> for $type {
            fn prefix() -> &'static str {
                $prefix
            }
            type NextState = $next;
            fn as_cow(&self) -> Cow<'a, str> {
                self.0.clone()
            }
        }
    };
}

impl_arn_component!(Domain<'a>, "arn:", Category<'a>);
impl_arn_component!(Category<'a>, "", Account<'a>);
impl_arn_component!(Account<'a>, "", Root<'a>);
impl_arn_component!(Root<'a>, "", Part<'a>);
impl_arn_component!(Part<'a>, "", Parts<'a>);

/// Implementation for the `Parts` component of a Arn.
impl<'a> ArnComponent<'a> for Parts<'a> {
    fn prefix() -> &'static str {
        ":"
    }
    type NextState = Parts<'a>;
    fn as_cow(&self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }
}
