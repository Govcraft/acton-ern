use std::fmt::Display;
use crate::model::{Category, Company, Domain, Parts};
use crate::FromStr;

/// A parser for decoding QRN strings into their constituent components.
pub struct QrnParser {
    qrn: String,
}

impl QrnParser {
    /// Constructs a new `QrnParser` for a given QRN string.
    pub fn new(qrn: &str) -> Self {
        Self {
            qrn: qrn.to_string(),
        }
    }

    /// Parses the QRN into its component parts and returns them as a structured result.
    /// Verifies correct QRN format and validates each part.
    pub fn parse(&self) -> Result<(Domain, Category, Company, Parts), &'static str> {
        assert!(self.qrn.starts_with("qrn:"), "QRN should start with 'qrn:'");

        let without_prefix = &self.qrn[4..]; // Skip the "qrn:" part
        let parts: Vec<&str> = without_prefix.split(':').collect();

        assert_eq!(parts.len(), 4, "There must be exactly four parts after 'qrn:'");
        parts.iter().enumerate().for_each(|(index, part)| {
            assert!(!part.is_empty(), "Part {} must not be empty", index);
            assert!(!part.contains(':'), "Part {} must not contain colons", index);
        });

        let domain = Domain::from_str(parts[0]).map_err(|_| "Failed to parse Domain")?;
        let category = Category::from_str(parts[1]).map_err(|_| "Failed to parse Category")?;
        let company = Company::from_str(parts[2]).map_err(|_| "Failed to parse Company")?;
        let part_sections = Parts::from_str(parts[3]).map_err(|_| "Failed to parse Parts")?;

        Ok((domain, category, company, part_sections))
    }
}
