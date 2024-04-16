use crate::model::Domain;
use crate::{Part, Parts};
use crate::traits::QrnComponent;

impl QrnBuilder<Parts> {

    pub fn build(self) -> String {
        self.builder.build()
    }
}
impl<T: QrnComponent> QrnBuilder<T> {
    pub fn add<N: QrnComponent>(self, part: &str) -> QrnBuilder<N::NextState> where N: QrnComponent<NextState=T::NextState> {
        // Check if we're transitioning to Parts and handle specially if needed
        QrnBuilder {
            builder: self.builder.add_part(&format!("{}{}", N::prefix(), part)),
            _marker: std::marker::PhantomData,
        }
    }
}


// Private lazy builder using functional combinators
struct PrivateQrnBuilder {
    operations: Vec<Box<dyn Fn(String) -> String>>,
}

impl PrivateQrnBuilder {
    fn new() -> Self {
        Self { operations: Vec::new() }
    }

    fn add_part(mut self, part: &str) -> Self {
        assert!(!part.is_empty(), "Part cannot be empty");
        assert!(part.len() <= 256, "Part is too long");  // Example limit, adjust as necessary

        let part = part.to_owned();

        // Ensure the part does not contain invalid characters
        assert!(!part.contains('\n'), "Part cannot contain newline characters");

        let operation: Box<dyn Fn(String) -> String> = Box::new(move |qrn: String| -> String {
            if !qrn.is_empty() {
                let result = format!("{}:{}", qrn, part);
                assert!(!result.ends_with(':'), "Result should not end with a colon");
                assert!(result.starts_with("qrn:"), "Result should start with 'qrn:' if not empty");
                result
            } else {
                assert!(part == part.clone(), "Cloned part should be equal to the original part");
                part.clone()
            }
        });

        self.operations.push(operation);

        // Check the increased length of operations
        assert!(!self.operations.is_empty(), "Operations should not be empty after adding a part");

        self
    }


    fn build(self) -> String {
        self.operations.into_iter().fold(String::new(), |qrn, func| func(qrn))
    }
}

// Public builder interface using generics and state transitions
pub struct QrnBuilder<State> {
    builder: PrivateQrnBuilder,
    _marker: std::marker::PhantomData<State>,
}

impl QrnBuilder<()> {
    pub fn new() -> QrnBuilder<Domain> {
        QrnBuilder {
            builder: PrivateQrnBuilder::new(),
            _marker: std::marker::PhantomData,
        }
    }
}
impl QrnBuilder<Part> {
    pub fn build(self) -> String {
        self.builder.build()
    }
}
