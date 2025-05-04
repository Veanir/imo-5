use crate::algorithm::{ProgressCallback, TspAlgorithm};
use crate::tsplib::{Solution, TsplibInstance, CycleId};
use crate::algorithms::local_search::base::LocalSearch;
// use crate::utils::generate_random_solution; // unused
use crate::algorithms::perturbation::repair;
use rand::{Rng, thread_rng};
use std::collections::HashSet;
use std::time::{Duration, Instant};

pub struct Hae {
    base_local_search: LocalSearch,
    pop_size: usize,
    min_diff: i32,
    with_local: bool,
    name_str: String,
}

impl Hae {
    pub fn new(
        base_local_search: LocalSearch,
        pop_size: usize,
        min_diff: i32,
        with_local: bool,
    ) -> Self {
        let variant = if with_local { "HAE+LS" } else { "HAE" };
        let name_str = format!(
            "{} (Base: {}, pop={}, min_diff={})",
            variant,
            base_local_search.name(),
            pop_size,
            min_diff
        );
        Self {
            base_local_search,
            pop_size,
            min_diff,
            with_local,
            name_str,
        }
    }

    pub fn name(&self) -> &str {
        &self.name_str
    }

    pub fn solve_timed(
        &self,
        instance: &TsplibInstance,
        time_limit: Duration,
        mut progress_callback: ProgressCallback,
    ) -> (Solution, usize) {
        let mut rng = thread_rng();
        let start_time = Instant::now();

        // 1. Generate initial population
        let mut pop: Vec<(Solution, i32)> = Vec::with_capacity(self.pop_size);
        for i in 0..self.pop_size {
            progress_callback(format!("[Init {}] Generating initial LS", i + 1));
            let sol = self
                .base_local_search
                .solve_with_feedback(instance, &mut |s| {
                    progress_callback(format!("[Init LS {}] {}", i + 1, s))
                });
            let cost = sol.calculate_cost(instance);
            pop.push((sol, cost));
        }

        // Determine initial best
        let mut best_idx = 0;
        let mut best_cost = pop[0].1;
        for (idx, (_, cost)) in pop.iter().enumerate().skip(1) {
            if *cost < best_cost {
                best_idx = idx;
                best_cost = *cost;
            }
        }
        let mut best_sol = pop[best_idx].0.clone();

        let mut iterations = 0;
        while start_time.elapsed() < time_limit {
            iterations += 1;

            // Select two distinct parents uniformly
            let i1 = rng.gen_range(0..self.pop_size);
            let mut i2 = rng.gen_range(0..self.pop_size);
            while i2 == i1 {
                i2 = rng.gen_range(0..self.pop_size);
            }
            let parent1 = &pop[i1].0;
            let parent2 = &pop[i2].0;

            // Recombination
            let mut child = self.recombine(parent1, parent2, instance, &mut rng);

            // Optional local search after recombination
            if self.with_local {
                child = self
                    .base_local_search
                    .solve_with_feedback(instance, &mut |s| {
                        progress_callback(format!("[Iter {}] LS on child: {}", iterations, s))
                    });
            }

            let child_cost = child.calculate_cost(instance);
            progress_callback(format!("[Iter {}] Child cost: {}", iterations, child_cost));

            // Check similarity
            let too_similar = pop.iter().any(|(_, cost)| (child_cost - *cost).abs() < self.min_diff);

            // Find worst solution index
            let mut worst_idx = 0;
            let mut worst_cost = pop[0].1;
            for (idx, (_, cost)) in pop.iter().enumerate().skip(1) {
                if *cost > worst_cost {
                    worst_idx = idx;
                    worst_cost = *cost;
                }
            }

            // Replacement
            if child_cost < best_cost {
                // replace worst
                pop[worst_idx] = (child.clone(), child_cost);
                best_cost = child_cost;
                best_sol = child;
                progress_callback(format!("[Iter {}] New global best: {}", iterations, best_cost));
            } else if child_cost < worst_cost && !too_similar {
                pop[worst_idx] = (child, child_cost);
                progress_callback(format!("[Iter {}] Replaced worst: idx={}, cost={}", iterations, worst_idx, child_cost));
            }
        }

        (best_sol, iterations)
    }

    fn recombine<R: Rng + ?Sized>(
        &self,
        p1: &Solution,
        p2: &Solution,
        instance: &TsplibInstance,
        rng: &mut R,
    ) -> Solution {
        // Start from parent1
        let mut child = p1.clone();
        let mut destroyed: HashSet<usize> = HashSet::new();

        // Remove edges not in parent2
        for &cycle_id in &[CycleId::Cycle1, CycleId::Cycle2] {
            let cycle = child.get_cycle(cycle_id);
            let n = cycle.len();
            for i in 0..n {
                let a = cycle[i];
                let b = cycle[(i + 1) % n];
                if p2.has_edge(a, b).is_none() {
                    destroyed.insert(a);
                    destroyed.insert(b);
                }
            }
        }

        // Optional random deletion for diversification (20% probability)
        for &node in child
            .cycle1
            .iter()
            .chain(child.cycle2.iter())
        {
            if rng.gen_bool(0.2) {
                destroyed.insert(node);
            }
        }

        // Remove destroyed nodes
        child.cycle1.retain(|v| !destroyed.contains(v));
        child.cycle2.retain(|v| !destroyed.contains(v));

        // Repair using regret insertion
        repair(&mut child, instance, destroyed);

        child
    }
} 