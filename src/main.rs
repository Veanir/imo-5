mod tsplib;
mod algorithm;
mod algorithms;

use std::path::Path;
use tsplib::TsplibInstance;
use algorithm::{TspAlgorithm, run_experiment, format_stats_row};
use algorithms::*;

fn main() {
    println!("Loading instances...");
    
    // Load both instances
    let instances = [
        ("kroa200", TsplibInstance::from_file(Path::new("tsplib/kroa200.tsp"))),
        ("krob200", TsplibInstance::from_file(Path::new("tsplib/krob200.tsp"))),
    ];

    // Create algorithms
    let algorithms: Vec<Box<dyn TspAlgorithm>> = vec![
        Box::new(NearestNeighbor),
        Box::new(GreedyCycle),
        Box::new(RegretCycle::new()),
        Box::new(WeightedRegretCycle::default()),
    ];
    
    // Run experiments for each instance
    for (name, instance_result) in instances.iter() {
        println!("\nResults for {}:", name);
        println!("| Algorithm | Cost (min - max) | Time (ms) |");
        println!("|-----------|------------------|-----------|");

        match instance_result {
            Ok(instance) => {
                for algorithm in &algorithms {
                    let stats = run_experiment(&**algorithm, instance, 100);
                    println!("{}", format_stats_row(&stats));
                }
            }
            Err(e) => println!("Error loading {}: {}", name, e),
        }
    }
}
