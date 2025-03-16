use std::time::Instant;
use crate::tsplib::{TsplibInstance, Solution};

// Trait that all TSP algorithms must implement
pub trait TspAlgorithm {
    fn name(&self) -> &str;
    fn solve(&self, instance: &TsplibInstance) -> Solution;
}

// Results of a single algorithm run
#[derive(Debug)]
pub struct RunResult {
    pub cost: i32,
    pub solution: Solution,
    pub time_ms: u128,
}

// Statistics for multiple runs
#[derive(Debug)]
pub struct ExperimentStats {
    pub algorithm_name: String,
    pub instance_name: String,
    pub min_cost: i32,
    pub max_cost: i32,
    pub avg_cost: f64,
    pub best_solution: Solution,
    pub avg_time_ms: f64,
    pub num_runs: usize,
}

// Run experiment multiple times and collect statistics
pub fn run_experiment(
    algorithm: &dyn TspAlgorithm,
    instance: &TsplibInstance,
    num_runs: usize,
) -> ExperimentStats {
    let mut results = Vec::with_capacity(num_runs);

    // Run the algorithm multiple times
    for _ in 0..num_runs {
        let start = Instant::now();
        let solution = algorithm.solve(instance);
        let elapsed = start.elapsed();

        // Validate solution
        assert!(solution.is_valid(instance), "Invalid solution produced by {}", algorithm.name());

        let result = RunResult {
            cost: solution.calculate_cost(instance),
            solution,
            time_ms: elapsed.as_millis(),
        };
        results.push(result);
    }

    // Calculate statistics
    let mut min_cost = i32::MAX;
    let mut max_cost = i32::MIN;
    let mut sum_cost = 0;
    let mut sum_time = 0;
    let mut best_solution = None;

    for result in &results {
        if result.cost < min_cost {
            min_cost = result.cost;
            best_solution = Some(result.solution.clone());
        }
        max_cost = max_cost.max(result.cost);
        sum_cost += result.cost;
        sum_time += result.time_ms;
    }

    ExperimentStats {
        algorithm_name: algorithm.name().to_string(),
        instance_name: instance.name.clone(),
        min_cost,
        max_cost,
        avg_cost: sum_cost as f64 / num_runs as f64,
        best_solution: best_solution.unwrap(),
        avg_time_ms: sum_time as f64 / num_runs as f64,
        num_runs,
    }
}

// Helper function to format experiment results as a table row
pub fn format_stats_row(stats: &ExperimentStats) -> String {
    format!(
        "| {} | {} ({} - {}) | {:.2} |",
        stats.algorithm_name,
        stats.avg_cost,
        stats.min_cost,
        stats.max_cost,
        stats.avg_time_ms
    )
} 