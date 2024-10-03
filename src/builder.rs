use std::hash::Hash;
use std::str::FromStr;

use crate::EntityRoot;
use crate::errors::ErnError;
use crate::model::{Account, Category, Domain, Ern, Part, Parts};
use crate::traits::ErnComponent;

/// A builder for constructing ERN (Entity Resource Name) instances using a state-driven approach with type safety.
pub struct ErnBuilder<State> {
    builder: PrivateErnBuilder,
    _marker: std::marker::PhantomData<State>,
}

/// Implementation of `ErnBuilder` for the initial state, starting with `Domain`.
impl ErnBuilder<()> {
    /// Creates a new ERN (Entity Resource Name) builder initialized to start building from the `Domain` component.
    pub fn new() -> ErnBuilder<Domain> {
        ErnBuilder {
            builder: PrivateErnBuilder::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Implementation of `ErnBuilder` for `Part` states, allowing for building the final ERN (Entity Resource Name).
impl ErnBuilder<Part> {
    /// Finalizes the building process and constructs the ERN (Entity Resource Name).
    pub fn build(self) -> Result<Ern, ErnError> {
        self.builder.build()
    }
}

/// Implementation of `ErnBuilder` for handling `Parts` states.
impl ErnBuilder<Parts> {
    /// Finalizes the building process and constructs the ERN (Entity Resource Name) when in the `Parts` state.
    pub fn build(self) -> Result<Ern, ErnError> {
        self.builder.build()
    }
}

/// Generic implementation of `ErnBuilder` for all states that can transition to another state.
impl<Component: ErnComponent + Hash + Clone + PartialEq + Eq> ErnBuilder<Component> {
    /// Adds a new part to the ERN (Entity Resource Name), transitioning to the next appropriate state.
    pub fn with<N>(
        self,
        part: String,
    ) -> Result<ErnBuilder<N::NextState>, ErnError>
    where
        N: ErnComponent<NextState=Component::NextState> + Hash,
    {
        Ok(ErnBuilder {
            builder: self.builder.add_part(N::prefix(), part)?,
            _marker: std::marker::PhantomData,
        })
    }
}

/// Represents a private, internal structure for building the ERN (Entity Resource Name).
struct PrivateErnBuilder {
    domain: Option<Domain>,
    category: Option<Category>,
    account: Option<Account>,
    root: Option<EntityRoot>,
    parts: Parts,
}

impl PrivateErnBuilder {
    /// Constructs a new private ERN (Entity Resource Name) builder.
    fn new() -> Self {
        Self {
            domain: None,
            category: None,
            account: None,
            root: None,
            parts: Parts::new(Vec::new()),
        }
    }

    fn add_part(mut self, prefix: &'static str, part: String) -> Result<Self, ErnError> {
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
                    self.root = Some(EntityRoot::from_str(part.as_str()).unwrap());
                } else {
                    // add the first part
                    self.parts = self.parts.add_part(Part::new(part)?);
                }
            }
            ":" => {
                self.parts = self.parts.add_part(Part::new(part)?);
            }
            _ => return Err(ErnError::InvalidPrefix(prefix.to_string())),
        }
        Ok(self)
    }

    /// Finalizes and builds the ERN (Entity Resource Name).
    fn build(self) -> Result<Ern, ErnError> {
        let domain = self
            .domain
            .ok_or(ErnError::MissingPart("domain".to_string()))?;
        let category = self
            .category
            .ok_or(ErnError::MissingPart("category".to_string()))?;
        let account = self
            .account
            .ok_or(ErnError::MissingPart("account".to_string()))?;
        let root = self.root.ok_or(ErnError::MissingPart("root".to_string()))?;

        Ok(Ern::new(domain, category, account, root, self.parts))
    }
}

