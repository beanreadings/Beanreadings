# Write-up
This is the write-up for the science talent search project "Beanreadings". Beanreadings is a library, and website for simulating the future based on prior data that we have collected from multiple sources (mostly consisting of official health sites, such as the National Institute of Health (of the USA)).

Beanreadings collects paramaters, such as the amount of calcium in a diet, and the percentage of people who smoke, and then uses these parameters to predict population growth. We have assigned weights to each different paramater in many different methods, ranging from stochastic gradient descent, to manual deciding which would have the most accurate results based on the papers we have analyzed.

The internals of Beanreadings are written in Rust, a general-purpose low-level language that is one of the fastest languages. It is compiled to a format known as WebAssembly, which can be embedded into different websites, such as the Beanreadings website. Our website is written in JavaScript, HTML and CSS, the core languages to the modern web. This approach allows us to have the best possible performance, while maintaining readability and ergonomics.

We opted to make Beanreadings a static website, so that it can be hosted on GitHub Pages, a free hosting service provided by GitHub. This allows us to have a free, fast, and reliable website that can be accessed by anyone with an internet connection. We have also made the website open-source, so that anyone can contribute to the project, and help us make the future a better place.

We used PyTorch to create a neural network that can statically-analyze the paramaters before they are fed into the Rust simulation code. This is a "second-opinion" to ensure that the data from the simulation will be accurate.

This project is a work in progress, and we are constantly updating it with new data and new simulations. We hope that Beanreadings can be a useful tool for predicting the future, and that it can help us make the world a better place, by discovering the places that are lowering the population into decline, and hopefully maintaining better, healthier and more sustainable population.

# The rust code

The code that is written in Rust is highly-optimized, and compiled to WebAssembly. However, to ensure that the simulations are accurate, we iterate over each human in the simulation. For small population counts, this will not matter, but the time complexity will be around O(population * N), which will scale depending on the population, really badly. Therefore, the code is written to have accurate predictions, but will take a while. This is why we use our custom neural network to detect beforehand.
