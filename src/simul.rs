//! Simulation logic for Beanreadings, simulating all of the beans in the world.

use crate::bean::*;
use crate::types::*;
use crate::AgeGenerator;

use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Simulation {
    pub fn long(&self) -> SimulationResult {
        let mut population = self.settings.population;

        let mut population_curve = Vec::new();

        let mut age_gen = AgeGenerator::new(30.5); // median age

        let mut rng = rand::thread_rng();

        for _ in 0..self.settings.years {
            // we simulate the population growth

            for _ in 0..population {
                let gender = rng.gen_bool(0.49);

                if gender {
                    // we have to generate this person a age

                    let age = age_gen.generate_age();

                    if age < 40 && age > 18 {
                        let child_birth = rng.gen_bool(0.2); // they have a child

                        if child_birth {
                            population += 1;
                        }
                    }
                }
            }

            for _ in 0..population {
                let bean = Bean::new(&self.settings, &mut age_gen);
                if bean.dies(&self.settings) {
                    population -= 1;
                }
            }

            population_curve.push(population);
        }

        SimulationResult::new(population, population_curve)
    }
}
