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

/// Defines the data a tile can contain.
pub(crate) struct TileData {
    pub(crate) tile_type: TileType,
    /// What is the base movement speed of this tile?
    ///
    /// This assumes only one form of movement.
    pub(crate) movement_speed: f32,
    /// How fast can flora grow on this tile?
    pub(crate) growability: f32,
    /// How wet is this tile?
    #[allow(dead_code)]
    moisture: f32,
    /// How much light is this tile receiving at this moment?
    #[allow(dead_code)]
    brightness: f32,
}

// TODO: Move this to a file or asset

const GRASS_DATA: TileData = TileData {
    tile_type: TileType::Grass,
    movement_speed: 1.0,
    growability: 1.0,
    moisture: 0.7,
    brightness: 0.0,
};
const SAND_DATA: TileData = TileData {
    tile_type: TileType::Sand,
    movement_speed: 0.9,
    growability: 0.8,
    moisture: 0.5,
    brightness: 0.0,
};
const ROCK_DATA: TileData = TileData {
    tile_type: TileType::Rock,
    movement_speed: 0.8,
    growability: 0.1,
    moisture: 0.0,
    brightness: 0.0,
};
const SHALLOW_WATER_DATA: TileData = TileData {
    tile_type: TileType::ShallowWater,
    movement_speed: 0.3,
    growability: 0.0,
    moisture: 1.0,
    brightness: 0.0,
};
const DEEP_WATER_DATA: TileData = TileData {
    tile_type: TileType::DeepWater,
    movement_speed: 0.0,
    growability: 0.0,
    moisture: 1.0,
    brightness: 0.0,
};

pub(crate) const fn get_data(tile_type: &TileType) -> TileData {
    match tile_type {
        TileType::Grass => GRASS_DATA,
        TileType::Sand => SAND_DATA,
        TileType::Rock => ROCK_DATA,
        TileType::ShallowWater => SHALLOW_WATER_DATA,
        TileType::DeepWater => DEEP_WATER_DATA,
    }
}

pub(crate) fn resolve_type(value: f64) -> TileType {
    if value > 0.9 {
        TileType::Rock
    } else if value > 0.2 {
        TileType::Grass
    } else if value > 0.0 {
        TileType::Sand
    } else if value > -0.3 {
        TileType::ShallowWater
    } else {
        TileType::DeepWater
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum TileType {
    Grass,
    Sand,
    Rock,
    ShallowWater,
    DeepWater,
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
        TileType::Sand => Color::rgb(0.55, 0.5, 0.3),
        TileType::Rock => Color::rgb(0.5, 0.5, 0.5),
        TileType::ShallowWater => Color::rgb(0.0, 0.4, 0.6),
        TileType::DeepWater => Color::rgb(0.0, 0.2, 0.7),
    }
}
