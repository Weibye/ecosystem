#[deny(missing_docs)]

// type Map = [i32; 2];

use crate::pos::Position;

/// Encodes a map that can be searched through 
pub struct Map {
    data: Vec<Position>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let mut map_data = vec![];
        for x in 0..width {
            for y in 0..height {
                map_data.push(Position::new(x as i32, y as i32));
            }
        }
        Self { 
            data: map_data,
            width,
            height,
        }
    }

    pub fn get_valid_neighbours(&self, source: Position) -> Vec<(Position, i32)> {
        // TODO: check if these are out of bounds.
        // TODO: Add cost of distance.

        let mut result = vec![];

        // Axial movement
        result.push((Position { x: source.x, y: source.y + 1}, 1)); // North
        result.push((Position { x: source.x + 1, y: source.y}, 1)); // East
        result.push((Position { x: source.x, y: source.y - 1}, 1)); // South
        result.push((Position { x: source.x - 1, y: source.y}, 1)); // West

        // Diagonal movement
        result.push((Position { x: source.x + 1, y: source.y + 1}, 1)); // NorthEast
        result.push((Position { x: source.x - 1, y: source.y + 1}, 1)); // NorthWest
        result.push((Position { x: source.x + 1, y: source.y - 1}, 1)); // SouthEast
        result.push((Position { x: source.x - 1, y: source.y - 1}, 1)); // SouthWest

        result
    }
}


pub trait Searchable {

}