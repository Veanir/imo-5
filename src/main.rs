mod algorithm;
mod algorithms;
mod moves;
mod tsplib;
mod utils;
mod visualization;

use algorithm::{
    ExperimentStats, TimedSolveFn, TspAlgorithm, format_stats_row, run_experiment,
    run_timed_experiment,
};
use algorithms::constructive::weighted_regret_cycle::WeightedRegretCycle;
use algorithms::ils::Ils;
use algorithms::lns::Lns;
use algorithms::local_search::base::{
    HeuristicAlgorithm, InitialSolutionType, LocalSearch, NeighborhoodType, SearchVariant,
};
use algorithms::msls::Msls;
use algorithms::perturbation::{LargePerturbation, Perturbation, SmallPerturbation};
use algorithms::random_walk::RandomWalk;
use algorithms::hae::Hae;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::Arc; // Keep Arc for TsplibInstance if needed across threads, but not for algos here
use std::time::Duration;
use tsplib::TsplibInstance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading instances...");

    create_dir_all("output")?;

    // Define instances
    let instance_files = ["kroa200", "krob200"];
    let mut instances = HashMap::new();
    for name in instance_files {
        match TsplibInstance::from_file(Path::new(&format!("tsplib/{}.tsp", name))) {
            Ok(mut instance) => {
                println!("  Precomputing nearest neighbors (k=10) for {}...", name);
                instance.precompute_nearest_neighbors(10);
                instances.insert(name.to_string(), Arc::new(instance)); // Keep Arc for instance for potential // parallelism
            }
            Err(e) => println!("Error loading {}: {}", name, e),
        }
    }

    // Define base local search - No Arc needed here
    let base_ls = LocalSearch::new(
        SearchVariant::CandidateSteepest(10),
        NeighborhoodType::EdgeExchange,
        InitialSolutionType::Random,
    );

    // Define algorithms - Use clone(), no Arc needed
    let msls_iterations = 200; // As per lab spec
    let msls_algo = Msls::new(base_ls.clone(), msls_iterations);

    // Define perturbations - No Arc needed
    let small_perturb = SmallPerturbation::new(10); // Example: 10 random moves
    let large_perturb = LargePerturbation::new(0.2); // Example: 20% destroy

    let num_runs = 10; // As per lab spec
    let mut all_results: Vec<(String, ExperimentStats)> = Vec::new();
    let mut msls_avg_times: HashMap<String, Duration> = HashMap::new();

    for (name, instance) in &instances {
        println!("\nProcessing instance: {}", name);

        // --- Run MSLS first ---
        println!("  Running algorithm: {}", msls_algo.name());
        // Pass instance by reference, algo by reference
        let msls_stats = run_experiment(&msls_algo, instance, num_runs);
        let avg_time_ms = msls_stats.avg_time_ms;
        let time_limit = Duration::from_millis(avg_time_ms.round() as u64);
        msls_avg_times.insert(name.clone(), time_limit);
        println!(
            "    MSLS Avg Time: {:.2} ms. Setting Time Limit for ILS/LNS: {:?}",
            avg_time_ms, time_limit
        );
        all_results.push((name.clone(), msls_stats.clone()));
        // Plot best MSLS solution
        let safe_algo_name = msls_algo
            .name()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "_")
            .replace("__", "_");
        let output_path = format!("output/{}_{}.png", name, safe_algo_name);
        visualization::plot_solution(
            instance, // Pass the Arc<TsplibInstance>
            &msls_stats.best_solution,
            &format!("{} - {}", msls_algo.name(), name),
            Path::new(&output_path),
        )?;

        // --- Run ILS ---
        // Use clone for perturbation
        let ils_algo = Ils::new(base_ls.clone(), small_perturb.clone());
        println!("  Running algorithm: {}", ils_algo.name());
        // Define the timed solve function as a closure
        // Closure takes &Ils<SmallPerturbation>
        let ils_solve_fn: TimedSolveFn<Ils<SmallPerturbation>> =
            Box::new(|algo, inst, cb| algo.solve_timed(inst, time_limit, cb));
        let ils_stats = run_timed_experiment(
            &ils_algo, // Pass reference to the algorithm struct
            ils_solve_fn,
            instance, // Pass Arc<TsplibInstance>
            num_runs,
            ils_algo.name(), // Pass name explicitly
        );
        all_results.push((name.clone(), ils_stats.clone()));
        // Plot best ILS solution
        let safe_algo_name = ils_algo
            .name()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "_")
            .replace("__", "_");
        let output_path = format!("output/{}_{}.png", name, safe_algo_name);
        visualization::plot_solution(
            instance,
            &ils_stats.best_solution,
            &format!("{} - {}", ils_algo.name(), name),
            Path::new(&output_path),
        )?;

        // --- Run LNS ---
        // Use clone for perturbation
        let lns_algo = Lns::new(
            base_ls.clone(),
            large_perturb.clone(),
            true, // apply_ls_after_repair
            true, // apply_ls_to_initial
        );
        println!("  Running algorithm: {}", lns_algo.name());
        let lns_solve_fn: TimedSolveFn<Lns<LargePerturbation>> =
            Box::new(|algo, inst, cb| algo.solve_timed(inst, time_limit, cb));
        let lns_stats =
            run_timed_experiment(&lns_algo, lns_solve_fn, instance, num_runs, lns_algo.name());
        all_results.push((name.clone(), lns_stats.clone()));
        // Plot best LNS solution
        let safe_algo_name = lns_algo
            .name()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "_")
            .replace("__", "_");
        let output_path = format!("output/{}_{}.png", name, safe_algo_name);
        visualization::plot_solution(
            instance,
            &lns_stats.best_solution,
            &format!("{} - {}", lns_algo.name(), name),
            Path::new(&output_path),
        )?;

        // --- Run LNSa (LNS without LS after repair) ---
        // Use clone for perturbation
        let lnsa_algo = Lns::new(
            base_ls.clone(),
            large_perturb.clone(),
            false, // apply_ls_after_repair = false
            true,  // apply_ls_to_initial
        );
        println!("  Running algorithm: {}", lnsa_algo.name());
        let lnsa_solve_fn: TimedSolveFn<Lns<LargePerturbation>> =
            Box::new(|algo, inst, cb| algo.solve_timed(inst, time_limit, cb));
        let lnsa_stats = run_timed_experiment(
            &lnsa_algo,
            lnsa_solve_fn,
            instance,
            num_runs,
            lnsa_algo.name(),
        );
        all_results.push((name.clone(), lnsa_stats.clone()));
        // Plot best LNSa solution
        let safe_algo_name = lnsa_algo
            .name()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "_")
            .replace("__", "_");
        let output_path = format!("output/{}_{}.png", name, safe_algo_name);
        visualization::plot_solution(
            instance,
            &lnsa_stats.best_solution,
            &format!("{} - {}", lnsa_algo.name(), name),
            Path::new(&output_path),
        )?;
        // --- Run HAE ---
        let hae_algo = Hae::new(base_ls.clone(), 20, 40, true);
        println!("  Running algorithm: {}", hae_algo.name());
        let hae_solve_fn: TimedSolveFn<Hae> =
            Box::new(|algo, inst, cb| algo.solve_timed(inst, time_limit, cb));
        let hae_stats =
            run_timed_experiment(&hae_algo, hae_solve_fn, instance, num_runs, hae_algo.name());
        all_results.push((name.clone(), hae_stats.clone()));
        // Plot best HAE solution
        let safe_algo_name = hae_algo
            .name()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "_")
            .replace("__", "_");
        let output_path = format!("output/{}_{}.png", name, safe_algo_name);
        visualization::plot_solution(
            instance,
            &hae_stats.best_solution,
            &format!("{} - {}", hae_algo.name(), name),
            Path::new(&output_path),
        )?;
        // --- Run HAE (no LS) ---
        let hae_nols_algo = Hae::new(base_ls.clone(), 20, 40, false);
        println!("  Running algorithm: {}", hae_nols_algo.name());
        let hae_nols_solve_fn: TimedSolveFn<Hae> =
            Box::new(|algo, inst, cb| algo.solve_timed(inst, time_limit, cb));
        let hae_nols_stats = run_timed_experiment(
            &hae_nols_algo,
            hae_nols_solve_fn,
            instance,
            num_runs,
            hae_nols_algo.name(),
        );
        all_results.push((name.clone(), hae_nols_stats.clone()));
        // Plot best HAE (no LS) solution
        let safe_algo_name = hae_nols_algo
            .name()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "_")
            .replace("__", "_");
        let output_path = format!("output/{}_{}.png", name, safe_algo_name);
        visualization::plot_solution(
            instance,
            &hae_nols_stats.best_solution,
            &format!("{} - {}", hae_nols_algo.name(), name),
            Path::new(&output_path),
        )?;
    }

    println!("\nSummary of Results:");
    // Use updated format string from algorithm.rs
    println!(
        "| Instance | Algorithm                    | Cost (min - avg - max) | Time (ms, avg) | Iterations (avg) |"
    );
    println!(
        "|----------|------------------------------|------------------------|----------------|------------------|"
    );
    for (instance_name, stats) in all_results {
        // format_stats_row now handles padding
        println!(
            "| {} {}", // Removed extra spaces around {}
            instance_name,
            format_stats_row(&stats)
        );
    }

    println!("\nVisualizations have been saved to the 'output' directory.");
    Ok(())
}
