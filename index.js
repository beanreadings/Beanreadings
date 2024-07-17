import init, { Simulation, Settings, SimulationResult } from "./simulation.js";

await init();
let graph;
const config = {
  drinkers: 0.2,
  fat: 0.5,
  hard_drugger: 0.05,
  max_age: 100,
  population: 1000,
  salt: 0.5,
  smokers: 0.2,
  sugar: 0.5,
  vapers: 0.1,
  vitamins: 0.5,
  wealth_factor: -0.1,
  weed_smokers: 0.1,
  years: 10
}

export function simulate() {
  console.log("Attempting to simulate");

  let settings = new Settings();
  settings.set_drinkers(config.drinkers);
  settings.set_fat(config.fat);
  settings.set_hard_drugger(config.hard_drugger);
  settings.set_max_age(config.max_age);
  settings.set_population(config.population);
  settings.set_salt(config.salt);
  settings.set_smokers(config.smokers);
  settings.set_sugar(config.sugar);
  settings.set_vapers(config.vapers);
  settings.set_vitamins(config.vitamins);
  settings.set_wealth_factor(config.wealth_factor);
  settings.set_weed_smokers(config.weed_smokers);
  settings.set_years(config.years);

  let simulation = new Simulation(settings);

  let simil = simulation.long();
  updateTable(simil.get_population_curve());

  graph ? graph.destroy() : null;

  graph = new Chart("graph", {
    type: "line",
    data: {
      labels: simil.get_population_curve().map((_, i) => i),
      datasets: [{
        label: 'Population',
        data: simil.get_population_curve(),
        fill: false,
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1,
        backgroundColor: 'rgba(75, 192, 192, 0.2)',
        pointBackgroundColor: 'rgb(75, 192, 192)',
        pointBorderColor: '#fff',
        pointHoverBackgroundColor: '#fff',
        pointHoverBorderColor: 'rgb(75, 192, 192)',
        hitRadius: 8,
      }]
    },
    options: {
      responsive: true,
      plugins: {
        legend: {
          display: true,
          onClick: null
        },
        title: {
          display: false,
          text: 'Population'
        }
      },
      plugins: {
        tooltip: {
          callbacks: {
            title: function(tooltipItems, data) {
              return `Year ${Number(tooltipItems[0].label) + 1}`;
            },
          }
        }
      },
      scales: {
        x: {
          ticks: {
            callback: function(value, index, ticks) {
              return 'Year ' + (value + 1);
            },
            color: '#ffffff'
          },
          grid: {
            color: 'rgba(255, 255, 255, 0.2)'
          }
        },
        y: {
          ticks: {
            color: '#ffffff'
          },
          grid: {
            color: 'rgba(255, 255, 255, 0.2)'
          }
        }
      }
    }
  });
}
simulate();

export function openConfigure() {
  document.getElementById("config-popup").style.display = "flex";
  setTimeout(() => {
    document.getElementById("config-popup").style.opacity = 1;
    document.getElementById("config-popup").children[0].style.transform = "scale(1)";
  }, 10);
}

export function closeConfigure() {
  document.getElementById("conf-status").style.opacity = 0;
  document.getElementById("config-popup").style.opacity = 0;
  document.getElementById("config-popup").children[0].style.transform = "scale(0)";
  setTimeout(() => {
    document.getElementById("config-popup").style.display = "";
  }, 500);
}
const confValues = [
  {name:'population', el: document.getElementById("population"), min: 1, max: 100000},
  {name:'years', el: document.getElementById("years"), min: 2, max: 100},
  {name:'max_age', el: document.getElementById("max-age"), min: 1, max: 140},
  {name:'wealth_factor', el: document.getElementById("wealth-factor"), min: -100, max: 100, percent: true},
  {name:'drinkers', el: document.getElementById("drinkers"), min: 0, max: 100, percent: true},
  {name:'hard_drugger', el: document.getElementById("drug-users"), min: 0, max: 100, percent: true},
  {name:'smokers', el: document.getElementById("smokers"), min: 0, max: 100, percent: true},
  {name:'vapers', el: document.getElementById("vapers"), min: 0, max: 100, percent: true},
  {name:'weed_smokers', el: document.getElementById("weed-smokers"), min: 0, max: 100, percent: true},
  {name:'fat', el: document.getElementById("fat"), min: 0, max: 100, percent: true},
  {name:'salt', el: document.getElementById("salt"), min: 0, max: 100, percent: true},
  {name:'sugar', el: document.getElementById("sugar"), min: 0, max: 100, percent: true},
  {name:'vitamins', el: document.getElementById("vitamins"), min: 0, max: 100, percent: true}
]
confValues.forEach((value) => {
  value.el.addEventListener('blur', (e) => {
    if (value.el.value === '') {
      return;
    }
    if (value.el.value > value.max) {
      value.el.value = value.max;
    } else if (value.el.value < value.min) {
      value.el.value = value.min;
    }
  });
});

export function applyConfig() {
  confValues.forEach((value) => {
    if (value.el.value === '') {
      value.el.value = value.min;
    } else if (value.el.value < value.min) {
      value.el.value = value.min;
    } else if (value.el.value > value.max) {
      value.el.value = value.max;
    }
    config[value.name] = value.percent ? Number(value.el.value) / 100 : Number(value.el.value);
  });
  document.getElementById("conf-status").style.opacity = 1;
  setTimeout(() => {
    document.getElementById("conf-status").style.opacity = 0;
  }, 2000);
}

function updateTable(populationCurve) {
  const tableBody = document.querySelector(".sim-results .inner table tbody");
  tableBody.innerHTML = '';
  function applyHeader() {
    const row = document.createElement("tr");
    const yearHeader = document.createElement("th");
    const populationHeader = document.createElement("th");

    yearHeader.textContent = "Year";
    populationHeader.textContent = "Population";

    row.appendChild(yearHeader);
    row.appendChild(populationHeader);
    tableBody.appendChild(row);
  }
  function year0() {
    const row = document.createElement("tr");
    const yearCell = document.createElement("td");
    const populationCell = document.createElement("td");

    yearCell.textContent = `Year 0`;
    populationCell.textContent = config.population;

    row.appendChild(yearCell);
    row.appendChild(populationCell);
    tableBody.appendChild(row);
  }
  applyHeader();
  year0();
  populationCurve.forEach((population, index) => {
    const row = document.createElement("tr");
    const yearCell = document.createElement("td");
    const populationCell = document.createElement("td");

    yearCell.textContent = `Year ${index + 1}`;
    populationCell.textContent = population;

    row.appendChild(yearCell);
    row.appendChild(populationCell);
    tableBody.appendChild(row);
  });
}
