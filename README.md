# Beanreadings
Beanreadings is a library, and binary for simulating the future. We make predictions about the future and model our predictions only using publicly available data. It has a nice User Interface made using
HTML and CSS. The main simulation logic is written in Rust and compiled to WebAssembly. This ensures that the simulation is fast and efficient, and completely safe. We used data from websites such as
Our World in Data, and the National Institute of Health to make our predictions
## Installation
You don't have to install beanreadings, it's at [beanreadings](https://beanreadings.github.io). Please try it out today!
## Building from source
If you would like to install Beanreadings from source, you will have to build it from scratch on any PoSIX compliant system. This includes MacOS, Linux and WSL2.
You will require:
- Rust (Cargo)
- Node.js with the simple-http-server package which is used for testing before production
- A web browser that supports WebAssembly
- Git SCM
- Bash/Zsh
```bash
git clone https://github.com/Beanreadings/beanreadings.git
cd beanreadings
sh build.sh
```
Make sure you have the wasm32-unknown-unknown target installed on Rustup. If you don't have it installed, you can install it by running:
```bash
rustup target add wasm32-unknown-unknown
```
Without this toolchain, the WebAssembly binary will not be built, and the website will not work.
# Findings
- Blah blah blah
# Features
- Lightning fast, written in Rust. Native, it is even faster than on the web, but it still has a reasonable speed.
- Beautiful User Interface made with HTML, CSS and JavaScript.
- Completely safe, as it is written in Rust, and compiled to WebAssembly.
- Easy to use, just input your data and press the button.
- Neural Network (BRNN) for making predictions before running the simulation (extremely fast and experimental).
# Beanreadings Neural Network
The neural network has the following structure:
- 1 Input layer, 16 input neurons
- 7 Hidden layers, 48 neurons each
- 1 Output layer, 1 output neuron
- 43,000 MSE loss
The loss is currently not ideal, but it is a work in progress. Since we are working with large numbers, 43,000 is not a terrible loss (for example, 1000^2 is 1,000,000. If the answer was 1000, and the
inference returned 1050, the loss would be 102,500, which is much larger than 43,000).
# Bibliography
- [NIH Calcium Information](https://ods.od.nih.gov/factsheets/Calcium-HealthProfessional/)
- [ARS FPED Data Tables 2017-2020](https://www.ars.usda.gov/ARSUserFiles/80400530/pdf/fped/Table_1_FPED_GEN_1720.pdf)
- [NIH Fibre Intake for Optimal Health](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9298262/)
- [NIH Smoking](https://www.ncbi.nlm.nih.gov/books/NBK537066/)
- [NIH Alcohol effects in the United States](https://www.niaaa.nih.gov/alcohols-effects-health/alcohol-topics/alcohol-facts-and-statistics/alcohol-use-united-states-age-groups-and-demographic-characteristics#prevalence-of-past-month-drinking)
- [NIH Scope of Nicotine Use](https://nida.nih.gov/publications/research-reports/tobacco-nicotine-e-cigarettes/what-scope-tobacco-use-its-cost-to-society)
- [NIH Level of Physical Activity](https://pubmed.ncbi.nlm.nih.gov/31438909/)
- [CDC Sleep](https://www.cdc.gov/sleep/data-and-statistics/adults.html)
- [NIH Anti Vaccination](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9009899/)

As we believe the National Institute of Health is a reliable source, we have used their information to make our predictions. We have also used the USDA's data tables, and CDC's statistics. Every site on this list should be from a reliable source, preferably from a government page. As the government of the United States is a large government, with much data and abundant research papers, much of our research and simulations are based on data from their sources.
# Code Structure
The code is structured as follows:
src/lib.rs: Just a wrapper for everything, with re-exports and it's the entry point for the WebAssembly binary.
src/age.rs: Super fast age generation library, with multiple methods for generating ages with a good distribution (centered around a specified median age).
src/types.rs: Contains some of the types required for the simulation to function properly.
src/causes.rs: Causes for death. This contains types relating to the way people could die. Data is from Our World in Data ([This Article](https://ourworldindata.org/causes-of-death)).
src/factors.rs: Contains re-exports for the factors that will contribute to dying.
