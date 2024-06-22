use crate::errors::ArnError;
use derive_more::{AsRef, From, Into};
use std::borrow::Cow;
use std::fmt;

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone)]
pub struct Part<'a>(pub(crate) Cow<'a, str>);
impl<'a> Part<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_owned(self) -> Part<'static> {
        Part(Cow::Owned(self.0.into_owned()))
    }

    pub fn new(value: impl Into<Cow<'a, str>>) -> Result<Part<'a>, ArnError> {
        let value = value.into();
        if value.contains(':') || value.contains('/') {
            return Err(ArnError::InvalidPartFormat);
        }
        if value.is_empty() {
            return Err(ArnError::ParseFailure(
                "Part",
                "cannot be empty".to_string(),
            ));
        }
        Ok(Part(value))
    }
}

impl<'a> fmt::Display for Part<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<'a> std::str::FromStr for Part<'a> {
    type Err = ArnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Part::new(Cow::Owned(s.to_owned()))
    }
}
impl<'a> From<Part<'a>> for String {
    fn from(part: Part<'a>) -> Self {
        part.0.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_creation() -> anyhow::Result<()> {
        let part = Part::new("segment")?;
        assert_eq!(part.as_str(), "segment");
        Ok(())
    }

    #[test]
    fn test_part_display() -> anyhow::Result<()> {
        let part = Part::new("example")?;
        assert_eq!(format!("{}", part), "example");
        Ok(())
    }

    #[test]
    fn test_part_from_str() {
        let part: Part = "test".parse().unwrap();
        assert_eq!(part.as_str(), "test");
    }

    #[test]
    fn test_part_equality() -> anyhow::Result<()> {
        let part1 = Part::new("segment1")?;
        let part2 = Part::new("segment1")?;
        let part3 = Part::new("segment2")?;
        assert_eq!(part1, part2);
        assert_ne!(part1, part3);
        Ok(())
    }

    #[test]
    fn test_part_into_string() -> anyhow::Result<()> {
        let part = Part::new("segment")?;
        let string: String = part.into();
        assert_eq!(string, "segment");
        Ok(())
    }
}
