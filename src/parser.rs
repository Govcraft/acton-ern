use crate::errors::ArnError;
use crate::model::{Account, Arn, Category, Domain, Part, Parts};
use crate::Root;
use std::borrow::Cow;
use std::str::FromStr;

/// A parser for decoding Arn strings into their constituent components.
pub struct ArnParser {
    /// The Arn string to be parsed.
    arn: Cow<'static, str>,
}

impl ArnParser {
    /// Constructs a new `ArnParser` for a given Arn string.
    ///
    /// # Arguments
    ///
    /// * `arn` - A string slice or owned String representing the Arn to be parsed.
    ///
    /// # Returns
    ///
    /// Returns an `ArnParser` instance initialized with the given Arn string.
    pub fn new(arn: impl Into<Cow<'static, str>>) -> Self {
        Self { arn: arn.into() }
    }

    /// Parses the Arn into its component parts and returns them as a structured result.
    /// Verifies correct Arn format and validates each part.
    ///
    /// # Returns
    ///
    /// Returns an `Arn` instance containing the parsed components.
    /// If parsing fails, returns an error message as a `String`.
    pub fn parse(&self) -> Result<Arn, ArnError> {
        let parts: Vec<&str> = self.arn.splitn(5, ':').collect();

        if parts.len() != 5 || parts[0] != "arn" {
            return Err(ArnError::InvalidFormat);
        }

        let domain = Domain::from_str(parts[1])?;
        let category = Category::from_str(parts[2])?;
        let account = Account::from_str(parts[3])?;

        // Split the root and the path part
        let root_path: Vec<&str> = parts[4].splitn(2, '/').collect();
        let root_str = root_path[0];
        let root = Root{ 0: root_str.to_string().into() };

        // Continue with the path parts
        let mut arn_parts = Vec::new();
        if root_path.len() > 1 {
            let path_parts: Vec<&str> = root_path[1].split('/').collect();
            for part in path_parts.iter() {
                arn_parts.push(Part::from_str(part)?);
            }
        }

        // // Process the remaining parts after root
        // let remaining_parts: Vec<&str> = parts[4].split("/").collect();
        // for part in remaining_parts.iter() {
        //     arn_parts.push(Part::from_str(part).map_err(|_| ParseError::InvalidPartFormat)?);
        // }

        let parts = Parts::new(arn_parts);
        Ok(Arn::new(domain, category, account, root, parts))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_arn_parsing() {
        let arn_str = "arn:custom:service:account123:root/resource/subresource";
        let parser = ArnParser::new(arn_str);
        let result = parser.parse();

        assert!(result.is_ok());
        let arn = result.unwrap();
        assert_eq!(arn.domain.as_str(), "custom");
    }

    #[test]
    fn test_invalid_arn_format() {
        let arn_str = "invalid:arn:format";
        let parser = ArnParser::new(arn_str);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ArnError::InvalidFormat);
        // assert_eq!(result.unwrap_err().to_string(), "Invalid ARN format");
    }

    #[test]
    fn test_arn_with_invalid_part() -> anyhow::Result<()> {
        let arn_str = "arn:domain:category:account:root/invalid:part";
        let parser = ArnParser::new(arn_str);
        let result = parser.parse();
        assert!(result.is_err());
        // assert!(result.unwrap_err().to_string().starts_with("Failed to parse Part"));
        Ok(())
    }

    #[test]
    fn test_arn_parsing_with_owned_string() {
        let arn_str = String::from("arn:custom:service:account123:root/resource");
        let parser = ArnParser::new(arn_str);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
