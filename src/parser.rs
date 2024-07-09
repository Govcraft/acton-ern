use std::borrow::Cow;
use std::str::FromStr;

use crate::{IdType, Root};
use crate::errors::EidError;
use crate::model::{Account, Category, Domain, Ern, Part, Parts};

/// A parser for decoding ERN (Entity Resource Name) strings into their constituent components.
pub struct ArnParser<T: IdType + Clone + PartialEq> {
    /// The ERN (Entity Resource Name) string to be parsed.
    eid: Cow<'static, str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IdType + Clone + PartialEq> ArnParser<T> {
    /// Constructs a new `ArnParser` for a given ERN (Entity Resource Name) string.
    ///
    /// # Arguments
    ///
    /// * `eid` - A string slice or owned String representing the ERN (Entity Resource Name) to be parsed.
    ///
    /// # Returns
    ///
    /// Returns an `ArnParser` instance initialized with the given ERN (Entity Resource Name) string.
    pub fn new(eid: impl Into<Cow<'static, str>>) -> Self {
        Self {
            eid: eid.into(),
            _marker: Default::default(),
        }
    }

    /// Parses the ERN (Entity Resource Name) into its component parts and returns them as a structured result.
    /// Verifies correct ERN (Entity Resource Name) format and validates each part.
    ///
    /// # Returns
    ///
    /// Returns an `ERN (Entity Resource Name)` instance containing the parsed components.
    /// If parsing fails, returns an error message as a `String`.
    pub fn parse(&self) -> Result<Ern<T>, EidError> {
        let parts: Vec<String> = self.eid.splitn(5, ':').map(|s| s.to_string()).collect();

        if parts.len() != 5 || parts[0] != "eid" {
            return Err(EidError::InvalidFormat);
        }

        let domain = Domain::from_str(&parts[1])?;
        let category = Category::from_str(&parts[2])?;
        let account = Account::from_str(&parts[3])?;

        // Split the root and the path part
        let root_path: Vec<String> = parts[4].splitn(2, '/').map(|s| s.to_string()).collect();
        let root_str = root_path[0].clone();
        let root: Root<T> = Root::<T>::from_str(root_str.as_str())?;

        // Continue with the path parts
        let mut eid_parts = Vec::new();
        if root_path.len() > 1 {
            let path_parts: Vec<String> = root_path[1].split('/').map(|s| s.to_string()).collect();
            for part in path_parts.iter() {
                eid_parts.push(Part::from_str(part)?);
            }
        }

        let parts = Parts::new(eid_parts);
        Ok(Ern::new(domain, category, account, root, parts))
    }



}

#[cfg(test)]
mod tests {
    use crate::UnixTime;
    use super::*;

    #[test]
    fn test_valid_eid_parsing() {
        let eid_str = "eid:custom:service:account123:root/resource/subresource";
        let parser: ArnParser<UnixTime> = ArnParser::new(eid_str);
        let result = parser.parse();

        assert!(result.is_ok());
        let eid = result.unwrap();
        assert_eq!(eid.domain.as_str(), "custom");
    }

    #[test]
    fn test_invalid_eid_format() {
        let eid_str = "invalid:eid:format";
        let parser: ArnParser<UnixTime> = ArnParser::new(eid_str);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), EidError::InvalidFormat);
        // assert_eq!(result.unwrap_err().to_string(), "Invalid Eid format");
    }

    #[test]
    fn test_eid_with_invalid_part() -> anyhow::Result<()> {
        let eid_str = "eid:domain:category:account:root/invalid:part";
        let parser: ArnParser<UnixTime> = ArnParser::new(eid_str);
        let result = parser.parse();
        assert!(result.is_err());
        // assert!(result.unwrap_err().to_string().starts_with("Failed to parse Part"));
        Ok(())
    }

    #[test]
    fn test_eid_parsing_with_owned_string() {
        let eid_str = String::from("eid:custom:service:account123:root/resource");
        let parser: ArnParser<UnixTime> = ArnParser::new(eid_str);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
