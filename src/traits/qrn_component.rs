use crate::{Category, Company, Domain, Part, Parts};

/// Represents a component of a QRN (Quasar Resource Name) that ensures type safety and ordering.
///
/// Each implementor of `QrnComponent` defines a prefix specific to its part of the QRN
/// and specifies the next expected component type, enabling compile-time checks of QRN construction.
pub trait QrnComponent {
    /// Returns the prefix string that should appear before this component in a QRN.
    fn prefix() -> &'static str;

    /// The type of the next QRN component in the sequence.
    type NextState;
}

/// Implementation for the `Domain` component of a QRN.
impl QrnComponent for Domain {
    /// The prefix for a domain component, typically the start of the QRN.
    fn prefix() -> &'static str { "qrn:" }

    /// The next component type following `Domain` is `Category`.
    type NextState = Category;
}

/// Implementation for the `Category` component of a QRN.
impl QrnComponent for Category {
    /// Categories do not have a prefix.
    fn prefix() -> &'static str { "" }

    /// The next component type following `Category` is `Company`.
    type NextState = Company;
}

/// Implementation for the `Company` component of a QRN.
impl QrnComponent for Company {
    /// Companies do not have a prefix.
    fn prefix() -> &'static str { "" }

    /// The next component type following `Company` is `Part`.
    type NextState = Part;  // Now correctly references the Part struct
}

/// Implementation for the `Part` component of a QRN.
impl QrnComponent for Part {
    /// Parts do not have a prefix.
    fn prefix() -> &'static str { "" }

    /// The next component type following `Part` is `Parts`.
    type NextState = Parts;  // Change to Parts if one Part leads to many Parts
}

/// Implementation for the `Parts` component of a QRN.
impl QrnComponent for Parts {
    /// Parts use a colon as a separator when multiple parts are chained.
    fn prefix() -> &'static str { ":" }

    /// `Parts` can be followed by additional `Parts`, allowing for a chain of multiple parts.
    type NextState = Parts;  // Allow continuous addition of parts within Parts
}
