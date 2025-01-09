use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &HashMap<usize, Vec<(usize, usize)>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        if let Some(neighbors) = graph.get(&position) {
            for &(neighbor, weight) in neighbors {
                let next_cost = cost + weight;

                if next_cost < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
                    heap.push(State { cost: next_cost, position: neighbor });
                    dist.insert(neighbor, next_cost);
                }
            }
        }
    }

    None
}

fn main() {
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    graph.insert(0, vec![(1, 4), (2, 1)]);
    graph.insert(1, vec![(3, 1)]);
    graph.insert(2, vec![(1, 2), (3, 5)]);
    graph.insert(3, vec![]);

    let start = 0;
    let goal = 3;

    match dijkstra(&graph, start, goal) {
        Some(cost) => println!("Shortest path cost from {} to {} is {}", start, goal, cost),
        None => println!("No path from {} to {}", start, goal),
    }
}
