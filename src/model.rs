use std::fmt;

// Base types used for both building and parsing
#[derive(Debug, PartialEq)]
pub struct Domain(pub(crate) String);
#[derive(Debug, PartialEq)]
pub struct Category(pub(crate) String);
#[derive(Debug, PartialEq)]
pub struct Company(pub(crate) String);

#[derive(Debug, PartialEq)]
pub struct Part(pub(crate) String);  // Represents a single part of the QRN

impl Part {
    pub fn new(value: &str) -> Self {
        Part(value.to_owned())
    }
}

#[derive(Debug, PartialEq)]
pub struct Parts(pub(crate) Vec<Part>);  // Represents a collection of parts

impl Parts {
    pub fn new(parts: Vec<Part>) -> Self {
        Parts(parts)
    }

    pub fn add_part(mut self, part: Part) -> Self {
        self.0.push(part);
        self
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts_str = self.0.iter().map(|p| p.0.as_ref()).collect::<Vec<_>>().join("/");
        write!(f, "{}", parts_str)
    }
}


impl Domain {
    pub fn new(value: &str) -> Self {
        Domain(value.to_owned())
    }
}

impl Category {
    pub fn new(value: &str) -> Self {
        Category(value.to_owned())
    }
}

impl Company {
    pub fn new(value: &str) -> Self {
        Company(value.to_owned())
    }
}


impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


// Using trait for common parsing functionality
trait FromStr {
    fn from_str(s: &str) -> Result<Self, &'static str> where Self: Sized;
}

impl FromStr for Domain {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Domain(s.to_string()))
        } else {
            Err("Invalid domain")
        }
    }
}

impl FromStr for Category {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Category(s.to_string()))
        } else {
            Err("Invalid category")
        }
    }
}

impl FromStr for Company {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Company(s.to_string()))
        } else {
            Err("Invalid company")
        }
    }
}


