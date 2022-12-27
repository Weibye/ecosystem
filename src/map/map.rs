use bevy::prelude::{Resource, Vec3};

use bracket_pathfinding::prelude::{
    Algorithm2D, BaseMap, DistanceAlg::Pythagoras, Point, SmallVec,
};

use super::{
    plugin::MapSettings,
    tiles::{pos_to_world, TileType},
};

#[derive(Resource)]
pub(crate) struct Map {
    pub(crate) tiles: Vec<TileType>,
    pub(crate) settings: MapSettings,
}

impl Map {
    fn valid_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let dest = location + delta;
        let index = self.point2d_to_index(dest);
        if self.in_bounds(dest) && !self.is_opaque(index) {
            Some(index)
        } else {
            None
        }
    }

    pub(crate) fn index_to_world(&self, index: usize) -> Vec3 {
        pos_to_world(self.index_to_point2d(index).into(), &self.settings)
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
        self.tiles[_idx] == TileType::Lava
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
