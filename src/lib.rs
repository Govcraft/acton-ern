//! # Akton Resource Name (Arn) Library
//!
//! `akton-arn` is a Rust library designed to handle Akton Resource Names (Arns), which are structured identifiers used within the [Akton distributed actor framework](https://github.com/GovCraft/akton-framework) to uniquely identify and manage hierarchical resources across different services and partitions.
//!
//! This crate provides tools for generating, parsing, and managing Arns, ensuring type safety and alignment with the hierarchical structure needed Akton-based cloud-native solutions.
//!
//! ## Features
//! - **Arn Parsing**: Parse Arn strings into structured components.
//! - **Arn Building**: Programmatically build Arns with validation.
//! - **Type Safety**: Strongly typed components prevent mixing parts of Arn.
//! - **Easy Integration**: Designed to be integrated with other systems managing hierarchical resources.
//!
//! ## Usage
//! This crate is structured into several modules, each providing distinct functionalities:
//! - `builder`: Module for building Arns.
//! - `parser`: Module for parsing Arns.
//! - `model`: Contains the models representing different parts of an Arn.
//! - `traits`: Traits used across the crate for common functionality.
//!
//! To get started, include the necessary components from the `prelude` module:
//!
//! ```
//! use akton_arn::prelude::*;
//! ```
//!
//! ### Example: Building an Arn
//!
//! ```
//! use akton_arn::prelude::*;
//!
//! let arn = ArnBuilder::new()
//!     .add::<Domain>("akton-internal")
//!     .add::<Category>("hr")
//!     .add::<Company>("company123")
//!     .add::<Part>("root")
//!     .add::<Part>("departmentA")
//!     .add::<Part>("team1")
//!     .build();
//! assert_eq!(arn.value, "arn:akton-internal:hr:company123:root/departmentA/team1");
//! ```
//!
//! ### Example: Parsing an Arn
//!
//! ```
//! use akton_arn::prelude::*;
//!
//! let parser = ArnParser::new("arn:akton-internal:hr:company123:root/departmentA/team1");
//! let result = parser.parse();
//!
//! assert!(result.is_ok());
//!
//! let (domain, category, company, parts) = result.unwrap();
//! assert_eq!(domain.to_string(), "akton-internal");
//! assert_eq!(category.to_string(), "hr");
//! assert_eq!(company.to_string(), "company123");
//! assert_eq!(parts.to_string(), "root/departmentA/team1");
//! ```
//!
//! ### Example: Using the Default Implementation
//!
//! ```
//! use akton_arn::prelude::*;
//!
//! // Create a default Arn
//! let default_arn: Arn = Default::default();
//! assert_eq!(default_arn.value, "arn:akton:system:framework:root");
//! ```

#![warn(missing_docs)]

extern crate core;

mod model;
mod traits;
mod parser;
mod builder;

pub mod prelude {
    //! The prelude module for `akton-arn`.
    //!
    //! This module re-exports essential traits and structures for easy use by downstream consumers.

    pub use super::model::{Domain, Category, Company, Part, Parts, Arn};
    pub use super::traits::ArnComponent;
    pub use super::builder::ArnBuilder;
    pub use super::parser::ArnParser;
}

// Re-exporting the public API under the root of the crate for direct access
pub use model::*;
pub use traits::*;
pub use parser::*;
pub use builder::*;
