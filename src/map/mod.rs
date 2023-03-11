use bevy::prelude::{Resource, Vec3};
use bevy_turborand::{rng::Rng, TurboRand};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, Point, SmallVec};

use self::{
    plugin::MapSettings,
    tiles::{pos_to_world, MapIndex, TileType},
};

pub(crate) mod pathfinding;
pub(crate) mod plugin;
pub(crate) mod tiles;

#[derive(Default)]
pub(crate) struct TileQuery {
    pub walkable: Option<bool>,
    pub growable: Option<bool>,
    pub distance: Option<(f32, usize)>,
    pub exclude: Option<Vec<usize>>,
    pub types: Option<Vec<TileType>>,
}

#[derive(Resource)]
pub(crate) struct Map {
    pub(crate) tiles: Vec<TileType>,
    pub(crate) settings: MapSettings,
}

impl Map {
    pub(crate) fn index_to_world(&self, index: MapIndex) -> Vec3 {
        pos_to_world(self.index_to_point2d(index.into()).into(), &self.settings)
    }

    /// Queries for a collection of tiles from the map.
    pub(crate) fn query(&self, query: &TileQuery) -> Vec<(usize, &TileType)> {
        self.tiles
            .iter()
            .enumerate()
            // Filter by range
            .filter(|(i, _)| {
                if let Some((distance, origin)) = query.distance {
                    self.get_pathing_distance(*i, origin) < distance
                } else {
                    true
                }
            })
            // Filter by walkable
            .filter(|(_, e)| {
                if let Some(walkable) = query.walkable {
                    e.is_walkable() == walkable
                } else {
                    true
                }
            })
            .filter(|(_, e)| {
                if let Some(growable) = query.growable {
                    e.is_growable() == growable
                } else {
                    true
                }
            })
            .filter(|(i, _)| {
                if let Some(excludes) = &query.exclude {
                    !excludes.contains(i)
                } else {
                    true
                }
            })
            .filter(|(_, e)| {
                if let Some(types) = &query.types {
                    types.contains(e)
                } else {
                    true
                }
            })
            .collect()
    }

    pub(crate) fn query_neighbours(
        &self,
        index: usize,
        query: &TileQuery,
    ) -> SmallVec<[usize; 10]> {
        let neighbours = self.get_neighbours(index);
        let mut result = SmallVec::new();
        for index in neighbours {
            let mut include = true;
            // Filter tile
            if let Some(walkable) = query.walkable {
                include = self.tiles[index].is_walkable() == walkable;
            }
            if let Some(growable) = query.growable {
                include = self.tiles[index].is_growable() == growable;
            }
            if let Some(types) = &query.types {
                include = types.contains(&self.tiles[index]);
            }

            if include {
                result.push(index);
            }
        }

        result
    }

    /// Returns a random tile from the query result
    pub(crate) fn rand_from_query(&self, rng: &mut Rng, query: &TileQuery) -> Option<MapIndex> {
        let result = self.query(query);

        if result.is_empty() {
            None
        } else {
            // Grab a random from the list
            let (index, _) = result[rng.usize(0..result.len())];
            Some(index.into())
        }
    }

    /// Returns true if the index exists on the map.
    #[allow(dead_code)]
    pub(crate) fn index_exist(&self, index: usize) -> bool {
        (0..self.tiles.len()).contains(&index)
    }

    pub(crate) fn get_neighbours(&self, index: usize) -> SmallVec<[usize; 10]> {
        let mut neighbours = SmallVec::new();
        let location = self.index_to_point2d(index);

        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(-1, 0)) {
            neighbours.push(neighbour_index);
        }
        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(1, 0)) {
            neighbours.push(neighbour_index);
        }
        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(0, -1)) {
            neighbours.push(neighbour_index);
        }
        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(0, 1)) {
            neighbours.push(neighbour_index);
        }
        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(-1, -1)) {
            neighbours.push(neighbour_index);
        }
        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(-1, 1)) {
            neighbours.push(neighbour_index);
        }
        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(1, -1)) {
            neighbours.push(neighbour_index);
        }
        if let Some(neighbour_index) = self.valid_neighbour(location, Point::new(1, 1)) {
            neighbours.push(neighbour_index);
        }

        neighbours
    }

    fn valid_neighbour(&self, location: Point, delta: Point) -> Option<usize> {
        let new_point = location + delta;
        if self.in_bounds(new_point) {
            Some(self.point2d_to_index(new_point))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::default;
    use bevy_turborand::{DelegatedRng, GlobalRng};

    use crate::map::TileQuery;

    use super::{
        plugin::{generate_map, MapSettings},
        tiles::TileType,
        Map,
    };

    const SETTINGS: MapSettings = MapSettings {
        width: 8,
        height: 8,
        tile_size: 1.0,
    };

    #[test]
    fn no_invalid_random_points() {
        let mut rng = GlobalRng::new();

        let map = generate_map(&SETTINGS, &mut rng);

        for n in 0..8 * 8 {
            let query = TileQuery {
                distance: Some((5.0, n)),
                exclude: Some(vec![n]),
                ..default()
            };

            let rand_point = map.rand_from_query(rng.get_mut(), &query);
            assert!(map.index_exist(rand_point.unwrap().0));
        }
    }

    /// Neighbours should not include self.
    /// And should only get adjacent tiles of distance 1 from the input.
    #[test]
    fn neighbour_corners() {
        // The map has the following indexes:
        // 0 1 2
        // 3 4 5
        // 6 7 8

        let settings = MapSettings {
            width: 3,
            height: 3,
            tile_size: 1.0,
        };

        let map = Map {
            tiles: vec![TileType::Grass; settings.width as usize * settings.height as usize],
            settings,
        };

        let top_left = map.get_neighbours(0);
        assert!(!top_left.contains(&0));
        assert!(top_left.contains(&1));
        assert!(!top_left.contains(&2));
        assert!(top_left.contains(&3));
        assert!(top_left.contains(&4));
        assert!(!top_left.contains(&5));
        assert!(!top_left.contains(&6));
        assert!(!top_left.contains(&7));
        assert!(!top_left.contains(&8));

        let top = map.get_neighbours(1);
        assert!(top.contains(&0));
        assert!(!top.contains(&1));
        assert!(top.contains(&2));
        assert!(top.contains(&3));
        assert!(top.contains(&4));
        assert!(top.contains(&5));
        assert!(!top.contains(&6));
        assert!(!top.contains(&7));
        assert!(!top.contains(&8));

        let top_right = map.get_neighbours(2);
        assert!(!top_right.contains(&0));
        assert!(top_right.contains(&1));
        assert!(!top_right.contains(&2));
        assert!(!top_right.contains(&3));
        assert!(top_right.contains(&4));
        assert!(top_right.contains(&5));
        assert!(!top_right.contains(&6));
        assert!(!top_right.contains(&7));
        assert!(!top_right.contains(&8));

        let left = map.get_neighbours(3);
        assert!(left.contains(&0));
        assert!(left.contains(&1));
        assert!(!left.contains(&2));
        assert!(!left.contains(&3));
        assert!(left.contains(&4));
        assert!(!left.contains(&5));
        assert!(left.contains(&6));
        assert!(left.contains(&7));
        assert!(!left.contains(&8));

        let middle = map.get_neighbours(4);
        assert!(middle.contains(&0));
        assert!(middle.contains(&1));
        assert!(middle.contains(&2));
        assert!(middle.contains(&3));
        assert!(!middle.contains(&4));
        assert!(middle.contains(&5));
        assert!(middle.contains(&6));
        assert!(middle.contains(&7));
        assert!(middle.contains(&8));

        let right = map.get_neighbours(5);
        assert!(!right.contains(&0));
        assert!(right.contains(&1));
        assert!(right.contains(&2));
        assert!(!right.contains(&3));
        assert!(right.contains(&4));
        assert!(!right.contains(&5));
        assert!(!right.contains(&6));
        assert!(right.contains(&7));
        assert!(right.contains(&8));

        let bottom_left = map.get_neighbours(6);
        assert!(!bottom_left.contains(&0));
        assert!(!bottom_left.contains(&1));
        assert!(!bottom_left.contains(&2));
        assert!(bottom_left.contains(&3));
        assert!(bottom_left.contains(&4));
        assert!(!bottom_left.contains(&5));
        assert!(!bottom_left.contains(&6));
        assert!(bottom_left.contains(&7));
        assert!(!bottom_left.contains(&8));

        let bottom = map.get_neighbours(7);
        assert!(!bottom.contains(&0));
        assert!(!bottom.contains(&1));
        assert!(!bottom.contains(&2));
        assert!(bottom.contains(&3));
        assert!(bottom.contains(&4));
        assert!(bottom.contains(&5));
        assert!(bottom.contains(&6));
        assert!(!bottom.contains(&7));
        assert!(bottom.contains(&8));

        let bottom_right = map.get_neighbours(8);
        assert!(!bottom_right.contains(&0));
        assert!(!bottom_right.contains(&1));
        assert!(!bottom_right.contains(&2));
        assert!(!bottom_right.contains(&3));
        assert!(bottom_right.contains(&4));
        assert!(bottom_right.contains(&5));
        assert!(!bottom_right.contains(&6));
        assert!(bottom_right.contains(&7));
        assert!(!bottom_right.contains(&8));
    }

    #[test]
    fn neighbour_query_filters_default() {
        // The map has the following indexes:
        // 0 1 2
        // 3 4 5
        // 6 7 8

        let settings = MapSettings {
            width: 3,
            height: 3,
            tile_size: 1.0,
        };

        let map = Map {
            tiles: vec![
                TileType::Grass,
                TileType::Grass,
                TileType::Grass,
                TileType::Sand,
                TileType::Rock,
                TileType::DeepWater,
                TileType::ShallowWater,
                TileType::ShallowWater,
                TileType::DeepWater,
            ],
            settings,
        };

        let result = map.query_neighbours(4, &TileQuery::default());
        // Should fetch all neighbours except self.
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(!result.contains(&4));
        assert!(result.contains(&5));
        assert!(result.contains(&6));
        assert!(result.contains(&7));
        assert!(result.contains(&8));
    }
}
