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

    // calcium: 1000mg required per day, average is around 1000mg which is good
    calcium: f64,

    // sugar: 50g max per day with a 2000 calorie diet, average is around 100g which is bad
    sugar: f64,

    // solid fats: includes trans fats, saturated fats. 24g max per day with a 2000 calorie diet
    // average intake is 37g which is bad. increases risk of cardiovascular disease
    solid_fats: f64,

    // fibre: who recommends 25g per day, average intake is 15g which is bad
    fibre: f64,

    // salt: generally less than 2.3g per day, average intake is 3.4g which is bad
    salt: f64,
}

impl Default for Diet {
    fn default() -> Self {
        // for diet, we assign them each a weight and what they corrolate to in the deaths category.
        // The weight will probably be calculated by something similar to Stochastic Gradient Descent
        // to optimize the weights and their corrolation on the death rate.
        Self {
            calcium: 1.0,    // we get 100% of the calcium we need, which is good
            sugar: 2.0,      // we get 2x the sugar we can get, maximum
            solid_fats: 1.5, // we get 1.5x the solid fats we can get, maximum
            fibre: 0.6,      // we get 60% of the fibre we need, which is bad
            salt: 1.5,       // we get 1.5x the salt we can get, maximum
        }
    }
}

#[wasm_bindgen]
pub struct Habits {
    // similar to diet, we assign them each a weight and what they corrolate to in the deaths category.
    // The weight will probably be calculated by something similar to Stochastic Gradient Descent
    // to optimize the weights and their corrolation on the death rate.
    // 1.00 = 100% of what we need, 0.50 = 50% of what we need

    // smoking: 0.5 = 50% of the population smokes, 1.0 = 100% of the population smokes
    smoking: f64,

    // alcohol: 0.5 = 50% of the population drinks, 1.0 = 100% of the population drinks (per month)
    alcohol: f64,

    // binge drinking: 0.5 = 50% of the population binge drinks, 1.0 = 100% of the population binge drinks (per month)
    binge_drinking: f64,

    // vaping: 0.5 = 50% of the population vapes, 1.0 = 100% of the population vapes
    vaping: f64,

    // proper exercise: 0.5 = 50% of the population exercises, 1.0 = 100% of the population exercises
    exercise: f64,

    // proper sleep: 0.5 = 50% of the population gets proper sleep, 1.0 = 100% of the population gets proper sleep
    sleep: f64,

    // proper hydration: 0.5 = 50% of the population is properly hydrated, 1.0 = 100% of the population is properly hydrated
    hydration: f64,

    // anti-vaccination: 0.5 = 50% of the population is anti-vaccination, 1.0 = 100% of the population is anti-vaccination
    anti_vaccination: f64,

    // drugs: 0.5 = 50% of the population uses drugs, 1.0 = 100% of the population uses drugs
    drugs: f64,
}

impl Default for Habits {
    fn default() -> Self {
        Self {
            smoking: 0.23, // 23% of the population smokes

            alcohol: 0.52, // its done based on a monthly-basis, 52% of the population drinks every month

            binge_drinking: 0.23, // its done based on a monthly-basis, 23.5% of the population binge drinks every month

            vaping: 0.05, // 5% of the population vapes, this is in 2022, above 12 years old

            exercise: 0.55, // 55% of the population exercises properly

            sleep: 0.63, // 63% of the population gets proper sleep

            hydration: 0.25, // 25% of the population is properly hydrated. Average is 2.5 cups per day

            anti_vaccination: 0.05, // 5% of the population is anti-vaccination,

            drugs: 0.05, // 5% of the population uses drugs
        }
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

        serde_json::to_string_pretty(&serde_json::json!({
            "settings": {
                "diet": {
                    "calcium": config.diet_settings.calcium,
                    "sugar": config.diet_settings.sugar,
                    "solid_fats": config.diet_settings.solid_fats,
                    "fibre": config.diet_settings.fibre,
                    "salt": config.diet_settings.salt,
                },
            },

        }))
        .unwrap()
    }
}
