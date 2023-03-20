//! Should work for 2D and 3D
//! 
//! 
//! 

use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};

use map::Map;
use pos::Position;

mod map;
mod pos;

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    position: Position,
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

/// A function to estimate the cost to reach the goal from node n.
type HeuristicFunc = fn(Position, Position) -> i32;

struct AStar {
    frontier: BinaryHeap<State>,
    came_from: HashMap<Position, Position>,
    gScore: HashMap<Position, i32>,
    fScore: HashMap<Position, i32>,
    heuristic_fn: HeuristicFunc,
}

impl AStar {
    // pub fn new(heuristic: HeuristicFunc) -> Self {
    //     Data { 
    //         heuristic_fn: heuristic,
    //         ..Default::default()
    //     }
    // }

    /// Traverse through the "path" from end to start and produce
    fn construct_path(history: &HashMap<Position, Position>, current: Position, start: Position) -> Vec<Position> {
        // We are done here and we should return the path.
        let mut path = vec![current];
        let mut current = current;
        while current != start {
            // Walk back through where we came from.
            current = history[&current];
            path.push(current);
        }
        path.reverse();
        return path;
    }

    pub fn search(map: &Map, start: Position, goal: Position, heuristic: HeuristicFunc) -> Option<Vec<Position>> {
        // 1. Add start to open set
        let mut frontier = BinaryHeap::new();
        frontier.push(State { position: start, cost: 0});
    
        // Tracks which tile
        let mut came_from = HashMap::new();
        came_from.insert(start, start);
    
        // gScore: Current cheapest path from start to node n, as far as we know.
        let mut g_score = HashMap::new();
        g_score.insert(start, 0);
    
        // fScore[n] = gScore[n] + h[n].
        // This is our current best guess as to how cheap the path would be if we walk through node n.
        let mut f_score = HashMap::new();
        f_score.insert(start, heuristic(start, goal));
    
        // while openSet is not empty
        while let Some(State { position: current_position, cost: _ }) = frontier.pop() {
            if current_position == goal { return Some(AStar::construct_path(&came_from, current_position, start)); }
            
            // Foreach traversable neighbour
            for (neighbour_position, neigbour_cost) in map.get_valid_neighbours(current_position) {
                // Check the cost of it
                // tentative gscore of neighbour
                let tentative_score = g_score[&current_position] + neigbour_cost;
                // If the g score does not exist yet, it should be considered infinity
                let should_consider_neighbour = if g_score.contains_key(&neighbour_position) {
                    tentative_score < g_score[&neighbour_position]
                } else { true };

                if should_consider_neighbour {
                    // This path to neighbour is better than any previous one. Record it!
                    came_from.insert(neighbour_position, current_position);
                    g_score.insert(neighbour_position, tentative_score);
                    f_score.insert(neighbour_position, tentative_score + heuristic(neighbour_position, goal));
                    // If the frontier does not already contain the node, add it.
                    if !frontier.iter().any(| e | e.position == neighbour_position) {
                        frontier.push(State { position: neighbour_position, cost: f_score[&neighbour_position] });
                    }
                }
            }
        }
    
        // Unable to find a valid path.
        return None;
    }

}




// /// From the current position of index, which are the valid neighbours to include?
// fn get_valid_neighbours(position: Position) -> Vec<(Position, i32)> {
   
// }

fn get_cost(index: usize) {

}

fn simple(current_position: Position, goal: Position) -> i32 {
    let delta_x = (current_position.x - goal.x).abs();
    let delta_y = (current_position.y - goal.y).abs();

    delta_x + delta_y
}

fn print_map(map: &Map, path: Vec<Position>, start: Position, goal: Position) {
    let mut output = "".to_owned();
    for y in 0..map.height {
        for x in 0..map.width {
            let current_position = Position { x: x as i32, y: y as i32};
            output += if current_position == start {
                "⊕"
            } else if current_position == goal {
                "◎"
            } else if path.contains(&current_position) {
                "◆"
            } else {
                "▫"
            }
        }
        output += "\n";
    }

    print!("{}", output);
}



#[cfg(test)]
mod tests {
    // Create a new map of 16 x 16 then search through it

    use crate::{AStar, pos::Position, map::Map, simple, print_map};

    // ===================
    // | S . . . . . . . |
    // | . . . . . . . . |
    // | . . . . . . . . |
    // | . . . . . . . . |
    // | . . . . . . . . |
    // | . . . . . . . . |
    // | . . . . . . . . |
    // | . . . . . . . F |
    // ===================
    
    #[test]
    fn testing() {
        let map: Map = Map::new(16, 16);
        let start = Position { x: 0, y: 0 };
        let goal = Position { x: 15, y: 15 };
        let result = AStar::search(&map, start, goal, simple);

        assert!(result.is_some());
        if let Some(path) = result {
            assert!(path.len() > 0);

            print_map(&map, path, start, goal);

            // print!("Path: ");
            // for Position { x, y } in path {
            //     print!("{}:{} -> ", x, y);
            // }
        }
    }


    // Test paths with scores


}
