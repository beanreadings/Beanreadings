//! Age generator for Beanreadings, a simulation for population growth.
//! We use a completely custom approach by me

use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AgeGenerator {
    pub median_age: f64,

    rng: ThreadRng,
}

#[wasm_bindgen]
impl AgeGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(median_age: f64) -> AgeGenerator {
        AgeGenerator {
            median_age,
            rng: rand::thread_rng(),
        }
    }

    pub fn update_median_age(&mut self, median_age: f64) {
        self.median_age = median_age;
    }

    pub fn generate_age(&mut self) -> i32 {
        // This is a completely custom approach to Age generation by me. We don't use distributions
        // like normal distributions or uniform. We use a custom approach by Aityz :)

        let median_age = self.median_age.round() as i32;

        // generally if you look at the current curves, most are around working class but if we
        // center on the median age, i can say around 40% of the population is within 10 years of
        // the median age

        let rng = &mut self.rng;

        let chance = rng.gen_bool(0.4);

        if chance {
            // we have got the age in the 20 year range, but we will randomly select a possible age
            // within this range

            rng.gen_range(median_age - 10..=median_age + 10)
        } else {
            // we are in the other 60% of the population.

            // 15% for the chance of being a minor, regardless of the median age. This is to
            // normalize the population

            let chance = rng.gen_bool(0.15);

            if chance {
                rng.gen_range(0..=17)
            } else {
                // this is 45% chance to be called here, so around half of the population is in
                // this category. So we split it into half. Peopel who are older than the median
                // age, and people who are younger than the median age, but are not minors. But
                // this will only be called if there is a 15 year gap between the median age and
                // the age of minors.

                if median_age - 15 > 15 {
                    // median age is above 30, so we call half between the median age and 15

                    let below_median = rng.gen_bool(0.5);

                    if below_median {
                        rng.gen_range(18..=median_age - 10)
                    } else {
                        rng.gen_range(median_age + 10..=100)
                    }
                } else {
                    // we just return median age + 10 to 100, because the median age is below 30

                    rng.gen_range(median_age + 10..=100)
                }
            }
        }
    }
}

// alternative age generator

#[wasm_bindgen]
/// This uses a Normal Distribution to generate ages, which may be less relatistic more faster
/// because we care about speed.
pub struct LegacyAgeGenerator {
    pub median_age: f64,
}

#[wasm_bindgen]
impl LegacyAgeGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(median_age: f64) -> LegacyAgeGenerator {
        LegacyAgeGenerator { median_age }
    }

    pub fn generate_age(&self) -> u32 {
        loop {
            // standard deviation is 20 years, because most people live to around 80 years old

            // i use my cool normal distribution function to generate the age of the person

            // we have to generate a number between 0 and 1 to use with the CDF function

            let distribution = NormalDistribution::new(self.median_age, 20.0);

            let mut rng = rand::thread_rng();

            let number = rng.gen_range(0.0..1.0);

            let age = distribution.quartile(number);

            if age > 0.0 {
                return age.ceil() as u32;
            }
        }
    }
}

#[wasm_bindgen]
pub struct NormalDistribution {
    mean: f64,
    standard_deviation: f64,
}

#[wasm_bindgen]
impl NormalDistribution {
    #[wasm_bindgen(constructor)]
    pub fn new(mean: f64, standard_deviation: f64) -> NormalDistribution {
        NormalDistribution {
            mean,
            standard_deviation,
        }
    }

    pub fn pdf(&self, x: f64) -> f64 {
        // probability density function

        // 1 / (σ * sqrt(2π)) * e^(-1/2 * ((x - μ) / σ)^2)

        1.0 / (self.standard_deviation * (2.0 * std::f64::consts::PI).sqrt())
            * (-0.5 * ((x - self.mean) / self.standard_deviation).powi(2)).exp()
    }

    pub fn cdf(&self, x: f64) -> f64 {
        // cumulative density function

        // 1/2 * (1 + erf((x - μ) / (σ * sqrt(2))))

        1f64 / 2f64
            * (1.0f64
                + errorfunctions::RealErrorFunctions::erf(
                    (x - self.mean) / (self.standard_deviation * (2.0f64).sqrt()),
                ))
    }

    pub fn quartile(&self, x: f64) -> f64 {
        // this is the inverse of the CDF function

        // μ + σ * sqrt(2) * erf^(-1)(2 * x - 1)

        self.mean
            + self.standard_deviation
                * (2.0f64).sqrt()
                * errorfunctions::RealErrorFunctions::erfi(2.0 * x - 1.0)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_distribution() {
        // we recommend using -- --nocapture flag to see the output of the test

        let age_gen = AgeGenerator::new(35.0);

        // we generate 1000 ages and see the distribution

        let mut age_count = vec![0; 101];

        for _ in 0..1000 {
            let age = age_gen.generate_age();
            age_count[age as usize] += 1;
        }

        for (i, count) in age_count.iter().enumerate() {
            if *count != 0 {
                println!("Age: {} Count: {}", i, count);
            }
        }
    }

    #[test]
    fn test_legacy_distribution() {
        // we recommend using -- --nocapture flag to see the output of the test

        let age_gen = LegacyAgeGenerator::new(35.0);

        // we generate 1000 ages and see the distribution

        let mut age_count = vec![0; 101];

        for _ in 0..1000 {
            let age = age_gen.generate_age();
            age_count[age as usize] += 1;
        }

        for (i, count) in age_count.iter().enumerate() {
            if *count != 0 {
                println!("Age: {} Count: {}", i, count);
            }
        }
    }
}
