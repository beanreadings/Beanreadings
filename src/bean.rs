//! Represents beans for Beanreadings Simulator. Each bean represents a human, but has a certain
//! set of factors upon initialization. This will be used to determine whether a bean dies of a
//! certain death or not, or whether it reproduces or not.

use crate::age::AgeGenerator;
use crate::factors::*;
use crate::types::Settings;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Bean {
    age: u32, // beans cant be older than 2.1 billion years

    factors: Factors, // this is a structure that represents all of the required factors for a bean
}

#[wasm_bindgen]
impl Bean {
    #[wasm_bindgen(constructor)]
    pub fn new(context: &Settings, age_gen: &AgeGenerator) -> Self {
        let mut rng = rand::thread_rng();

        let age = age_gen.generate_age().abs() as u32; // using my custom age generator logic

        let alcoholism = rng.gen_bool(context.drinkers);

        let smoker = rng.gen_bool(context.smokers);

        let binge_drinkers = rng.gen_bool(context.drinkers % 3.0); // one third of drinkers are binge drinkers

        let smokes_weed = rng.gen_bool(context.weed_smokers);

        let vaper = rng.gen_bool(context.vapers);

        let hard_drugger = rng.gen_bool(context.hard_drugger);

        let sugar = Sugar {
            value: rng
                .gen_range(context.sugar - 0.5..=context.sugar + 0.5)
                .abs(),
        };

        let salt = Salt {
            value: rng.gen_range(context.salt - 0.5..=context.salt + 0.5).abs(),
        };

        let fat = Fat {
            value: rng.gen_range(context.fat - 0.5..=context.fat + 0.5).abs(),
        };

        let vitamins = Vitamins {
            value: rng
                .gen_range(context.vitamins - 0.6..=context.vitamins + 0.6)
                .abs(),
        };

        // wealth in Context is (upper class - lower class). So, depending on the wealth factor, we
        // assign this bean a value from 0 to 1 to represent their wealth.

        let mut wealth = Wealth { value: 0.0 };

        if context.wealth_factor < 0.0 {
            // this is a poor civilization

            let upper_class = rng.gen_bool(0.2); // 20% of the population is upper class

            if upper_class {
                wealth.value = rng.gen_range(0.8..=1.0);
            } else {
                wealth.value = rng.gen_range(0.0..=0.7);
            }
        } else {
            wealth.value = rng.gen_range(0.0..=1.0);
        }

        let factors = Factors {
            alcoholism,
            binge_drinker: binge_drinkers,
            smokes_weed,
            smokes_cigarettes: smoker,
            smokes_vape: vaper,
            consumes_hard_drugs: hard_drugger,
            sugar,
            salt,
            fat,
            vitamins,
            wealth,
        };

        return Bean { age, factors };
    }

    #[wasm_bindgen]
    pub fn get_age(&self) -> u32 {
        self.age
    }

    #[wasm_bindgen]
    pub fn get_factors(&self) -> Factors {
        self.factors.clone()
    }

    pub fn dies(&self, context: &Settings) -> bool {
        // whether this bean dies or not depending on the factors

        if self.age > context.max_age {
            return true;
        }

        let mut rng = rand::thread_rng();

        // depending on their habits, they could die

        let mut cardiovascular_disease = 0.0;
        let mut cancer = 0.0;
        let mut respiratory_disease = 0.0;
        let mut digestive_disease = 0.0;
        let mut infection = 0.0;
        let mut neonatal = 0.0;
        let mut dimentia = 0.0;
        let mut diabetes = 0.0;
        let mut diarrheal_disease = 0.0;
        let mut liver_disease = 0.0;
        let mut kidney_disease = 0.0;
        let mut malnutrition = 0.0;

        if self.age < 50 {
            cancer += 0.03;
            respiratory_disease += 0.02;
            digestive_disease += 0.01;
            dimentia += 0.02;
            liver_disease += 0.01;
            kidney_disease += 0.01;
        } else if self.age < 10 {
            neonatal += 0.02;
            diarrheal_disease += 0.01;
            malnutrition += 0.01;
        }

        infection += 0.01;
        diabetes += 0.03;
        cardiovascular_disease += 0.07; // much people die from cardiovascular disease

        if self.factors.smokes_cigarettes {
            // cigarette smokers tend to die more from lung cancer, respitory diseases and even
            // cardiovascular diseases

            cancer += 0.02;
            respiratory_disease += 0.03;
            cardiovascular_disease += 0.02;
        }

        false
    }
}
