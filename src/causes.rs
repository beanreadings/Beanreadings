//! Causes for death. This will be used to determine whether a bean dies of a certain death or not.
//! This will contain rates, and based on the factors, which are of course a re-export of the
//! factors that we are requiring.

use crate::factors::*;

/// Represents a cause of death. This consists of many factors, and the rate of death.
pub trait Cause {
    /// Represents whether a bean died of that death or not.
    pub fn is_dead(&self, bean: &Bean) -> bool;
}
