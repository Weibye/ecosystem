use bevy::prelude::{
    default, info, shape, App, Assets, Color, Commands, Mesh, PbrBundle, Plugin, Query, Res,
    ResMut, StandardMaterial, Transform,
};
use bevy_mod_picking::PickableBundle;
use bevy_turborand::{DelegatedRng, GlobalRng, TurboRand};
use bracket_pathfinding::prelude::Algorithm2D;

use crate::{
    map::{
        map::Map,
        tiles::{TileData, TilePos, TileType},
    },
    resource::{FoodSource, WaterSource},
    utils::lerp_range,
    AppStage,
};

pub(crate) struct FloraPlugin;

// Plants grow and become food. The more they grow, the more food they contain.
impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(AppStage::SpawnFlora, spawn_food)
            .add_startup_system_to_stage(AppStage::SpawnFlora, spawn_water);
    }
}

const FOOD_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
const WATER_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);

/// Spawns food on all grass tiles
fn spawn_food(
    map: Res<Map>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    info!("spawning food");
    for n in 0..map.tiles.len() {
        if map.tiles[n] == TileType::Grass {
            let rand = lerp_range(rng.get_mut().f32(), 0.0..100.0);
            if rand <= 50.0 {
                continue;
            }

            cmd.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(FOOD_COLOR.into()),
                    transform: Transform::from_translation(map.index_to_world(n)),
                    ..default()
                },
                FoodSource { content: rand },
                // This should get a tileposition component as well
                TilePos::from_point(map.index_to_point2d(n)),
                PickableBundle::default(),
            ));
        }
    }
}

fn spawn_water(
    map: Res<Map>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("spawning water");
    for n in 0..map.tiles.len() {
        if map.tiles[n] == TileType::Water {
            cmd.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(WATER_COLOR.into()),
                    transform: Transform::from_translation(map.index_to_world(n)),
                    ..default()
                },
                WaterSource { content: 100.0 },
                // This should get a tileposition component as well
                TilePos::from_point(map.index_to_point2d(n)),
                PickableBundle::default(),
            ));
        }
    }
}
