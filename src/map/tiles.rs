//! Collection of functionality tied to individual tiles.

use bevy::prelude::{Color, Component, Vec3};
use bevy_turborand::{rng::Rng, TurboRand};
use bracket_pathfinding::prelude::Point;

use crate::utils::Vec2;

use super::plugin::MapSettings;

/// Marks where on the map an entitiy is located.
#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub(crate) struct TilePos {
    pub(crate) pos: Vec2<i32>,
}

impl TilePos {
    /// Returns the world position equivalent of this tile position
    pub fn to_world(self, settings: &MapSettings) -> Vec3 {
        pos_to_world(self.pos, settings)
    }

    pub fn from_point(point: Point) -> Self {
        TilePos {
            pos: Vec2::new(point.x, point.y),
        }
    }

    pub fn from_vec2(vec: Vec2<i32>) -> Self {
        TilePos { pos: vec }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum TileType {
    Grass,
    Rock,
    Water,
    Lava,
}

/// Creates a random tile-position within the map.
pub(crate) fn create_rand_pos(rng: &mut Rng, settings: &MapSettings) -> TilePos {
    TilePos {
        pos: Vec2::new(rng.i32(0..settings.width), rng.i32(0..settings.height)),
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

/// Gets the corresponding material color for a `GroundType`.
/// TODO: Replace with actual textures and assets.
pub(crate) fn get_color(tile_type: TileType) -> Color {
    match tile_type {
        TileType::Grass => Color::rgb(0.1, 0.7, 0.25),
        TileType::Rock => Color::rgb(0.4, 0.45, 0.4),
        TileType::Water => Color::rgb(0.0, 0.4, 0.6),
        TileType::Lava => Color::rgb(1.0, 0.4, 0.0),
    }
}
