use crate::{Account, Category, Domain, EntityRoot, Part, Parts};

/// Represents a component of an Entity Resource Name (ERN).
///
/// This trait is used to ensure type safety and proper ordering when building ERNs.
/// Each component in an ERN implements this trait, defining its prefix and the
/// type of the next component that should follow it in the ERN structure.
///
/// The trait is primarily used by the `ErnBuilder` to enforce the correct order
/// of components during ERN construction.
pub trait ErnComponent {
    /// Returns the prefix string that should appear before this component in an ERN.
    ///
    /// For example, the `Domain` component has the prefix "ern:" to indicate the
    /// start of an ERN string.
    fn prefix() -> &'static str;
    
    /// The type of the next component that should follow this one in the ERN structure.
    ///
    /// This associated type is used by the builder pattern to enforce the correct
    /// sequence of components. For example, `Domain::NextState` is `Category`,
    /// indicating that a `Category` should follow a `Domain` in an ERN.
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

/// Implementation for the `Parts` component of an ERN.
///
/// The `Parts` component represents a collection of path parts in the ERN.
/// Its `NextState` is itself, allowing for multiple parts to be added.
impl ErnComponent for Parts {
    fn prefix() -> &'static str {
        ":"
    }
    type NextState = Parts;
}
