use bevy::prelude::{Resource, Vec3};
use bevy_turborand::{rng::Rng, TurboRand};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap};

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
    pub(crate) fn query_tiles(&self, query: &TileQuery) -> Vec<(usize, &TileType)> {
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

    /// Returns a random tile from the query result
    pub(crate) fn rand_from_query(&self, rng: &mut Rng, query: &TileQuery) -> Option<MapIndex> {
        let result = self.query_tiles(query);

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
}

#[cfg(test)]
mod tests {
    use bevy_turborand::{DelegatedRng, GlobalRng};

    use crate::map::TileQuery;

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
            let query = TileQuery {
                walkable: None,
                distance: Some((5.0, n)),
                exclude: Some(vec![n]),
                types: None,
            };

            let rand_point = map.rand_from_query(rng.get_mut(), &query);
            assert_eq!(map.index_exist(rand_point.unwrap().0), true);
        }
    }
}
