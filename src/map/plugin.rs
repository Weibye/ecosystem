use std::collections::HashMap;

use bevy::prelude::{
    default, shape, App, Assets, Commands, Mesh, PbrBundle, Plugin, Res, ResMut, Resource,
    StandardMaterial, Transform,
};
use bevy_mod_picking::PickableBundle;
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};
use bracket_pathfinding::prelude::Point;
use noise::{NoiseFn, Perlin};

use crate::AppStage;

use super::{
    tiles::{get_color, get_data, resolve_type},
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
    let seed = rng.get_mut().u32(0..10_000);
    cmd.insert_resource(generate_map(&settings, seed));
}

const SCALE: f64 = 3.5;

/// Generates the map data
pub(crate) fn generate_map(settings: &MapSettings, seed: u32) -> Map {
    // Get a
    let noise = Perlin::new(seed);

    let mut index: usize = 0;
    let mut indexes = HashMap::new();
    let mut data = HashMap::new();

    for x in 0..settings.width {
        for y in 0..settings.height {
            let point = Point::new(x, y);
            indexes.insert(index, point);

            // Output is -1..=1
            // Sampling must be done withing 0..=1 on X and Y
            let noise_value = noise.get([
                x as f64 * SCALE / settings.width as f64,
                y as f64 * SCALE / settings.height as f64,
            ]);

            let tile_type = resolve_type(noise_value);

            let tile_data = get_data(&tile_type);
            data.insert(index, tile_data);
            index += 1;
        }
    }

    Map {
        settings: settings.clone(),
        indexes,
        data,
    }
}

fn spawn_map(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<Map>,
) {
    for index in 0..map.indexes.len() {
        cmd.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane {
                    size: map.settings.tile_size,
                })),
                material: materials.add(get_color(map.data[&index].tile_type).into()),
                transform: Transform::from_translation(map.index_to_world(index.into())),
                ..default()
            },
            PickableBundle::default(),
        ));
    }
}
