use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs;

// Copy of Dijkstra's algorithm
// from https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn lowest_risk_path_risk(input: &[Vec<u32>]) -> u32 {
    let mut dists: Vec<Vec<_>> = (0..input.len())
        .map(|_| (0..input[0].len()).map(|_| u32::MAX).collect())
        .collect();
    let mut heap = BinaryHeap::new();
    let start = (0, 0);
    dists[start.0][start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == (input.len() - 1, input[0].len() - 1) {
            return cost;
        }

        // Important as we may have already found a better way
        if cost > dists[position.0][position.1] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let mut neighbors = Vec::new();
        if position.0 > 0 {
            neighbors.push((position.0 - 1, position.1));
        }
        if position.0 < input.len() - 1 {
            neighbors.push((position.0 + 1, position.1));
        }
        if position.1 > 0 {
            neighbors.push((position.0, position.1 - 1));
        }
        if position.1 < input[0].len() - 1 {
            neighbors.push((position.0, position.1 + 1));
        }
        for new_p in neighbors.iter() {
            let next = State {
                cost: cost + input[new_p.0][new_p.1],
                position: *new_p,
            };

            // If so, add it to the frontier and continue
            if next.cost < dists[next.position.0][next.position.1] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dists[next.position.0][next.position.1] = next.cost;
            }
        }
    }
    0
}

fn enlarge(input: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut new: Vec<Vec<u32>> = (0..input.len() * 5)
        .map(|_| (0..input[0].len() * 5).map(|_| 0).collect())
        .collect();
    for (i, row) in new.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            let increase = ((i / input.len()) + (j / input.len())) as u32;
            *val = input[i % input.len()][j % input[0].len()] + increase;
            if *val > 9 {
                *val = *val % 10 + 1;
            }
        }
    }
    new
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str
        .trim()
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("{}", lowest_risk_path_risk(&input));
    println!("{}", lowest_risk_path_risk(&enlarge(&input)));
}
