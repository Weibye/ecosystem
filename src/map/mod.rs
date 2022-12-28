use bevy::prelude::{Resource, Vec3};
use bevy_turborand::{rng::Rng, TurboRand};
use bracket_pathfinding::prelude::{Algorithm2D, Point};

use self::{
    plugin::MapSettings,
    tiles::{pos_to_world, TileType},
};

pub(crate) mod pathfinding;
pub(crate) mod plugin;
pub(crate) mod tiles;

#[derive(Resource)]
pub(crate) struct Map {
    pub(crate) tiles: Vec<TileType>,
    pub(crate) settings: MapSettings,
}

impl Map {
    pub(crate) fn index_to_world(&self, index: usize) -> Vec3 {
        pos_to_world(self.index_to_point2d(index).into(), &self.settings)
    }

    /// Returns the index of a random point on the map.
    #[allow(dead_code)]
    pub(crate) fn rand_point(&self, rng: Rng) -> usize {
        rng.usize(0..self.tiles.len())
    }

    pub(crate) fn rand_index_in_range(&self, rng: &mut Rng, origin: usize, radius: i32) -> usize {
        let origin_point = self.index_to_point2d(origin);

        let half_radius = (radius as f32 / 2.0) as i32;

        let new_point = Point {
            x: rng
                .i32((origin_point.x - half_radius)..(origin_point.x + half_radius))
                .clamp(0, self.settings.width - 1),
            y: rng
                .i32((origin_point.y - half_radius)..(origin_point.y + half_radius))
                .clamp(0, self.settings.width - 1),
        };

        self.point2d_to_index(new_point)
    }

    /// Returns true if the point exist on the map.
    #[allow(dead_code)]
    pub(crate) fn point_exist(&self, point: Point) -> bool {
        (0..self.settings.width).contains(&point.x) && (0..self.settings.height).contains(&point.y)
    }

    /// Returns true if the index exists on the map.
    pub(crate) fn index_exist(&self, index: usize) -> bool {
        (0..self.tiles.len()).contains(&index)
    }
}

#[cfg(test)]
mod tests {
    use bevy_turborand::{DelegatedRng, GlobalRng};

    use super::plugin::{generate_map, MapSettings};

    #[test]
    fn no_invalid_random_points() {
        let settings = MapSettings {
            width: 16,
            height: 16,
            tile_size: 1.0,
        };

        let mut rng = GlobalRng::new();

        let map = generate_map(&settings, &mut rng);

        for n in 0..16 * 16 {
            let rand_point = map.rand_index_in_range(rng.get_mut(), n, 5);
            assert_eq!(map.index_exist(rand_point), true);
        }
    }
}
