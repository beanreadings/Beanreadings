//! Represents beans for Beanreadings Simulator. Each bean represents a human, but has a certain
//! set of factors upon initialization. This will be used to determine whether a bean dies of a
//! certain death or not, or whether it reproduces or not.

use crate::age::AgeGenerator;
use crate::factors::*;
use crate::types::Settings;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Bean {
    age: u32, // pawns cant be older than 2.1 billion years

    factors: Factors, // this is a structure that represents all of the required factors for a bean
}

impl Bean {
    pub fn new(context: &Settings, age_gen: &AgeGenerator, rng: &mut impl rand::Rng) -> Self {
        // we use a reference to the RNG to avoid creating a new one every time we create a bean

        let age = age_gen.generate_age(); // using my custom age generator logic
    }
}
