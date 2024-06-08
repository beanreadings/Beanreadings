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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Settings {
    pub population: u32, // base population
    pub max_age: u32, // Longevity Escape Velocity could come in the future, especially with AGI on
    // the horizon, so this should be a customizable parameter
    pub smokers: f64, // percentage of population who smokes

    pub weed_smokers: f64, // percentage of population who smokes weed

    pub vapers: f64, // percentage of population who vapes

    pub drinkers: f64, // percentage of population who drinks alcohol

    pub hard_drugger: f64, // percentage of population who uses hard drugs

    pub sugar: f64, // % of the RDI of sugar in the diet

    pub salt: f64, // % of the RDI of salt in the diet

    pub fat: f64, // % of the RDI of fat in the diet

    pub vitamins: f64, // % of the RDI of vitamins in the diet (generally below 100%)

    pub wealth_factor: f64, // What the discreptancy in wealth is between the richest and the poorest. Right now it is a percentage, percentage of the upper class people - the lower class, so generally a negative factor
}

#[wasm_bindgen]
impl Settings {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Settings {
            population: 1000,
            max_age: 100,
            smokers: 0.2,
            weed_smokers: 0.1,
            vapers: 0.1,
            drinkers: 0.2,
            hard_drugger: 0.05,
            sugar: 0.5,
            salt: 0.5,
            fat: 0.5,
            vitamins: 0.5,
            wealth_factor: -0.1, // 20% of the population is upper class, 50% is middle class, 30% is lower class
        }
    }

    // since its WASM we have to add methods to set the values

    #[wasm_bindgen]
    pub fn set_population(&mut self, population: u32) {
        self.population = population;
    }

    #[wasm_bindgen]
    pub fn set_max_age(&mut self, max_age: u32) {
        self.max_age = max_age;
    }

    #[wasm_bindgen]
    pub fn set_smokers(&mut self, smokers: f64) {
        self.smokers = smokers;
    }

    #[wasm_bindgen]
    pub fn set_weed_smokers(&mut self, weed_smokers: f64) {
        self.weed_smokers = weed_smokers;
    }

    #[wasm_bindgen]
    pub fn set_vapers(&mut self, vapers: f64) {
        self.vapers = vapers;
    }

    #[wasm_bindgen]
    pub fn set_drinkers(&mut self, drinkers: f64) {
        self.drinkers = drinkers;
    }

    #[wasm_bindgen]
    pub fn set_sugar(&mut self, sugar: f64) {
        self.sugar = sugar;
    }

    #[wasm_bindgen]
    pub fn set_salt(&mut self, salt: f64) {
        self.salt = salt;
    }

    #[wasm_bindgen]
    pub fn set_fat(&mut self, fat: f64) {
        self.fat = fat;
    }

    #[wasm_bindgen]
    pub fn set_vitamins(&mut self, vitamins: f64) {
        self.vitamins = vitamins;
    }

    #[wasm_bindgen]
    pub fn set_wealth_factor(&mut self, wealth_factor: f64) {
        self.wealth_factor = wealth_factor;
    }

    #[wasm_bindgen]
    pub fn set_hard_drugger(&mut self, hard_drugger: f64) {
        self.hard_drugger = hard_drugger;
    }
}
