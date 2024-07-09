use std::borrow::Cow;
use std::str::FromStr;

use crate::{IdType, Root};
use crate::errors::EidError;
use crate::model::{Account, Category, Domain, Ein, Part, Parts};

/// A parser for decoding Ein strings into their constituent components.
pub struct ArnParser<T: IdType + Clone + PartialEq> {
    /// The Ein string to be parsed.
    eid: Cow<'static, str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IdType + Clone + PartialEq> ArnParser<T> {
    /// Constructs a new `ArnParser` for a given Ein string.
    ///
    /// # Arguments
    ///
    /// * `arn` - A string slice or owned String representing the Ein to be parsed.
    ///
    /// # Returns
    ///
    /// Returns an `ArnParser` instance initialized with the given Ein string.
    pub fn new(eid: impl Into<Cow<'static, str>>) -> Self {
        Self {
            eid: eid.into(),
            _marker: Default::default(),
        }
    }

    /// Parses the Ein into its component parts and returns them as a structured result.
    /// Verifies correct Ein format and validates each part.
    ///
    /// # Returns
    ///
    /// Returns an `Ein` instance containing the parsed components.
    /// If parsing fails, returns an error message as a `String`.
    pub fn parse(&self) -> Result<Ein<T>, EidError> {
        let parts: Vec<String> = self.eid.splitn(5, ':').map(|s| s.to_string()).collect();

        if parts.len() != 5 || parts[0] != "arn" {
            return Err(EidError::InvalidFormat);
        }

        let domain = Domain::from_str(&parts[1])?;
        let category = Category::from_str(&parts[2])?;
        let account = Account::from_str(&parts[3])?;

        // Split the root and the path part
        let root_path: Vec<String> = parts[4].splitn(2, '/').map(|s| s.to_string()).collect();
        let root_str = root_path[0].clone();
        let root: Root<T> = Root::<T>::new(root_str)?;

        // Continue with the path parts
        let mut arn_parts = Vec::new();
        if root_path.len() > 1 {
            let path_parts: Vec<String> = root_path[1].split('/').map(|s| s.to_string()).collect();
            for part in path_parts.iter() {
                arn_parts.push(Part::from_str(part)?);
            }
        }

        let parts = Parts::new(arn_parts);
        Ok(Ein::new(domain, category, account, root, parts))
    }



}

#[cfg(test)]
mod tests {
    use crate::UnixTime;
    use super::*;

    #[test]
    fn test_valid_arn_parsing() {
        let arn_str = "arn:custom:service:account123:root/resource/subresource";
        let parser: ArnParser<UnixTime> = ArnParser::new(arn_str);
        let result = parser.parse();

        assert!(result.is_ok());
        let arn = result.unwrap();
        assert_eq!(arn.domain.as_str(), "custom");
    }

    #[test]
    fn test_invalid_arn_format() {
        let arn_str = "invalid:arn:format";
        let parser: ArnParser<UnixTime> = ArnParser::new(arn_str);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), EidError::InvalidFormat);
        // assert_eq!(result.unwrap_err().to_string(), "Invalid ARN format");
    }

    #[test]
    fn test_arn_with_invalid_part() -> anyhow::Result<()> {
        let arn_str = "arn:domain:category:account:root/invalid:part";
        let parser: ArnParser<UnixTime> = ArnParser::new(arn_str);
        let result = parser.parse();
        assert!(result.is_err());
        // assert!(result.unwrap_err().to_string().starts_with("Failed to parse Part"));
        Ok(())
    }

    #[test]
    fn test_arn_parsing_with_owned_string() {
        let arn_str = String::from("arn:custom:service:account123:root/resource");
        let parser: ArnParser<UnixTime> = ArnParser::new(arn_str);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
