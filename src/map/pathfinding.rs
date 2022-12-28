use bracket_pathfinding::prelude::{
    Algorithm2D, BaseMap, DistanceAlg::Pythagoras, Point, SmallVec,
};

use super::Map;

impl Map {
    fn valid_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let dest = location + delta;
        if self.in_bounds(dest) {
            let index = self.point2d_to_index(dest);
            if !self.is_opaque(index) {
                Some(index)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.settings.width, self.settings.height)
    }
}

impl BaseMap for Map {
    // Anything that is walls or completely blocking.
    fn is_opaque(&self, _idx: usize) -> bool {
        !self.tiles[_idx].is_walkable()
    }

    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(_idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

#[cfg(test)]
mod tests {
    use bevy_turborand::GlobalRng;
    use bracket_pathfinding::prelude::Point;

    use crate::map::plugin::{generate_map, MapSettings};

    #[test]
    fn out_of_bounds_should_be_none() {
        let settings = MapSettings {
            width: 16,
            height: 16,
            tile_size: 1.0,
        };

        let mut rng = GlobalRng::new();

        let map = generate_map(&settings, &mut rng);

        let result = map.valid_exit(Point { x: 0, y: 0 }, Point { x: -1, y: -1 });
        assert_eq!(result, None);
    }
}
