use bevy::prelude::{
    default, shape, Assets, Color, Commands, Component, Mesh, PbrBundle, Plugin, Res, ResMut,
    Resource, StandardMaterial, Transform, Vec3,
};
use bevy_mod_picking::PickableBundle;
use bevy_turborand::{rng::Rng, DelegatedRng, GlobalRng, TurboRand};

use crate::AppStage;

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct TileData {
    pub(crate) position: TilePosition,
    pub(crate) ground_type: GroundType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct TilePosition {
    x: i8,
    y: i8,
    height: i8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum GroundType {
    Grass,
    Rock,
    Water,
}

pub(crate) struct LandscapePlugin {
    pub(crate) tile_size: f32,
    pub(crate) map_size: (i8, i8),
}

impl Default for LandscapePlugin {
    fn default() -> Self {
        Self {
            tile_size: 1.0,
            map_size: (16, 16),
        }
    }
}

#[derive(Resource)]
pub(crate) struct TileSettings {
    pub(crate) tile_size: f32,
    pub(crate) height_layers: i8,
    pub(crate) map_size: (i8, i8),
}

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TileSettings {
            tile_size: self.tile_size,
            map_size: self.map_size,
            height_layers: 1,
        })
        .add_startup_system_to_stage(AppStage::SeedBoard, create_tiles);
    }
}

fn create_tiles(
    mut cmd: Commands,
    settings: Res<TileSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    // let mut logical_tiles: Vec<TileData> = Vec::new();

    for x in 0..settings.map_size.0 {
        for y in 0..settings.map_size.1 {
            let index = rng.get_mut().i32(0..=4);
            let ground_type = match index {
                0..=2 => GroundType::Grass,
                3 => GroundType::Rock,
                4 => GroundType::Water,
                _ => panic!("Out of range"),
            };

            let tile_pos = TilePosition { x, y, height: 0 };

            cmd.spawn((
                TileData {
                    position: tile_pos,
                    ground_type,
                },
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane {
                        size: settings.tile_size,
                    })),
                    material: materials.add(get_color(ground_type).into()),
                    transform: Transform::from_translation(pos_to_world(&tile_pos, &settings)),
                    ..default()
                },
                PickableBundle::default(),
            ));
        }
    }

    // // Spawn tile data
    // for e in &logical_tiles {
    //     cmd.spawn(*e);
    // }

    // // Spawn visual tiles
    // for e in &logical_tiles {
    //     cmd.spawn((
    //         PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Plane {
    //                 size: settings.tile_size,
    //             })),
    //             material: materials.add(get_color(e.ground_type).into()),
    //             transform: Transform::from_translation(pos_to_world(&e.position, &settings)),
    //             ..default()
    //         },
    //         PickableBundle::default(),
    //     ));
    // }
}

pub(crate) fn get_rand_pos(rng: &mut Rng, settings: &TileSettings) -> TilePosition {
    TilePosition {
        x: rng.i8(0..settings.map_size.0),
        y: rng.i8(0..settings.map_size.1),
        height: rng.i8(0..settings.height_layers),
    }
}

/// Converts from a tile-position to a world-position.
pub(crate) fn pos_to_world(pos: &TilePosition, settings: &TileSettings) -> Vec3 {
    Vec3::new(
        pos.x as f32 * settings.tile_size - settings.map_size.0 as f32 / 2.0,
        pos.height as f32,
        pos.y as f32 * settings.tile_size - settings.map_size.1 as f32 / 2.0,
    )
}

/// Converts from world-position to tile-position.
pub(crate) fn world_to_pos(pos: &Vec3, settings: &TileSettings) -> TilePosition {
    TilePosition {
        x: ((pos.x + (settings.map_size.0 as f32 / 2.0)) / settings.tile_size) as i8,
        y: ((pos.z + (settings.map_size.1 as f32 / 2.0)) / settings.tile_size) as i8,
        height: pos.y as i8,
    }
}

/// Gets the corresponding material color for a `GroundType`.
/// TODO: Replace with actual textures and assets.
fn get_color(ground_type: GroundType) -> Color {
    match ground_type {
        GroundType::Grass => Color::rgb(0.1, 0.7, 0.25),
        GroundType::Rock => Color::rgb(0.4, 0.45, 0.4),
        GroundType::Water => Color::rgb(0.0, 0.4, 0.6),
    }
}

#[cfg(test)]
mod tests {
    use super::{pos_to_world, world_to_pos, TilePosition, TileSettings};

    /// We should should be able to convert from tile-space to
    /// world-spaceand back again and still have the same output.
    #[test]
    fn pos_test() {
        let settings = TileSettings {
            tile_size: 1.0,
            height_layers: 1,
            map_size: (1, 1),
        };
        let pos = TilePosition {
            x: 0,
            y: 0,
            height: 0,
        };
        let translation = pos_to_world(&pos, &settings);
        assert_eq!(world_to_pos(&translation, &settings), pos);
    }
}
