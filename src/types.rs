//! Here in Beanreadings, we define crucial types to the simulation.
//! These include types such as the results, the simulation parameters and the species.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Simulation {
    pub config: SimulationConfig,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: SimulationConfig) -> Simulation {
        Simulation { config }
    }
    // we add other simulation related methods in src/simulation.rs and not here
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct SimulationConfig {
    pub params: u32, // TODO
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub population: u32,
    population_curve: Vec<u32>,
}

#[wasm_bindgen]
impl SimulationResult {
    pub fn get_population(&self) -> u32 {
        self.population
    }

    pub fn get_population_curve(&self) -> Vec<u32> {
        // it cant implement Copy because Vec is not Copy
        self.population_curve.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum ErrorCode {
    InvalidParameters,
    Overflow,
    Other,
}

impl From<ErrorCode> for String {
    fn from(error: ErrorCode) -> String {
        match error {
            ErrorCode::InvalidParameters => "Invalid parameters".to_string(),
            ErrorCode::Overflow => "Overflow".to_string(),
            ErrorCode::Other => "Other error".to_string(),
        }
    }
}
