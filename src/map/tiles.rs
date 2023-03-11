//! Collection of functionality tied to individual tiles.

use bevy::prelude::{Color, Component, Vec3};
use bracket_pathfinding::prelude::Algorithm2D;

use crate::utils::Vec2;

use super::{plugin::MapSettings, Map};

/// Marks where on the map an entitiy is located.
#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub(crate) struct MapIndex(pub usize);

impl From<usize> for MapIndex {
    fn from(value: usize) -> Self {
        MapIndex(value)
    }
}

impl From<MapIndex> for usize {
    fn from(value: MapIndex) -> Self {
        value.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum TileType {
    Grass,
    Dirt,
    Rock,
    Water,
    DeepWater,
}

impl TileType {
    pub(crate) fn is_walkable(&self) -> bool {
        !matches!(self, TileType::DeepWater)
    }

    pub(crate) fn is_growable(&self) -> bool {
        matches!(self, TileType::Grass | TileType::Dirt)
    }
}
/// Converts from a tile-position to a world-position.
pub(crate) fn pos_to_world(pos: Vec2<i32>, settings: &MapSettings) -> Vec3 {
    Vec3::new(
        pos.x as f32 * settings.tile_size - settings.width as f32 / 2.0,
        0.0,
        pos.y as f32 * settings.tile_size - settings.height as f32 / 2.0,
    )
}

/// Converts from world-position to tile-position.
pub(crate) fn world_to_pos(world_pos: &Vec3, settings: &MapSettings) -> Vec2<i32> {
    Vec2::new(
        ((world_pos.x + (settings.width as f32 / 2.0)) / settings.tile_size) as i32,
        ((world_pos.z + (settings.height as f32 / 2.0)) / settings.tile_size) as i32,
    )
}

pub(crate) fn world_to_index(world: &Vec3, map: &Map) -> MapIndex {
    let pos = world_to_pos(world, &map.settings);
    let index = map.point2d_to_index(pos.into());

    MapIndex(index)
}

/// Gets the corresponding material color for a `GroundType`.
/// TODO: Replace with actual textures and assets.
pub(crate) fn get_color(tile_type: TileType) -> Color {
    match tile_type {
        TileType::Grass => Color::rgb(0.1, 0.7, 0.25),
        TileType::Dirt => Color::rgb(0.55, 0.5, 0.3),
        TileType::Rock => Color::rgb(0.5, 0.5, 0.5),
        TileType::Water => Color::rgb(0.0, 0.4, 0.6),
        TileType::DeepWater => Color::rgb(0.0, 0.2, 0.7),
    }
}
