use crate::tsplib::{TsplibInstance, Solution};
use crate::algorithm::TspAlgorithm;
use rand::Rng;

pub struct GreedyCycle;

impl GreedyCycle {
    fn find_max_distance_pair(&self, instance: &TsplibInstance) -> (usize, usize) {
        let n = instance.size();
        (0..n)
            .flat_map(|i| ((i + 1)..n).map(move |j| (i, j)))
            .max_by_key(|&(i, j)| instance.distance(i, j))
            .unwrap_or((0, 1))
    }

    fn find_nearest(&self, from: usize, available: &[usize], instance: &TsplibInstance) -> usize {
        available
            .iter()
            .min_by_key(|&&vertex| instance.distance(from, vertex))
            .copied()
            .unwrap_or(available[0])
    }

    fn calculate_insertion_cost(
        &self,
        vertex: usize,
        pos: usize,
        cycle: &[usize],
        instance: &TsplibInstance,
    ) -> i32 {
        if cycle.is_empty() {
            return 0;
        }
        if cycle.len() == 1 {
            return instance.distance(cycle[0], vertex) * 2;
        }

        let prev = cycle[if pos == 0 { cycle.len() - 1 } else { pos - 1 }];
        let next = cycle[pos % cycle.len()];

        // Cost of new edges minus cost of removed edge
        instance.distance(prev, vertex) + 
        instance.distance(vertex, next) - 
        instance.distance(prev, next)
    }

    fn find_best_insertion(
        &self,
        vertex: usize,
        cycle: &[usize],
        instance: &TsplibInstance,
    ) -> (usize, i32) {
        if cycle.is_empty() {
            return (0, 0);
        }

        (0..=cycle.len())
            .map(|pos| (pos, self.calculate_insertion_cost(vertex, pos, cycle, instance)))
            .min_by_key(|&(_, cost)| cost)
            .unwrap()
    }

    fn build_cycle(
        &self,
        start: usize,
        mut available: Vec<usize>,
        target_size: usize,
        instance: &TsplibInstance,
    ) -> Vec<usize> {
        // Initialize cycle with start vertex
        let mut cycle = vec![start];

        // Find nearest vertex to start and add it
        if !available.is_empty() {
            let nearest = self.find_nearest(start, &available, instance);
            cycle.push(nearest);
            available.retain(|&x| x != nearest);
        }

        // Keep adding vertices until target size is reached
        while cycle.len() < target_size && !available.is_empty() {
            // Find best vertex and position to insert
            let (best_vertex, best_pos, _) = available.iter()
                .map(|&vertex| {
                    let (pos, cost) = self.find_best_insertion(vertex, &cycle, instance);
                    (vertex, pos, cost)
                })
                .min_by_key(|&(_, _, cost)| cost)
                .unwrap();

            // Insert vertex at best position
            cycle.insert(best_pos, best_vertex);
            available.retain(|&x| x != best_vertex);
        }

        cycle
    }
}

impl TspAlgorithm for GreedyCycle {
    fn name(&self) -> &str {
        "Greedy Cycle"
    }

    fn solve(&self, instance: &TsplibInstance) -> Solution {
        let n = instance.size();
        let (start1, start2) = self.find_max_distance_pair(instance);
        
        // Create two complementary sets of available vertices
        let mut vertices: Vec<usize> = (0..n).filter(|&x| x != start1 && x != start2).collect();
        let (available1, available2) = vertices.iter()
            .enumerate()
            .fold((Vec::new(), Vec::new()), |(mut odd, mut even), (idx, &v)| {
                if idx % 2 == 0 {
                    even.push(v);
                } else {
                    odd.push(v);
                }
                (odd, even)
            });

        // Build cycles with their respective available vertices
        let cycle1 = self.build_cycle(start1, available1, (n + 1) / 2, instance);
        let cycle2 = self.build_cycle(start2, available2, n / 2, instance);

        Solution::new(cycle1, cycle2)
    }
}

/*
Algorithm: Modified Greedy Cycle for Two Cycles
--------------------------------------------
Input: Graph G with vertices V and distances D
Output: Two cycles (C1, C2) containing all vertices from V

1. Find Starting Points:
   - Find pair of vertices (s1, s2) with maximum distance between them
   - These will be starting points for cycles C1 and C2

2. Distribute Available Vertices:
   - Remove s1 and s2 from available vertices
   - Split remaining vertices into two groups:
     * A1 = vertices at even indices
     * A2 = vertices at odd indices

3. For each cycle Ci starting with si:
   a. Initialize cycle with si
   b. Find nearest vertex v to si in Ai
   c. Add v to cycle and remove from Ai
   d. While cycle not at target size and Ai not empty:
      - For each vertex v in Ai:
        * Try inserting v between each pair of consecutive vertices in cycle
        * Calculate cost: new_edges_cost - removed_edge_cost
      - Choose vertex and position with minimum insertion cost
      - Insert chosen vertex at chosen position
      - Remove vertex from Ai

4. Return (C1, C2)

Properties:
- Considers all possible insertion positions for each vertex
- Minimizes the cost increase at each step
- Maintains disjoint cycles of required sizes
- Starting points are maximally distant
*/ 