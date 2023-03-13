//! Should work for 2D and 3D
//! 
//! 
//! 

use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};


#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    index: usize,
    cost: i32
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

struct Data {
    /// Maps between the 1D index and the 2D position.
    positions: HashMap<usize, [i32; 2]>,
    open: Vec<usize>
}

/// A function to estimate the cost to reach the goal from node n.
type HeuristicFunc = fn(usize) -> f32;


pub fn astar_search_2d(start: usize, goal: usize, heuristic: HeuristicFunc) -> Option<Vec<usize>> {
    // 1. Add start to open set
    let mut frontier = BinaryHeap::new();
    frontier.push(State { index: start, cost: 0});

    // Tracks which tile
    let mut came_from = HashMap::new();
    came_from.insert(start, start);

    // gScore: Current cheapest path from start to node n, as far as we know.
    let mut gScore = HashMap::new();
    gScore.insert(start, 0.0);

    // fScore[n] = gScore[n] + h[n].
    // This is our current best guess as to how cheap the path would be if we walk through node n.
    // let mut fScore = HashMap::new();
    // fScore.insert(start, heuristic(start));

    // while openSet is not empty
    while let Some(State { index, cost }) = frontier.pop() {
        if index == goal {
            // We are done here and we should return the path.
            let mut path = vec![index];
            let mut current = index;
            while current != start {
                // Walk back through where we came from.
                current = came_from[&current];
                path.push(current);
            }
            path.reverse();
            return Some(path);
        }
        
        // Foreach traversable neighbour
        // Check the cost of it
        // 
    }
    return None;
}

/// From my current position
fn get_valid_neighbours(data: Data, index: usize) {
    

}

/// Traverse through the "path" from end to start and produce
fn construct_path() -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
}
