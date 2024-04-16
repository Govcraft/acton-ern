use crate::model::Domain;
use crate::{Part, Parts};
use crate::traits::QrnComponent;

/// A builder for constructing QRN strings using a state-driven approach with type safety.
pub struct QrnBuilder<State> {
    builder: PrivateQrnBuilder,
    _marker: std::marker::PhantomData<State>,
}

/// Implementation of `QrnBuilder` for the initial state, starting with `Domain`.
impl QrnBuilder<()> {
    /// Creates a new QRN builder initialized to start building from the `Domain` component.
    pub fn new() -> QrnBuilder<Domain> {
        QrnBuilder {
            builder: PrivateQrnBuilder::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Implementation of `QrnBuilder` for `Part` states, allowing for building the final QRN string.
impl QrnBuilder<Part> {
    /// Finalizes the building process and constructs the full QRN string.
    pub fn build(self) -> String {
        self.builder.build()
    }
}

/// Implementation of `QrnBuilder` for handling `Parts` states, specifically when QRN involves multiple parts.
impl QrnBuilder<Parts> {
    /// Finalizes the building process and constructs the full QRN string when in the `Parts` state.
    pub fn build(self) -> String {
        self.builder.build()
    }
}

/// Generic implementation of `QrnBuilder` for all states that can transition to another state.
impl<T: QrnComponent> QrnBuilder<T> {
    /// Adds a new part to the QRN, transitioning to the next appropriate state.
    pub fn add<N: QrnComponent>(self, part: &str) -> QrnBuilder<N::NextState> where N: QrnComponent<NextState=T::NextState> {
        QrnBuilder {
            builder: self.builder.add_part(&format!("{}{}", N::prefix(), part)),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Represents a private, internal structure for building the QRN string, using functional combinators.
struct PrivateQrnBuilder {
    operations: Vec<Box<dyn Fn(String) -> String>>,
}

impl PrivateQrnBuilder {
    /// Constructs a new private QRN builder with an empty set of operations.
    fn new() -> Self {
        Self { operations: Vec::new() }
    }

    /// Adds a new part to the builder's operations, using a closure to encapsulate the addition logic.
    fn add_part(mut self, part: &str) -> Self {
        assert!(!part.is_empty(), "Part cannot be empty");
        assert!(part.len() <= 256, "Part is too long");

        let part = part.to_owned();
        assert!(!part.contains('\n'), "Part cannot contain newline characters");

        let operation: Box<dyn Fn(String) -> String> = Box::new(move |qrn: String| -> String {
            if !qrn.is_empty() {
                let result = format!("{}:{}", qrn, part);
                assert!(!result.ends_with(':'), "Result should not end with a colon");
                assert!(result.starts_with("qrn:"), "Result should start with 'qrn:' if not empty");
                result
            } else {
                assert_eq!(part, part.clone(), "Cloned part should be equal to the original part");
                part.clone()
            }
        });

        self.operations.push(operation);
        assert!(!self.operations.is_empty(), "Operations should not be empty after adding a part");
        self
    }

    /// Finalizes and builds the QRN string by applying all operations sequentially.
    fn build(self) -> String {
        self.operations.into_iter().fold(String::new(), |qrn, func| func(qrn))
    }
}
