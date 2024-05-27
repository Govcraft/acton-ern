use crate::model::Domain;
use crate::{Part, Parts, Arn};
use crate::traits::ArnComponent;

/// A builder for constructing Arn strings using a state-driven approach with type safety.
pub struct ArnBuilder<State> {
    builder: PrivateArnBuilder,
    _marker: std::marker::PhantomData<State>,
}

/// Implementation of `ArnBuilder` for the initial state, starting with `Domain`.
impl ArnBuilder<()> {
    /// Creates a new Arn builder initialized to start building from the `Domain` component.
    ///
    /// # Returns
    ///
    /// Returns an `ArnBuilder` instance set to start building from the `Domain` state.
    pub fn new() -> ArnBuilder<Domain> {
        ArnBuilder {
            builder: PrivateArnBuilder::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Implementation of `ArnBuilder` for `Part` states, allowing for building the final Arn string.
impl ArnBuilder<Part> {
    /// Finalizes the building process and constructs the full Arn string.
    ///
    /// # Returns
    ///
    /// Returns an `Arn` instance representing the complete Arn string.
    pub fn build(self) -> Arn {
        self.builder.build()
    }
}

/// Implementation of `ArnBuilder` for handling `Parts` states, specifically when Arn involves multiple parts.
impl ArnBuilder<Parts> {
    /// Finalizes the building process and constructs the full Arn string when in the `Parts` state.
    ///
    /// # Returns
    ///
    /// Returns an `Arn` instance representing the complete Arn string.
    pub fn build(self) -> Arn {
        self.builder.build()
    }
}

/// Generic implementation of `ArnBuilder` for all states that can transition to another state.
impl<T: ArnComponent> ArnBuilder<T> {
    /// Adds a new part to the Arn, transitioning to the next appropriate state.
    ///
    /// # Arguments
    ///
    /// * `part` - A string slice representing the part to be added to the Arn.
    ///
    /// # Returns
    ///
    /// Returns a new `ArnBuilder` instance transitioning to the next state.
    pub fn add<N: ArnComponent>(self, part: &str) -> ArnBuilder<N::NextState> where N: ArnComponent<NextState=T::NextState> {
        ArnBuilder {
            builder: self.builder.add_part(&format!("{}{}", N::prefix(), part)),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Represents a private, internal structure for building the Arn string, using functional combinators.
struct PrivateArnBuilder {
    operations: Vec<Box<dyn Fn(String) -> String>>,
}

impl PrivateArnBuilder {
    /// Constructs a new private Arn builder with an empty set of operations.
    ///
    /// # Returns
    ///
    /// Returns a new `PrivateArnBuilder` instance.
    fn new() -> Self {
        Self { operations: Vec::new() }
    }

    /// Adds a new part to the builder's operations, using a closure to encapsulate the addition logic.
    /// Uses ':' or '/' based on the position in the Arn string.
    ///
    /// # Arguments
    ///
    /// * `part` - A string slice representing the part to be added.
    ///
    /// # Returns
    ///
    /// Returns an updated `PrivateArnBuilder` instance with the new part added.
    ///
    /// # Panics
    ///
    /// This function will panic if `part` is empty, longer than 256 characters, or contains newline characters.
    fn add_part(mut self, part: &str) -> Self {
        assert!(!part.is_empty(), "Part cannot be empty");
        assert!(part.len() <= 256, "Part is too long");
        assert!(!part.contains('\n'), "Part cannot contain newline characters");

        let part = part.to_owned();
        let operation: Box<dyn Fn(String) -> String> = Box::new(move |arn: String| -> String {
            if !arn.is_empty() {
                // Determine the correct separator based on the existing content of the Arn
                let separator = if arn.contains('/') || arn.ends_with("root") {
                    "/"
                } else {
                    ":"
                };
                let result = format!("{}{}{}", arn, separator, part);
                assert!(!result.ends_with(':'), "Result should not end with a colon");
                result
            } else {
                part.clone()
            }
        });

        self.operations.push(operation);
        assert!(!self.operations.is_empty(), "Operations should not be empty after adding a part");
        self
    }

    /// Finalizes and builds the Arn string by applying all operations sequentially.
    ///
    /// # Returns
    ///
    /// Returns an `Arn` instance representing the complete Arn string.
    fn build(self) -> Arn {
        let arn_string = self.operations.into_iter().fold(String::new(), |arn, func| func(arn));
        Arn { value: arn_string }
    }
}
