import init, { Simulation, SimulationConfig } from "./simulation.js";
await init();
export function simulate() {
    let cancer_death = document.getElementById("cancer_death").value;
    let base = document.getElementById("base").value;
    let cancer_rate = document.getElementById("cancer_rate").value;
    let years = document.getElementById("years").value;
    let death_rate = document.getElementById("death_rate").value;
    let birth_rate = document.getElementById("birth_rate").value;
    let config = new SimulationConfig();
    config.cancer_death_rate(cancer_death);
    config.cancer_rate(cancer_rate);
    config.base_population(base);
    config.years(years);
    config.death_rate(death_rate);
    config.birth_rate(birth_rate);
    let simulation = new Simulation(config).simulate();
    document.getElementById("result").innerHTML = simulation;
}
