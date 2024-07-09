use crate::errors::ArnError;
use crate::model::{Account, Ein, Category, Domain, Part, Parts};
use crate::traits::EidComponent;
use crate::{IdType, Root, UnixTime};
use std::borrow::Cow;

/// A builder for constructing Ein instances using a state-driven approach with type safety.
pub struct ArnBuilder<State, T: IdType + Clone + PartialEq> {
    builder: PrivateArnBuilder<T>,
    _marker: std::marker::PhantomData<(State, T)>,
}

/// Implementation of `ArnBuilder` for the initial state, starting with `Domain`.
impl<T:IdType+Clone+PartialEq> ArnBuilder<(),T> {
    /// Creates a new Ein builder initialized to start building from the `Domain` component.
    pub fn new() -> ArnBuilder<Domain, T> {
        ArnBuilder {
            builder: PrivateArnBuilder::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Implementation of `ArnBuilder` for `Part` states, allowing for building the final Ein.
impl<T:IdType+Clone+PartialEq> ArnBuilder<Part,T> {
    /// Finalizes the building process and constructs the Ein.
    pub fn build(self) -> Result<Ein<T>, ArnError> {
        self.builder.build()
    }
}

/// Implementation of `ArnBuilder` for handling `Parts` states.
impl<T:IdType+Clone+PartialEq> ArnBuilder<Parts,T> {
    /// Finalizes the building process and constructs the Ein when in the `Parts` state.
    pub fn build(self) -> Result<Ein<T>, ArnError> {
        self.builder.build()
    }
}

/// Generic implementation of `ArnBuilder` for all states that can transition to another state.
impl<Component: EidComponent, T:IdType+Clone+PartialEq> ArnBuilder<Component, T> {
    /// Adds a new part to the Ein, transitioning to the next appropriate state.
    pub fn with<N>(
        self,
        part: impl Into<Cow<'static, str>>,
    ) -> Result<ArnBuilder<N::NextState, T>, ArnError>
    where
        N: EidComponent<NextState = Component::NextState>,
    {
        Ok(ArnBuilder {
            builder: self.builder.add_part(N::prefix(), part.into())?,
            _marker: std::marker::PhantomData,
        })
    }
}

/// Represents a private, internal structure for building the Ein.
struct PrivateArnBuilder<T: IdType + Clone + PartialEq> {
    domain: Option<Domain>,
    category: Option<Category>,
    account: Option<Account>,
    root: Option<Root<T>>,
    parts: Parts,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IdType + Clone + PartialEq> PrivateArnBuilder<T> {
    /// Constructs a new private Ein builder.
    fn new() -> Self {
        Self {
            domain: None,
            category: None,
            account: None,
            root: None,
            parts: Parts::new(Vec::new()),
            _marker: Default::default(),
        }
    }

    fn add_part(mut self, prefix: &'static str, part: Cow<'static, str>) -> Result<Self, ArnError> {
        match prefix {
            p if p == Domain::prefix() => {
                self.domain = Some(Domain::new(part)?);
            }
            "" => {
                if self.domain.is_some() && self.category.is_none() {
                    self.category = Some(Category::new(part));
                } else if self.category.is_some() && self.account.is_none() {
                    self.account = Some(Account::new(part));
                } else if self.account.is_some() && self.root.is_none() {
                    self.root = Some(Root::new(part)?);
                } else {
                    // add the first part
                    self.parts = self.parts.add_part(Part::new(part)?);
                }
            }
            ":" => {
                self.parts = self.parts.add_part(Part::new(part)?);
            }
            _ => return Err(ArnError::InvalidPrefix(prefix.to_string())),
        }
        Ok(self)
    }

    /// Finalizes and builds the Ein.
    fn build(self) -> Result<Ein<T>, ArnError> {
        let domain = self
            .domain
            .ok_or(ArnError::MissingPart("domain".to_string()))?;
        let category = self
            .category
            .ok_or(ArnError::MissingPart("category".to_string()))?;
        let account = self
            .account
            .ok_or(ArnError::MissingPart("account".to_string()))?;
        let root = self.root.ok_or(ArnError::MissingPart("root".to_string()))?;

        Ok(Ein::new(domain, category, account, root, self.parts))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;
    use super::*;
    use crate::errors::ArnError;
    use crate::tests::init_tracing;
    use crate::{ArnBuilder, ArnParser};

    #[test]
    fn test() -> anyhow::Result<()> {
        // Create an Ein using the ArnBuilder with specified components
        let arn: Result<Ein<UnixTime>, ArnError> = ArnBuilder::new()
            .with::<Domain>("akton-internal")?
            .with::<Category>("hr")?
            .with::<Account>("company123")?
            .with::<Root<UnixTime>>("root")?
            .with::<Part>("departmentA")?
            .with::<Part>("team1")?
            .build();

        // Verify the constructed Ein matches the expected value
        assert!(
            arn.is_ok(),
            "arn:akton-internal:hr:company123:root/departmentA/team1"
        );
        Ok(())
    }
    #[test]
    fn test_arn_builder() -> anyhow::Result<()> {
        let arn: Ein<UnixTime> = ArnBuilder::new()
            .with::<Domain>("custom")?
            .with::<Category>("service")?
            .with::<Account>("account123")?
            .with::<Root<UnixTime>>("resource")?
            .with::<Part>("subresource")?
            .build()?;

        assert!(
            arn.to_string().ends_with("/subresource"),
            "{} did not end with expected string",
            arn
        );

        Ok(())
    }

    #[test]
    fn test_arn_builder_with_default_parts() -> anyhow::Result<(), ArnError> {
        init_tracing();
        let arn: Ein<UnixTime> = Ein::default();
        tracing::debug!("{}", arn);
        let parser:ArnParser<UnixTime> = ArnParser::new(arn.to_string());
        let parsed: Ein<UnixTime> = parser.parse()?;
        assert_eq!(parsed.domain.as_str(), "akton");
        // assert_eq!(arn.to_string(), "arn:akton:system:default:root");
        Ok(())
    }

    #[test]
    fn test_arn_builder_with_owned_strings() -> anyhow::Result<(), ArnError> {
        let arn: Ein<UnixTime> = ArnBuilder::new()
            .with::<Domain>(String::from("custom"))?
            .with::<Category>(String::from("service"))?
            .with::<Account>(String::from("account123"))?
            .with::<Root<UnixTime>>(String::from("resource"))?
            .build()?;

        assert!(arn
            .to_string()
            .starts_with("arn:custom:service:account123:resource"));
        Ok(())
    }
}
