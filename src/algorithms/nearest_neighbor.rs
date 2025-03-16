use crate::tsplib::{TsplibInstance, Solution};
use crate::algorithm::TspAlgorithm;

/*
Algorithm: Modified Nearest Neighbor for Two Cycles
------------------------------------------------
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
   This ensures fair distribution and implicit alternation

3. Build First Cycle (C1):
   - Start with s1
   - While |C1| < ⌈n/2⌉ and A1 not empty:
     * Find vertex v in A1 closest to last vertex in C1
     * Add v to C1
     * Remove v from A1

4. Build Second Cycle (C2):
   - Start with s2
   - While |C2| < ⌊n/2⌋ and A2 not empty:
     * Find vertex v in A2 closest to last vertex in C2
     * Add v to C2
     * Remove v from A2

5. Return (C1, C2)

Properties:
- C1 and C2 are disjoint
- |C1| = ⌈n/2⌉, |C2| = ⌊n/2⌋
- Each vertex is used exactly once
- Starting points are maximally distant
- Vertices are distributed evenly between cycles
*/

pub struct NearestNeighbor;

impl NearestNeighbor {
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

    fn build_cycle(
        start: usize,
        mut available: Vec<usize>,
        target_size: usize,
        instance: &TsplibInstance,
    ) -> Vec<usize> {
        let mut cycle = vec![start];
        
        while cycle.len() < target_size && !available.is_empty() {
            let last = cycle.last().unwrap();
            let nearest = NearestNeighbor::find_nearest(&NearestNeighbor, *last, &available, instance);
            cycle.push(nearest);
            available.retain(|&x| x != nearest);
        }
        
        cycle
    }
}

impl TspAlgorithm for NearestNeighbor {
    fn name(&self) -> &str {
        "Nearest Neighbor"
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
        let cycle1 = Self::build_cycle(start1, available1, (n + 1) / 2, instance);
        let cycle2 = Self::build_cycle(start2, available2, n / 2, instance);

        Solution::new(cycle1, cycle2)
    }
} 