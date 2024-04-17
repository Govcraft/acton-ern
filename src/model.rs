use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub struct Qrn {
    pub value: String
}

impl Default for Qrn {
    fn default() -> Self {
        Qrn{ value: "qrn:quasar:system:framework:root".to_string() }
    }
}


impl fmt::Display for Qrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


/// Represents a domain in the QRN system, which is a segment of the identifier.
#[derive(Debug, PartialEq)]
pub struct Domain(pub(crate) String);

impl Domain {
    /// Constructs a new `Domain`.
    pub fn new(value: &str) -> Self {
        Domain(value.to_owned())
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a category in the QRN system, typically indicating the service.
#[derive(Debug, PartialEq)]
pub struct Category(pub(crate) String);

impl Category {
    /// Constructs a new `Category`.
    pub fn new(value: &str) -> Self {
        Category(value.to_owned())
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a company or account identifier in the QRN system.
#[derive(Debug, PartialEq)]
pub struct Company(pub(crate) String);

impl Company {
    /// Constructs a new `Company`.
    pub fn new(value: &str) -> Self {
        Company(value.to_owned())
    }
}

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a single part of a QRN, typically one segment of a path.
#[derive(Debug, PartialEq)]
pub struct Part(pub(crate) String);

impl Part {
    /// Constructs a new `Part`.
    pub fn new(value: &str) -> Self {
        Part(value.to_owned())
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a collection of parts in the QRN, handling multiple segments.
#[derive(Debug, PartialEq)]
pub struct Parts(pub(crate) Vec<Part>);

impl Parts {
    /// Constructs a new collection of `Parts`.
    pub fn new(parts: Vec<Part>) -> Self {
        Parts(parts)
    }

    /// Adds a part to the collection.
    pub fn add_part(mut self, part: Part) -> Self {
        self.0.push(part);
        self
    }
}

impl fmt::Display for Parts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts_str = self.0.iter().map(|p| p.0.as_ref()).collect::<Vec<_>>().join("/");
        write!(f, "{}", parts_str)
    }
}