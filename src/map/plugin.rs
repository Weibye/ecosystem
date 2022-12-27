use bevy::prelude::{
    default, shape, App, Assets, Commands, Mesh, PbrBundle, Plugin, Res, ResMut, Resource,
    StandardMaterial, Transform,
};
use bevy_mod_picking::PickableBundle;
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};

use crate::AppStage;

use super::{
    tiles::{get_color, TileType},
    Map,
};

#[derive(Resource, Clone)]
pub(crate) struct MapSettings {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) tile_size: f32,
}

pub(crate) struct MapPlugin {
    pub(crate) tile_size: f32,
    pub(crate) map_size: (i32, i32),
}

impl Default for MapPlugin {
    fn default() -> Self {
        Self {
            tile_size: 1.0,
            map_size: (16, 16),
        }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapSettings {
            tile_size: self.tile_size,
            width: self.map_size.0,
            height: self.map_size.1,
        })
        .add_startup_system_to_stage(AppStage::SeedMap, seed_map)
        .add_startup_system_to_stage(AppStage::SpawnMap, spawn_map);
    }
}

// 1. Create the map
fn seed_map(mut cmd: Commands, settings: Res<MapSettings>, mut rng: ResMut<GlobalRng>) {
    cmd.insert_resource(generate_map(&settings, &mut rng));
}

fn generate_map(settings: &MapSettings, rng: &mut GlobalRng) -> Map {
    let mut tiles: Vec<TileType> = vec![];

    for _ in 0..settings.width {
        for _ in 0..settings.height {
            let index = rng.get_mut().i32(0..=5);
            tiles.push(match index {
                0..=2 => TileType::Grass,
                3 => TileType::Rock,
                4 => TileType::Water,
                5 => TileType::Lava,
                _ => panic!("Out of range"),
            });
        }
    }

    Map {
        settings: settings.clone(),
        tiles,
    }
}

fn spawn_map(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<Map>,
) {
    for index in 0..map.tiles.len() {
        cmd.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane {
                    size: map.settings.tile_size,
                })),
                material: materials.add(get_color(map.tiles[index]).into()),
                transform: Transform::from_translation(map.index_to_world(index)),
                ..default()
            },
            PickableBundle::default(),
        ));
    }
}
