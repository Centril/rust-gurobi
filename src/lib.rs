//! A crate which provides low-level Rust API of Gurobi Optimizer.
//!
//! This crate provides wrappers of the Gurobi solver which supports some
//! types of mathematical programming problems (e.g. Linear programming; LP,
//! Mixed Integer Linear Programming; MILP, and so on).
//!
//! ## Installation
//! Before using this crate, you should install Gurobi and obtain a license.
//! The instruction can be found
//! [here](http://www.gurobi.com/downloads/licenses/license-center).
//!
//! ## Examples
//! Work in progress...

extern crate gurobi_sys as ffi;

pub mod attr;
pub mod env;
pub mod error;
pub mod model;
pub mod param;
mod util;

// re-exports
pub use env::Env;
pub use error::{Error, Result};
pub use param::HasParam;
pub use attr::HasAttr;
pub use model::VarType::*;
pub use model::ConstrSense::*;
pub use model::ModelSense::*;
pub use model::SOSType::*;

// vim: set foldmethod=syntax :
