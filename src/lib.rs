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
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Diet {
    // 1.00 = 100% of what we need, 0.50 = 50% of what we need
    calcium: f64,
}

impl Default for Diet {
    fn default() -> Self {
        Self { calcium: 1.0 }
    }
}

#[wasm_bindgen]
pub struct SimulationConfig {
    diet_settings: Diet,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            diet_settings: Diet {
                ..Default::default()
            },
        }
    }
}

#[wasm_bindgen]
impl SimulationConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SimulationConfig {
        SimulationConfig {
            ..SimulationConfig::default()
        }
    }
}

#[wasm_bindgen]
pub struct Simulation {
    config: SimulationConfig,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: SimulationConfig) -> Simulation {
        Simulation { config }
    }
    pub fn simulate(&mut self) -> String {
        // load the config
        let config = &self.config;

        // now we calculate death rates for each scenario

        let cardiovacular_disease_rate = 0.1;

        serde_json::to_string_pretty(&serde_json::json!({
            "loaded": true
        }))
        .unwrap()
    }
}
