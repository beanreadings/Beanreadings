//! Factors of death, and whether they will die or not. These are more-or-less randomly initialized
//! upon creation of a bean, and will be used to determine whether a bean dies of a certain death or
//! reproduces.

use wasm_bindgen::prelude::*;

/// Factor trait, which contains a name and a value. The name is the name of the factor, and the
/// value is the value of the factor. The value is a floating point number between 0 and 1, and
/// represents the likelihood of the bean dying of a certain death.

pub trait Factor {
    fn get_name(&self) -> String;
    fn get_value(&self) -> f64;
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sugar {
    pub value: f64,
}

impl Factor for Sugar {
    fn get_name(&self) -> String {
        "Sugar".to_string()
    }

    fn get_value(&self) -> f64 {
        self.value
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Salt {
    pub value: f64,
}

impl Factor for Salt {
    fn get_name(&self) -> String {
        "Salt".to_string()
    }

    fn get_value(&self) -> f64 {
        self.value
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fat {
    pub value: f64,
}

impl Factor for Fat {
    fn get_name(&self) -> String {
        "Fat".to_string()
    }

    fn get_value(&self) -> f64 {
        self.value
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vitamins {
    pub value: f64,
}

impl Factor for Vitamins {
    fn get_name(&self) -> String {
        "Vitamins".to_string()
    }

    fn get_value(&self) -> f64 {
        self.value
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Wealth {
    pub value: f64,
}

impl Factor for Wealth {
    fn get_name(&self) -> String {
        "Wealth".to_string()
    }

    fn get_value(&self) -> f64 {
        self.value
    }
}

// perhaps i can use a procedural macro to generate all of this code. However, that is probably not
// reliable to be used in production code, so I will not use it.

/// Beans each have a set of factors, along with "global facotrs", that are shared among all beans,
/// such as a global food shortage factor.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Factors {
    // recreation related factors
    pub alcoholism: bool,

    pub binge_drinker: bool,

    pub smokes_weed: bool, // recreational marijuana

    pub smokes_cigarettes: bool, // cigarettes

    pub smokes_vape: bool, // vaping

    pub consumes_hard_drugs: bool, // highest chance of death

    // diet related factors
    pub sugar: Sugar,

    pub salt: Salt,

    pub fat: Fat,

    pub vitamins: Vitamins,

    // wealth
    pub wealth: Wealth,
}

#[wasm_bindgen]
impl Factors {
    #[wasm_bindgen(constructor)]
    pub fn new(
        alcoholism: bool,
        binge_drinker: bool,
        smokes_weed: bool,
        smokes_cigarettes: bool,
        smokes_vape: bool,
        consumes_hard_drugs: bool,
        sugar: Sugar,
        salt: Salt,
        fat: Fat,
        vitamins: Vitamins,
        wealth: Wealth,
    ) -> Factors {
        Factors {
            alcoholism,
            binge_drinker,
            smokes_weed,
            smokes_cigarettes,
            smokes_vape,
            consumes_hard_drugs,
            sugar,
            salt,
            fat,
            vitamins,
            wealth,
        }
    }
}
