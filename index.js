import init, { Simulation, Settings, SimulationResult } from "./simulation.js";

await init();
export function simulate() {
  // we do stuff

  console.log("Attempting to simulate");

  let settings = new Settings();

  settings.set_population(1000);

  let simulation = new Simulation(settings);

  let simil = simulation.long();

  console.log(simil.population);

  console.log(simil.get_population_curve());
}
