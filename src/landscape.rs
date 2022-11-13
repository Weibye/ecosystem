use bevy::prelude::{
    default, shape, Assets, Color, Commands, Component, Mesh, PbrBundle, Plugin, Res, ResMut,
    Resource, StandardMaterial, Transform, Vec3,
};
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};

#[derive(Component, Copy, Clone, Debug)]
pub(crate) struct TileData {
    position: TilePosition,
    ground_type: GroundType,
}

#[derive(Copy, Clone, Debug)]
struct TilePosition {
    x: i8,
    y: i8,
    height: i8,
}

#[derive(Copy, Clone, Debug)]
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
struct TileSettings {
    tile_size: f32,
    map_size: (i8, i8),
}

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TileSettings {
            tile_size: self.tile_size,
            map_size: self.map_size,
        })
        .add_startup_system(create_tiles);
    }
}

fn create_tiles(
    mut cmd: Commands,
    settings: Res<TileSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    let mut logical_tiles: Vec<TileData> = Vec::new();

    for x in 0..settings.map_size.0 {
        for y in 0..settings.map_size.1 {
            let index = rng.get_mut().i32(0..=4);
            let ground_type = match index {
                0..=2 => GroundType::Grass,
                3 => GroundType::Rock,
                4 => GroundType::Water,
                _ => panic!("Out of range"),
            };

            logical_tiles.push(TileData {
                position: TilePosition { x, y, height: 0 },
                ground_type,
            });
        }
    }

    // Spawn tile data
    for e in &logical_tiles {
        cmd.spawn(e.clone());
    }

    // Spawn visual tiles
    for e in &logical_tiles {
        cmd.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: settings.tile_size,
            })),
            material: materials.add(get_color(e.ground_type).into()),
            transform: Transform::from_translation(pos_to_world(&e.position, &settings)),
            ..default()
        });
    }
}

/// Converts from a tile-position to a world-position.
fn pos_to_world(pos: &TilePosition, settings: &TileSettings) -> Vec3 {
    Vec3::new(
        pos.x as f32 * settings.tile_size - settings.map_size.0 as f32 / 2.0,
        pos.height as f32,
        pos.y as f32 * settings.tile_size - settings.map_size.1 as f32 / 2.0,
    )
}

// fn world_to_pos(pos: &Vec3, settings: &TileSettings) -> TilePosition {
//     TilePosition { x: (), y: (), height: () }
// }

/// Gets the corresponding material color for a `GroundType`.
/// TODO: Replace with actual textures and assets.
fn get_color(ground_type: GroundType) -> Color {
    match ground_type {
        GroundType::Grass => Color::rgb(0.1, 0.7, 0.25),
        GroundType::Rock => Color::rgb(0.4, 0.45, 0.4),
        GroundType::Water => Color::rgb(0.0, 0.4, 0.6),
    }
}
