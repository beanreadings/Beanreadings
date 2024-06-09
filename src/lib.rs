//! Beanreadings simulation library
//!
//! Forward: Right now, the death rate is much, much lower than the birth rate. However, it is
//! forcasted that the death rate will increase in the future. This simulation is designed to help
//! forecast the future death rate, and to help us understand what we can do to prevent it.
//!
//! Our current projections show that the death rate will be higher than the birth rate by 2100.
//! Source: Our World in Data
//!
//! Beanreadings helps us understand the future death rate by simulating different scenarios.

// Importing our modules

mod age;
mod bean;
mod factors;
mod simul;
mod types;

// RE EXPORTS

pub use age::*;
pub use bean::*;
pub use factors::*;
pub use types::*;
