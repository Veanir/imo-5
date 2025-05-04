use crate::algorithm::{ProgressCallback, TspAlgorithm};
use crate::algorithms::local_search::base::LocalSearch;
use crate::tsplib::{Solution, TsplibInstance};
use crate::utils::generate_random_solution;
use std::time::Instant;

pub struct Msls {
    base_local_search: LocalSearch,
    iterations: usize,
    name_str: String,
}

impl Msls {
    pub fn new(base_local_search: LocalSearch, iterations: usize) -> Self {
        let name_str = format!(
            "MSLS (Base: {}, Iterations: {})",
            base_local_search.name(),
            iterations
        );
        // Ensure the base LS starts randomly for MSLS
        // We might need to adjust LocalSearch or clone+modify it here if it doesn't always start randomly.
        // Assuming the passed base_local_search is configured for random starts.
        Self {
            base_local_search,
            iterations,
            name_str,
        }
    }
}

impl TspAlgorithm for Msls {
    fn name(&self) -> &str {
        &self.name_str
    }

    fn solve_with_feedback(
        &self,
        instance: &TsplibInstance,
        progress_callback: ProgressCallback,
    ) -> Solution {
        let mut best_solution: Option<Solution> = None;
        let mut best_cost = i32::MAX;

        let start_time = Instant::now();

        for i in 0..self.iterations {
            // Generate a new random solution for each iteration
            // Note: The base LocalSearch also generates an initial solution.
            // We rely on the base_local_search being configured with InitialSolutionType::Random.
            // If not, we would need to generate random here and pass it to a modified LS::solve.

            let iter_start_time = Instant::now();
            let mut iter_callback = |status: String| {
                progress_callback(format!(
                    "[MSLS Iter {}/{}] BaseLS: {}",
                    i + 1,
                    self.iterations,
                    status
                ));
            };

            // Run the base local search
            let current_solution = self
                .base_local_search
                .solve_with_feedback(instance, &mut iter_callback);

            let current_cost = current_solution.calculate_cost(instance);
            let iter_elapsed = iter_start_time.elapsed();

            progress_callback(format!(
                "[MSLS Iter {}/{}] Finished. Cost: {}, Time: {:?}. Best: {}",
                i + 1,
                self.iterations,
                current_cost,
                iter_elapsed,
                best_cost
            ));

            if current_cost < best_cost {
                best_cost = current_cost;
                best_solution = Some(current_solution);
                progress_callback(format!(
                    "[MSLS Iter {}/{}] New best solution found: {}",
                    i + 1,
                    self.iterations,
                    best_cost
                ));
            }
        }

        let total_elapsed = start_time.elapsed();
        progress_callback(format!(
            "[MSLS Finished] Total time: {:?}, Best cost: {}",
            total_elapsed, best_cost
        ));

        best_solution.expect("MSLS should find at least one solution")
    }
}
