//! # Akton Resource Name (Ein) Library
//!
//! `akton-arn` is a Rust library designed to handle Akton Resource Names (Arns), which are structured identifiers used within the [Akton distributed actor framework](https://github.com/GovCraft/akton-framework) to uniquely identify and manage hierarchical resources across different services and partitions.
//!
//! This crate provides tools for generating, parsing, and managing Arns, ensuring type safety and alignment with the hierarchical structure needed Akton-based cloud-native solutions.
//!
//! ## Features
//! - **Ein Parsing**: Parse Ein strings into structured components.
//! - **Ein Building**: Programmatically build Arns with validation.
//! - **Type Safety**: Strongly typed components prevent mixing parts of Ein.
//! - **Easy Integration**: Designed to be integrated with other systems managing hierarchical resources.
//!
//! ## Usage
//! This crate is structured into several modules, each providing distinct functionalities:
//! - `builder`: Module for building Arns.
//! - `parser`: Module for parsing Arns.
//! - `model`: Contains the models representing different parts of an Ein.
//! - `traits`: Traits used across the crate for common functionality.
//!

#![allow(missing_docs)]

extern crate core;

mod builder;
mod errors;
mod model;
mod parser;
mod traits;

pub mod prelude {
    //! The prelude module for `akton-arn`.
    //!
    //! This module re-exports essential traits and structures for easy use by downstream consumers.

    pub use super::builder::ArnBuilder;
    pub use super::model::{Account, Ein, Category, Domain, Part, Parts};
    pub use super::parser::ArnParser;
    pub use super::traits::EidComponent;
    pub use super::errors::EidError;
}

// Re-exporting the public API under the root of the crate for direct access
pub use builder::*;
pub use model::*;
pub use parser::*;
pub use traits::*;

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::Level;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    static INIT: Once = Once::new();

    pub fn init_tracing() {
        INIT.call_once(|| {
            // Define an environment filter to suppress logs from the specific function

            // let filter = EnvFilter::new("")
            //     // .add_directive("akton_core::common::context::emit_pool=trace".parse().unwrap())
            //     // .add_directive("akton_core::common::context::my_func=trace".parse().unwrap())
            //     .add_directive("akton_core::common::context[my_func]=trace".parse().unwrap())
            //     .add_directive(Level::INFO.into()); // Set global log level to INFO

            let filter = EnvFilter::new("")
                .add_directive("akton-eid::parser::tests=trace".parse().unwrap())
                .add_directive("broker_tests=trace".parse().unwrap())
                .add_directive("launchpad_tests=trace".parse().unwrap())
                .add_directive("lifecycle_tests=info".parse().unwrap())
                .add_directive("actor_tests=info".parse().unwrap())
                .add_directive("load_balancer_tests=info".parse().unwrap())
                .add_directive(
                    "akton::tests::setup::actors::pool_item=info"
                        .parse()
                        .unwrap(),
                )
                .add_directive("messaging_tests=info".parse().unwrap());
            // .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()); // Set global log level to TRACE

            let subscriber = FmtSubscriber::builder()
                // .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
                .with_span_events(FmtSpan::NONE)
                .with_max_level(Level::TRACE)
                .compact()
                .with_line_number(true)
                .without_time()
                .with_env_filter(filter)
                .finish();

            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");
        });
    }
}
