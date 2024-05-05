use rand::Rng;

pub struct NormalDistribution {
    mean: f64,
    standard_deviation: f64,
}

impl NormalDistribution {
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

fn generate_age(median: f64) -> u32 {
    loop {
        // standard deviation is 20 years, because most people live to around 80 years old

        // i use my cool normal distribution function to generate the age of the person

        // we have to generate a number between 0 and 1 to use with the CDF function

        let distribution = NormalDistribution::new(median, 20.0);

        let mut rng = rand::thread_rng();

        let number = rng.gen_range(0.0..1.0);

        let age = distribution.quartile(number);

        if age > 0.0 {
            return age.ceil() as u32;
        }
    }
}

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

impl Diet {
    pub fn new(calcium: f64, sugar: f64, solid_fats: f64, fibre: f64, salt: f64) -> Diet {
        Diet {
            calcium,
            sugar,
            solid_fats,
            fibre,
            salt,
        }
    }
}

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

            drugs: 0.13, // primarily marijuana, 13% of the population uses drugs
        }
    }
}

impl Habits {
    pub fn new(
        smoking: f64,
        alcohol: f64,
        binge_drinking: f64,
        vaping: f64,
        exercise: f64,
        sleep: f64,
        hydration: f64,
        anti_vaccination: f64,
        drugs: f64,
    ) -> Habits {
        Habits {
            smoking,
            alcohol,
            binge_drinking,
            vaping,
            exercise,
            sleep,
            hydration,
            anti_vaccination,
            drugs,
        }
    }
}

pub struct SimulationConfig {
    diet_settings: Diet,
    habit_settings: Habits,
    years: u64,
    population: u64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            diet_settings: Diet::default(),
            habit_settings: Habits::default(),
            years: 100,
            population: 8100000000, // 8.1 billion people
        }
    }
}

impl SimulationConfig {
    pub fn new(
        diet_settings: Diet,
        habit_settings: Habits,
        years: u64,
        population: u64,
    ) -> SimulationConfig {
        SimulationConfig {
            diet_settings,
            habit_settings,
            years,
            population,
        }
    }
}

pub struct Simulation {
    config: SimulationConfig,
}

impl Simulation {
    pub fn new(config: SimulationConfig) -> Simulation {
        Simulation { config }
    }
    pub fn simulate(&mut self) -> Vec<u64> {
        // load the config
        let config = &self.config;

        let mut population = config.population;

        let mut population_curve: Vec<u64> = vec![population];

        let mut log: Vec<String> = vec![];

        let mut rng = rand::thread_rng();

        let mut asfrs = [
            (15..20, 35.0),
            (20..25, 105.0),
            (25..30, 145.0),
            (30..35, 125.0),
            (35..40, 75.0),
            (40..45, 35.0),
            (45..50, 25.0),
        ];

        let mut median_age = 30.8;

        let mut bonus_rate = 1.05; // near the beginning, increase the birth rate

        // now we simulate every person in the population

        for i in 0..config.years {
            println!("Working on year {}", i);
            // this is your average joe, so now we have to calculate whats gonna happen to him

            // first, a random disaster chance, like a meteor hitting the earth, or a nuclear war
            // this is a very low chance, but it can happen

            let chance = rng.gen_range(0..100);

            // we have a disaster occuring around 7% of the time, similar to the rate of every 15
            // years in the 21st century

            if chance < 7 {
                log.push(format!("Year {}: A disaster has occured", i));
                // disaster occurs

                // now we gotta calculate the disaster

                // 1. meteor hitting the earth
                // 2. nuclear war
                // 3. global warming
                // 4. pandemic
                // 5. natural disaster

                let disaster = rng.gen_range(0..5);

                match disaster {
                    0 => {
                        // most deadly disaster

                        let deaths = rng.gen_range(90..99);
                        let rate = deaths as f64 / 100.0;

                        population -= (population as f64 * rate).ceil() as u64;

                        log.push(format!(
                            "Year {}: A meteor has hit the earth, {}% of the population has died",
                            i, deaths
                        ));
                    }
                    1 => {
                        // nuclear wars have the potential to kill billions of people in the real
                        // world, if tensions rise between countries, this could be a possibility
                        // in the future. https://ourworldindata.org/nuclear-weapons

                        let deaths = rng.gen_range(50..90);
                        let rate = deaths as f64 / 100.0;

                        population -= (population as f64 * rate).ceil() as u64;

                        log.push(format!(
                            "Year {}: A nuclear war has occured, {}% of the population has died",
                            i, deaths
                        ));
                    }
                    2 => {
                        // global warming is less likely to kill everyone, but in poorer countries
                        // will cause famine and drought, which will kill millions of people

                        let deaths = rng.gen_range(30..50);
                        let rate = deaths as f64 / 100.0;

                        population -= (population as f64 * rate).ceil() as u64;

                        log.push(format!(
                            "Year {}: Global warming has caused a famine, {}% of the population has died",
                            i, deaths
                        ));
                    }
                    3 => {
                        // as we saw with the SARS-CoV-2 pandemic, a pandemic can kill millions of
                        // people, and can spread very quickly. However, as COVID-19 was not very
                        // deadly, it did not kill as many people as it could have. If something
                        // with a high death rate like the Spanish Flu were to come back, it could
                        // kill millions of people

                        let deaths = rng.gen_range(20..30);
                        let rate = deaths as f64 / 100.0;

                        population -= (population as f64 * rate).ceil() as u64;

                        log.push(format!(
                            "Year {}: A pandemic has occured, {}% of the population has died",
                            i, deaths
                        ));
                    }
                    4 => {
                        // natural disasters are becoming more common due to global warming, and
                        // can kill millions of people. For example, the 2004 Indian Ocean tsunami
                        // killed over 230,000 people. The 2011 Tōhoku earthquake and tsunami killed
                        // over 15,000 people, but it started a nuclear disaster which makes Fukushima
                        // to this day uninhabitable. The 2010 Haiti earthquake killed over 200,000 people.

                        let deaths = rng.gen_range(1..20);
                        let rate = deaths as f64 / 100.0;

                        population -= (population as f64 * rate).ceil() as u64;

                        log.push(format!(
                            "Year {}: A natural disaster has occured, {}% of the population has died",
                            i, deaths
                        ));
                    }
                    _ => {}
                }
            }

            // generally the cost of having a child is increasing, so we have to account for that

            for asfr in asfrs.iter_mut() {
                if asfr.1 > 1.0 {
                    asfr.1 -= 0.1;
                }
            }

            if bonus_rate > 1.0 {
                population = (population as f64 * bonus_rate).ceil() as u64;
            }

            bonus_rate -= 0.01; // decrease the birth rate

            // we increase the population early

            let mut births = 0;

            for _ in 0..population {
                let female = rng.gen_bool(0.5);

                if female {
                    let age = generate_age(median_age) - 5; // hard coded weight

                    for asfr in asfrs.iter() {
                        if asfr.0.contains(&age) {
                            let birth_chance = (asfr.1 / 1000.0f64) * 100.0; // 2 / 1000 = 0.002,

                            let chance = rng.gen_range(0.0..100.0);

                            if birth_chance > chance {
                                births += 1;
                            }

                            break;
                        }
                    }
                }
            }

            let birth_rate = births as f64 / population as f64;

            population += births;

            // so we calculate the median age of the population

            // median age should be lower than it was before this, but it will be significantly
            // increased after we calculate the deaths

            // now that we have had our disaster chance, we can calculate the average joe's death

            // first, we calculate the death chances for everything

            let mut deaths = 0;

            for _ in 0..population {
                // we calculate their age
                // for now we are using a random number generator, but in the future we will use
                // more accurate data
                // we will use the median age of the population to calculate the age of the person

                // we will use a normal distribution for now, but if the age is negative, reroll
                // lols
                let age = generate_age(median_age);

                // we calculate disease rates etc

                // as a general rule of thumb as of 2024, 1/150 people die = 0.7% of the population

                // 33% of those deaths were from cardiovascular disease, proving that its the most
                // common cause of death
                // 0.2333% of the population dies from cardiovascular disease
                // however as a base rate we will use 0.2% and then add some modifiers such as the
                // habit and diet

                // now we calculate diet and habit modifiers

                let _calcium = rng
                    .gen_range(
                        (config.diet_settings.calcium - 0.5)..(config.diet_settings.calcium + 0.5),
                    )
                    .abs();

                let sugar = rng
                    .gen_range(
                        (config.diet_settings.sugar - 0.5)..(config.diet_settings.sugar + 0.5),
                    )
                    .abs();

                let solid_fats = rng
                    .gen_range(
                        (config.diet_settings.solid_fats - 0.5)
                            ..(config.diet_settings.solid_fats + 0.5),
                    )
                    .abs();

                let salt = rng
                    .gen_range((config.diet_settings.salt - 0.5)..(config.diet_settings.salt + 0.5))
                    .abs();

                let fibre = rng
                    .gen_range(
                        (config.diet_settings.fibre - 0.5)..(config.diet_settings.fibre + 0.5),
                    )
                    .abs();

                let smoking = rng
                    .gen_range(
                        (config.habit_settings.smoking - 0.5)
                            ..(config.habit_settings.smoking + 0.5),
                    )
                    .abs();

                let alcohol = rng
                    .gen_range(
                        (config.habit_settings.alcohol - 0.5)
                            ..(config.habit_settings.alcohol + 0.5),
                    )
                    .abs();

                let binge_drinking = rng
                    .gen_range(
                        (config.habit_settings.binge_drinking - 0.5)
                            ..(config.habit_settings.binge_drinking + 0.5),
                    )
                    .abs();

                let vaping = rng
                    .gen_range(
                        (config.habit_settings.vaping - 0.5)..(config.habit_settings.vaping + 0.5),
                    )
                    .abs();

                let exercise = rng
                    .gen_range(
                        (config.habit_settings.exercise - 0.5)
                            ..(config.habit_settings.exercise + 0.5),
                    )
                    .abs();

                let sleep = rng
                    .gen_range(
                        (config.habit_settings.sleep - 0.5)..(config.habit_settings.sleep + 0.5),
                    )
                    .abs();

                let _hydration = rng
                    .gen_range(
                        (config.habit_settings.hydration - 0.5)
                            ..(config.habit_settings.hydration + 0.5),
                    )
                    .abs();

                let _anti_vaccination = rng
                    .gen_range(
                        (config.habit_settings.anti_vaccination - 0.5)
                            ..(config.habit_settings.anti_vaccination + 0.5),
                    )
                    .abs();

                let drugs = rng
                    .gen_range(
                        (config.habit_settings.drugs - 0.5)..(config.habit_settings.drugs + 0.5),
                    )
                    .abs();

                // diet modifiers

                let mut cardiovascular_disease_rate = 0.2;

                // if sugar == 1 then we add no modifier, otherwise subtract
                // we add 0.02 per 0.1 sugar, so if sugar is 0.5, we add 0.1 to the rate

                if sugar > 1.0 {
                    cardiovascular_disease_rate += ((sugar - 1.0) * 10.0) * 0.02;
                }

                // solid fats also have a **HUGE** effect on cardiovascular disease

                // trans fats have a HUGE effect on cardiovascular disease, so we add 0.2 per 0.1
                // same as sugar but arguably trans fats are worse
                if solid_fats > 1.0 {
                    cardiovascular_disease_rate += ((solid_fats - 1.0) * 10.0) * 0.02;
                }

                // salt has a moderate effect on cardiovascular disease, so we add 0.1 per 0.1
                // salt

                if salt > 1.0 {
                    cardiovascular_disease_rate += ((salt - 1.0) * 10.0) * 0.01;
                }

                // fibre helps prevent cardiovascular disease, so we subtract 0.1 per 0.1 fibre

                if fibre > 1.0 {
                    cardiovascular_disease_rate -= ((fibre - 1.0) * 10.0) * 0.01;
                }

                // smoking = cardiovascular disease, so we add 0.2 per 0.1 smoking

                if smoking > 1.0 {
                    cardiovascular_disease_rate += ((smoking - 1.0) * 10.0) * 0.02;
                }

                // alcohol doesnt have a huge effect on cardiovascular disease, but it does have an
                // effect, so we add 0.1 per 0.1 alcohol. sometimes it can be good for you, but
                // most of the time its bad

                if smoking < 1.0 {
                    cardiovascular_disease_rate -= 0.02;
                } else {
                    cardiovascular_disease_rate += ((alcohol - 1.0) * 10.0) * 0.01;
                }

                // binge drinking is very bad for you, so we add 0.2 per 0.1 binge drinking

                if binge_drinking > 1.0 {
                    cardiovascular_disease_rate += ((binge_drinking - 1.0) * 10.0) * 0.02;
                }

                // vaping is bad for you, so we add 0.1 per 0.1 vaping

                if vaping > 1.0 {
                    cardiovascular_disease_rate += ((vaping - 1.0) * 10.0) * 0.01;
                }

                // exercise is good for you, so we subtract 0.1 per 0.1 exercise

                if exercise > 1.0 {
                    cardiovascular_disease_rate -= ((exercise - 1.0) * 10.0) * 0.01;
                }

                // sleep only has an effect if you dont get enough, so we add 0.1 per 0.1 sleep

                if sleep < 0.5 {
                    cardiovascular_disease_rate += 0.01;
                }

                // drugs are bad for you, so we add 0.2 per 0.1 drugs

                if drugs > 1.0 {
                    cardiovascular_disease_rate += ((drugs - 1.0) * 10.0) * 0.02;
                }

                let chance = rng.gen_range(0.0..100.0) as f64;

                if chance < cardiovascular_disease_rate {
                    population -= 1;
                    deaths += 1;
                    continue;
                }

                // now we calculate the death rate for cancers

                let mut cancer_rate = 0.13;

                // solid fats effect several types of cancer, so we add 0.2 per 0.1 solid fats

                if solid_fats > 1.0 {
                    cancer_rate += ((solid_fats - 1.0) * 10.0) * 0.02;
                }

                // salt can cause cancer, so we add 0.1 per 0.1 salt

                if salt > 1.0 {
                    cancer_rate += ((salt - 1.0) * 10.0) * 0.01;
                }

                // smoking causes lung cancer, so we add 0.2 per 0.1 smoking

                if smoking > 1.0 {
                    cancer_rate += ((smoking - 1.0) * 10.0) * 0.02;
                }

                // alcohol can cause cancer, so we add 0.1 per 0.1 alcohol

                if alcohol > 1.0 {
                    cancer_rate += ((alcohol - 1.0) * 10.0) * 0.01;
                }

                // binge drinking can cause cancer, so we add 0.2 per 0.1 binge drinking

                if binge_drinking > 1.0 {
                    cancer_rate += ((binge_drinking - 1.0) * 10.0) * 0.02;
                }

                // vaping can cause cancer, so we add 0.1 per 0.1 vaping

                if vaping > 1.0 {
                    cancer_rate += ((vaping - 1.0) * 10.0) * 0.01;
                }

                // now we calculate the likelihood of dying from cancer

                let chance = rng.gen_range(0.0..100.0) as f64;

                if chance < cancer_rate {
                    // bro died from cancer
                    population -= 1;
                    deaths += 1;
                    continue;
                }

                // respitory diseases

                let mut respitory_rate = 0.05;

                // smoking causes respitory diseases, so we add 0.2 per 0.1 smoking

                if smoking > 1.0 {
                    respitory_rate += ((smoking - 1.0) * 10.0) * 0.02;
                }

                // vaping can cause respitory diseases, so we add 0.1 per 0.1 vaping

                if vaping > 1.0 {
                    respitory_rate += ((vaping - 1.0) * 10.0) * 0.01;
                }

                // now we calculate the likelihood of dying from respitory diseases

                let chance = rng.gen_range(0.0..100.0) as f64;

                if chance < respitory_rate {
                    // bro died from respitory diseases
                    population -= 1;
                    deaths += 1;
                    continue;
                }

                // now we calculate the likelihood of dying from digestive diseases

                let mut digestive_rate = 0.02;

                // solid fats can cause digestive diseases, so we add 0.1 per 0.1 solid fats

                if solid_fats > 1.0 {
                    digestive_rate += ((solid_fats - 1.0) * 10.0) * 0.01;
                }

                // salt can cause digestive diseases, so we add 0.1 per 0.1 salt

                if salt > 1.0 {
                    digestive_rate += ((salt - 1.0) * 10.0) * 0.01;
                }

                // now we calculate the likelihood of dying from digestive diseases

                let chance = rng.gen_range(0.0..100.0) as f64;

                if chance < digestive_rate {
                    // bro died from digestive diseases
                    population -= 1;
                    deaths += 1;
                    continue;
                }

                // who dies from lower respitory infections

                let mut lower_respitory_rate = 0.01;

                // smoking causes lower respitory infections, so we add 0.1 per 0.1 smoking
                // the reason for lower modifiers now, is because all of these cauess of death are
                // lower and less common

                if smoking > 1.0 {
                    lower_respitory_rate += ((smoking - 1.0) * 10.0) * 0.01;
                }

                // vaping can cause lower respitory infections, so we add 0.1 per 0.1 vaping

                if vaping > 1.0 {
                    lower_respitory_rate += ((vaping - 1.0) * 10.0) * 0.01;
                }

                // now we calculate the likelihood of dying from lower respitory infections

                let chance = rng.gen_range(0.0..100.0) as f64;

                if chance < lower_respitory_rate {
                    // bro died from lower respitory infections
                    population -= 1;
                    deaths += 1;
                    continue;
                }

                // neonatal disease

                if age < 10 {
                    // up to 10 because the age generator rarely generates babies
                    // neonatal disease kills millions of babies every year, so we have to account
                    // for that

                    let neonatal_rate = 20; // 20% of babies die from neonatal disease. Rest
                                            // assured, my flawed age generator will not generate many babies, but they
                                            // will all have a 20% chance of dying

                    let chance = rng.gen_range(0..100);

                    if chance < neonatal_rate {
                        // bro died from neonatal disease
                        population -= 1;
                        deaths += 1;
                        continue;
                    }
                }

                // dimentia kills people

                if age > 65 {
                    // dimentia is a common cause of death in the elderly, so we have to account for
                    // that

                    let dimentia_rate = 5; // 5% of people die from dimentia

                    let chance = rng.gen_range(0..100);

                    if chance < dimentia_rate {
                        // bro died from dimentia
                        population -= 1;
                        deaths += 1;
                        continue;
                    }
                }

                // next cycle lol
            }

            let death_rate = deaths as f64 / population as f64;

            if death_rate > birth_rate {
                // we increase the median age of the population

                median_age += (death_rate - birth_rate) / 100.0;
            }

            population_curve.push(population);
        }
        population_curve
    }
}

fn generate_data() -> String {
    println!("Generating data for training BRNN");
}
