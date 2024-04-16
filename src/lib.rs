extern crate core;

use std::marker::PhantomData;
mod parser;
mod traits;
mod builder;
pub use parser::*;
pub use builder::*;
pub use model::*;
mod model;