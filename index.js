import init, {
  Simulation,
  SimulationConfig,
  Habits,
  Diet,
} from "./simulation.js";
await init();
export function simulate() {
  // we do stuff

  console.log("Attempting to simulate");

  let food = new Diet(1.0, 2.0, 1.5, 0.6, 1.5);

  console.log("Food: ", food);

  let habits = new Habits(0.23, 0.52, 0.23, 0.05, 0.55, 0.63, 0.25, 0.05, 0.13);

  console.log("Habits: ", habits);

  let simulation_config = new SimulationConfig(
    food,
    habits,
    BigInt(10),
    BigInt(1000),
  );

  console.log("Simulation Config: ", simulation_config);

  let simulation = new Simulation(simulation_config);

  console.log("Simulation: ", simulation);

  let result = simulation.simulate();

  document.getElementById("result").innerHTML = result;
}
