use std::fmt::Display;
use crate::model::{Category, Company, Domain, Parts};
use crate::Part;

// Define the markpub traits for each QRN component
trait QrnComponent {
    fn from_str(s: &str) -> Result<Self, &'static str>
        where
            Self: Sized;
}
impl QrnComponent for Domain {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Domain(s.to_string()))
        } else {
            Err("Invalid domain")
        }
    }
}

impl QrnComponent for Category {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Category(s.to_string()))
        } else {
            Err("Invalid category")
        }
    }
}

impl QrnComponent for Company {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if !s.is_empty() && !s.contains(' ') {
            Ok(Company(s.to_string()))
        } else {
            Err("Invalid company")
        }
    }
}

impl QrnComponent for Parts {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        let parts = s.split('/')
            .map(|part| Part::new(part)) // Create a Part from each split string
            .collect(); // Collect into Vec<Part>
        Ok(Parts(parts))
    }
}


pub struct QrnParser {
    qrn: String,
}

impl QrnParser {
    pub fn new(qrn: &str) -> Self {
        Self {
            qrn: qrn.to_string(),
        }
    }

    // Parses the QRN into parts and returns them as a structured result
    pub fn parse(&self) -> Result<(Domain, Category, Company, Parts), &'static str> {
        // Verify the QRN starts with "qrn:"
        assert!(self.qrn.starts_with("qrn:"), "QRN should start with 'qrn:'");

        let without_prefix = &self.qrn[4..];  // Skip the "qrn:" part
        let parts: Vec<&str> = without_prefix.split(':').collect();

        // Ensure there are exactly four parts after the prefix
        assert_eq!(parts.len(), 4, "There must be exactly four parts after 'qrn:'");

        // Validate each part is not empty and well-formed
        for (index, part) in parts.iter().enumerate() {
            assert!(!part.is_empty(), "Part {} must not be empty", index);
            // ensure no internal colons or illegal characters
            assert!(!part.contains(':'), "Part {} must not contain colons", index);
        }

        // Try to construct each component from the parts
        let domain = Domain::from_str(parts[0])
            .map_err(|_| "Failed to parse Domain")?;
        let category = Category::from_str(parts[1])
            .map_err(|_| "Failed to parse Category")?;
        let company = Company::from_str(parts[2])
            .map_err(|_| "Failed to parse Company")?;
        let part_sections = Parts::from_str(parts[3])
            .map_err(|_| "Failed to parse Parts")?;

        //ensure the parts meet any specific criteria
        assert!(domain.0.len() > 2, "Domain is too short");
        assert!(category.0.len() > 1, "Category is too short");
        assert!(company.0.len() > 2, "Company is too short");

        Ok((domain, category, company, part_sections))
    }

}

