use crate::{Category, Company, Domain, Part, Parts};

// Define marker traits for QRN parts to ensure type safety and ordering
pub trait QrnComponent {
    fn prefix() -> &'static str;
    type NextState;  // Removed the lifetime specifier since it's not needed here
}

impl QrnComponent for Domain {
    fn prefix() -> &'static str { "qrn:" }
    type NextState = Category;
}

impl QrnComponent for Category {
    fn prefix() -> &'static str { "" }
    type NextState = Company;
}

impl QrnComponent for Company {
    fn prefix() -> &'static str { "" }
    type NextState = Part;  // Now correctly references the Part struct
}

impl QrnComponent for Part {
    fn prefix() -> &'static str { "" }
    type NextState = Parts;  // Change to Parts if one Part leads to many Parts
}

impl QrnComponent for Parts {
    fn prefix() -> &'static str { ":" }
    type NextState = Parts;  // Allow continuous addition of parts within Parts
}
