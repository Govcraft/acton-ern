use crate::Part;
use std::fmt;
use std::hash::{Hash, Hasher};
use derive_new::new;
/// Represents a collection of parts in the ERN (Entity Resource Name), handling multiple segments.
#[derive(new, Debug, PartialEq, Clone, Eq, Default, PartialOrd)]
pub struct Parts(pub(crate) Vec<Part>);

impl Parts {
    /// Adds a part to the collection.
    ///
    /// # Arguments
    ///
    /// * `part` - The `Part` to be added to the collection.
    pub fn add_part<T>(mut self, part: T) -> Self
    where
        T: Into<Part>,
    {
        self.0.push(part.into());
        self
    }

    /// Converts the Parts into an owned version with 'static lifetime
    pub fn into_owned(self) -> Parts {
        Parts(self.0.into_iter().map(|part| part.into_owned()).collect())
    }

    /// Returns the number of parts in the collection.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl Hash for Parts {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
        for part in &self.0 {
            part.hash(state);
        }
    }
}
impl FromIterator<Part> for Parts {
    fn from_iter<T: IntoIterator<Item = Part>>(iter: T) -> Self {
        Parts(iter.into_iter().collect())
    }
}

impl fmt::Display for Parts {
    /// Formats the collection of parts as a string, joining them with '/'.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|p| p.as_str()).collect::<Vec<_>>().join("/"))
    }
}

impl IntoIterator for Parts {
    type Item = Part;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Parts {
    type Item = &'a Part;
    type IntoIter = std::slice::Iter<'a, Part>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
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
        let owned_parts: Parts = parts.into_owned();
        assert_eq!(owned_parts.to_string(), "segment1/segment2");
        Ok(())
    }

    #[test]
    fn test_parts_iterator() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let collected: Vec<_> = parts.into_iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].as_str(), "segment1");
        assert_eq!(collected[1].as_str(), "segment2");
        Ok(())
    }

    #[test]
    fn test_parts_ref_iterator() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let collected: Vec<_> = (&parts).into_iter().map(|p| p.as_str()).collect();
        assert_eq!(collected, vec!["segment1", "segment2"]);
        Ok(())
    }

    #[test]
    fn test_parts_for_loop() -> anyhow::Result<()> {
        let parts = Parts::new(vec![Part::new("segment1")?, Part::new("segment2")?]);
        let mut collected = Vec::new();
        for part in parts {
            collected.push(part.as_str().to_string());
        }
        assert_eq!(collected, vec!["segment1".to_string(), "segment2".to_string()]);
        Ok(())
    }
}