use std::ops::{RangeBounds, Range};

use bevy::prelude::{Resource, Vec3};
use bevy_turborand::{rng::Rng, TurboRand};
use bracket_pathfinding::prelude::{Algorithm2D, Point, BaseMap};

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
    pub types: Option<Vec<TileType>>
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

    /// Returns the index of a random point on the map.
    #[allow(dead_code)]
    pub(crate) fn rand_point(&self, rng: &mut Rng, walkable: bool) -> MapIndex {
        // TODO: Instead of bruteforcing until we find a valid tile,
        // we should isntead filter for all walkable tiles,
        // then grab a random one of those
        loop {
            let index = rng.usize(0..self.tiles.len());
            if walkable {
                if self.tiles[index].is_walkable() {
                    break index.into();
                }
            } else {
                break index.into();
            }
        }
    }


    /// Queries for a collection of tiles from the map.
    pub(crate) fn query_tiles(&self, query: &TileQuery) -> Vec<(usize, &TileType)> {

        self.tiles
            .iter()
            .enumerate()
            // Filter by range
            .filter(| (i, _) | {
                if let Some((distance, origin)) = query.distance {
                    self.get_pathing_distance(*i, origin) < distance
                } else {
                    true
                }
            })
            // Filter by walkable
            .filter(| (_, e)| {
                if let Some(walkable) = query.walkable {
                    e.is_walkable() == walkable
                } else {
                    true
                }
            })
            .filter(|(i, _) | {
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

    /// Returns a random tile within a range of the original
    pub(crate) fn rand_with_query(
        &self,
        rng: &mut Rng,
        query: &TileQuery
    ) -> Option<MapIndex> {

        let result = self.query_tiles(query);

        // let half_radius = (radius as f32 / 2.0) as i32;
        // let origin_point = self.index_to_point2d(origin.0);
        // let x_bounds = (origin_point.x - half_radius)..(origin_point.x + half_radius).clamp(0, self.settings.width - 1);
        // let y_bounds = (origin_point.y - half_radius)..(origin_point.y + half_radius).clamp(0, self.settings.height - 1);
        
        // // TODO: More complex filters
        // let indexes = self.get_tiles_of_type(TileType::Grass);
        // let in_range: Vec<&usize> = indexes.iter().filter(|e| self.index_in_bounds(**e, &x_bounds, &y_bounds)).collect();

        if result.len() == 0 {
            return None;
        } else {
            // Grab a random from the list
            Some(rng.usize(0..result.len()).into())
        }
    }

    /// Returns true if the point exist on the map.
    #[allow(dead_code)]
    pub(crate) fn point_exist(&self, point: Point) -> bool {
        (0..self.settings.width).contains(&point.x) && (0..self.settings.height).contains(&point.y)
    }

    /// Returns true if the index exists on the map.
    #[allow(dead_code)]
    pub(crate) fn index_exist(&self, index: usize) -> bool {
        (0..self.tiles.len()).contains(&index)
    }

    pub(crate) fn index_in_bounds(&self, index: usize, x_bounds: &Range<i32>, y_bounds: &Range<i32>) -> bool {
        let point = self.index_to_point2d(index);
        x_bounds.contains(&point.x) && y_bounds.contains(&point.y)
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

            let rand_point = map.rand_with_query(rng.get_mut(), &query);
            assert_eq!(map.index_exist(rand_point.unwrap().0), true);
        }
    }
}
