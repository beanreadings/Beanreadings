//! Beanreadings neural network
//! CLI tool to generate data for beanreadings. We use the same data from the original beanreadings
//! project, but we use the data to train a neural network.

mod beanreadings;

use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("Beanreadings Neural Network Project");
        println!();
        println!("Arguments:");
        println!("  -h, --help    Show this help message");
        println!("  -v, --version Show the version of the program");
        println!("  -d, --data    Generate trainable data for the neural network");
        println!("  -t, --train   Train the neural network");
        println!("  -i, --infer   Run inference on the neural network");
        println!();
        println!("Usage:");
        println!("  brn -d");
        println!("  brn -t");
        println!("  brn -i");
        println!();
        println!("Made by the Beanreadings team, Limitfinity and Aityz");
    } else if args.contains(&String::from("-v")) || args.contains(&String::from("--version")) {
        println!("Beanreadings Neural Network Project v0.1.0");
    } else if args.contains(&String::from("-d")) || args.contains(&String::from("--data")) {
    }
}
