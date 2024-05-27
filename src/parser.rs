use crate::model::{Category, Company, Domain, Parts};
use crate::FromStr;

/// A parser for decoding Arn strings into their constituent components.
pub struct ArnParser {
    /// The Arn string to be parsed.
    arn: String,
}

impl ArnParser {
    /// Constructs a new `ArnParser` for a given Arn string.
    ///
    /// # Arguments
    ///
    /// * `arn` - A string slice representing the Arn to be parsed.
    ///
    /// # Returns
    ///
    /// Returns an `ArnParser` instance initialized with the given Arn string.
    pub fn new(arn: &str) -> Self {
        Self {
            arn: arn.to_string(),
        }
    }

    /// Parses the Arn into its component parts and returns them as a structured result.
    /// Verifies correct Arn format and validates each part.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the `Domain`, `Category`, `Company`, and `Parts` components of the Arn.
    /// If parsing fails, returns an error message as a `&'static str`.
    ///
    /// # Panics
    ///
    /// This function will panic if the Arn does not start with "arn:", if there are not exactly four parts after "arn:",
    /// or if any part is empty or contains colons.
    pub fn parse(&self) -> Result<(Domain, Category, Company, Parts), &'static str> {
        // Ensure the Arn starts with the correct prefix
        assert!(self.arn.starts_with("arn:"), "Arn should start with 'arn:'");

        // Remove the "arn:" prefix for further processing
        let without_prefix = &self.arn[4..]; // Skip the "arn:" part
        // Split the remaining string into parts using ':' as the delimiter
        let parts: Vec<&str> = without_prefix.split(':').collect();

        // Ensure there are exactly four parts after the "arn:" prefix
        assert_eq!(parts.len(), 4, "There must be exactly four parts after 'arn:'");
        // Validate each part to ensure it is not empty and does not contain colons
        parts.iter().enumerate().for_each(|(index, part)| {
            assert!(!part.is_empty(), "Part {} must not be empty", index);
            assert!(!part.contains(':'), "Part {} must not contain colons", index);
        });

        // Parse each part into its respective component type, returning an error if any parsing fails
        let domain = Domain::from_str(parts[0]).map_err(|_| "Failed to parse Domain")?;
        let category = Category::from_str(parts[1]).map_err(|_| "Failed to parse Category")?;
        let company = Company::from_str(parts[2]).map_err(|_| "Failed to parse Company")?;
        let part_sections = Parts::from_str(parts[3]).map_err(|_| "Failed to parse Parts")?;

        // Return the parsed components as a tuple
        Ok((domain, category, company, part_sections))
    }
}
