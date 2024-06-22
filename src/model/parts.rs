use crate::Part;
use std::fmt;

/// Represents a collection of parts in the Arn, handling multiple segments.
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Parts<'a>(pub(crate) Vec<Part<'a>>);

impl<'a> Parts<'a> {
    /// Constructs a new collection of `Parts`.
    ///
    /// # Arguments
    ///
    /// * `parts` - A vector of `Part` representing the parts of the Arn.
    pub fn new(parts: Vec<Part<'a>>) -> Self {
        Parts(parts)
    }

    /// Adds a part to the collection.
    ///
    /// # Arguments
    ///
    /// * `part` - The `Part` to be added to the collection.
    pub fn add_part<T>(mut self, part: T) -> Self
    where
        T: Into<Part<'a>>,
    {
        self.0.push(part.into());
        self
    }

    /// Converts the Parts into an owned version with 'static lifetime
    pub fn into_owned(self) -> Parts<'static> {
        Parts(self.0.into_iter().map(|part| part.into_owned()).collect())
    }
}

impl<'a> FromIterator<Part<'a>> for Parts<'a> {
    fn from_iter<T: IntoIterator<Item = Part<'a>>>(iter: T) -> Self {
        Parts(iter.into_iter().collect())
    }
}

impl<'a> fmt::Display for Parts<'a> {
    /// Formats the collection of parts as a string, joining them with '/'.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts_str = self
            .0
            .iter()
            .map(|p| p.as_str())
            .collect::<Vec<_>>()
            .join("/");
        write!(f, "{}", parts_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts_creation() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        assert_eq!(parts.to_string(), "segment1/segment2");
        Ok(())
    }

    #[test]
    fn test_parts_add_part() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?])
            .add_part(Part::new("segment2")?)
            .add_part(Part::new("segment3")?);
        assert_eq!(parts.to_string(), "segment1/segment2/segment3");
        Ok(())
    }

    #[test]
    fn test_parts_from_iterator() -> anyhow::Result<()> {
        let parts: Result<Parts, _> = vec!["segment1", "segment2", "segment3"]
            .into_iter()
            .map(Part::new)
            .collect();
        match parts {
            Ok(parts) => {
                assert_eq!(parts.to_string(), "segment1/segment2/segment3");
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    #[test]
    fn test_parts_into_owned() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let owned_parts: Parts<'static> = parts.into_owned();
        assert_eq!(owned_parts.to_string(), "segment1/segment2");
        Ok(())
    }
}
