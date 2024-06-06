use std::fmt;

/// Represents an Akton Resource Name (Arn), which uniquely identifies resources within the Akton framework.
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Arn {
    /// The full Arn string value.
    pub value: String,
}

impl Arn {
    /// Appends a new part to the Arn.
    ///
    /// # Arguments
    ///
    /// * `part` - A string slice representing the part to be added.
    ///
    /// # Panics
    ///
    /// This function will panic if `part` starts with ':' or contains '/'.
    pub fn append_part(&mut self, part: &str) {
        // Ensure the new part does not start with an erroneous separator
        assert!(!part.starts_with(':') && !part.contains('/'), "New part must not start with ':' or contain '/'");

        // Determine the correct separator based on the last segment in the Arn string
        // If the last segment is 'root' or if any part after the root has been added (contains '/'),
        // we should use '/', otherwise use ':'.
        let separator = if self.value.ends_with("root") || self.value.contains('/') {
            "/"
        } else {
            ":"
        };

        // Append the new part with the correct separator
        self.value.push_str(separator);
        self.value.push_str(part);
    }
}

impl Default for Arn {
    /// Provides a default value for Arn.
    ///
    /// The default Arn represents the root of the Akton framework system.
    fn default() -> Self {
        Arn { value: "arn:akton:system:framework:root".to_string() }
    }
}

impl fmt::Display for Arn {
    /// Formats the Arn as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Represents a domain in the Arn system, which is a segment of the identifier.
#[derive(Debug, PartialEq)]
pub struct Domain(pub(crate) String);

impl Domain {
    /// Constructs a new `Domain`.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice representing the domain value.
    pub fn new(value: &str) -> Self {
        Domain(value.to_owned())
    }
}

impl fmt::Display for Domain {
    /// Formats the domain as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a category in the Arn system, typically indicating the service.
#[derive(Debug, PartialEq)]
pub struct Category(pub(crate) String);

impl Category {
    /// Constructs a new `Category`.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice representing the category value.
    pub fn new(value: &str) -> Self {
        Category(value.to_owned())
    }
}

impl fmt::Display for Category {
    /// Formats the category as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a company or account identifier in the Arn system.
#[derive(Debug, PartialEq)]
pub struct Company(pub(crate) String);

impl Company {
    /// Constructs a new `Company`.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice representing the company value.
    pub fn new(value: &str) -> Self {
        Company(value.to_owned())
    }
}

impl fmt::Display for Company {
    /// Formats the company as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a single part of an Arn, typically one segment of a path.
#[derive(Debug, PartialEq)]
pub struct Part(pub(crate) String);

impl Part {
    /// Constructs a new `Part`.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice representing the part value.
    pub fn new(value: &str) -> Self {
        Part(value.to_owned())
    }
}

impl fmt::Display for Part {
    /// Formats the part as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a collection of parts in the Arn, handling multiple segments.
#[derive(Debug, PartialEq)]
pub struct Parts(pub(crate) Vec<Part>);

impl Parts {
    /// Constructs a new collection of `Parts`.
    ///
    /// # Arguments
    ///
    /// * `parts` - A vector of `Part` representing the parts of the Arn.
    pub fn new(parts: Vec<Part>) -> Self {
        Parts(parts)
    }

    /// Adds a part to the collection.
    ///
    /// # Arguments
    ///
    /// * `part` - The `Part` to be added to the collection.
    pub fn add_part(mut self, part: Part) -> Self {
        self.0.push(part);
        self
    }
}

impl fmt::Display for Parts {
    /// Formats the collection of parts as a string, joining them with '/'.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts_str = self.0.iter().map(|p| p.0.as_ref()).collect::<Vec<_>>().join("/");
        write!(f, "{}", parts_str)
    }
}
