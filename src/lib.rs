//! # Quasar Resource Name (QRN) Library
//!
//! `quasar-qrn` is a Rust library designed to handle Quasar Resource Names (QRNs), which are structured identifiers used within the Quasar framework to uniquely identify and manage hierarchical resources across different services and partitions.
//!
//! This crate provides tools for generating, parsing, and managing QRNs, ensuring type safety and alignment with the hierarchical structure needed in government cloud-native solutions.
//!
//! ## Features
//! - **QRN Parsing**: Parse QRN strings into structured components.
//! - **QRN Building**: Programmatically build QRNs with validation.
//! - **Type Safety**: Strongly typed components prevent mixing parts of QRN.
//! - **Easy Integration**: Designed to be integrated with other systems managing hierarchical resources.
//!
//! ## Usage
//! This crate is structured into several modules, each providing distinct functionalities:
//! - `builder`: Module for building QRNs.
//! - `parser`: Module for parsing QRNs.
//! - `model`: Contains the models representing different parts of a QRN.
//! - `traits`: Traits used across the crate for common functionality.
//!
//! To get started, include the necessary components from the `prelude` module:
//!
//! ```
//! use quasar_qrn::prelude::*;
//! ```

#![warn(missing_docs)]

extern crate core;

mod model;
mod traits;
mod parser;
mod builder;

pub mod prelude {
    //! The prelude module for `quasar-qrn`.
    //!
    //! This module re-exports essential traits and structures for easy use by downstream consumers.

    pub use super::model::{Domain, Category, Company, Part, Parts};
    pub use super::traits::QrnComponent;
    pub use super::builder::QrnBuilder;
    pub use super::parser::QrnParser;
}

// Re-exporting the public API under the root of the crate for direct access
pub use model::*;
pub use traits::*;
pub use parser::*;
pub use builder::*;
