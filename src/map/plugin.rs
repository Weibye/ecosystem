use bevy::prelude::{
    default, shape, App, Assets, Commands, IntoSystemConfig, Mesh, PbrBundle, Plugin, Res, ResMut,
    Resource, StandardMaterial, Transform,
};
use bevy_mod_picking::PickableBundle;
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};

use crate::WorldCreationSet;

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
        .add_system(seed_map.in_set(WorldCreationSet::SeedMap))
        .add_system(spawn_map.in_set(WorldCreationSet::SpawnMap));
    }
}

// 1. Create the map
fn seed_map(mut cmd: Commands, settings: Res<MapSettings>, mut rng: ResMut<GlobalRng>) {
    cmd.insert_resource(generate_map(&settings, &mut rng));
}

pub(crate) fn generate_map(settings: &MapSettings, rng: &mut GlobalRng) -> Map {
    let mut tiles: Vec<TileType> = vec![];

    for _ in 0..settings.width {
        for _ in 0..settings.height {
            let index = rng.get_mut().i32(0..=13);
            tiles.push(match index {
                0..=5 => TileType::Grass,
                6..=8 => TileType::Dirt,
                9 => TileType::Rock,
                10..=12 => TileType::Water,
                13 => TileType::Lava,
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
                    subdivisions: 0,
                })),
                material: materials.add(get_color(map.tiles[index]).into()),
                transform: Transform::from_translation(map.index_to_world(index.into())),
                ..default()
            },
            PickableBundle::default(),
        ));
    }
}
