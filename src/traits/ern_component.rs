use crate::{Account, Category, Domain, EntityRoot, Part, Parts};

/// Represents a component of a ERN (Entity Resource Name) (Acton Resource Name) that ensures type safety and ordering.
pub trait ErnComponent {
    /// Returns the prefix string that should appear before this component in a ERN (Entity Resource Name).
    fn prefix() -> &'static str;
    /// The type of the next ERN (Entity Resource Name) component in the sequence.
    type NextState;
}

macro_rules! impl_ern_component {
    ($type:ty, $prefix:expr, $next:ty) => {
        impl ErnComponent for $type {
            fn prefix() -> &'static str {
                $prefix
            }
            type NextState = $next;
        }
    };
}
impl ErnComponent for EntityRoot {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = Part;
}

impl ErnComponent for Account {
    fn prefix() -> &'static str {
        ""
    }
    type NextState = EntityRoot;
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
}
