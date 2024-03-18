use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct SimulationConfig {
    death_rate: f64,
    birth_rate: f64,
    base: isize,
    years: isize,
    cancer_rate: f64,
    cancer_death_rate: f64,
    cancer_recovery_rate: f64,
    people_with_cancer: isize,
    disaster_rate: f64,
    disasters: isize,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            death_rate: 0.0185,
            birth_rate: 0.0370,
            cancer_rate: 10.0,
            cancer_recovery_rate: 10.0,
            cancer_death_rate: 5.0,
            people_with_cancer: 500,
            base: 1000,
            years: 10,
            disaster_rate: 1.0,
            disasters: 0,
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
    pub fn cancer_recovery_rate(&mut self, amount: f64) {
        self.cancer_recovery_rate = amount;
    }
    pub fn cancer_death_rate(&mut self, amount: f64) {
        self.cancer_death_rate = amount;
    }
    pub fn people_with_cancer(&mut self, amount: isize) {
        self.people_with_cancer = amount;
    }
    pub fn cancer_rate(&mut self, amount: f64) {
        self.cancer_rate = amount;
    }
    pub fn death_rate(&mut self, amount: f64) {
        self.death_rate = amount;
    }
    pub fn birth_rate(&mut self, amount: f64) {
        self.birth_rate = amount;
    }
    pub fn base_population(&mut self, amount: isize) {
        self.base = amount;
    }
    pub fn years(&mut self, amount: isize) {
        self.years = amount;
    }
    pub fn disaster_rate(&mut self, amount: f64) {
        self.disaster_rate = amount;
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
        let mut cancered: isize = self.config.people_with_cancer;
        let mut rng = rand::thread_rng();
        let mut population = self.config.base;
        let mut leap = 1;

        // Year tracking
        let mut yss = 0;
        // Years since start

        for _year in 0..self.config.years {
            if population > 0 {
                let death_percentage = self.config.death_rate * population as f64;
                let mut deaths = 0;

                for _person in 0..population {
                    let death = rng.gen_range(0.0..100000.0);
                    if death < death_percentage {
                        deaths += 1;
                    }
                }

                for _cancerererererererere in 0..self.config.people_with_cancer.clone() {
                    let rand: f64 = rng.gen_range(0..=100) as f64;
                    if rand < self.config.cancer_recovery_rate {
                        self.config.people_with_cancer -= 1;
                    }
                    let rand: f64 = rng.gen_range(0..=100) as f64;
                    if rand < self.config.cancer_death_rate {
                        population -= 1;
                        self.config.people_with_cancer -= 1;
                    }
                }

                // Give people cancer
                let new_people =
                    (self.config.cancer_rate / 100.0 * population as f64).floor() as isize;

                cancered += new_people;
                self.config.people_with_cancer += new_people;

                // Potential disaster

                let disaster = rng.gen_range(0.0..100.0);
                if disaster < self.config.disaster_rate {
                    let deaths: f64 = population as f64 / rng.gen_range(1..10) as f64;
                    population -= deaths.floor() as isize;
                    self.config.disasters += 1;
                }

                population -= deaths;

                let addition: isize = (self.config.birth_rate * population as f64).floor() as isize;
                population += addition;

                leap += 1;
                if leap == 4 {
                    leap = 1;
                }
                yss += 1;
            }
        }
        serde_json::to_string_pretty(&serde_json::json!(
            {
                "end": {
                    "population": population,
                    "cancer": self.config.people_with_cancer,
                },
                "total": {
                    "cancer": cancered,
                    "disasters": self.config.disasters,
                },
            }
        ))
        .unwrap()
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
